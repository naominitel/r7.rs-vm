use gc::collect;

// a garbage-collected Scheme string

#[packed]
pub struct GCString {
    str: ~str,
    mutable: bool,
    mark: bool
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

pub struct String(*mut GCString);

impl Clone for String {
    fn clone(&self) -> String {
        String(**self)
    }
}

impl ToStr for String {
    fn to_str(&self) -> ~str {
        unsafe { (***self).str.clone() }
    }
}

impl Str for String {
    fn as_slice<'a>(&'a self) -> &'a str {
        unsafe { (***self).str.as_slice() }
    }

    fn into_owned(self) -> ~str {
        // we can't avoid copy since there are possibly other String instances
        // pointing to this string (thing that rustc doesn't know because of
        // raw pointers, so he would allow us to move the string out of self).
        // so making a copy is the only safe thing to do
        self.to_str()
    }
}
