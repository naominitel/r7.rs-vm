use gc::value;
use vm::Frame;
use vm::Stack;

// Visitor trait that any object the GC will have to look through must
// implement.
// each time the visitor reaches a garbage-collected object (a GC-collect),
// it marks it and continues through it

pub trait Visitor {
    fn visit(&mut self, mark: bool);
}

impl Visitor for Frame {
    fn visit(&mut self, m: bool) {
        self.env.visit(m);
        self.caller.as_mut().map(|f| f.visit(m));
    }
}

impl Visitor for value::Value {
    fn visit(&mut self, m: bool) {
        match self {
            &mut value::Pair(ref mut pair) => { pair.visit(m); }
            &mut value::Closure(ref mut cl) => { cl.visit(m); }

            // values other than pairs and closures
            // doesn't need to be GC'd
            _ => ()
        }
    }
}

impl Visitor for Stack {
    fn visit(&mut self, m: bool) {
        for v in self.iter_mut() {
            v.visit(m);
        }
    }
}
