use gc::Value;
use gc::value::Unit;
use primitives::Arguments;

pub fn display(argv: Arguments) -> Value {
    print!("{:s}", argv[0].to_str());
    Unit
}

pub fn newline(_: Arguments) -> Value {
    print("\n");
    Unit
}
