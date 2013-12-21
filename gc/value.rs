use gc;
use gmp;
use primitives;
use vm;

pub mod list {
    use gc;
    use super::Null;
    use super::Pair;
    use super::Value;
    use super::Unit;

    // various functions for manipulating Scheme lists and pairs

    pub fn cons(car: &Value, cdr: &Value, gc: &mut gc::GC) -> gc::Pair {
        let p = gc.alloc_pair();

        p.setcar(car);
        p.setcdr(cdr);
        p
    }

    pub fn is_list(value: &Value) -> bool {
        let mut ret = true;

        invalid_list::cond.trap(|_| {
            // non-pair value
            ret = false;
            None
        }).inside(|| { for _ in iter(value) { } });

        ret
    }

    // utility struct for efficiently building lists iteratively

    #[deriving(Clone)]
    struct ListBuilder {
        fst: gc::pair::GCPair,
        ptr: gc::Pair,
    }

    pub static LIST_BUILDER: ListBuilder = ListBuilder {
        // allocate a dummy pair that will in fact point to the first
        // true element of the list to allow fast insertions
        fst: gc::pair::GCPair {
            car: Unit,
            cdr: Null,
            mark: false
        },

        ptr: gc::Pair(0 as *mut gc::pair::GCPair)
    };

    impl ListBuilder {
        #[inline(always)]
        pub fn init(&mut self) {
            self.ptr = gc::Pair(&mut self.fst);
        }

        #[inline(always)]
        pub fn append(&mut self, v: &Value, gc: &mut gc::GC) {
            // this one is GC'd
            let pair = cons(v, &Null, gc);
            self.ptr.setcdr(&Pair(pair));
            self.ptr = pair;
        }

        #[inline(always)]
        pub fn get_list(&self) -> Value {
            // remove dummy node
            self.fst.cdr.clone()
        }
    }

    // struct for iterator over Scheme linked lists
    // iterates until it reaches a Null. If a non-pair or non-null value is
    // encountered, it raises the invalid_list condition

    condition! {
        pub invalid_list: ::gc::Value -> Option<::gc::Value>;
    }

    struct ListIterator<'a> {
        cur: &'a Value
    }

    impl<'a> Iterator<Value> for ListIterator<'a> {
        fn next(&mut self) -> Option<Value> {
            match self.cur {
                &Null => None,
                &Pair(ref p) => {
                    let ret = p.car();
                    self.cur = p.cdr_ref();
                    Some(ret)
                }

                _ => invalid_list::cond.raise(self.cur.clone())
            }
        }
    }

    pub fn iter<'a>(v: &'a Value) -> ListIterator<'a> {
        ListIterator { cur: v }
    }
}

// Type for representing Scheme values manipulated by the VM
// a Value can be either
//   * a pair of two values (managed by the GC)
//   * a closure with its program and environment managed by the GC
//   * a primitive (in-VM implemented function)
//   * integer data types managed by copy
//   * unit, the void value
//   * null, a singleton value for '()

// FIXME: bug #10501 #[deriving(Clone)]
pub enum Value {
    Bool(bool),
    Closure(gc::Closure),
    Null,
    Num(gmp::Mpz),
    Pair(gc::Pair),
    Primitive(primitives::Prim, &'static str),
    // FIXME: garbage-collect strings
    String(gc::String),
    Symbol(vm::Handle),
    Unit
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            &Bool(b) => Bool(b),
            &Closure(cl) => Closure(cl),
            &Null => Null,
            &Num(ref n) => Num(n.clone()),
            &Pair(p) => Pair(p),
            &Primitive(p, n) => Primitive(p, n),
            &String(s) => String(s),
            &Symbol(h) => Symbol(h),
            &Unit => Unit
        }
    }
}

impl ToStr for Value {
    fn to_str(&self) -> ~str {
        match self {
            &Bool(true) => ~"#t",
            &Bool(false) => ~"#f",
            &Closure(_) => ~"#<procedure>",
            &Null => ~"'()",
            &Num(ref i) => i.to_str(),
            &Pair(p) => format!("({:s})", p.to_str()),
            &Primitive(_, _) => ~"#<procedure>",
            &String(s) => s.to_str(),
            &Symbol(h) => format!("'{:s}", h.to_str()),
            &Unit => ~""
        }
    }
}

impl Eq for Value {
    // physical compareason
    fn eq(&self, v: &Value) -> bool {
        use std::cast::transmute;

        match (self, v) {
            (&Pair(p1), &Pair(p2)) => {
                // eq do object-compareason
                (*p1) == (*p2)
            }

            (&Closure(cl1), &Closure(cl2)) => {
                (*cl1) == (*cl2)
            }

            (&Primitive(p1, _), &Primitive(p2, _)) => {
                let p1: *() = unsafe { transmute(p1) };
                let p2: *() = unsafe { transmute(p2) };
                p1 == p2
            }

            (&Num(ref i), &Num(ref j)) => i == j,
            (&Bool(b1), &Bool(b2)) => b1 == b2,
            (&Symbol(h1), &Symbol(h2)) => h1 == h2,

            (&Null, &Null) => true,
            (&Unit, &Unit) => true,

            _ => false
        }
    }
}

impl Value {
    // structural compareason
    pub fn compare(&self, other: &Value) -> bool {
        match (self, other) {
            (&Pair(p1), &Pair(p2)) => {
                p1.car().compare(&p2.car()) &&
                    p1.cdr().compare(&p2.cdr())
            }

            (&Closure(cl1), &Closure(cl2)) => {
                (*cl1) == (*cl2)
            }

            (&Primitive(p1, _), &Primitive(p2, _)) => {
                use std::cast::transmute;
                let p1: *() = unsafe { transmute(p1) };
                let p2: *() = unsafe { transmute(p2) };
                p1 == p2
            }

            (&Num(ref i), &Num(ref j)) => i == j,
            (&Bool(b1), &Bool(b2)) => b1 == b2,
            (&Symbol(h1), &Symbol(h2)) => h1 == h2,

            (&Null, &Null) => true,
            (&Unit, &Unit) => true,

            _ => false
        }
    }
}
