use gc;

// Internal representation of a closure

#[deriving(PartialEq, Clone)]
pub struct Closure {
    pub pc: u64,
    pub env: gc::Ptr<gc::Env>,
    pub arity: u8,
    pub variadic: bool
}

impl gc::visit::Visitor for Closure {
    fn visit(&mut self, m: bool) {
        self.env.visit(m);
    }
}
