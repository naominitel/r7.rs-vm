use gc;
use gc::value;
use gc::value::list;

pub fn list(argv: super::Arguments) -> gc::Value {
    let mut ret = value::Null;
    let mut i = argv.len() as isize - 1;

    while i >= 0 {
        let v = argv[i as u8].clone();
        let pair = argv.vm.gc.alloc(gc::Pair {
            car: v.clone(),
            cdr: ret.clone()
        });

        ret = value::Pair(pair);
        i -= 1
    }

    ret
}

pub fn is_list(argv: super::Arguments) -> gc::Value {
    match argv.vec() {
        [ref v] => value::Bool(list::is_list(v)),
        _ => panic!("Wrong number of arguments")
    }

}

pub fn map(argv: super::Arguments) -> gc::Value {
    let (fun, lst) = match argv.vec() {
        [ref fun, ref lst] => (fun.clone(), lst.clone()),
        _ => panic!("Wrong number of arguments")
    };

    let mut builder = list::LIST_BUILDER.clone();
    builder.init();

    for v in list::iter(&lst, |_| panic!("Error: expected a pair")) {
        // function calls requires arguments to be placed
        // on-stack before passing control to the function
        argv.vm.stack.push(v);
        let ret = argv.vm.fun_call_ret(&fun, 1);
        builder.append(&ret, &mut *argv.vm.gc);
    }

    builder.get_list()
}

pub fn filter(argv: super::Arguments) -> gc::Value {
    let (fun, lst) = match argv.vec() {
        [ref fun, ref lst] => (fun.clone(), lst.clone()),
        _ => panic!("Wrong number of arguments")
    };

    let mut builder = list::LIST_BUILDER.clone();
    builder.init();

    for v in list::iter(&lst, |_| panic!("Error: expected a pair")) {
        argv.vm.stack.push(v.clone());
        let ret = argv.vm.fun_call_ret(&fun, 1);

        match ret {
            value::Bool(false) => (),
            _ => builder.append(&v, &mut *argv.vm.gc),
        }
    }

    builder.get_list()
}
