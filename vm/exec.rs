use common::bytecode;
use common::bytecode::base;
use common::bytecode::off;
use common::bytecode::Opcode;
use common::bytecode::Read;
use common::bytecode::Type;
use gc::Env;
use gc::GC;
use gc::value;
use std::hashmap::HashMap;
use vm::Frame;
use vm::Library;
use vm::library::LibName;
use vm::Stack;
use vm::primitive::primEnv;

struct VM {
    frame: ~Frame,
    stack: Stack,
    gc: ~GC,

    loaded_mods: HashMap<LibName, uint>,
    modules: ~[~Library]
}

impl VM {
    pub fn new() -> ~VM {
        let stack = ~[];

        // FIXME: add primitive env here
        let mut gc = GC::new();
        let env = primEnv(gc);
        let frame = Frame::new(env, 0, 0);
        let loaded_mods = HashMap::new();
        let mods = ~[];

        ~VM { frame: frame, stack: stack, gc: gc, loaded_mods: loaded_mods,
            modules: mods }
    }

    fn read<T: Read>(&mut self) -> T {
        let base = base(self.frame.pc);
        let lib = &self.modules[base];

        do Read::read { 
            let b = lib.prog[off(self.frame.pc)];
            self.frame.pc += 1;
            b
        }
    }

    fn next_op(&mut self) -> Opcode {
        let opcode: u8 = self.read();
        unsafe { ::std::cast::transmute(opcode) }
    }

    fn next_ty(&mut self) -> Type {
        let ty: u8 = self.read();
        unsafe { ::std::cast::transmute(ty) }
    }

    fn push_frame(&mut self, pc: u64, env: Env) {
        let mut frame = Frame::new(env, self.stack.len(), pc);
        let mut nframe = Frame::new(env, 0, 0);

        ::std::util::swap(&mut self.frame, &mut nframe);
        frame.caller = Some(nframe);

        ::std::util::swap(&mut self.frame, &mut frame);
    }

    fn pop_frame(&mut self) {
        let mut nframe = Frame::new((0 as Env), 0, 0);

        match self.frame.caller {
            Some(ref mut f) => {
                ::std::util::swap(f, &mut nframe);
            }

            None => fail!()
        }

        ::std::util::swap(&mut self.frame, &mut nframe);
    }

    pub fn run(&mut self, lib: ~LibName) {
        let l = Library::load(self.gc, &*lib, Library::library_path(None));
        self.load_module(l);
    }

    pub fn run_file(&mut self, prog: ~str) {
        let p = Path::new(prog);
        let name = ~[~"main"];
        let l = Library::load_file(self.gc, &p, ~LibName(name));
        self.load_module(l);
    }

    fn load_module(&mut self, lib: ~Library) {
        let mut env = Some(primEnv(self.gc));
        for i in lib.imports.iter() {
            let m = self.loaded_mods.find_copy(&**i);

            let l = if m == None {
                let l = Library::load(self.gc, *i, Library::library_path(None));
                self.load_module(l);
                self.modules.last()
            }

            else {
                &self.modules[m.unwrap()]
            };

            let nenv = self.gc.alloc_env(l.exports, env);
            let mut i = 0;

            unsafe {            
                for e in (*l.env).values.iter() {
                    (*nenv).store(e, i);
                    i += 1;
                }
            }

            env = Some(nenv);
        } 

        let nenv = lib.env;
        unsafe { (*nenv).next = env; }

        let idx = self.modules.len();
        self.loaded_mods.insert(*lib.name.clone(), idx);
        self.modules.push(lib);
        let base = (idx as u64) << 32;

        // load initial frame
        self.stack = ~[];
        self.frame.env = nenv;
        self.frame.pc = base;
        self.frame.sp = 0;
        self.frame.caller = None;

        // exec module
        self.exec_module();
    }

    fn exec_instr(&mut self) {
        let opcode = self.next_op();
        println!("Executing next instruction: {:?}", opcode);

        match opcode {
            bytecode::Alloc => {
                let envsize = self.read();
                self.frame.alloc(self.gc, envsize);
            }

            // bytecode::Delete => {
                // TODO: implement env delete
            // }

            // stack to env primitives

            bytecode::Store => {
                let addr: u64 = self.read();
                let value = self.stack.pop();
                self.frame.store(&value, addr);
            }

            bytecode::Fetch => {
                let addr: u64 = self.read();
                let value = self.frame.fetch(addr);
                self.stack.push(value);
            }

            bytecode::Push => {
                let ty = self.next_ty();

                let val = match ty {
                    bytecode::Unit => {
                        value::Unit
                    } 

                    bytecode::Bool => {
                        fail!("Unimplemented")
                    }

                    bytecode::Int => {
                        let i: i64 = self.read();
                        value::Num(i)
                    }

                    bytecode::Sym => {
                        fail!("Unimplemented")
                    }

                    bytecode::Fun => {
                        // closure
                        let arg: u32 = self.read();
                        let base = self.frame.pc & 0xFFFF0000;
                        let clpc = (arg as u64) | base;
                        let env = self.frame.env;
                        value::Closure(clpc, env)
                    }

                    bytecode::Prim => {
                        fail!("Unimplemented");
                    }
                };

                self.stack.push(val);
            }

            bytecode::Pop => {
                self.stack.pop();
            }

            bytecode::Call => {
                let fval = self.stack.pop();
                let argc: u8 = self.read();

                match fval {
                    value::Closure(pc, env) => {
                        let env = self.gc.alloc_env(argc as u64, Some(env));

                        for _ in range(0, argc) {
                            let arg = self.stack.pop();
                            unsafe { (*env).values.push(arg); }
                        }

                        self.push_frame(pc, env);
                    }

                    value::Primitive(prim) => {
                        let mut args = ~[];

                        for _ in range(0, argc) {
                            let arg = self.stack.pop();
                            args.push(arg);
                        }

                        let ret = prim(args);
                        self.stack.push(ret);
                    }

                    _ => fail!("Attempting to call a non-function value")
                }
            }

            bytecode::Jump => {
                let dst: u32 = self.read();
                self.frame.pc = self.frame.pc & 0x00000000;
                self.frame.pc = self.frame.pc | (dst as u64);
            }

            bytecode::Return => {
                let ret = self.stack.pop();

                // unwind stack used by the function
                while self.stack.len() > self.frame.sp {
                    self.stack.pop();
                }

                self.stack.push(ret);
                self.pop_frame();
            }

            _ => {
                fail!("Unkwown bytecode instruction {:u}", opcode as u8);
            }
        }
    }

    fn exec_module(&mut self) {
        println!("Begin module execution");
        let prog_len = self.modules.last().prog.len();
        println!("Module section is {:u} long", prog_len);

        while (off(self.frame.pc) as uint) < prog_len {
            self.exec_instr();
        }
    }
}