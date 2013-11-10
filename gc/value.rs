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
    Closure(uint, gc::Env),
    Null,
    Unit
}


