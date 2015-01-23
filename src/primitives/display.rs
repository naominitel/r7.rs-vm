use gc;

pub fn display(argv: super::Arguments) -> gc::Value {
    print!("{}", argv[0].to_string());
    gc::value::Unit
}

pub fn newline(_: super::Arguments) -> gc::Value {
    print!("\n");
    gc::value::Unit
}
