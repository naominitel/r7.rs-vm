use gc;
use gc::Value;
use gc::value::Unit;
use gc::visit::Visitor;
use std::vec;
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

impl Drop for GCPair {
    fn drop(&mut self) {
        debug!("Dropping pair whose car is {:?}", self.car);
    }
}

// a garbage-collected Scheme environment
pub struct GCEnv {
    values: ~[Value],
    next: Option<gc::Env>,
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

impl GCEnv {
    pub fn store(&mut self, value: &Value, addr: u64) {
        if addr < self.values.len() as u64 {
            self.values[addr] = *value;
        }

        match self.next {
            Some(e) => unsafe {
                (*e).store(value, addr - self.values.len() as u64)
            },

            None => fail!("Value not in environment")
        }
    }

    pub fn fetch(&mut self, addr: u64) -> Value {
        if addr < self.values.len() as u64 {
            self.values[addr]
        }

        else { match self.next {
            Some(e) => unsafe {
                (*e).fetch(addr - self.values.len() as u64)
            },

            None => fail!("Value not in environment")
        }}
    }

    pub fn dump(&self) {
        print("[ ");

        for i in self.values.iter() {
            debug!("{:?} ", i);
        }

        print("]");

        match self.next {
            Some(e) => {
                print(" => ");
                unsafe { (*e).dump() };
            }

            None => debug!(" End.")
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
        ~GC {
            heap: list::List::new(),
            white: list::List::new(),
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

                if match nnext {
                    ~list::Node(ref mut t, ref mut tail) => {
                        if !t.is_marked(m) {
                            swap(next, tail);
                            false
                        }

                        else { true }
                    }

                    _ =>  true
                } {
                    // nothing to remove, put everything
                    // back in place and continue
                    swap(next, &mut nnext);
                    return GC::check_node(m, &mut **next);
                }
            }

            _ => return
        }

        // we didn't finished checking this node
        GC::check_node(m, node)
    }

    pub fn sweep(&mut self, roots: &mut [&mut Visitor]) {
        self.current_mark = !self.current_mark;
        self.mark(roots);
        GC::check_node(self.current_mark, self.heap.head);
    }

    pub fn alloc_pair(&mut self) -> gc::Pair {
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
        ::gc::Pair(ptr)
    }

    pub fn alloc_env(&mut self, size: u64, next: Option<gc::Env>) -> gc::Env {
        let mut env = ~GCEnv { 
            values: vec::with_capacity(size as uint),
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
