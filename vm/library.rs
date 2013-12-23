use gc::Env;
use gc::String;
use std::io;
use std::to_bytes::Cb;
use std::vec::VecIterator;
use std::vec;

static DEFAULT_PREFIX: &'static str = "/usr/local/";

#[deriving(Eq, Clone)]
pub struct LibName(~[~str]);

impl LibName {
    fn iter<'a>(&'a self) -> VecIterator<'a, ~str> {
        (**self).iter()
    }
}

impl IterBytes for LibName {
    fn iter_bytes(&self, lsb0: bool, f: Cb) -> bool {
        (**self).iter_bytes(lsb0, f)
    }
}

// FIXME: for some reason this requires an impl of std::str::Str
// for LibName
// impl fmt::String for LibName {
//    fn fmt(obj: &LibName, f: &mut fmt::Formatter) {
//        write!(f.buf, "( ");
//
//        for part in obj.iter() {
//            write!(f.buf, "{:s} ", *part);
//        }
//
//        write!(f.buf, ")");
//    }
// }

pub struct Library {
    name: ~LibName,
    prog: ~[u8],
    env: Env,

    imports: ~[~LibName],
    sym_table: ~[String],
    exports: u64
}

impl Library {
    pub fn library_path(prefix: Option<~str>) -> ~[~Path] {
        let prfx = match prefix {
            Some(s) => ~Path::new(s),
            None => ~Path::new(DEFAULT_PREFIX.into_owned())
        };

        ~[prfx]
    }

    pub fn load_file(gc: &mut ::gc::GC, path: &Path, name: ~LibName) -> ~Library {
        /* found library */
        let mut f = match io::File::open(path) {
            Some(f) => f,
            None => fail!("Impossible to open library file")
        };

        let mut magic = [0, .. 3];
        f.read(magic.mut_slice_from(0));

        if f.read_u8() != 0x01 {
            fail!("Unsupported file format version.");
        }

        // reserved
        f.seek(28, io::SeekCur);

        let sym_tab_off = f.read_be_u64();
        let imports_off = f.read_be_u64();
        let exports_off = f.read_be_u64();
        let text_off = f.read_be_u64();

        f.seek(imports_off as i64, io::SeekSet);
        let imports_count = f.read_be_u64();

        let mut imports = ::std::vec::with_capacity(imports_count as uint);
        for _ in range(0, imports_count) {
            // read libname
            let length = f.read_be_u64();
            let mut lname = ::std::vec::with_capacity(length as uint);

            for _ in range(0, length) {
                let size = f.read_be_u64();
                let mut part = ~"";

                for _ in range(0, size) {
                    let ch = f.read_u8();
                    part.push_char(ch as char);
                }

                lname.push(part);
            }

            imports.push(~LibName(lname));
        }

        f.seek(sym_tab_off as i64, io::SeekSet);
        let sym_count = f.read_be_u64();
        let mut mod_symt = vec::with_capacity(sym_count as uint);
        debug!("{:u} symbols in table", sym_count);

        for _ in range(0, sym_count) {
            let sz = f.read_be_u64();
            let mut s = ~"";

            for _ in range(0, sz) {
                let b = f.read_u8();
                s.push_char(b as char);
            }

            let h = gc.intern(s);
            mod_symt.push(h);
        }

        f.seek(exports_off as i64, io::SeekSet);
        let exports_count = f.read_be_u64();

        let env = gc.alloc_env(exports_count, None);

        debug!("Trying to access program text section at {:x}", text_off);
        f.seek(text_off as i64, io::SeekSet);
        let text_size = f.read_be_u64();
        let mut text = ::std::vec::with_capacity(text_size as uint);

        for _ in range(0, text_size) {
            let b = f.read_u8();
            text.push(b);
        }

        debug!("Sucessfully loaded library");
        ~Library {
            env: env, prog: text, name: name, sym_table: mod_symt,
            imports: imports, exports: exports_count
        }
    }

    pub fn load(gc: &mut ::gc::GC, name: &LibName, lpath: ~[~Path]) -> ~Library {
        let mut lpath = lpath;

        for p in lpath.mut_iter() {
            for part in name.iter() {
                p.push(part.as_slice());
            }

            if p.is_file() {
                debug!("Trying {:s}", p.display().to_str());
                Library::load_file(gc, &**p, ~name.clone());
            }
        }

        fail!("Impossible to find library in path");
    }
}
