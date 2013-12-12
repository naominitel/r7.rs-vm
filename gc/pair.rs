use gc::collect;
use gc::value;
use gc::visit::Visitor;

// a garbage-collected Scheme pair

#[packed]
#[deriving(Clone)]
pub struct GCPair {
    car: value::Value,
    cdr: value::Value,
    mark: bool
}

impl collect::GCollect for GCPair {
    fn is_marked(&self, m: bool) -> bool {
        self.mark == m
    }

    fn mark(&mut self, m: bool) {
        self.mark = m;
        self.car.visit(m);
        self.cdr.visit(m);
    }
}

// wrapper type for GC-managed Pairs to avoid unsafe blocks everywhere
// this struct is used as the external representation of a pair, used
// by Value and by the VM.

pub struct Pair(*mut GCPair);

impl Clone for Pair {
    fn clone(&self) -> Pair {
        Pair(**self)
    }
}

impl ToStr for Pair {
    fn to_str(&self) -> ~str {
        let car = unsafe { (***self).car.to_str() };

        match unsafe { &(***self).cdr } {
            &value::Pair(p) => format!("{:s} {:s}", car, p.to_str()),
            &value::Null => format!("{:s}", car),
            v => format!("{:s} . {:s}", car, v.to_str())
        }
    }
}

impl Pair {
    pub fn car(self) -> value::Value {
        unsafe { (**self).car.clone() }
    }

    pub fn cdr(self) -> value::Value {
        unsafe { (**self).cdr.clone() }
    }

    pub fn setcar(self, car: &value::Value) {
        unsafe { (**self).car = car.clone(); }
    }

    pub fn setcdr(self, cdr: &value::Value) {
        unsafe { (**self).cdr = cdr.clone(); }
    }
}

