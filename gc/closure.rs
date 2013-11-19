use gc::collect;
use gc::Env;
use gc::visit::Visitor;

// Internal representation of a closure

#[deriving(Eq, Clone)]
pub struct GClosure {
    pc: u64,
    env: Env,
    arity: u8,
    variadic: bool,
    mark: bool
}

impl collect::GCollect for GClosure {
    fn is_marked(&self, m: bool) -> bool {
        self.mark == m
    }

    fn mark(&mut self, m: bool) {
        self.mark = m;
        self.env.visit(m);
    }
}

// wrapper type for GC-managed Pairs to avoid unsafe blocks everywhere
// this struct is used as the external representation of a pair, used
// by Value and by the VM.

pub struct Closure(*mut GClosure);

impl Clone for Closure {
    fn clone(&self) -> Closure {
        Closure(**self)
    }
}

impl Closure {
    fn env(&self) -> Env {
        unsafe { (***self).env }
    }

    fn pc(&self) -> u64 {
        unsafe { (***self).pc }
    }

    fn arity(&self) -> u8 {
        unsafe { (***self).arity }
    }

    fn variadic(&self) -> bool {
        unsafe { (***self).variadic }
    }
}

