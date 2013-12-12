use gc;
use vm;
use gmp;

pub mod list {
    use gc;
    use super::Null;
    use super::Pair;
    use super::Value;
    use super::Unit;

    // various functions for manipulating Scheme lists
    pub fn is_list(value: &Value) -> bool {
        match value {
            &Pair(p) => is_list(&p.cdr()),
            &Null => true,
            _ => false
        }
    }

    // utility struct for efficiently building lists iteratively

    #[deriving(Clone)]
    struct ListBuilder {
        fst: gc::pair::GCPair,
        ptr: gc::Pair,
    }

    pub static LIST_BUILDER: ListBuilder = ListBuilder {
        // allocate a dummy pair that will in fact point to the first
        // true element of the list to allow fast insertions
        fst: gc::pair::GCPair {
            car: Unit,
            cdr: Null,
            mark: false
        },

        ptr: gc::Pair(0 as *mut gc::pair::GCPair)
    };

    impl ListBuilder {
        #[inline(always)]
        pub fn init(&mut self) {
            self.ptr = gc::Pair(&mut self.fst);
        }

        #[inline(always)]
        pub fn append(&mut self, v: &Value, gc: &mut gc::GC) {
            // this one is GC'd
            let pair = gc.alloc_pair();
            pair.setcar(v);
            pair.setcdr(&Null);

            self.ptr.setcdr(&Pair(pair));
            self.ptr = pair;
        }

        #[inline(always)]
        pub fn get_list(&self) -> Value {
            // remove dummy node
            self.fst.cdr.clone()
        }
    }
}

// Type for representing Scheme values manipulated by the VM
// a Value can be either
//   * a pair of two values (managed by the GC)
//   * a closure with its program and environment managed by the GC
//   * a primitive (in-VM implemented function)
//   * integer data types managed by copy
//   * unit, the void value
//   * null, a singleton value for '()

// FIXME: bug #10501 #[deriving(Clone)]
pub enum Value {
    Bool(bool),
    Closure(gc::Closure),
    Null,
    Num(gmp::Mpz),
    Pair(gc::Pair),
    Primitive(vm::Prim),
    Symbol(vm::Handle),
    Unit
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            &Bool(b) => Bool(b),
            &Closure(cl) => Closure(cl),
            &Null => Null,
            &Num(ref n) => Num(n.clone()),
            &Pair(p) => Pair(p),
            &Primitive(p) => Primitive(p),
            &Symbol(h) => Symbol(h),
            &Unit => Unit
        }
    }
}

impl ToStr for Value {
    fn to_str(&self) -> ~str {
        match self {
            &Bool(true) => ~"#t",
            &Bool(false) => ~"#f",
            &Closure(_) => ~"#<procedure>",
            &Null => ~"'()",
            &Num(ref i) => format!("{:s}", i.to_str()),
            &Pair(p) => p.to_str(),
            &Primitive(_) => ~"#<procedure>",
            &Symbol(h) => format!("'{:s}", h.to_str()),
            &Unit => ~""
        }
    }
}

pub fn setcar(val: &mut Value, car: &Value) {
    match val {
        &Pair(p) => {
            p.setcar(car);
        }

        _ => fail!("Attempting to setcar! on a non-pair value")
    }
}

pub fn setcdr(val: &mut Value, cdr: &Value) {
    match val {
        &Pair(p) => {
            p.setcdr(cdr);
        }

        _ => fail!("Attempting to setcar! on a non-pair value")
    }
}

impl Value {
    // structural copareason
    pub fn compare(&self, other: &Value) -> bool {
        match (self, other) {
            (&Pair(p1), &Pair(p2)) => {
                p1.car().compare(&p2.car()) &&
                    p1.cdr().compare(&p2.cdr())
            }

            (&Closure(cl1), &Closure(cl2)) => {
                (*cl1) == (*cl2)
            }

            (&Primitive(p1), &Primitive(p2)) => {
                use std::cast::transmute;
                let p1: *() = unsafe { transmute(p1) };
                let p2: *() = unsafe { transmute(p2) };
                p1 == p2
            }

            (&Num(ref i), &Num(ref j)) => i == j,
            (&Bool(b1), &Bool(b2)) => b1 == b2,
            (&Symbol(h1), &Symbol(h2)) => h1 == h2,

            (&Null, &Null) => true,
            (&Unit, &Unit) => true,

            _ => false
        }
    }
}
