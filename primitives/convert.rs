use gc::Value;
use gc::value::String;
use gc::value::Symbol;
use primitives::Arguments;

pub fn symbol_to_string(argv: Arguments) -> Value {
    match argv.vec() {
        [Symbol(h)] => String(h.to_str()),
        [_] => fail!("Argument is not a symbol"),
        _ => fail!("Wrong number of parameters")
    }
}

pub fn string_to_symbol(argv: Arguments) -> Value {
    let sym = match argv.vec() {
        [String(ref s)] => s.clone(),
        [_] => fail!("Argument is not a string"),
        _ => fail!("Wrong number of parameters")
    };

    Symbol(argv.vm.sym_table.get_or_create(sym))
}
