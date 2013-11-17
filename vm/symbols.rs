use std::hashmap::HashMap;

struct Symbol {
    name: ~str
}

pub type Handle = *Symbol;

impl Symbol {
    // allocates a new symbol to put on the symbol table
    // a symbol stays in memory forever once it has been
    // initialized
    fn new(tb: &mut SymTable, name: ~str) -> Handle {
        let sym = ~Symbol { name: name };
        let ptr = {
            let r: &Symbol = sym;
            r as *Symbol
        };

        // keep a list of all allocated symbols to avoid
        // memleaks when the VM terminates
        tb.syms.push(sym);
        ptr
    }

    fn to_string(&self) -> ~str {
        self.name.clone()
    }
}

struct SymTable {
    known_symbols: HashMap<~str, Handle>,

    // list of all allocated symbols
    syms: ~[~Symbol]
}

impl SymTable {
    pub fn dump(&self) {
        debug!("Symbols in table: ");

        for (s, _) in self.known_symbols.iter() {
            debug!("- {:s}", *s);
        }
    }

    pub fn new() -> ~SymTable {
        ~SymTable {
            known_symbols: HashMap::new(),
            syms: ~[]
        }
    }

    pub fn reserve(&mut self, count: uint) {
        self.syms.reserve(count);
    }

    pub fn get_handle(&self, sym: &str) -> Handle {
        *self.known_symbols.get(&sym.into_owned())
    }

    pub fn create_symbol(&mut self, sym: ~str) -> Handle {
        if !self.known_symbols.contains_key(&sym.to_owned()) {
            let h = Symbol::new(self, sym.clone());
            self.known_symbols.insert(sym, h);
            return h
        }

        fail!("Symbol already in table")
    }

    pub fn get_or_create(&mut self, sym: ~str) -> Handle {
        match self.known_symbols.find(&sym) {
            Some(h) => return *h,
            None => ()
        }

        // not found
        self.create_symbol(sym)
    }
}
