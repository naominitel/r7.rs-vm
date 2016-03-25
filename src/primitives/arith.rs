use gc;
use gc::value;
use gmp;

pub fn add(argv: super::Arguments) -> gc::Value {
    let mut res = gmp::mpz::Mpz::zero();

    for i in 0 .. argv.len() {
        match &argv[i] {
            &value::Num(ref n) => res = &res + n,
            _ => panic!("gc::Value is not a number")
        }
    }

    value::Num(res)
}

pub fn min(argv: super::Arguments) -> gc::Value {
    if argv.len() == 0 {
        panic!("No arguments")
    }

    match argv.vec() {
        [value::Num(ref i)] => value::Num(-i),
        [value::Num(ref i), ref r ..] => {
            let mut res = i.clone();

            for i in r.iter() {
                match i {
                    &value::Num(ref n) => res = &res - n,
                    _ => panic!("gc::Value is not a number")
                }
            }

            value::Num(res)
        }

        _ => panic!("gc::Value is not a number")
    }
}

pub fn mul(argv: super::Arguments) -> gc::Value {
    let mut res = gmp::mpz::Mpz::one();

    for i in 0 .. argv.len() {
        match &argv[i] {
            &value::Num(ref n) => res = &res * n,
            _ => panic!("gc::Value is not a number")
        }
    }

    value::Num(res)
}

pub fn div(_: super::Arguments) -> gc::Value {
    // requires exact numbers implementation
    panic!("Unimplemented")
}
