use std::collections::hashmap::HashMap;
use gc;

#[path = "list.rs"]
mod list;

// Basic trait for garbage collected values
// must keep a boolean mark for the mark and
// sweep collection algorithm

// we have to keep a list of dynamically allocated Cell<T> with Ts of
// differents sizes, so we need dynamic dispatch over their destructors
// this trait will allow us to do so
// unfortunately, checking a cell's mark will also required dynamic dispatch
// so we have to add a method for that
// this trait is private to it won't have other implementors than Cell

trait Collected {
    fn mark(&self) -> bool;
}

impl<T> Collected for gc::ptr::Cell<T> {
    fn mark(&self) -> bool {
        self.mark
    }
}

// The GC itself
// Keeps all the allocated values in a linked list of cells, where a cell
// is a couple of an owned box containing the allocated object, and a raw
// ptr to the same object

pub struct GC {
    heap: Box<list::List<Box<Collected + 'static>>>,

    // string interner
    // keeps in memory all the string constants loaded by the program.
    // They include notably string literals, but also symbol names
    // Currently, the interned strings are managed by the GC although they are
    // not currently collected. This may change in the future
    // all interned strings are immutable
    interner: HashMap<String, gc::Ptr<gc::String>>,

    current_mark: bool
}

impl GC {
    pub fn new() -> Box<GC> {
        box GC {
            heap: list::List::new(),
            interner: HashMap::new(),
            current_mark: false
        }
    }

    fn mark(&mut self, roots: &mut [&mut gc::visit::Visitor]) {
        for v in roots.iter_mut() {
            v.visit(self.current_mark);
        }
    }

    fn check_node(m: bool, node: &mut list::ListNode<Box<Collected>>) {
        use std::mem::swap;

        match node {
            &list::Node(_, ref mut next) => {
                let mut nnext = box list::Empty;
                swap(next, &mut nnext);

                if match *&mut *nnext {
                    list::Node(ref mut t, ref mut tail) => {
                        if t.mark() != m {
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

    pub fn sweep(&mut self, roots: &mut [&mut gc::visit::Visitor]) {
        debug!("GC: Start collection");
        self.current_mark = !self.current_mark;
        self.mark(roots);
        GC::check_node(self.current_mark, &mut *self.heap.head);
    }

    // intern a new string into the interner, and return an handle to it
    // if the string is already interned, just returns an handle on it

    pub fn intern(&mut self, s: String) -> gc::Ptr<gc::String> {
        match self.interner.find(&s) {
            Some(h) => return *h,
            None => ()
        }

        // not found, allocate a new interned string
        // FIXME: could-we avoid copy here ?
        let interned = self.alloc(gc::String {
            str: s.clone(),
            mutable: false
        });

        self.interner.insert(s, interned);
        interned
    }

    pub fn alloc<T: 'static>(&mut self, data: T) -> gc::Ptr<T> {
        use gc::ptr::Cell;

        let mut cell = box Cell {
            data: data,
            mark: self.current_mark
        };

        let ptr: *mut Cell<T> = &mut *cell;
        self.heap.insert(cell as Box<Collected>);

        gc::Ptr(ptr)
    }
}
