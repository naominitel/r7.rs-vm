use std::fmt;
use gc;

// a garbage-collected Scheme pair

#[packed]
#[derive(Clone)]
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

impl fmt::String for Pair {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let car = self.car.to_string();

        match self.cdr {
            gc::value::Pair(p) => fmt.pad(
                &format!("{} {}", car, (*p).to_string())[]
            ),

            gc::value::Null => fmt.pad(&format!("{}", car)[]),
            ref v => fmt.pad(&format!("{} . {}", car, v.to_string())[])
        }
    }
}
