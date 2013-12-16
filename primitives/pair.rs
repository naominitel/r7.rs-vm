use gc::Value;
use gc::value::Pair;
use gc::value::Unit;
use gc::value::list;
use primitives::Arguments;

pub fn cons(argv: Arguments) -> Value {
    match argv.vec() {
        [ref v1, ref v2] => Pair (list::cons(v1, v2, argv.vm().gc)),
        _ => fail!("Wrong number of arguments")
    }
}

pub fn car(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(p)] => p.car(),
        _ => fail!("Bad argument")
    }
}

pub fn cdr(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(p)] => p.cdr(),
        _ => fail!("Bad arguments")
    }
}

pub fn setcar(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(p), ref v] => p.setcar(v),
        _ => fail!("Attempting to setcar! on a non-pair value")
    }

    Unit
}

pub fn setcdr(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(p), ref v] => p.setcdr(v),
        _ => fail!("Attempting to setcdr! on a non-pair value")
    }

    Unit
}
