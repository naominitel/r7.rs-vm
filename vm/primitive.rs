use gc;
use gc::Env;
use gc::Value;
use gc::value::Bool;
use gc::value::Closure;
use gc::value::Null;
use gc::value::Num;
use gc::value::Pair;
use gc::value::Primitive;
use gc::value::Unit;
use vm::VM;

pub type Prim = fn(~[Value], &mut VM) -> Value;

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
            Primitive(add),
            Primitive(min),
            Primitive(mul),
            Primitive(div),
            Primitive(eq),
            Primitive(list),
            Primitive(cons),
            Primitive(car),
            Primitive(cdr),
            Primitive(display),
            Primitive(newline),
            Primitive(setcar),
            Primitive(setcdr)
        ];
    };
    env
}

// default primitives

fn add(vals: ~[Value], _: &mut VM) -> Value {
    let mut res = 0;

    for v in vals.move_iter() {
        match v {
            Num(n) => res += n,
            _ => fail!("Value is not a number")
        }
    }

    Num(res)
}

fn min(vals: ~[Value], _: &mut VM) -> Value {
    match vals {
        [] => fail!("No arguments"),
        [Num(i)] => Num(-i),
        [Num(i), .. r ] => {
            let mut res = i;

            for v in r.iter() {
                match v {
                    &Num(n) => res -= n,
                    _ => fail!("Value is not a number")
                }
            }

            Num(res)
        }

        _ => fail!("Value is not a number")
    }
}

fn mul(vals: ~[Value], _: &mut VM) -> Value {
    let mut res = 1;

    for v in vals.move_iter() {
        match v {
            Num(n) => res *= n,
            _ => fail!("Value is not a number")
        }
    }

    Num(res)
}

fn div(_: ~[Value], _: &mut VM) -> Value {
    // requires exact numbers implementation
    fail!("Unimplemented")
}

fn eq(vals: ~[Value], _: &mut VM) -> Value {
    match vals {
        [v1, v2] => {
            match (v1, v2) {
                (Pair(p1), Pair(p2)) => {
                    // eq do object-compareason
                    Bool((*p1) == (*p2))
                }

                (Closure(p1, e1), Closure(p2, e2)) => {
                    Bool((p1 == p2) && (e1 == e2))
                }

                (Primitive(p1), Primitive(p2)) => {
                    // FIXME: raw function pointers compareason
                    // are not allowed. One should change primitive
                    // internal representation
                    // Bool(p1 == p2)
                    fail!("Unimplemented")
                }

                (Num(i), Num(j)) => {
                    Bool(i == j)
                }

                (Null, Null) => Bool(true),
                (Unit, Unit) => Bool(true),
                _ => Bool(false)
            }
        }

        _ => fail!("Wrong number of parameters")
    }
}

fn list(vals: ~[Value], vm: &mut VM) -> Value {
    let mut ret = Null;

    for v in vals.rev_iter() {
        let pair = vm.gc.alloc_pair();
        pair.setcar(v);
        pair.setcdr(&ret);
        ret = Pair(pair)
    }

    ret
}

fn cons(vals: ~[Value], vm: &mut VM) -> Value {
    match vals {
        [ref v1, ref v2] => {
            let p = vm.gc.alloc_pair();
            p.setcar(v1);
            p.setcar(v2);
            Pair(p)
        }

        _ => fail!("Bad argument count")
    }
}

fn car(vals: ~[Value], _: &mut VM) -> Value {
    match vals {
        [Pair(p)] => p.car(),
        _ => fail!("Bad arguments")
    }
}

fn cdr(vals: ~[Value], _: &mut VM) -> Value {
    match vals {
        [Pair(p)] => p.car(),
        _ => fail!("Bad arguments")
    }
}

fn display(vals: ~[Value], _: &mut VM) -> Value {
    print!("{:?}", vals[0]);
    Unit
}

fn newline(_: ~[Value], _: &mut VM) -> Value {
    print("\n");
    Unit
}

fn setcar(vals: ~[Value], _: &mut VM) -> Value {
    match vals {
        [Pair(p), ref v] => p.setcar(v),
        _ => fail!("Bad arguments")
    }

    Unit
}

fn setcdr(vals: ~[Value], _: &mut VM) -> Value {
    match vals {
        [Pair(p), ref v] => p.setcdr(v),
        _ => fail!("Bad arguments")
    }

    Unit
}
