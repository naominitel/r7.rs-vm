use gc;

pub fn symbol_to_string(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [gc::value::Symbol(h)] => gc::value::String(h),
        [_] => panic!("Argument is not a symbol"),
        _ => panic!("Wrong number of parameters")
    }
}

pub fn string_to_symbol(argv: super::Arguments) -> gc::Value {
    let sym = match argv.vec() {
        [gc::value::String(s)] => s.str.clone(),
        [_] => panic!("Argument is not a string"),
        _ => panic!("Wrong number of parameters")
    };

    gc::value::Symbol(argv.vm.gc.intern(sym))
}
