use common::bytecode;
use common::bytecode::base;
use common::bytecode::off;
use common::bytecode::Opcode;
use common::bytecode::Type;
use gc;
use gc::GC;
use gc::Ptr;
use gc::value;
use primitives;
use std::collections::hashmap::HashMap;
use std::io::Reader;
use std::num::FromPrimitive;
use vm::frame::Frame;
use vm::library::Library;
use vm::library::LibName;
use vm::stack::Stack;

pub struct VM {
    pub frame: Box<Frame>,
    pub stack: Stack,
    pub gc: Box<GC>,

    pub loaded_mods: HashMap<LibName, uint>,
    pub modules: Vec<Box<Library>>
}

fn extend_sign(val: u64, nbytes: uint) -> i64 {
    let shift = (8 - nbytes) * 8;
    (val << shift) as i64 >> shift
}

impl VM {
    pub fn new() -> Box<VM> {
        let stack = vec!();

        let mut gc = GC::new();
        let env = primitives::env(&mut *gc);
        let frame = Frame::new(env, 0, 0);
        let loaded_mods = HashMap::new();
        let mods = vec!();

        box VM { frame: frame, stack: stack, gc: gc, loaded_mods: loaded_mods,
            modules: mods }
    }

    #[inline(always)]
    fn next_op(&mut self) -> Opcode {
        let opcode = self.read_u8();
        unsafe { ::std::mem::transmute(opcode) }
    }

    #[inline(always)]
    fn next_ty(&mut self) -> Type {
        let ty = self.read_u8();
        unsafe { ::std::mem::transmute(ty) }
    }

    fn push_frame(&mut self, pc: u64, env: Ptr<gc::Env>) {
        let mut frame = Frame::new(env, self.stack.len(), pc);
        let mut nframe = Frame::new(env, 0, 0);

        ::std::mem::swap(&mut self.frame, &mut nframe);
        frame.caller = Some(nframe);

        ::std::mem::swap(&mut self.frame, &mut frame);
    }

    fn pop_frame(&mut self) {
        let mut nframe = Frame::new(Ptr(0 as *mut gc::ptr::Cell<gc::Env>), 0, 0);

        match self.frame.caller {
            Some(ref mut f) => {
                ::std::mem::swap(f, &mut nframe);
            }

            None => panic!()
        }

        ::std::mem::swap(&mut self.frame, &mut nframe);
    }

    #[allow(dead_code)]
    fn eof(&mut self) -> bool {
        let base = base(self.frame.pc);
        self.modules[base as uint].prog.len() == off(self.frame.pc) as uint
    }

    #[allow(dead_code)]
    fn read_u8(&mut self) -> u8 {
        let base = base(self.frame.pc);
        let lib = &self.modules[base as uint];

        let b = lib.prog[off(self.frame.pc) as uint];
        self.frame.pc += 1;
        b
    }

    #[allow(dead_code)]
    fn read_le_uint_n(&mut self, nbytes: uint) -> u64 {
        assert!(nbytes > 0 && nbytes <= 8);

        let mut val = 0u64;
        let mut pos = 0;
        let mut i = nbytes;
        while i > 0 {
            val += (self.read_u8() as u64) << pos;
            pos += 8;
            i -= 1;
        }
        val
    }

    #[allow(dead_code)]
    fn read_le_int_n(&mut self, nbytes: uint) -> i64 {
        extend_sign(self.read_le_uint_n(nbytes), nbytes)
    }

    #[allow(dead_code)]
    fn read_be_uint_n(&mut self, nbytes: uint) -> u64 {
        assert!(nbytes > 0 && nbytes <= 8);

        let mut val = 0u64;
        let mut i = nbytes;
        while i > 0 {
            i -= 1;
            val += (self.read_u8() as u64) << i * 8;
        }
        val
    }

    #[allow(dead_code)]
    fn read_be_int_n(&mut self, nbytes: uint) -> i64 {
        extend_sign(self.read_be_uint_n(nbytes), nbytes)
    }

    #[allow(dead_code)]
    fn read_be_u64(&mut self) -> u64 {
        self.read_be_uint_n(8)
    }

    #[allow(dead_code)]
    fn read_be_u32(&mut self) -> u32 {
        self.read_be_uint_n(4) as u32
    }

    #[allow(dead_code)]
    fn read_be_u16(&mut self) -> u16 {
        self.read_be_uint_n(2) as u16
    }

    #[allow(dead_code)]
    fn read_be_i64(&mut self) -> i64 {
        self.read_be_int_n(8)
    }

    #[allow(dead_code)]
    fn read_be_i32(&mut self) -> i32 {
        self.read_be_int_n(4) as i32
    }

    #[allow(dead_code)]
    fn read_be_i16(&mut self) -> i16 {
        self.read_be_int_n(2) as i16
    }

    #[allow(dead_code)]
    fn read_le_u64(&mut self) -> u64 {
        self.read_le_uint_n(8)
    }

    #[allow(dead_code)]
    fn read_le_u32(&mut self) -> u32 {
        self.read_le_uint_n(4) as u32
    }

    #[allow(dead_code)]
    fn read_le_u16(&mut self) -> u16 {
        self.read_le_uint_n(2) as u16
    }

    #[allow(dead_code)]
    fn read_le_i64(&mut self) -> i64 {
        self.read_le_int_n(8)
    }

    #[allow(dead_code)]
    fn read_le_i32(&mut self) -> i32 {
        self.read_le_int_n(4) as i32
    }

    #[allow(dead_code)]
    fn read_le_i16(&mut self) -> i16 {
        self.read_le_int_n(2) as i16
    }

    pub fn run(&mut self, prog: &str) {
        let p = Path::new(prog);
        let name = vec!(String::from_str("main"));
        let l = Library::load_file(&mut *self.gc, &p, box LibName(name));
        self.load_module(l);
    }

    fn load_module(&mut self, lib: Box<Library>) {
        let mut env = Some(primitives::env(&mut *self.gc));

        for i in lib.imports.iter() {
            debug!("Require lib");
            let m = self.loaded_mods.find_copy(&**i);

            let l = if m == None {
                let l = Library::load(&mut *self.gc, &**i, Library::library_path(None));
                self.load_module(l);
                &**self.modules.last().unwrap()
            }

            else {
                &*self.modules[m.unwrap()]
            };

            let mut nenv = self.gc.alloc(gc::Env {
                values: Vec::with_capacity(l.exports as uint),
                next: env
            });

            let mut i = 0;

            // FIXME: why is this neccessary?
            let mut e = l.env;

            for &(_, ref e) in e.values.iter() {
                nenv.store(e, i);
                i += 1;
            }

            env = Some(nenv);
        }

        let mut nenv = lib.env;
        nenv.next = env;

        let idx = self.modules.len();
        self.loaded_mods.insert(*lib.name.clone(), idx);
        self.modules.push(lib);
        let base = (idx as u64) << 32;

        // load initial frame
        self.stack = vec!();
        self.frame.env = nenv;
        self.frame.pc = base;
        self.frame.sp = 0;
        self.frame.caller = None;

        // exec module
        self.exec_module();
    }

    // Returns an environment containings the arguments of a closure,
    // taken on the stack
    #[inline(always)]
    fn get_args_env(&mut self, argc: u8, cl: Ptr<gc::Closure>) -> Ptr<gc::Env> {
        let arity = cl.arity;
        let variadic = cl.variadic;

        if variadic {
            if argc < arity {
                panic!("Wrong number of arguments");
            }

            let mut env = self.gc.alloc(gc::Env {
                values: Vec::with_capacity(arity as uint + 1),
                next: Some(cl.env)
            });

            let va_count = argc - arity;
            let va_args = self.prim_call(primitives::list, va_count);

            let base = self.stack.len() - arity as uint;

            for i in range(0, arity) {
                let arg = self.stack[base + i as uint].clone();
                env.values.push((true, arg));
            }

            // remove arguments from the stack
            self.stack.truncate(base);

            env.values.push((true, va_args));
            env
        }

        else {
            if argc != arity {
                panic!("Wrong number of arguments");
            }

            let mut env = self.gc.alloc(gc::Env {
                values: Vec::with_capacity(argc as uint),
                next: Some(cl.env)
            });

            let base = self.stack.len() - argc as uint;

            for i in range(0, argc) {
                let arg = self.stack[base + i as uint].clone();
                env.values.push((true, arg));
            }

            // remove arguments from the stack
            self.stack.truncate(base);

            env
        }
    }

    #[inline(always)]
    fn closure_call(&mut self, cl: Ptr<gc::Closure>, argc: u8) {
        let env = self.get_args_env(argc, cl);
        self.push_frame(cl.pc, env);
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
            _ => panic!("Attempting to call a non-function value")
        }
    }

    #[inline(always)]
    pub fn fun_call_ret(&mut self, fun: &value::Value, argc: u8) -> value::Value {
        match fun {
            &value::Primitive(prim, _) => self.prim_call(prim, argc),

            &value::Closure(cl) => {
                #[inline(always)]
                fn get_cur_frame(v: &mut VM) -> *const Frame {
                    let f: &Frame = &*v.frame; f as *const Frame
                }

                let caller: *const Frame = get_cur_frame(self);
                self.closure_call(cl, argc);
                let mut cur_frame = get_cur_frame(self);

                // exec until frame returns
                // FIXME: I don't know how this is going to behave
                // when we will have continuation...
                while cur_frame != caller {
                    self.exec_instr();
                    cur_frame = get_cur_frame(self);
                }

                self.stack.pop().unwrap()
            }

            _ => panic!("Attempting to call a non-function value")
        }
    }

    fn exec_instr(&mut self) {
        let opcode = self.next_op();
        debug!("Executing next instruction: {}", opcode);

        match opcode {
            bytecode::Alloc => {
                let envsize = self.read_be_u64();
                self.frame.alloc(&mut *self.gc, envsize);
            }

            bytecode::Store => {
                let addr = self.read_be_u64();
                let value = self.stack.pop().unwrap();
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
                        let lib = &self.modules[base as uint];
                        let h = lib.sym_table[arg as uint];
                        value::Symbol(h)
                    }

                    bytecode::Fun => {
                        // closure
                        let arg = self.read_be_u32();
                        let arity = self.read_u8();
                        let variadic = self.read_u8() != 0x00;
                        let base = self.frame.pc & 0xFFFF0000;
                        let clpc = (arg as u64) | base;
                        let env = self.frame.env;

                        value::Closure(self.gc.alloc(
                            gc::Closure {
                                arity: arity,
                                variadic: variadic,
                                env: env,
                                pc: clpc
                            }
                        ))
                    }

                    bytecode::Prim => {
                        panic!("Unimplemented");
                    }
                };

                self.stack.push(val);
            }

            bytecode::Pop => {
                self.stack.pop();
            }

            bytecode::Call => {
                let fval = self.stack.pop().unwrap();
                let argc = self.read_u8();
                self.fun_call(&fval, argc);
            }

            bytecode::Tcall => {
                // last call optimization
                // call a closure without allocating a frame
                let fval = self.stack.pop().unwrap();
                let argc = self.read_u8();

                match fval {
                    value::Closure(cl) => {
                        let env = self.get_args_env(argc, cl);

                        // do not allocate a frame to prevent O(n) memory
                        // usage for recursive last calls. The current env
                        // of the frame will be collected if it is not still
                        // captured by a visible closure
                        self.frame.sp = self.stack.len();
                        self.frame.pc = cl.pc;
                        self.frame.env = env;
                    }

                    // the compiler doesn't make the difference between
                    // a closure and a primitive, so a tail-call to a primitive
                    // may happen here
                    value::Primitive(prim, _) => {
                        let ret = self.prim_call(prim, argc);
                        self.stack.push(ret);
                    }

                    _ => panic!("Attempting a tail-call on a non-closure value")
                }
            }

            bytecode::Jump => {
                let dst = self.read_be_u32();
                self.frame.pc = self.frame.pc & 0x00000000;
                self.frame.pc = self.frame.pc | (dst as u64);
            }

            bytecode::Branch => {
                let dst = self.read_be_u32();
                let expr = self.stack.pop().unwrap();

                match expr {
                    value::Bool(false) => {
                        self.frame.pc = self.frame.pc & 0x00000000;
                        self.frame.pc = self.frame.pc | (dst as u64);
                    }

                    _ => ()
                }
            }

            bytecode::Return => {
                let ret = self.stack.pop().unwrap();

                // unwind stack used by the function
                while self.stack.len() > self.frame.sp {
                    self.stack.pop().unwrap();
                }

                self.stack.push(ret);
                self.pop_frame();
            }

            bytecode::Nop => (),

            // FIXME: this pattern is not *really* exhaustive because of the
            // unsafe u8 -> enum cast
            // _ => {
            //     panic!("Unkwown bytecode instruction {:u}", opcode as u8);
            // }
        }
    }

    fn exec_module(&mut self) {
        use gc::visit::Visitor;

        debug!("Begin module execution");
        let prog_len = self.modules.last().unwrap().prog.len();
        debug!("Module section is {:u} long", prog_len);
        let mut counter = 0u16;

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
