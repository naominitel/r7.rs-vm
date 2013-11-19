use gc;
use gc::env::GCEnv;
use gc::pair::GCPair;
use gc::value;
use gc::visit::Visitor;
mod list;

// Basic trait for garbage collected values
// must keep a boolean mark for the mark and
// sweep collection algorithm

pub trait GCollect {
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

// The GC itself
// Keeps all the allocated values in a linked list of cells, where a cell
// is a couple of an owned box containing the allocated object, and a raw
// ptr to the same object

struct GC {
    heap:  ~list::List<~GCollect>,
    current_mark: bool
}

impl GC {
    pub fn new() -> ~GC {
        ~GC {
            heap: list::List::new(),
            current_mark: false
        }
    }

    fn mark(&mut self, roots: &mut [&mut Visitor]) {
        for v in roots.mut_iter() {
            v.visit(self.current_mark);
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
        debug!("GC: Start collection");
        self.current_mark = !self.current_mark;
        self.mark(roots);
        GC::check_node(self.current_mark, self.heap.head);
    }

    pub fn alloc_pair(&mut self) -> gc::Pair {
        let mut p = ~GCPair {
            mark: self.current_mark,
            car: value::Unit,
            cdr: value::Unit
        };

        let ptr = {
            let r: &mut GCPair = p;
            r as *mut GCPair
        };
        
        self.heap.insert(p as ~GCollect);
        gc::Pair(ptr)
    }

    pub fn alloc_env(&mut self, size: u64, next: Option<gc::Env>) -> gc::Env {
        let mut env = ~GCEnv { 
            values: ::std::vec::with_capacity(size as uint),
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
