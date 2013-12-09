use gc;
use vm;
use gmp;

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
