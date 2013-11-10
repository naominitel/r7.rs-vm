pub use self::collect::GC;
pub use self::value::Value;

mod collect;
mod value;
mod visit;

// wrapper type for GC-managed Envs
pub type Env = *mut collect::GCEnv;

// wrapper type for GC-managed Pairs to avoid unsafe blocks everywhere
pub struct Pair(*mut collect::GCPair);

impl Clone for Pair {
    fn clone(&self) -> Pair {
        Pair(**self)
    }
}

impl Pair {
    pub fn car(self) -> Value {
        unsafe { (**self).car }
    }

    pub fn cdr(self) -> Value {
        unsafe { (**self).cdr }
    }
}


