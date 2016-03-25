#![deny(non_camel_case_types)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![feature(slice_patterns)]
#![feature(const_fn)]

extern crate gmp;

#[macro_use]
extern crate log;

mod common;
#[macro_use]
mod gc;
mod primitives;
mod vm;

fn main() {
    let mut args = ::std::env::args();
    if args.len() < 2 {
        panic!("usage: {} <program>", args.nth(0).unwrap());
    } else {
        let mut vm = vm::VM::new();
        vm.run(&args.nth(1).unwrap());
    }
}
