use gc::value;
use gc::collect;
use gc::visit::Visitor;

// a garbage-collected Scheme environment
pub struct GCEnv {
    values: ~[value::Value],
    next: Option<Env>,
    mark: bool
}

impl collect::GCollect for GCEnv {
    fn is_marked(&self, m: bool) -> bool {
        self.mark == m
    }

    fn mark(&mut self, m: bool) {
        self.mark = m;

        for v in self.values.mut_iter() {
            v.visit(m);
        }

        match self.next {
            Some(ref mut e) => e.visit(m),
            None => ()
        }
    }
}

impl GCEnv {
    pub fn store(&mut self, value: &value::Value, addr: u64) {
        if addr < self.values.capacity() as u64 {
            if addr < self.values.len() as u64 {
                self.values[addr] = value.clone();
            }

            else if addr == self.values.len() as u64 {
                self.values.push(value.clone());
            }

            else {
                for _ in range(self.values.len() as u64, addr - 1) {
                    self.values.push(value::Unit);
                }

                self.values.push(value.clone());
            }

            return
        }

        match self.next {
            Some(e) => unsafe {
                (*e).store(value, addr - self.values.len() as u64)
            },

            None => fail!("Value not in environment")
        }
    }

    pub fn fetch(&mut self, addr: u64) -> value::Value {
        if addr < self.values.len() as u64 {
            self.values[addr].clone()
        }

        else { match self.next {
            Some(e) => unsafe {
                (*e).fetch(addr - self.values.len() as u64)
            },

            None => fail!("Value not in environment")
        }}
    }

    pub fn dump(&self) {
        print("[ ");

        for i in self.values.iter() {
            debug!("{:?} ", i);
        }

        print("]");

        match self.next {
            Some(e) => {
                print(" => ");
                unsafe { (*e).dump() };
            }

            None => debug!(" End.")
        }
    }
}

// wrapper type for GC-managed Envs
pub type Env = *mut GCEnv;

