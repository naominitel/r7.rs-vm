use gc::Env;
use gc::Value;
use gc::value::Pair;
use gc::value::Unit;
use gc::visit::Visitor;
mod list;

// Basic trait for garbage collected values
// must keep a boolean mark for the mark and
// sweep collection algorithm

trait GCollect {
    fn is_marked(&self, m: bool) -> bool;
    fn mark(&mut self, mark: bool);
}

// FIXME: this should work
// see https://github.com/mozilla/rust/issues/8075
// impl<T: GCollect> Visitor for T {
//    fn visit(&mut self, m: bool) {
//        if !self.is_marked(m) {
//            self.mark(m);
//        }
//    }
// }
//
// Instead we have to do this:

impl Visitor for GCPair {
    fn visit(&mut self, m: bool) {
        if !self.is_marked(m) {
            self.mark(m);
        }
    }
}

impl Visitor for GCEnv {
    fn visit(&mut self, m: bool) {
        if !self.is_marked(m) {
            self.mark(m);
        }
    }
}

// a garbage-collected Scheme pair
pub struct GCPair {
    car: Value,
    cdr: Value,
    mark: bool
}

impl GCollect for GCPair {
    fn is_marked(&self, m: bool) -> bool {
        self.mark == m
    }

    fn mark(&mut self, m: bool) {
        self.mark = m;
        self.car.visit(m);
        self.cdr.visit(m);
    }
}

// a garbage-collected Scheme environment
pub struct GCEnv {
    values: ~[Value],
    next: Option<Env>,
    mark: bool
}

impl GCollect for GCEnv {
    fn is_marked(&self, m: bool) -> bool {
        self.mark == m
    }

    fn mark(&mut self, m: bool) {
        self.mark = m;

        for v in self.values.mut_iter() {
            v.visit(m);
        }

        match self.next {
            Some(ref mut e) => e.visit(m),
            None => ()
        }
    }
}

// The GC itself
// Keeps all the allocated values in a linked list of cells, where a cell
// is a couple of an owned box containing the allocated object, and a raw
// ptr to the same object

struct GC {
    heap:  ~list::List<~GCollect>,
    white: ~list::List<~GCollect>,
    current_mark: bool
}

impl GC {
    pub fn new() -> ~GC {
        ~GC { heap: list::List::new(), white: list::List::new(),
            current_mark: false
        }
    }

    fn mark(&mut self, roots: &mut [&mut Visitor]) {
        for v in roots.mut_iter() {
            v.visit(true);
        }
    }

    fn check_node(m: bool, node: &mut list::ListNode<~GCollect>) {
        use std::util::swap;

        match node {
            &list::Node(_, ref mut next) => {
                let mut nnext = ~list::Empty;
                swap(next, &mut nnext);

                match nnext {
                    ~list::Node(ref mut t, ref mut tail) => {
                        if !t.is_marked(m) {
                            swap(next, tail)
                        }
                    }

                    _ => ()
                }

                GC::check_node(m, &mut **next);
            }

            _ => ()
        }
    }

    pub fn sweep(&mut self, roots: &mut [&mut Visitor]) {
        self.current_mark = !self.current_mark;
        self.mark(roots);
        GC::check_node(self.current_mark, self.heap.head);
    }

    pub fn alloc_pair(&mut self) -> Value {
        let mut p = ~GCPair {
            mark: self.current_mark,
            car: Unit,
            cdr: Unit
        };

        let ptr = {
            let r: &mut GCPair = p;
            r as *mut GCPair
        };
        
        self.heap.insert(p as ~GCollect);
        Pair(::gc::Pair(ptr))
    }

    pub fn alloc_env(&mut self, next: Option<Env>) -> Env {
        let mut env = ~GCEnv { 
            values: ~[], 
            mark: self.current_mark,
            next: next
        };

        let ptr = { 
            let r: &mut GCEnv = env;
            r as *mut GCEnv 
        };

        self.heap.insert(env as ~GCollect);
        ptr
    }
}
