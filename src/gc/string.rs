use std::fmt;

// a garbage-collected Scheme string

#[repr(packed)]
pub struct String {
    pub str: ::std::string::String,
    pub mutable: bool
}

impl fmt::Display for String {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(&self.str)
    }
}
