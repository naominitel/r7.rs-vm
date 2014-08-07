use gc;
use gc::Env;
use gc::GC;
use gc::value::Value;

pub struct Frame {
    pub env: gc::Ptr<Env>,
    pub sp: uint,
    pub pc: u64,
    pub caller: Option<Box<Frame>>
}

impl Frame {
    pub fn new(base_env: gc::Ptr<Env>, sp: uint, pc: u64) -> Box<Frame> {
        box Frame { env: base_env, sp: sp, pc: pc, caller: None }
    }

    pub fn alloc(&mut self, gc: &mut GC, size: u64) {
        debug!("Allocating an env of size {:x}", size);
        let nenv = gc.alloc(Env {
            values: Vec::with_capacity(size as uint),
            next: Some(self.env)
        });
        self.env = nenv;
    }

    pub fn store(&mut self, value: &Value, addr: u64) {
        debug!("Store in env at addr {:x}", addr);
        unsafe { (*self.env).store(value, addr) }
    }

    pub fn fetch(&mut self, addr: u64) -> Value {
        // unsafe { (*self.env).dump(); };
        unsafe { (*self.env).fetch(addr) }
    }
}
