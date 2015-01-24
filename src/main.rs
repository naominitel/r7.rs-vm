#![deny(non_camel_case_types)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]

extern crate gmp;

#[macro_use]
extern crate log;

mod common;
#[macro_use]
mod gc;
mod primitives;
mod vm;

fn main() {
    let args = ::std::os::args();
    if args.len() < 2 {
        panic!("usage: {} <program>", args[0]);
    } else {
        let mut vm = vm::VM::new();
        vm.run(&args[1][]);
    }
}
