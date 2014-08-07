use gc;
use std::fmt;

// a garbage-collected Scheme pair

#[packed]
#[deriving(Clone)]
pub struct Pair {
    pub car: gc::Value,
    pub cdr: gc::Value
}

impl gc::visit::Visitor for Pair {
    fn visit(&mut self, m: bool) {
        self.car.visit(m);
        self.cdr.visit(m);
    }
}

impl fmt::Show for Pair {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let car = self.car.to_string();

        match self.cdr {
            gc::value::Pair(mut p) => fmt.pad(
                format!("{:s} {:s}", car, (*p).to_string()).as_slice()
            ),

            gc::value::Null => fmt.pad(format!("{:s}", car).as_slice()),
            ref v => fmt.pad(format!("{:s} . {:s}", car, v.to_string()).as_slice())
        }
    }
}
