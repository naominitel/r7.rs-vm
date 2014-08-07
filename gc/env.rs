use gc;

// a garbage-collected Scheme environment

type EnvItem = (bool, gc::value::Value);

pub struct Env {
    pub values: Vec<EnvItem>,
    pub next: Option<gc::Ptr<Env>>
}

impl gc::visit::Visitor for Env {
    fn visit(&mut self, m: bool) {
        for &(d, ref mut v) in self.values.mut_iter() {
            if d { v.visit(m) };
        }

        match self.next {
            Some(ref mut e) => e.visit(m),
            None => ()
        }
    }
}

impl Env {
    pub fn store(&mut self, value: &gc::value::Value, addr: u64) {
        if addr < self.values.capacity() as u64 {
            if addr < self.values.len() as u64 {
                *self.values.get_mut(addr as uint) = (true, value.clone());
            }

            else if addr == self.values.len() as u64 {
                self.values.push((true, value.clone()));
            }

            else {
                for _ in range(self.values.len() as u64, addr - 1) {
                    self.values.push((false, gc::value::Unit));
                }

                self.values.push((true, value.clone()));
            }

            return
        }

        match self.next {
            Some(mut e) => e.store(value, addr - self.values.capacity() as u64),
            None => fail!("Value not in environment")
        }
    }

    pub fn fetch(&mut self, addr: u64) -> gc::value::Value {
        if addr < self.values.capacity() as u64 {
            if addr < self.values.len() as u64 {
                let &(d, ref v) = &self.values[addr as uint];

                if d {
                    v.clone()
                }

                else { fail!("Reference to an identifier before its definition") }
            }

            else { fail!("Reference to an identifier before its definition") }
        }

        else { match self.next {
            Some(mut e) => e.fetch(addr - self.values.capacity() as u64),

            None => fail!("Value not in environment")
        }}
    }

    // dump he environment for debugging purposes

    #[allow(dead_code)]
    pub fn dump(&mut self) {
        print!("[ ");

        for i in self.values.iter() {
            debug!("{:?} ", i);
        }

        print!("]");

        match self.next {
            Some(mut e) => {
                print!(" => ");
                e.dump();
            }

            None => debug!(" End.")
        }
    }
}

