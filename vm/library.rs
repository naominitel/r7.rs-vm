use gc::Env;
use std::fmt;
use std::rt::io;
use std::rt::io::File;
use std::str;
use std::to_bytes::Cb;
use std::vec::VecIterator;

static DEFAULT_PREFIX: &'static str = "/usr/local/";

#[deriving(Eq, Clone)]
struct LibName(~[~str]);

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

priv struct LibHeader {
    magic:          [u8, .. 3],
    version:        u8,
    reserved:       [u8, .. 28],
    sym_tab_off:    u64,
    imports_off:    u64,
    exports_off:    u64,
    text_off:       u64
}

struct Library {
    name: ~LibName,
    prog: ~[u8],
    env: Env,

    imports: ~[~LibName],
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

    pub fn load(gc: &mut ::gc::GC, name: &LibName, lpath: ~[~Path]) -> ~Library {
        let mut lpath = lpath;

        for p in lpath.mut_iter() {
            for part in name.iter() {
                p.push(part.as_slice());
            }

            println!("Trying {:s}", p.display().to_str());

            if p.is_file() {
                /* found library */
                let mut f = match File::open(&**p) {
                    Some(f) => f,
                    None => fail!("Impossible to open library file")
                };

                let mut magic = [0, .. 3];
                f.read(magic.mut_slice_from(0));

                let version = f.read_byte();

                // reserved
                f.seek(28, io::SeekCur);

                let sym_tab_off = f.read_be_u64();
                let imports_off = f.read_be_u64();
                let exports_off = f.read_be_u64();
                let text_off = f.read_be_u64();

                f.seek(imports_off as i64, io::SeekSet);
                let imports_count = f.read_be_u64();

                println!("Allocating a vec of size {:u}", imports_count);
                let mut imports = ::std::vec::with_capacity(imports_count as uint);
                for i in range(0, imports_count) {
                    // read libname
                    let length = f.read_be_u64();
                    println!("Allocating a libname of size {:u}", length);
                    let mut lname = ::std::vec::with_capacity(length as uint);

                    for i in range(0, length) {
                        let size = f.read_be_u64();
                        let mut part = ~"";
                        
                        for i in range(0, size) {
                            let ch = f.read_u8();
                            part.push_char(ch as char);
                        }

                        lname.push(part);
                    }

                    imports.push(~LibName(lname));
                }

                f.seek(exports_off as i64, io::SeekSet);
                let exports_count = f.read_be_u64();

                let env = gc.alloc_env(exports_count, None);

                println!("Trying to access program text section at {:x}", text_off);
                f.seek(text_off as i64, io::SeekSet);
                let text_size = f.read_be_u64();
                let mut text = ::std::vec::with_capacity(text_size as uint);

                for i in range(0, text_size) {
                    let b = f.read_u8();
                    text.push(b);
                }

                println!("Sucessfully loaded library");
                return ~Library { env: env, name: ~name.clone(), prog: text, imports: imports, exports: exports_count }
            }
        }

        fail!("Impossible to find library in path");
    }
}
