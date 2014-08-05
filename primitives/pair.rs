use gc::Value;
use gc::value::Pair;
use gc::value::Unit;
use gc::value::list;
use primitives::Arguments;

pub fn cons(argv: Arguments) -> Value {
    let (car, cdr) = match argv.vec() {
        [ref car, ref cdr] => (car.clone(), cdr.clone()),
        _ => fail!("Wrong number of arguments")
    };

    Pair (list::cons(&car, &cdr, &mut *argv.vm.gc))
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
