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
    let mut it = vals.iter();

    // first element: don't print the comma before
    match it.next() {
        Some(&(_, gc::value::Primitive(_, name))) =>
            write!(out, "    \"{:s}\"", name),

        Some(_) => (),
        None => return
    }

    for i in it {
        match i {
            &(_, gc::value::Primitive(_, name)) => {
                write!(out, ",\n");
                write!(out, "    \"{:s}\"", name);
            }

            _ => ()
        }
    }

    writeln!(out, "\n    ]");
}

fn main() {
    gen_primitives_env(&mut ::std::io::stdout() as &mut Writer);
}
