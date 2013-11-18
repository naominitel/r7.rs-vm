use gc::Env;
use gc::Value;
use gc::value::Pair;
use gc::value::Closure;
use vm::Frame;
use vm::Stack;

// Visitor trait that any object the GC will have to look through must
// implement.
// each time the visitor reaches a garbage-collected object (a GC-collect),
// it marks it and continues through it

pub trait Visitor {
    fn visit(&mut self, gcmark: bool);
}

impl Visitor for Frame {
    fn visit(&mut self, m: bool) {
        self.env.visit(m);

        do self.caller.as_mut().map |f| {
            f.visit(m);
        };
    }
}

impl Visitor for Env {
    fn visit(&mut self, m: bool) {
        unsafe { (**self).visit(m); }
    }
}

impl Visitor for ::gc::Pair {
    fn visit(&mut self, m: bool) {
        unsafe { (***self).visit(m); }
    }
}

impl Visitor for Value {
    fn visit(&mut self, m: bool) {
        match self {
            &Pair(ref mut pair) => { pair.visit(m); }
            &Closure(_, ref mut env) => { env.visit(m); }

            // values other than pairs and closures
            // doesn't need to be GC'd
            _ => ()
        }
    }
}

impl Visitor for Stack {
    fn visit(&mut self, m: bool) {
        for v in self.mut_iter() {
            v.visit(m);
        }
    }
}


