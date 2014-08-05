use gc::Value;
use gc::value::String;
use gc::value::Symbol;
use primitives::Arguments;

pub fn symbol_to_string(argv: Arguments) -> Value {
    match argv.vec() {
        [Symbol(h)] => String(h),
        [_] => fail!("Argument is not a symbol"),
        _ => fail!("Wrong number of parameters")
    }
}

pub fn string_to_symbol(argv: Arguments) -> Value {
    let sym = match argv.vec() {
        [String(s)] => s.to_string(),
        [_] => fail!("Argument is not a string"),
        _ => fail!("Wrong number of parameters")
    };

    Symbol(argv.vm.gc.intern(String::from_str(sym.as_slice())))
}
