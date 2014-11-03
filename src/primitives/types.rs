use gc;
use gc::value;

pub fn boolean(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Bool(_)] => value::Bool(true),
        [_] => value::Bool(false),
        _ => panic!("Bad arguments")
    }
}

pub fn null(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Null] => value::Bool(true),
        [_] => value::Bool(false),
        _ => panic!("Bad arguments")
    }
}

pub fn pair(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Pair(_)] => value::Bool(true),
        [_] => value::Bool(false),
        _ => panic!("Bad arguments")
    }
}

pub fn procedure(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Closure(_)] | [value::Primitive(_, _)] => value::Bool(true),
        [_] => value::Bool(false),
        _ => panic!("Bad arguments")
    }
}

pub fn symbol(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Symbol(_)] => value::Bool(true),
        [_] => value::Bool(false),
        _ => panic!("Bad arguments")
    }
}

pub fn number(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Num(_)] => value::Bool(true),
        [_] => value::Bool(false),
        _ => panic!("Bad arguments")
    }
}
