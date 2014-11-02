use gc::Value;
use gc::value::Pair;
use gc::value::Unit;
use gc::value::list;
use primitives::Arguments;

pub fn cons(argv: Arguments) -> Value {
    let (car, cdr) = match argv.vec() {
        [ref car, ref cdr] => (car.clone(), cdr.clone()),
        _ => panic!("Wrong number of arguments")
    };

    Pair (list::cons(&car, &cdr, &mut *argv.vm.gc))
}

pub fn car(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(mut p)] => p.car.clone(),
        _ => panic!("Bad argument")
    }
}

pub fn cdr(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(mut p)] => p.cdr.clone(),
        _ => panic!("Bad arguments")
    }
}

pub fn setcar(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(mut p), ref v] => p.car = v.clone(),
        _ => panic!("Attempting to setcar! on a non-pair value")
    }

    Unit
}

pub fn setcdr(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(mut p), ref v] => p.cdr = v.clone(),
        _ => panic!("Attempting to setcdr! on a non-pair value")
    }

    Unit
}
