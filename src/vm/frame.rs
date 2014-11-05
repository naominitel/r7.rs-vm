use gc;

pub struct Frame {
    pub env: gc::Ptr<gc::Env>,
    pub sp: uint,
    pub pc: u64,
    pub caller: Option<Box<Frame>>
}

impl Frame {
    pub fn new(base_env: gc::Ptr<gc::Env>, sp: uint, pc: u64) -> Box<Frame> {
        box Frame { env: base_env, sp: sp, pc: pc, caller: None }
    }

    pub fn alloc(&mut self, gc: &mut gc::GC, size: u64) {
        debug!("Allocating an env of size {:x}", size);
        let nenv = gc.alloc(gc::Env {
            values: Vec::with_capacity(size as uint),
            next: Some(self.env)
        });
        self.env = nenv;
    }

    pub fn store(&mut self, value: &gc::Value, addr: u64) {
        debug!("Store in env at addr {:x}", addr);
        self.env.store(value, addr)
    }

    pub fn fetch(&mut self, addr: u64) -> gc::Value {
        self.env.fetch(addr)
    }
}
