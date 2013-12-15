use gc;
use gc::Env;
use gc::Value;
use gc::value::Bool;
use gc::value::Null;
use gc::value::Num;
use gc::value::Pair;
use gc::value::Primitive;
use gc::value::Unit;
use gc::value::list;
use gmp::Mpz;
use std::num::One;
use std::num::Zero;
use vm::VM;

pub type Prim = fn(argv: Arguments) -> Value;

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
            (true, Primitive(is_list)),
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

pub struct Arguments<'a> {
    priv vm: &'a mut VM,
    priv argc: u8
}

impl<'a> Arguments<'a> {
    #[inline(always)]
    pub fn new(vm: &'a mut VM, argc: u8) -> Arguments<'a> {
        Arguments { vm: vm, argc: argc }
    }

    #[inline(always)]
    fn len(&self) -> u8 {
        self.argc
    }

    #[inline(always)]
    fn vec(&'a self) -> &'a [Value] {
        self.vm.stack.slice_from(self.vm.stack.len() - self.argc as uint)
    }

    #[inline(always)]
    fn vm(&'a self) -> &'a mut VM {
        &'a mut *self.vm
    }
}

impl<'a> Index<u8, Value> for Arguments<'a> {
    #[inline(always)]
    fn index(&self, index: &u8) -> Value {
        // first arguments are at the top of the stack
        let idx = self.vm.stack.len() - self.argc as uint + *index as uint;
        self.vm.stack[idx].clone()
    }
}

// default primitives

fn add(argv: Arguments) -> Value {
    let mut res: Mpz = Zero::zero();

    for i in range(0, argv.len()) {
        match argv[i] {
            Num(n) => res = res.add(&n),
            _ => fail!("Value is not a number")
        }
    }

    Num(res)
}

fn min(argv: Arguments) -> Value {
    if argv.len() == 0 {
        fail!("No arguments")
    }

    match argv.vec() {
        [Num(ref i)] => Num(-i),
        [Num(ref i), .. r] => {
            let mut res = i.clone();

            for i in r.iter() {
                match i {
                    &Num(ref n) => res = res.sub(n),
                    _ => fail!("Value is not a number")
                }
            }

            Num(res)
        }

        _ => fail!("Value is not a number")
    }
}

fn mul(argv: Arguments) -> Value {
    let mut res: Mpz = One::one();

    for i in range(0, argv.len()) {
        match argv[i] {
            Num(n) => res = res.mul(&n),
            _ => fail!("Value is not a number")
        }
    }

    Num(res)
}

fn div(_: Arguments) -> Value {
    // requires exact numbers implementation
    fail!("Unimplemented")
}

fn cmp(argv: Arguments) -> Value {
    match argv.vec() {
        [Num(ref v), .. r] => {
            for i in r.iter() {
                match i {
                    &Num(ref v2) if *v2 == *v => (),
                    &Num(_) => return Bool(false),
                    _ => fail!("Bad argument")
                }
            }

            Bool(true)
        }

        _ => fail!("Bad argument")
    }
}

fn eq(argv: Arguments) -> Value {
    match argv.vec() {
        [ref v1, ref v2] => Bool(v1 == v2),
        _ => fail!("Wrong number of arguments")
    }
}

fn equal(argv: Arguments) -> Value {
    match argv.vec() {
        [ref v1, ref v2] => Bool(v1.compare(v2)),
        _ => fail!("Wrong number of arguments")
    }
}

pub fn list(argv: Arguments) -> Value {
    let mut ret = Null;
    let mut i = argv.len() as int - 1;

    while i >= 0 {
        let v = argv[i as u8].clone();
        let pair = argv.vm().gc.alloc_pair();
        pair.setcar(&v);
        pair.setcdr(&ret);
        ret = Pair(pair);
        i -= 1
    }

    ret
}

pub fn is_list(argv: Arguments) -> Value {
    match argv.vec() {
        [ref v] => Bool(list::is_list(v)),
        _ => fail!("Wrong number of arguments")
    }

}

pub fn map(argv: Arguments) -> Value {
    match argv.vec() {
        [ref fun, ref lst] => {
            let vm = argv.vm();
            let mut builder = list::LIST_BUILDER.clone();
            builder.init();

            list::invalid_list::cond.trap(|_| {
                fail!("Error: expected a pair");
            }).inside(|| {
                for v in list::iter(lst) {
                    // function calls requires arguments to be placed
                    // on-stack before passing control to the function
                    vm.stack.push(v);
                    let ret = vm.fun_call_ret(fun, 1);
                    builder.append(&ret, argv.vm().gc);
                }
            });

            builder.get_list()
        }

        _ => fail!("Wrong number of arguments")
    }
}

pub fn filter(argv: Arguments) -> Value {
    match argv.vec() {
        [ref fun, ref lst] => {
            let vm = argv.vm();
            let mut builder = list::LIST_BUILDER.clone();
            builder.init();

            list::invalid_list::cond.trap(|_| {
                fail!("Error: expected a pair");
            }).inside(|| {
                for v in list::iter(lst) {
                    vm.stack.push(v.clone());
                    let ret = vm.fun_call_ret(fun, 1);

                    match ret {
                        Bool(false) => (),
                        _ => builder.append(&v, argv.vm().gc),
                    }
                }
            });

            builder.get_list()
        }

        _ => fail!("Wrong number of arguments")
    }
}

fn cons(argv: Arguments) -> Value {
    match argv.vec() {
        [ref v1, ref v2] => Pair (list::cons(v1, v2, argv.vm().gc)),
        _ => fail!("Wrong number of arguments")
    }
}

fn car(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(p)] => p.car(),
        _ => fail!("Bad argument")
    }
}

fn cdr(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(p)] => p.cdr(),
        _ => fail!("Bad arguments")
    }
}

fn display(argv: Arguments) -> Value {
    print!("{:s}", argv[0].to_str());
    Unit
}

fn newline(_: Arguments) -> Value {
    print("\n");
    Unit
}

fn setcar(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(p), ref v] => p.setcar(v),
        _ => fail!("Attempting to setcar! on a non-pair value")
    }

    Unit
}

fn setcdr(argv: Arguments) -> Value {
    match argv.vec() {
        [Pair(p), ref v] => p.setcdr(v),
        _ => fail!("Attempting to setcdr! on a non-pair value")
    }

    Unit
}

fn exit(_: Arguments) -> Value {
    // FIXME: handle exit value
    fail!()
}

fn assert(argv: Arguments) -> Value {
    match argv.vec() {
        [Bool(true)] => Unit,
        _ => fail!("Assertion failed")
    }
}
