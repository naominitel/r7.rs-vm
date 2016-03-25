use std::fs;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use std::path::PathBuf;
use std::slice::Iter;

use gc;

static DEFAULT_PREFIX: &'static str = "/usr/local/";

#[derive(Eq, Clone, Hash, PartialEq)]
pub struct LibName(pub Vec<String>);

impl LibName {
    fn iter<'a>(&'a self) -> Iter<'a, String> {
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

// WHY THE FUCK IS THIS NOT IN STDLIB
fn read_be_u64<T>(file: &mut T) -> io::Result<u64> where T: Read {
    let mut buf = 0u64;
    try!(unsafe {
        file.read_exact(::std::slice::from_raw_parts_mut(
                &mut buf as *mut u64 as *mut u8, 8))
    });
    Ok(u64::from_be(buf))
}

fn read_u8<T>(file: &mut T) -> io::Result<u8> where T: Read {
    let mut buf = [0u8];
    try!(file.read_exact(&mut buf));
    Ok(buf[0])
}

impl Library {
    pub fn library_path(prefix: Option<String>) -> Vec<PathBuf> {
        let prfx = match prefix {
            Some(s) => PathBuf::from(&s),
            None => PathBuf::from(&DEFAULT_PREFIX)
        };

        vec!(prfx)
    }

    pub fn load_file(gc: &mut ::gc::GC, path: &Path, name: Box<LibName>) -> Box<Library> {
        /* found library */
        let mut f = match fs::File::open(path) {
            Ok(f) => f,
            Err(_) => panic!("Impossible to open library file")
        };

        let mut magic = [0; 3];
        let _ = f.read(&mut magic);

        if read_u8(&mut f).unwrap() != 0x01 {
            panic!("Unsupported file format version.");
        }

        // reserved
        let _ = f.seek(io::SeekFrom::Current(28));

        let sym_tab_off = read_be_u64(&mut f).unwrap();
        let imports_off = read_be_u64(&mut f).unwrap();
        let exports_off = read_be_u64(&mut f).unwrap();
        let text_off = read_be_u64(&mut f).unwrap();

        let _ = f.seek(io::SeekFrom::Start(imports_off));
        let imports_count = read_be_u64(&mut f).unwrap();

        let mut imports = Vec::with_capacity(imports_count as usize);
        for _ in 0 .. imports_count {
            // read libname
            let length = read_be_u64(&mut f).unwrap();
            let mut lname = Vec::with_capacity(length as usize);

            for _ in 0 .. length {
                let size = read_be_u64(&mut f).unwrap();
                let mut part = String::with_capacity(size as usize);

                for _ in 0 .. size {
                    let ch = read_u8(&mut f).unwrap();
                    part.push(ch as char);
                }

                lname.push(part);
            }

            imports.push(Box::new(LibName(lname)));
        }

        let _ = f.seek(io::SeekFrom::Start(sym_tab_off));
        let sym_count = read_be_u64(&mut f).unwrap();
        let mut mod_symt = Vec::with_capacity(sym_count as usize);
        debug!("{} symbols in table", sym_count);

        for _ in 0 .. sym_count {
            let sz = read_be_u64(&mut f).unwrap();
            let mut s = String::with_capacity(sz as usize);

            for _ in 0 .. sz {
                let b = read_u8(&mut f).unwrap();
                s.push(b as char);
            }

            let h = gc.intern(s);
            mod_symt.push(h);
        }

        let _ = f.seek(io::SeekFrom::Start(exports_off));
        let exports_count = read_be_u64(&mut f).unwrap();

        let env = gc.alloc(gc::Env {
            values: Vec::with_capacity(exports_count as usize),
            next: None
        });

        debug!("Trying to access program text section at {:x}", text_off);
        let _ = f.seek(io::SeekFrom::Start(text_off));
        let text_size = read_be_u64(&mut f).unwrap();
        let mut text = Vec::with_capacity(text_size as usize);

        for _ in 0 .. text_size {
            let b = read_u8(&mut f).unwrap();
            text.push(b);
        }

        debug!("Sucessfully loaded library");
        Box::new(Library {
            env: env, prog: text, name: name, sym_table: mod_symt,
            imports: imports, exports: exports_count
        })
    }

    pub fn load(gc: &mut ::gc::GC, name: &LibName, lpath: Vec<PathBuf>) -> Box<Library> {
        let mut lpath = lpath;

        for p in lpath.iter_mut() {
            for part in name.iter() {
                p.push(&part);
            }

            if p.is_file() {
                debug!("Trying {}", p.display().to_string());
                Library::load_file(gc, &**p, Box::new(name.clone()));
            }
        }

        panic!("Impossible to find library in path");
    }
}
