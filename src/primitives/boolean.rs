use gc;
use gc::value;

pub fn cmp(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [value::Num(ref v), r..] => {
            for i in r.iter() {
                match i {
                    &value::Num(ref v2) if *v2 == *v => (),
                    &value::Num(_) => return value::Bool(false),
                    _ => panic!("Bad argument")
                }
            }

            value::Bool(true)
        }

        _ => panic!("Bad argument")
    }
}

pub fn eq(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [ref v1, ref v2] => value::Bool(v1 == v2),
        _ => panic!("Wrong number of arguments")
    }
}

pub fn equal(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [ref v1, ref v2] => value::Bool(v1.compare(v2)),
        _ => panic!("Wrong number of arguments")
    }
}
