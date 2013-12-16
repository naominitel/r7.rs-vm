use gc::Value;
use gc::value::Num;
use gmp::Mpz;
use primitives::Arguments;
use std::num::One;
use std::num::Zero;

pub fn add(argv: Arguments) -> Value {
    let mut res: Mpz = Zero::zero();

    for i in range(0, argv.len()) {
        match argv[i] {
            Num(n) => res = res.add(&n),
            _ => fail!("Value is not a number")
        }
    }

    Num(res)
}

pub fn min(argv: Arguments) -> Value {
    if argv.len() == 0 {
        fail!("No arguments")
    }

    match argv.vec() {
        [Num(ref i)] => Num(-i),
        [Num(ref i), .. r] => {
            let mut res = i.clone();

            for i in r.iter() {
                match i {
                    &Num(ref n) => res = res.sub(n),
                    _ => fail!("Value is not a number")
                }
            }

            Num(res)
        }

        _ => fail!("Value is not a number")
    }
}

pub fn mul(argv: Arguments) -> Value {
    let mut res: Mpz = One::one();

    for i in range(0, argv.len()) {
        match argv[i] {
            Num(n) => res = res.mul(&n),
            _ => fail!("Value is not a number")
        }
    }

    Num(res)
}

pub fn div(_: Arguments) -> Value {
    // requires exact numbers implementation
    fail!("Unimplemented")
}
