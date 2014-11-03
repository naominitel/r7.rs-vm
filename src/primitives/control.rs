use gc;

pub fn exit(_: super::Arguments) -> gc::Value {
    // FIXME: handle exit value
    panic!()
}

pub fn assert(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [gc::value::Bool(true)] => gc::value::Unit,
        [_] => panic!("Assertion failed"),
        _ => panic!("Wrong number of arguments")
    }
}
