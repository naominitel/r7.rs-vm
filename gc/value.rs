use gc;

// Type for representing Scheme values manipulated by the VM
// a Value can be either
//   * a pair of two values (managed by the GC)
//   * a closure with its program and environment managed by the GC
//   * integer data types managed by copy
//   * unit, the void value
//   * null, a singleton value for '()

#[deriving(Clone)]
pub enum Value {
    Pair(gc::Pair),
    Closure(u64, gc::Env),
    Num(i64),
    Null,
    Unit
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
