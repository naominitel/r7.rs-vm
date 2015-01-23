use std::fmt;
use std::ops;
use gc;

// an abstraction for GC'd pointers

#[derive(Clone)]
pub struct Cell<T> {
    pub data: T,
    pub mark: bool
}

pub struct Ptr<T>(pub *mut Cell<T>);
impl<T> Copy for Ptr<T> {}

// FIXME: why aren't those derivable ?

impl<T> PartialEq for Ptr<T> {
    fn eq(&self, &Ptr(other): &Ptr<T>) -> bool {
        let &Ptr(ptr) = self;
        ptr == other
    }
}

impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Ptr<T> {
        let &Ptr(ptr) = self;
        Ptr(ptr)
    }
}

impl<T: gc::visit::Visitor> gc::visit::Visitor for Ptr<T> {
    fn visit(&mut self, m: bool) {
        let &mut Ptr(ptr) = self;
        unsafe {
            let Cell { ref mut data, ref mut mark } = *ptr;
            if *mark != m {
                *mark = m;
                data.visit(m);
            }
        }
    } 
}

impl<T> ops::Deref for Ptr<T> {
    type Target = T;
    // should be optimized as a no-op
    #[inline(always)]
    fn deref<'a>(&'a self) -> &'a T {
        let &Ptr(ptr) = self;
        unsafe {
            let Cell { ref data, .. } = *ptr;
            ::std::mem::transmute(data)
        }
    }
}

impl<T> ops::DerefMut for Ptr<T> {
    // should be optimized as a no-op
    #[inline(always)]
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        let &mut Ptr(ptr) = self;
        unsafe {
            let Cell { ref data, .. } = *ptr;
            ::std::mem::transmute(data)
        }
    }
}

impl<T: fmt::String> fmt::String for Ptr<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let &Ptr(ptr) = self;
        unsafe { (*ptr).data.fmt(fmt) }
    }
}
