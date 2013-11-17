pub use self::collect::GC;
pub use self::collect::GCEnv;
pub use self::value::Value;

mod collect;
pub mod value;
pub mod visit;

// wrapper type for GC-managed Envs
pub type Env = *mut GCEnv;

// wrapper type for GC-managed Pairs to avoid unsafe blocks everywhere
pub struct Pair(*mut collect::GCPair);

impl Clone for Pair {
    fn clone(&self) -> Pair {
        Pair(**self)
    }
}

impl ToStr for Pair {
    fn to_str(&self) -> ~str {
        let car = unsafe { (***self).car.to_str() };

        match unsafe { (***self).cdr } {
            value::Pair(p) => format!("{:s} {:s}", car, p.to_str()),
            value::Null => format!("{:s}", car),
            v => format!("{:s} . {:s}", car, v.to_str())
        }
    }
}

impl Pair {
    pub fn car(self) -> Value {
        unsafe { (**self).car }
    }

    pub fn cdr(self) -> Value {
        unsafe { (**self).cdr }
    }

    pub fn setcar(self, car: &Value) {
        unsafe { (**self).car = *car; }
    }

    pub fn setcdr(self, cdr: &Value) {
        unsafe { (**self).cdr = *cdr; }
    }
}


