use gc::Value;
use gc::value::Unit;
use primitives::Arguments;

pub fn display(argv: Arguments) -> Value {
    print!("{:s}", argv[0].to_string());
    Unit
}

pub fn newline(_: Arguments) -> Value {
    print!("\n");
    Unit
}
