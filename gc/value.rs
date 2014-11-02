use std::fmt;

use gc;
use gmp;
use primitives;

pub mod list {
    use gc;
    use super::Null;
    use super::Pair;
    use super::Value;
    use super::Unit;

    // various functions for manipulating Scheme lists and pairs

    pub fn cons(car: &Value, cdr: &Value, gc: &mut gc::GC) -> gc::Ptr<gc::Pair> {
        gc.alloc(gc::Pair {
            car: car.clone(),
            cdr: cdr.clone()
        })
    }

    pub fn is_list(value: &Value) -> bool {
        let mut ret = true;

        for _ in iter(value, |_| {
            // non-pair value
            ret = false;
            None
        }) { };

        ret
    }

    // utility struct for efficiently building lists iteratively

    #[deriving(Clone)]
    struct ListBuilder {
        fst: gc::ptr::Cell<gc::Pair>,
        ptr: gc::Ptr<gc::Pair>,
    }

    pub static LIST_BUILDER: ListBuilder = ListBuilder {
        // allocate a dummy pair that will in fact point to the first
        // true element of the list to allow fast insertions
        fst: gc::ptr::Cell {
            data: gc::Pair {
                car: Unit,
                cdr: Null
            },

            mark: false
        },

        ptr: gc::Ptr(0 as *mut gc::ptr::Cell<gc::Pair>)
    };

    impl ListBuilder {
        #[inline(always)]
        pub fn init(&mut self) {
            self.ptr = gc::Ptr(&mut self.fst);
        }

        #[inline(always)]
        pub fn append(&mut self, v: &Value, gc: &mut gc::GC) {
            // this one is GC'd
            let pair = cons(v, &Null, gc);
            self.ptr.cdr = Pair(pair);
            self.ptr = pair;
        }

        #[inline(always)]
        pub fn get_list(&self) -> Value {
            // remove dummy node
            self.fst.data.cdr.clone()
        }
    }

    // struct for iterator over Scheme linked lists
    // iterates until it reaches a Null. If a non-pair or non-null value is
    // encountered, it raises the invalid_list condition

    struct ListIterator<'a, 'b> {
        cur: &'a Value,
        trap: |Value|:'b -> Option<Value>
    }

    impl<'a, 'b> Iterator<Value> for ListIterator<'a, 'b> {
        fn next(&mut self) -> Option<Value> {
            match self.cur {
                &Null => None,
                &Pair(ref p) => {
                    self.cur = &p.cdr;
                    Some(p.car.clone())
                }

                _ => (self.trap)(self.cur.clone())
            }
        }
    }

    pub fn iter<'a, 'b>(v: &'a Value, trap: |Value|:'b -> Option<Value>) -> ListIterator<'a, 'b> {
        ListIterator { cur: v, trap: trap }
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
    Closure(gc::Ptr<gc::Closure>),
    Null,
    Num(gmp::Mpz),
    Pair(gc::Ptr<gc::Pair>),
    Primitive(primitives::Prim, &'static str),
    String(gc::Ptr<gc::String>),
    Symbol(gc::Ptr<gc::String>),
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

impl fmt::Show for Value {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Bool(true) => fmt.pad("#t"),
            &Bool(false) => fmt.pad("#f"),
            &Closure(_) => fmt.pad("#<procedure>"),
            &Null => fmt.pad("'()"),
            &Num(ref i) => fmt.pad(format!("{:s}", i.to_string()).as_slice()),
            &Pair(p) => fmt.pad(format!("({:s})", p.to_string()).as_slice()),
            &Primitive(_, _) => fmt.pad("#<procedure>"),
            &String(s) => fmt.pad(format!("{:s}", s.to_string()).as_slice()),
            &Symbol(h) => fmt.pad(format!("'{:s}", h.to_string()).as_slice()),
            &Unit => fmt.pad("")
        }
    }
}

impl PartialEq for Value {
    // physical compareason
    fn eq(&self, v: &Value) -> bool {
        use std::mem::transmute;

        match (self, v) {
            // eq do object-compareason
            (&Pair(p1), &Pair(p2)) => p1 == p2,
            (&Closure(cl1), &Closure(cl2)) => *cl1 == *cl2,

            (&Primitive(p1, _), &Primitive(p2, _)) => {
                let p1: *const () = unsafe { transmute(p1) };
                let p2: *const () = unsafe { transmute(p2) };
                p1 == p2
            }

            (&Num(ref i), &Num(ref j)) => i == j,
            (&Bool(b1), &Bool(b2)) => b1 == b2,
            (&Symbol(h1), &Symbol(h2)) => (h1) == (h2),
            (&String(s1), &String(s2)) => (s1) == (s2),
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
            (&Pair(mut p1), &Pair(p2)) => {
                p1.car.compare(&p2.car) && p1.cdr.compare(&p2.cdr)
            }

            (&Closure(cl1), &Closure(cl2)) => *cl1 == *cl2,

            (&Primitive(p1, _), &Primitive(p2, _)) => {
                use std::mem::transmute;
                let p1: *const () = unsafe { transmute(p1) };
                let p2: *const () = unsafe { transmute(p2) };
                p1 == p2
            }

            (&Num(ref i), &Num(ref j)) => i == j,
            (&Bool(b1), &Bool(b2)) => b1 == b2,
            (&Symbol(h1), &Symbol(h2)) => (h1) == (h2),

            (&String(mut s1), &String(mut s2)) => {
                let s1 = s1.str.as_slice();
                let s2 = s2.str.as_slice();
                s1 == s2
            }

            (&Null, &Null) => true,
            (&Unit, &Unit) => true,
            _ => false
        }
    }
}
