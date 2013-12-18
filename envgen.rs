#[allow(dead_code)];

extern mod gmp;

mod common;
mod gc;
mod primitives;
mod vm;

fn gen_primitives_env(out: &mut Writer) {
    let mut vm = vm::VM::new();
    let penv = primitives::env(vm.gc);
    let vals = unsafe { &(*penv).values };

    writeln!(out, "module Primitives");
    writeln!(out, "(");
    writeln!(out, "    primEnv");
    writeln!(out, ") where");
    writeln!(out, "");
    writeln!(out, "primEnv :: [String]");
    writeln!(out, "primEnv = [");

    for i in vals.iter() {
        match i {
            &(_, gc::value::Primitive(_, name)) => {
                writeln!(out, "    \"{:s}\",", name);
            }

            _ => ()
        }
    }

    writeln!(out, "]");
}

fn main() {
    gen_primitives_env(&mut ::std::io::stdout() as &mut Writer);
}
