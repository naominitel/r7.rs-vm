use common::bytecode;
use common::bytecode::base;
use common::bytecode::off;
use common::bytecode::Opcode;
use common::bytecode::Type;
use gc::Closure;
use gc::Env;
use gc::GC;
use gc::value;
use primitives;
use std::hashmap::HashMap;
use std::io::Reader;
use std::num::FromPrimitive;
use vm::Frame;
use vm::Library;
use vm::library::LibName;
use vm::Stack;
use vm::symbols::SymTable;

struct VM {
    frame: ~Frame,
    stack: Stack,
    gc: ~GC,

    sym_table: ~SymTable,
    loaded_mods: HashMap<LibName, uint>,
    modules: ~[~Library]
}

impl Reader for VM {
    fn read(&mut self, buf: &mut [u8]) -> Option<uint> {
        let mut ret = 0;

        for i in range(0, buf.len()) {
            if(!self.eof()) {
                let b = self.read_u8();
                buf[i] = b;
                ret += 1;
            }

            else {
                break;
            }
        }

        match ret {
            0 => None,
            i => Some(i)
        }
    }

    fn read_u8(&mut self) -> u8 {
        let base = base(self.frame.pc);
        let lib = &self.modules[base];

        let b = lib.prog[off(self.frame.pc)];
        self.frame.pc += 1;
        b
    }

    fn eof(&mut self) -> bool {
        let base = base(self.frame.pc);
        self.modules[base].prog.len() == off(self.frame.pc) as uint
    }
}

impl VM {
    pub fn new() -> ~VM {
        let stack = ~[];

        let mut gc = GC::new();
        let env = primitives::env(gc);
        let frame = Frame::new(env, 0, 0);
        let loaded_mods = HashMap::new();
        let symtable = SymTable::new();
        let mods = ~[];

        ~VM { frame: frame, stack: stack, gc: gc, loaded_mods: loaded_mods,
            sym_table: symtable, modules: mods }
    }

    #[inline(always)]
    fn next_op(&mut self) -> Opcode {
        let opcode = self.read_u8();
        unsafe { ::std::cast::transmute(opcode) }
    }

    #[inline(always)]
    fn next_ty(&mut self) -> Type {
        let ty = self.read_u8();
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

    pub fn run(&mut self, prog: ~str) {
        let p = Path::new(prog);
        let name = ~[~"main"];
        let l = Library::load_file(self.gc, self.sym_table, &p, ~LibName(name));
        self.load_module(l);
    }

    fn load_module(&mut self, lib: ~Library) {
        let mut env = Some(primitives::env(self.gc));

        for i in lib.imports.iter() {
            debug!("Require lib");
            let m = self.loaded_mods.find_copy(&**i);

            let l = if m == None {
                let l = Library::load(self.gc, self.sym_table, *i,
                    Library::library_path(None));
                self.load_module(l);
                self.modules.last()
            }

            else {
                &self.modules[m.unwrap()]
            };

            let nenv = self.gc.alloc_env(l.exports, env);
            let mut i = 0;

            unsafe {            
                for &(_, ref e) in (*l.env).values.iter() {
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

        self.sym_table.dump();
    }

    // Returns an environment containings the arguments of a closure,
    // taken on the stack
    #[inline(always)]
    fn get_args_env(&mut self, argc: u8, cl: Closure) -> Env {
        let arity = cl.arity();
        let variadic = cl.variadic();

        if variadic {
            if argc < arity {
                fail!("Wrong number of arguments");
            }

            let env = self.gc.alloc_env((arity + 1) as u64, Some(cl.env()));

            let va_count = argc - arity;
            let va_args = self.prim_call(primitives::list, va_count);

            let base = self.stack.len() - arity as uint;

            for i in range(0, arity) {
                let arg = self.stack[base + i as uint].clone();
                unsafe { (*env).values.push((true, arg)); }
            }

            // remove arguments from the stack
            self.stack.truncate(base);

            unsafe { (*env).values.push((true, va_args)); }
            env
        }

        else {
            if argc != arity {
                fail!("Wrong number of arguments");
            }

            let env = self.gc.alloc_env(argc as u64, Some(cl.env()));
            let base = self.stack.len() - argc as uint;

            for i in range(0, argc) {
                let arg = self.stack[base + i as uint].clone();
                unsafe { (*env).values.push((true, arg)); }
            }

            // remove arguments from the stack
            self.stack.truncate(base);

            env
        }
    }

    #[inline(always)]
    fn closure_call(&mut self, cl: Closure, argc: u8) {
        let env = self.get_args_env(argc, cl);
        self.push_frame(cl.pc(), env);
    }

    #[inline(always)]
    fn prim_call(&mut self, prim: primitives::Prim, argc: u8) -> value::Value {
        let ret = prim(primitives::Arguments::new(self, argc));
        let len = self.stack.len() - argc as uint;
        self.stack.truncate(len);
        ret
    }

    #[inline(always)]
    pub fn fun_call(&mut self, fun: &value::Value, argc: u8) {
        match fun {
            &value::Closure(cl) => self.closure_call(cl, argc),
            &value::Primitive(prim, _) => {
                let ret = self.prim_call(prim, argc);
                self.stack.push(ret);
            }
            _ => fail!("Attempting to call a non-function value")
        }
    }

    #[inline(always)]
    pub fn fun_call_ret(&mut self, fun: &value::Value, argc: u8) -> value::Value {
        match fun {
            &value::Primitive(prim, _) => self.prim_call(prim, argc),

            &value::Closure(cl) => {
                #[inline(always)]
                fn get_cur_frame(v: &mut VM) -> *Frame {
                    let f: &Frame = v.frame; f as *Frame
                }

                let caller: *Frame = get_cur_frame(self);
                self.closure_call(cl, argc);
                let mut cur_frame = get_cur_frame(self);

                // exec until frame returns
                // FIXME: I don't know how this is going to behave
                // when we will have continuation...
                while cur_frame != caller {
                    self.exec_instr();
                    cur_frame = get_cur_frame(self);
                }

                self.stack.pop()
            }

            _ => fail!("Attempting to call a non-function value")
        }
    }

    fn exec_instr(&mut self) {
        let opcode = self.next_op();
        debug!("Executing next instruction: {:?}", opcode);

        match opcode {
            bytecode::Alloc => {
                let envsize = self.read_be_u64();
                self.frame.alloc(self.gc, envsize);
            }

            bytecode::Store => {
                let addr = self.read_be_u64();
                let value = self.stack.pop();
                self.frame.store(&value, addr);
            }

            bytecode::Fetch => {
                let addr = self.read_be_u64();
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
                        let i = self.read_u8();
                        match i {
                            0 => value::Bool(false),
                            _ => value::Bool(true)
                        }
                    }

                    bytecode::Int => {
                        let i = self.read_be_i64();
                        value::Num(FromPrimitive::from_i64(i).unwrap())
                    }

                    bytecode::Sym => {
                        let base = base(self.frame.pc);
                        let arg = self.read_be_u64();
                        let lib = &self.modules[base];
                        let h = lib.sym_table[arg];
                        value::Symbol(h)
                    }

                    bytecode::Fun => {
                        // closure
                        let arg = self.read_be_u32();
                        let arity = self.read_u8();
                        let variadic = (self.read_u8() != 0x00);
                        let base = self.frame.pc & 0xFFFF0000;
                        let clpc = (arg as u64) | base;
                        let env = self.frame.env;

                        value::Closure(self.gc.alloc_closure(
                            arity, variadic, env, clpc))
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
                let argc = self.read_u8();
                self.fun_call(&fval, argc);
            }

            bytecode::Tcall => {
                // last call optimization
                // call a closure without allocating a frame
                let fval = self.stack.pop();
                let argc = self.read_u8();

                match fval {
                    value::Closure(cl) => {
                        let env = self.get_args_env(argc, cl);

                        // do not allocate a frame to prevent O(n) memory
                        // usage for recursive last calls. The current env
                        // of the frame will be collected if it is not still
                        // captured by a visible closure
                        self.frame.sp = self.stack.len();
                        self.frame.pc = cl.pc();
                        self.frame.env = env;
                    }

                    // the compiler doesn't make the difference between
                    // a closure and a primitive, so a tail-call to a primitive
                    // may happen here
                    value::Primitive(prim, _) => {
                        let ret = self.prim_call(prim, argc);
                        self.stack.push(ret);
                    }

                    _ => fail!("Attempting a tail-call on a non-closure value")
                }
            }

            bytecode::Jump => {
                let dst = self.read_be_u32();
                self.frame.pc = self.frame.pc & 0x00000000;
                self.frame.pc = self.frame.pc | (dst as u64);
            }

            bytecode::Branch => {
                let dst = self.read_be_u32();
                let expr = self.stack.pop();

                match expr {
                    value::Bool(false) => {
                        self.frame.pc = self.frame.pc & 0x00000000;
                        self.frame.pc = self.frame.pc | (dst as u64);
                    }

                    _ => ()
                }
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

            bytecode::Nop => (),

            // FIXME: this pattern is not *really* exhaustive because of the
            // unsafe u8 -> enum cast
            // _ => {
            //     fail!("Unkwown bytecode instruction {:u}", opcode as u8);
            // }
        }
    }

    fn exec_module(&mut self) {
        use gc::visit::Visitor;

        debug!("Begin module execution");
        let prog_len = self.modules.last().prog.len();
        debug!("Module section is {:u} long", prog_len);
        let mut counter = 0;

        while (off(self.frame.pc) as uint) < prog_len {
            self.exec_instr();
            counter += 1;

            if counter == 2000 {
                // garbage-collect
                let visitors = &mut [&mut self.stack as &mut Visitor,
                    &mut *self.frame as &mut Visitor];
                self.gc.sweep(visitors);
                counter = 0;
            }
        }
    }
}
