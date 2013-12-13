#[pkgid = "scmrun#0.1"];
#[deny(unnecessary_qualification)];
#[deny(non_camel_case_types)];
#[deny(non_uppercase_statics)];

extern mod gmp;

mod common;
mod gc;
mod vm;

fn main() {
    let args = ::std::os::args();
    if args.len() < 2 {
        fail!("usage: {:s} <program>", args[0]);
    }

    else {
        let mut vm = vm::VM::new();
        vm.run_file(args[1]);
    }
}
