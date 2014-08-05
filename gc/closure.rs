use gc::collect;
use gc::Env;
use gc::visit::Visitor;

// Internal representation of a closure

#[deriving(PartialEq, Clone)]
pub struct GClosure {
    pub pc: u64,
    pub env: Env,
    pub arity: u8,
    pub variadic: bool,
    pub mark: bool
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

pub struct Closure(pub *mut GClosure);

impl Deref<GClosure> for Closure {
    #[inline(always)]
    fn deref<'a>(&'a self) -> &'a GClosure {
        let &Closure(ptr) = self;
        unsafe { ::std::mem::transmute(ptr) }
    }
}

impl Closure {
    #[inline(always)]
    pub fn env(self) -> Env {
        (*self).env
    }

    #[inline(always)]
    pub fn pc(self) -> u64 {
        (*self).pc
    }

    #[inline(always)]
    pub fn arity(self) -> u8 {
        (*self).arity
    }

    #[inline(always)]
    pub fn variadic(self) -> bool {
        (*self).variadic
    }
}

