use std::libc;
mod list;

trait GCollect {
    fn is_marked(&self, m: bool) -> bool;
    fn mark(&mut self, mark: bool);
}

struct GCPair {
    car: Value,
    cdr: Value,
    mark: bool
}

struct _Env {
    values: ~[Value],
    next: Option<Env>,
    mark: bool
}

impl GCollect for GCPair {
    fn is_marked(&self, m: bool) -> bool {
        self.mark == m
    }

    fn mark(&mut self, m: bool) {
        self.mark = m;
    }
}

impl GCollect for _Env {
    fn is_marked(&self, m: bool) -> bool {
        self.mark == m
    }

    fn mark(&mut self, m: bool) {
        self.mark = m;
    }
}


#[deriving(Clone)]
enum Value {
    Pair(C<GCPair>),
    Closure(uint, Env),
    Unit
}

type Env = C<_Env>;

struct Frame {
    env: Env,
    sp: uint,
    pc: uint,
    caller: Option<~Frame> 
}

struct Stack {
    values: ~[Value]
}

trait GCVisit {
    fn visit(&mut self, gcmark: bool);
}

impl GCVisit for Frame {
    fn visit(&mut self, m: bool) {
        self.env.visit(m);

        do self.caller.as_mut().map |f| {
            f.visit(m);
        };
    }
}

impl GCVisit for C<_Env> {
    fn visit(&mut self, m: bool) {
        if !self.is_marked(m) {
            self.mark(m);

            for v in self.values().mut_iter() {
                v.visit(m);
            }

            let mut n = self.next();
            do n.as_mut().map |e| {
                e.visit(m)
            };
        }
    }
}

impl GCVisit for Value {
    fn visit(&mut self, m: bool) {
        match self {
            &Pair(ref mut pair) => {
                if !pair.is_marked(m) {
                    pair.mark(m);
                    pair.car().visit(m);
                    pair.cdr().visit(m);
                }
            }

            &Closure(_, ref mut env) => {
                env.visit(m);
            }

            // values other than pairs and closures
            // doesn't need to be GC'd
            _ => ()
        }
    }
}

impl GCVisit for Stack {
    fn visit(&mut self, m: bool) {
        for v in self.values.mut_iter() {
            v.visit(m);
        }
    }
}

struct C<T> {
    ptr:    *mut T
}

impl<T: GCollect> Clone for C<T> {
    fn clone(&self) -> C<T> {
        C { ptr: self.ptr }
    }
}

impl<T: GCollect> C<T> {
    fn is_marked(&self, m: bool) -> bool {
        unsafe { (*self.ptr).is_marked(m) }
    }

    fn mark(&mut self, m: bool) {
        unsafe { (*self.ptr).mark(m); }
    }
}

impl C<_Env> {
    fn values<'a>(&'a mut self) -> &'a mut ~[Value] {
        unsafe { &'a mut (*self.ptr).values }
    }

    fn next(&self) -> Option<Env> {
        unsafe { (*self.ptr).next }
    }
}

impl C<GCPair> {
    fn car(&self) -> Value {
        unsafe { (*self.ptr).car }
    }

    fn cdr(&self) -> Value {
        unsafe { (*self.ptr).cdr }
    }
}

type GCCell = ~(~GCollect, *libc::c_void);

struct GC {
    heap:  ~list::List<GCCell>,
    white: ~list::List<GCCell>,
    current_mark: bool
}

impl GC {
    fn mark(&mut self, roots: &mut [&mut GCVisit]) {
        for v in roots.mut_iter() {
            v.visit(true);
        }
    }

    fn alloc_pair(&mut self) -> Value {
        let p = ~GCPair {
            mark: !self.current_mark,
            car: Unit,
            cdr: Unit
        };
        let ptr: *GCPair = {
            let r: &GCPair = p;
            r as *GCPair
        };
        
        self.heap.insert(~(p as ~GCollect, ptr as *libc::c_void));
        Pair( C { ptr: ptr as *mut GCPair })
    }
}

fn main() {
}
