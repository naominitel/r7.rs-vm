use gc::Value;
use gc::value::Bool;
use gc::value::Num;
use primitives::Arguments;

pub fn cmp(argv: Arguments) -> Value {
    match argv.vec() {
        [Num(ref v), .. r] => {
            for i in r.iter() {
                match i {
                    &Num(ref v2) if *v2 == *v => (),
                    &Num(_) => return Bool(false),
                    _ => fail!("Bad argument")
                }
            }

            Bool(true)
        }

        _ => fail!("Bad argument")
    }
}

pub fn eq(argv: Arguments) -> Value {
    match argv.vec() {
        [ref v1, ref v2] => Bool(v1 == v2),
        _ => fail!("Wrong number of arguments")
    }
}

pub fn equal(argv: Arguments) -> Value {
    match argv.vec() {
        [ref v1, ref v2] => Bool(v1.compare(v2)),
        _ => fail!("Wrong number of arguments")
    }
}
