use gc::Value;
use gc::value::Bool;
use gc::value::Unit;
use primitives::Arguments;

pub fn exit(_: Arguments) -> Value {
    // FIXME: handle exit value
    fail!()
}

pub fn assert(argv: Arguments) -> Value {
    match argv.vec() {
        [Bool(true)] => Unit,
        [_] => fail!("Assertion failed"),
        _ => fail!("Wrong number of arguments")
    }
}
