use std::io;
use std::io::fs::PathExtensions;
use std::slice::Items;

use gc;

static DEFAULT_PREFIX: &'static str = "/usr/local/";

#[deriving(Eq, Clone, Hash, PartialEq)]
pub struct LibName(pub Vec<String>);

impl LibName {
    fn iter<'a>(&'a self) -> Items<'a, String> {
        let &LibName(ref vec) = self;
        vec.iter()
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
    pub name: Box<LibName>,
    pub prog: Vec<u8>,
    pub env: gc::Ptr<gc::Env>,

    pub imports: Vec<Box<LibName>>,
    pub sym_table: Vec<gc::Ptr<gc::String>>,
    pub exports: u64
}

impl Library {
    pub fn library_path(prefix: Option<String>) -> Vec<Box<Path>> {
        let prfx = match prefix {
            Some(s) => box Path::new(s),
            None => box Path::new(DEFAULT_PREFIX.into_string())
        };

        vec!(prfx)
    }

    pub fn load_file(gc: &mut ::gc::GC, path: &Path, name: Box<LibName>) -> Box<Library> {
        /* found library */
        let mut f = match io::File::open(path) {
            Ok(f) => f,
            Err(_) => panic!("Impossible to open library file")
        };

        let mut magic = [0, .. 3];
        let _ = f.read(magic.as_mut_slice());

        if f.read_u8().unwrap() != 0x01 {
            panic!("Unsupported file format version.");
        }

        // reserved
        let _ = f.seek(28, io::SeekCur);

        let sym_tab_off = f.read_be_u64().unwrap();
        let imports_off = f.read_be_u64().unwrap();
        let exports_off = f.read_be_u64().unwrap();
        let text_off = f.read_be_u64().unwrap();

        let _ = f.seek(imports_off as i64, io::SeekSet);
        let imports_count = f.read_be_u64().unwrap();

        let mut imports = Vec::with_capacity(imports_count as uint);
        for _ in range(0, imports_count) {
            // read libname
            let length = f.read_be_u64().unwrap();
            let mut lname = Vec::with_capacity(length as uint);

            for _ in range(0, length) {
                let size = f.read_be_u64().unwrap();
                let mut part = ::std::string::String::from_str("");

                for _ in range(0, size) {
                    let ch = f.read_u8().unwrap();
                    part.push(ch as char);
                }

                lname.push(part);
            }

            imports.push(box LibName(lname));
        }

        let _ = f.seek(sym_tab_off as i64, io::SeekSet);
        let sym_count = f.read_be_u64().unwrap();
        let mut mod_symt = Vec::with_capacity(sym_count as uint);
        debug!("{:u} symbols in table", sym_count);

        for _ in range(0, sym_count) {
            let sz = f.read_be_u64().unwrap();
            let mut s = String::from_str("");

            for _ in range(0, sz) {
                let b = f.read_u8().unwrap();
                s.push(b as char);
            }

            let h = gc.intern(s);
            mod_symt.push(h);
        }

        let _ = f.seek(exports_off as i64, io::SeekSet);
        let exports_count = f.read_be_u64().unwrap();

        let env = gc.alloc(gc::Env {
            values: Vec::with_capacity(exports_count as uint),
            next: None
        });

        debug!("Trying to access program text section at {:x}", text_off);
        let _ = f.seek(text_off as i64, io::SeekSet);
        let text_size = f.read_be_u64().unwrap();
        let mut text = Vec::with_capacity(text_size as uint);

        for _ in range(0, text_size) {
            let b = f.read_u8().unwrap();
            text.push(b);
        }

        debug!("Sucessfully loaded library");
        box Library {
            env: env, prog: text, name: name, sym_table: mod_symt,
            imports: imports, exports: exports_count
        }
    }

    pub fn load(gc: &mut ::gc::GC, name: &LibName, lpath: Vec<Box<Path>>) -> Box<Library> {
        let mut lpath = lpath;

        for p in lpath.iter_mut() {
            for part in name.iter() {
                p.push(part.as_slice());
            }

            if p.is_file() {
                debug!("Trying {:s}", p.display().to_string());
                Library::load_file(gc, &**p, box name.clone());
            }
        }

        panic!("Impossible to find library in path");
    }
}
