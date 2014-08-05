use gc::collect;
use gc::value;
use gc::visit::Visitor;
use std::fmt;

// a garbage-collected Scheme pair

#[packed]
#[deriving(Clone)]
pub struct GCPair {
    pub car: value::Value,
    pub cdr: value::Value,
    pub mark: bool
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

#[deriving(PartialEq)]
pub struct Pair(pub *mut GCPair);

impl Clone for Pair {
    fn clone(&self) -> Pair {
        let &Pair(ptr) = self;
        Pair(ptr)
    }
}

impl fmt::Show for Pair {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let car = self.car().to_string();

        match unsafe { self.cdr() } {
            value::Pair(p) => fmt.pad(format!("{:s} {:s}", car, p.to_string()).as_slice()),
            value::Null => fmt.pad(format!("{:s}", car).as_slice()),
            v => fmt.pad(format!("{:s} . {:s}", car, v.to_string()).as_slice())
        }
    }
}

impl Pair {
    pub fn car(self) -> value::Value {
        let Pair(ptr) = self;
        unsafe {(*ptr).car.clone() }
    }

    pub fn cdr(self) -> value::Value {
        let Pair(ptr) = self;
        unsafe { (*ptr).cdr.clone() }
    }

    pub fn cdr_ref<'a>(&'a self) -> &'a value::Value {
        let &Pair(ptr) = self;
        unsafe { &(*ptr).cdr }
    }

    pub fn setcar(mut self, car: &value::Value) {
        let Pair(ptr) = self;
        unsafe { (*ptr).car = car.clone(); }
    }

    pub fn setcdr(mut self, cdr: &value::Value) {
        let Pair(ptr) = self;
        unsafe { (*ptr).cdr = cdr.clone(); }
    }
}

