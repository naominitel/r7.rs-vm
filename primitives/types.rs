use gc::Value;
use gc::value::Bool;
use gc::value::Null;
use gc::value::Pair;
use gc::value::Closure;
use gc::value::Primitive;
use gc::value::Symbol;
use gc::value::Num;
use primitives::Arguments;

pub fn boolean(argv: Arguments) -> Value {
    match argv.vec() {
        [Bool(_)] => Bool(true),
        [_] => Bool(false),
        _ => panic!("Bad arguments")
    }
}

pub fn null(argv: Arguments) -> Value {
    match argv.vec() {
        [Null] => Bool(true),
        [_] => Bool(false),
        _ => panic!("Bad arguments")
    }
}

pub fn pair(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(_)] => Bool(true),
        [_] => Bool(false),
        _ => panic!("Bad arguments")
    }
}

pub fn procedure(argv: Arguments) -> Value {
    match argv.vec() {
        [Closure(_)] | [Primitive(_, _)] => Bool(true),
        [_] => Bool(false),
        _ => panic!("Bad arguments")
    }
}

pub fn symbol(argv: Arguments) -> Value {
    match argv.vec() {
        [Symbol(_)] => Bool(true),
        [_] => Bool(false),
        _ => panic!("Bad arguments")
    }
}

pub fn number(argv: Arguments) -> Value {
    match argv.vec() {
        [Num(_)] => Bool(true),
        [_] => Bool(false),
        _ => panic!("Bad arguments")
    }
}
