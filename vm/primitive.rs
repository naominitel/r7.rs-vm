use gc;
use gc::Env;
use gc::Value;
use gc::value::Bool;
use gc::value::Closure;
use gc::value::Null;
use gc::value::Num;
use gc::value::Pair;
use gc::value::Primitive;
use gc::value::Symbol;
use gc::value::Unit;
use gc::value::list;
use gmp::Mpz;
use std::cast::transmute;
use std::num::One;
use std::num::Zero;
use vm::VM;

pub type Prim = fn(argc: u8, &mut VM) -> Value;

/*
static env: &'static GCEnv = &GCEnv {
    values: ~[
        Primitive(add),
        Primitive(min),
        Primitive(mul),
        Primitive(div),
        Primitive(eq),
        Primitive(list),
        Primitive(cons),
        Primitive(car),
        Primitive(display),
        Primitive(newline),
        Primitive(setcar),
        Primitive(setcdr)
    ],

    mark: false, // this env is not garbage-collected. never read
    next: None
};

static primEnv: Env = (env as *GCEnv) as *mut GCEnv;
*/

pub fn primEnv(gc: &mut gc::GC) -> Env {
    let env = gc.alloc_env(0, None);
    unsafe {
        (*env).values = ~[
            (true, Primitive(add)),
            (true, Primitive(min)),
            (true, Primitive(mul)),
            (true, Primitive(div)),
            (true, Primitive(cmp)),
            (true, Primitive(eq)),
            (true, Primitive(equal)),
            (true, Primitive(list)),
            (true, Primitive(map)),
            (true, Primitive(filter)),
            (true, Primitive(cons)),
            (true, Primitive(car)),
            (true, Primitive(cdr)),
            (true, Primitive(display)),
            (true, Primitive(newline)),
            (true, Primitive(setcar)),
            (true, Primitive(setcdr)),
            (true, Primitive(exit)),
            (true, Primitive(assert))
        ];
    };
    env
}

#[inline(always)]
fn getarg(vm: &mut VM) -> Value {
    vm.stack.pop()
}

// default primitives

fn add(argc: u8, vm: &mut VM) -> Value {
    let mut res: Mpz = Zero::zero();

    for _ in range(0, argc) {
        match getarg(vm) {
            Num(n) => res = res.add(&n),
            _ => fail!("Value is not a number")
        }
    }

    Num(res)
}

fn min(argc: u8, vm: &mut VM) -> Value {
    if argc == 0 {
        fail!("No arguments")
    }

    match getarg(vm) {
        Num(ref i) if argc == 1 => Num(-i),
        Num(i) => {
            let mut res = i;

            for _ in range(0, argc - 1) {
                match getarg(vm) {
                    Num(n) => res = res.sub(&n),
                    _ => fail!("Value is not a number")
                }
            }

            Num(res)
        }

        _ => fail!("Value is not a number")
    }
}

fn mul(argc: u8, vm: &mut VM) -> Value {
    let mut res: Mpz = One::one();

    for _ in range(0, argc) {
        match getarg(vm) {
            Num(n) => res = res.mul(&n),
            _ => fail!("Value is not a number")
        }
    }

    Num(res)
}

fn div(_: u8, _: &mut VM) -> Value {
    // requires exact numbers implementation
    fail!("Unimplemented")
}

fn cmp(argc: u8, vm: &mut VM) -> Value {
    if argc < 2 {
        fail!("Wrong number of arguments")
    }

    match getarg(vm) {
        Num(v) => {
            for _ in range(0, argc - 1) {
                match getarg(vm) {
                    Num(ref v2) if *v2 == v => (),
                    Num(_) => return Bool(false),
                    _ => fail!("Bad argument")
                }
            }

            Bool(true)
        }

        _ => fail!("Bad argument")
    }
}

fn eq(argc: u8, vm: &mut VM) -> Value {
    if argc != 2 {
        fail!("Wrong number of arguments");
    }

    let v1 = getarg(vm);
    let v2 = getarg(vm);

    match (v1, v2) {
        (Pair(p1), Pair(p2)) => {
            // eq do object-compareason
            Bool((*p1) == (*p2))
        }

        (Closure(cl1), Closure(cl2)) => {
            Bool((*cl1) == (*cl2))
        }

        (Primitive(p1), Primitive(p2)) => {
            let p1: *() = unsafe { transmute(p1) };
            let p2: *() = unsafe { transmute(p2) };
            Bool(p1 == p2)
        }

        (Num(i), Num(j)) => Bool(i == j),
        (Bool(b1), Bool(b2)) => Bool(b1 == b2),
        (Symbol(h1), Symbol(h2)) => Bool(h1 == h2),

        (Null, Null) => Bool(true),
        (Unit, Unit) => Bool(true),

        _ => Bool(false)
    }
}

fn equal(argc: u8, vm: &mut VM) -> Value {
    if argc != 2 {
        fail!("Wrong number of arguments");
    }

    let v1 = getarg(vm);
    let v2 = getarg(vm);

    Bool(v1.compare(&v2))
}

pub fn list(argc: u8, vm: &mut VM) -> Value {
    let mut ret = Null;
    let mut i = argc as uint;
    let stlen = vm.stack.len();

    while i > 0 {
        let v = vm.stack[stlen - i].clone();
        let pair = vm.gc.alloc_pair();
        pair.setcar(&v);
        pair.setcdr(&ret);
        ret = Pair(pair);
        i = i - 1;
    }

    vm.stack.truncate(stlen - argc as uint);
    ret
}

pub fn map(argc: u8, vm: &mut VM) -> Value {
    if argc != 2 {
        fail!("Wrong number of arguments")
    }

    let fun = getarg(vm);
    let mut lst = getarg(vm);
    let mut builder = list::LIST_BUILDER.clone();
    builder.init();

    loop {
        match lst {
            Pair(p) => {
                // function calls requires arguments to be placed
                // on-stack before passing control to the function
                vm.stack.push(p.car());
                let ret = vm.fun_call_ret(&fun, 1);

                builder.append(&ret, vm.gc);
                lst = p.cdr();
            }

            Null => break,

            _ => {
                fail!("Error: expected a pair");
            }
        }
    }

    builder.get_list()
}

pub fn filter(argc: u8, vm: &mut VM) -> Value {
    if argc != 2 {
        fail!("Wrong number of arguments")
    }

    let fun = getarg(vm);
    let mut lst = getarg(vm);
    let mut builder = list::LIST_BUILDER.clone();
    builder.init();

    loop {
        match lst {
            Pair(p) => {
                vm.stack.push(p.car());
                let ret = vm.fun_call_ret(&fun, 1);

                match ret {
                    Bool(false) => (),
                    _ => builder.append(&p.car(), vm.gc),
                }

                lst = p.cdr();
            }

            Null => break,

            _ => {
                fail!("Error: expected a pair");
            }
        }
    }

    builder.get_list()
}

fn cons(argc: u8, vm: &mut VM) -> Value {
    if argc != 2 {
        fail!("Wrong number of arguments")
    }

    let v1 = getarg(vm);
    let v2 = getarg(vm);
    Pair(list::cons(&v1, &v2, vm.gc))
}

fn car(argc: u8, vm: &mut VM) -> Value {
    if argc != 1 {
        fail!("Bad arguments")
    }

    match getarg(vm) {
        Pair(p) => p.car(),
        _ => fail!("Bad argument")
    }
}

fn cdr(argc: u8, vm: &mut VM) -> Value {
    if argc != 1 {
        fail!("Bad arguments")
    }

    match getarg(vm) {
        Pair(p) => p.cdr(),
        _ => fail!("Bad arguments")
    }
}

fn display(_: u8, vm: &mut VM) -> Value {
    print!("{:s}", getarg(vm).to_str());
    Unit
}

fn newline(_: u8, _: &mut VM) -> Value {
    print("\n");
    Unit
}

fn setcar(argc: u8, vm: &mut VM) -> Value {
    if argc != 2 {
        fail!("Wrong number of arguments")
    }

    match getarg(vm) {
        Pair(p) => p.setcar(&getarg(vm)),
        _ => fail!("Bad arguments")
    }

    Unit
}

fn setcdr(argc: u8, vm: &mut VM) -> Value {
    if argc != 2 {
        fail!("Wrong number of arguments")
    }

    match getarg(vm) {
        Pair(p) => p.setcdr(&getarg(vm)),
        _ => fail!("Bad arguments")
    }

    Unit
}

fn exit(_: u8, _: &mut VM) -> Value {
    // FIXME: handle exit value
    fail!()
}

fn assert(argc: u8, vm: &mut VM) -> Value {
    if argc != 1 {
        fail!("Wrong number of arguments")
    }

    match getarg(vm) {
        Bool(true) => Unit,
        _ => fail!("Assertion failed")
    }
}
