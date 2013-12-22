use gc;
use gc::closure::GClosure;
use gc::env::GCEnv;
use gc::pair::GCPair;
use gc::string::GCString;
use gc::value;
use gc::visit::Visitor;
use std::hashmap::HashMap;
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

    // string interner
    // keeps in memory all the string constants loaded by the program.
    // They include notably stirng literals, but also symbol names
    // Currently, the interned strings are managed by the GC although they are
    // not currently collected. This may change in the future
    // all interned strings are immutable
    interner: HashMap<~str, gc::String>,

    current_mark: bool
}

impl GC {
    pub fn new() -> ~GC {
        ~GC {
            heap: list::List::new(),
            interner: HashMap::new(),
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

    // intern a new string into the interner, and return an handle to it
    // if the string is already interned, just returns an handle on it

    pub fn intern(&mut self, s: ~str) -> gc::String {
        match self.interner.find(&s) {
            Some(h) => return *h,
            None => ()
        }

        // not found, allocate a new interned string
        // FIXME: could-we avoid copy here ?
        let interned = self.alloc_string(s.clone(), false);
        self.interner.insert(s, interned);
        interned
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

    pub fn alloc_closure(&mut self, arity: u8, variadic: bool,
        env: gc::Env, pc: u64) -> gc::Closure {
        let mut cl = ~GClosure {
            pc: pc,
            arity: arity,
            env: env,
            variadic: variadic,
            mark: self.current_mark
        };
        let ptr = {
            let r: &mut GClosure = cl;
            r as *mut GClosure
        };

        self.heap.insert(cl as ~GCollect);
        gc::Closure(ptr)
    }

    pub fn alloc_string(&mut self, str: ~str, mutable: bool) -> gc::String {
        let mut s = ~GCString {
            str: str,
            mark: self.current_mark,
            mutable: mutable
        };
        let ptr = { let r: &mut GCString = s; r as *mut GCString };
        self.heap.insert(s as ~GCollect);
        gc::String(ptr)
    }
}
