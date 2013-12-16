use gc::Value;
use gc::value::Bool;
use gc::value::Null;
use gc::value::Pair;
use gc::value::list;
use primitives::Arguments;

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
