use gc;
use vm;

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
    Closure(u64, gc::Env),
    Null,
    Num(i64),
    Pair(gc::Pair),
    Primitive(vm::Prim),
    Symbol(vm::Handle),
    Unit
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            &Bool(b) => Bool(b),
            &Closure(pc, e) => Closure(pc, e),
            &Null => Null,
            &Num(n) => Num(n),
            &Pair(p) => Pair(p),
            &Primitive(p) => Primitive(p),
            &Symbol(h) => Symbol(h),
            &Unit => Unit
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
