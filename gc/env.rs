use gc::value;
use gc::collect;
use gc::visit::Visitor;

// a garbage-collected Scheme environment

type EnvItem = (bool, value::Value);

pub struct GCEnv {
    pub values: Vec<EnvItem>,
    pub next: Option<Env>,
    pub mark: bool
}

impl collect::GCollect for GCEnv {
    fn is_marked(&self, m: bool) -> bool {
        self.mark == m
    }

    fn mark(&mut self, m: bool) {
        self.mark = m;

        for &(d, ref mut v) in self.values.mut_iter() {
            if d {
                v.visit(m);
            }
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
                *self.values.get_mut(addr as uint) = (true, value.clone());
            }

            else if addr == self.values.len() as u64 {
                self.values.push((true, value.clone()));
            }

            else {
                for _ in range(self.values.len() as u64, addr - 1) {
                    self.values.push((false, value::Unit));
                }

                self.values.push((true, value.clone()));
            }

            return
        }

        match self.next {
            Some(e) => unsafe {
                (*e).store(value, addr - self.values.capacity() as u64)
            },

            None => fail!("Value not in environment")
        }
    }

    pub fn fetch(&mut self, addr: u64) -> value::Value {
        if addr < self.values.capacity() as u64 {
            if addr < self.values.len() as u64 {
                let &(d, ref v) = self.values.get(addr as uint);

                if d {
                    v.clone()
                }

                else { fail!("Reference to an identifier before its definition") }
            }

            else { fail!("Reference to an identifier before its definition") }
        }

        else { match self.next {
            Some(e) => unsafe {
                (*e).fetch(addr - self.values.capacity() as u64)
            },

            None => fail!("Value not in environment")
        }}
    }

    // dump he environment for debugging purposes

    #[allow(dead_code)]
    pub fn dump(&self) {
        print!("[ ");

        for i in self.values.iter() {
            debug!("{:?} ", i);
        }

        print!("]");

        match self.next {
            Some(e) => {
                print!(" => ");
                unsafe { (*e).dump() };
            }

            None => debug!(" End.")
        }
    }
}

// wrapper type for GC-managed Envs
pub type Env = *mut GCEnv;

