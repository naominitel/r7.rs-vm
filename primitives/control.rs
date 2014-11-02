use gc::Value;
use gc::value::Bool;
use gc::value::Unit;
use primitives::Arguments;

pub fn exit(_: Arguments) -> Value {
    // FIXME: handle exit value
    panic!()
}

pub fn assert(argv: Arguments) -> Value {
    match argv.vec() {
        [Bool(true)] => Unit,
        [_] => panic!("Assertion failed"),
        _ => panic!("Wrong number of arguments")
    }
}
