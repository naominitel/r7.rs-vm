use gc::collect;
use std::fmt;

// a garbage-collected Scheme string

#[packed]
pub struct GCString {
    pub str: ::std::string::String,
    pub mutable: bool,
    pub mark: bool
}

impl collect::GCollect for GCString {
    #[inline(always)]
    fn is_marked(&self, m: bool) -> bool {
        self.mark == m
    }

    #[inline(always)]
    fn mark(&mut self, m: bool) {
        self.mark = m
    }
}

#[deriving(PartialEq)]
pub struct String(pub *mut GCString);

impl Clone for String {
    fn clone(&self) -> String {
        let &String(ptr) = self;
        String(ptr)
    }
}

impl fmt::Show for String {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let &String(ptr) = self;
        unsafe { fmt.pad((*ptr).str.as_slice()) }
    }
}

impl Str for String {
    fn as_slice<'a>(&'a self) -> &'a str {
        let &String(ptr) = self;
        unsafe { (*ptr).str.as_slice() }
    }
}
