use std::fmt;

// a garbage-collected Scheme string

#[packed]
pub struct String {
    pub str: ::std::string::String,
    pub mutable: bool
}

impl fmt::Show for String {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(self.str.as_slice())
    }
}