use gc;
use vm;

pub use self::list::list;

mod arith;
mod boolean;
mod control;
mod display;
mod list;
mod pair;

pub type Prim = fn(argv: Arguments) -> gc::Value;

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

pub fn env(gc: &mut gc::GC) -> gc::Env {
    use gc::value::Primitive;

    let env = gc.alloc_env(0, None);
    unsafe {
        (*env).values = ~[
            /* arith primitives */
            (true, Primitive(arith::add)),
            (true, Primitive(arith::min)),
            (true, Primitive(arith::mul)),
            (true, Primitive(arith::div)),

            /* boolean primitives */
            (true, Primitive(boolean::cmp)),
            (true, Primitive(boolean::eq)),
            (true, Primitive(boolean::equal)),

            /* pair utils */
            (true, Primitive(pair::cons)),
            (true, Primitive(pair::car)),
            (true, Primitive(pair::cdr)),
            (true, Primitive(pair::setcar)),
            (true, Primitive(pair::setcdr)),

            /* list utils */
            (true, Primitive(list)),
            (true, Primitive(list::is_list)),
            (true, Primitive(list::map)),
            (true, Primitive(list::filter)),

            /* display */
            (true, Primitive(display::display)),
            (true, Primitive(display::newline)),

            /* misc */
            (true, Primitive(control::exit)),
            (true, Primitive(control::assert))
        ];
    };
    env
}

pub struct Arguments<'a> {
    priv vm: &'a mut vm::VM,
    priv argc: u8
}

impl<'a> Arguments<'a> {
    #[inline(always)]
    pub fn new(vm: &'a mut vm::VM, argc: u8) -> Arguments<'a> {
        Arguments { vm: vm, argc: argc }
    }

    #[inline(always)]
    fn len(&self) -> u8 {
        self.argc
    }

    #[inline(always)]
    fn vec(&'a self) -> &'a [gc::Value] {
        self.vm.stack.slice_from(self.vm.stack.len() - self.argc as uint)
    }

    #[inline(always)]
    fn vm(&'a self) -> &'a mut vm::VM {
        &'a mut *self.vm
    }
}

impl<'a> Index<u8, gc::Value> for Arguments<'a> {
    #[inline(always)]
    fn index(&self, index: &u8) -> gc::Value {
        // first arguments are at the top of the stack
        if self.argc == *index { fail!("waaaat") };
        let idx = self.vm.stack.len() - self.argc as uint + *index as uint;
        self.vm.stack[idx].clone()
    }
}
