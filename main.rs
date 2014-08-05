#![crate_id = "scmrun#0.1"]
#![deny(unnecessary_qualification)]
#![deny(non_camel_case_types)]
#![deny(non_uppercase_statics)]
#![feature(phase)]

extern crate debug;
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
        fail!("usage: {:s} <program>", args.get(0).as_slice());
    }

    else {
        let mut vm = vm::VM::new();
        vm.run(args.get(1).as_slice());
    }
}
