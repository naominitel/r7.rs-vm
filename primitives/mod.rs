use gc;
use vm;

// public primitives

pub use self::list::list;

mod arith;
mod boolean;
mod control;
mod convert;
mod display;
mod list;
mod pair;
mod types;

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

pub fn env(gc: &mut gc::GC) -> gc::Ptr<gc::Env> {
    use gc::value::Primitive;
    gc.alloc(gc::Env {
        values: vec!(
            /* arith primitives */
            (true, Primitive(arith::add, "+")),
            (true, Primitive(arith::min, "-")),
            (true, Primitive(arith::mul, "*")),
            (true, Primitive(arith::div, "/")),

            /* boolean primitives */
            (true, Primitive(boolean::cmp, "=")),
            (true, Primitive(boolean::eq, "eq?")),
            (true, Primitive(boolean::equal, "equal?")),

            /* type predicates */
            (true, Primitive(types::boolean, "boolean?" )),
            (true, Primitive(types::null, "null?")),
            (true, Primitive(types::pair, "pair?")),
            (true, Primitive(types::procedure, "procedure?")),
            (true, Primitive(types::symbol, "symbol?")),
            (true, Primitive(types::number, "number?")),

            /* type converters */
            (true, Primitive(convert::symbol_to_string, "symbol->string")),
            (true, Primitive(convert::string_to_symbol, "string->symbol")),

            /* pair utils */
            (true, Primitive(pair::cons, "cons")),
            (true, Primitive(pair::car, "car")),
            (true, Primitive(pair::cdr, "cdr")),
            (true, Primitive(pair::setcar, "set-car!")),
            (true, Primitive(pair::setcdr, "set-cdr!")),

            /* list utils */
            (true, Primitive(list, "list")),
            (true, Primitive(list::is_list, "list?")),
            (true, Primitive(list::map, "map")),
            (true, Primitive(list::filter, "filter")),

            /* display */
            (true, Primitive(display::display, "display")),
            (true, Primitive(display::newline, "newline")),

            /* misc */
            (true, Primitive(control::exit, "exit")),
            (true, Primitive(control::assert, "assert"))
        ),
        next: None
    })
}

pub struct Arguments<'a> {
    pub vm: &'a mut vm::VM,
    pub argc: u8
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
}

impl<'a> Index<u8, gc::Value> for Arguments<'a> {
    #[inline(always)]
    fn index<'a>(&'a self, index: &u8) -> &'a gc::Value {
        // first arguments are at the top of the stack
        if self.argc == *index { fail!("waaaat") };
        let idx = self.vm.stack.len() - self.argc as uint + *index as uint;
        self.vm.stack.get(idx)
    }
}
