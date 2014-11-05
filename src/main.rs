#![deny(non_camel_case_types)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![feature(phase)]

extern crate gmp;

#[phase(plugin, link)]
extern crate log;

mod common;
mod gc;
mod primitives;
mod vm;

fn main() {
    let args = ::std::os::args();
    if args.len() < 2 {
        panic!("usage: {:s} <program>", args[0]);
    } else {
        let mut vm = vm::VM::new();
        vm.run(args[1].as_slice());
    }
}