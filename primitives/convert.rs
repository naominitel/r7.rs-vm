use gc::Value;
use gc::value::String;
use gc::value::Symbol;
use primitives::Arguments;

pub fn symbol_to_string(argv: Arguments) -> Value {
    let str = match argv.vec() {
        [Symbol(h)] => h.to_str(),
        [_] => fail!("Argument is not a symbol"),
        _ => fail!("Wrong number of parameters")
    };

    String(argv.vm().gc.alloc_string(str))
}

pub fn string_to_symbol(argv: Arguments) -> Value {
    let sym = match argv.vec() {
        [String(s)] => s.into_owned(),
        [_] => fail!("Argument is not a string"),
        _ => fail!("Wrong number of parameters")
    };

    Symbol(argv.vm.sym_table.get_or_create(sym))
}
