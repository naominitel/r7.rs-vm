use gc;
use gc::value;
use gc::value::list;

pub fn cons(argv: super::Arguments) -> gc::Value {
    let (car, cdr) = match argv.vec() {
        [ref car, ref cdr] => (car.clone(), cdr.clone()),
        _ => panic!("Wrong number of arguments")
    };

    value::Pair(list::cons(&car, &cdr, &mut *argv.vm.gc))
}

pub fn car(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Pair(mut p)] => p.car.clone(),
        _ => panic!("Bad argument")
    }
}

pub fn cdr(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Pair(mut p)] => p.cdr.clone(),
        _ => panic!("Bad arguments")
    }
}

pub fn setcar(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Pair(mut p), ref v] => p.car = v.clone(),
        _ => panic!("Attempting to setcar! on a non-pair value")
    }

    value::Unit
}

pub fn setcdr(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Pair(mut p), ref v] => p.cdr = v.clone(),
        _ => panic!("Attempting to setcdr! on a non-pair value")
    }

    value::Unit
}
