use gc;
use gc::Env;
use gc::Value;
use gc::value::Num;
use gc::value::Primitive;
use gc::value::Unit;

pub type Prim = fn(~[Value]) -> Value;

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

fn add(vals: ~[Value]) -> Value {
    let mut res = 0;

    for v in vals.move_iter() {
        match v {
            Num(n) => res += n,
            _ => fail!("Value is not a number")
        }
    }

    Num(res)
}

fn min(vals: ~[Value]) -> Value {
    let mut res = 0;

    for v in vals.move_iter() {
        match v {
            Num(n) => res -= n,
            _ => fail!("Value is not a number")
        }
    }

    Num(res)
}

fn mul(vals: ~[Value]) -> Value {
    fail!("Unimplemented")
}

fn div(vals: ~[Value]) -> Value {
    fail!("Unimplemented")
}

fn eq(vals: ~[Value]) -> Value {
    fail!("Unimplemented")
}

fn list(vals: ~[Value]) -> Value {
    fail!("Unimplemented")
}

fn cons(vals: ~[Value]) -> Value {
    fail!("Unimplemented")
}

fn car(vals: ~[Value]) -> Value {
    fail!("Unimplemented")
}

fn cdr(vals: ~[Value]) -> Value {
    fail!("Unimplemented")
}

fn display(vals: ~[Value]) -> Value {
    print!("{:?}", vals[0]);
    Unit
}

fn newline(vals: ~[Value]) -> Value {
    print("\n");
    Unit
}

fn setcar(vals: ~[Value]) -> Value {
    fail!("Unimplemented")
}

fn setcdr(vals: ~[Value]) -> Value {
    fail!("Unimplemented")
}
