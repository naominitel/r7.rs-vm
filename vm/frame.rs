use gc::Env;
use gc::GC;
use gc::value::Value;

pub struct Frame {
    env: Env,
    sp: uint,
    pc: u64,
    caller: Option<~Frame> 
}

impl Frame {
    pub fn new(base_env: Env, sp: uint, pc: u64) -> ~Frame {
        ~Frame { env: base_env, sp: sp, pc: pc, caller: None }
    }

    pub fn alloc(&mut self, gc: &mut GC, size: u64) {
        let mut nenv = gc.alloc_env(size, Some(self.env));
        self.env = nenv;
    }

    pub fn store(&mut self, value: &Value, addr: u64) {
        unsafe { (*self.env).store(value, addr) }
    }

    pub fn fetch(&mut self, addr: u64) -> Value {
        unsafe { (*self.env).fetch(addr) }
    }
}
