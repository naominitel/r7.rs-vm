use gc::collect::GCollect;
use gc::env::Env;
use gc::env::GCEnv;
use gc::pair::Pair;
use gc::pair::GCPair;
use gc::Closure;
use gc::value;
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

impl Visitor for GCEnv {
    fn visit(&mut self, m: bool) {
        if !self.is_marked(m) {
            self.mark(m);
        }
    }
}

impl Visitor for Pair {
    fn visit(&mut self, m: bool) {
        unsafe { (***self).visit(m); }
    }
}

impl Visitor for GCPair {
    fn visit(&mut self, m: bool) {
        if !self.is_marked(m) {
            self.mark(m);
        }
    }
}

impl Visitor for Closure {
    fn visit(&mut self, m: bool) {
        unsafe {
            if !(***self).is_marked(m) {
                (***self).mark(m);
            }
        }
    }
}

impl Visitor for value::Value {
    fn visit(&mut self, m: bool) {
        match self {
            &value::Pair(ref mut pair) => { pair.visit(m); }
            &value::Closure(ref mut cl) => { cl.visit(m); }

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


