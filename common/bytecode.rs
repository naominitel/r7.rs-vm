#[repr(u8)]
#[allow(dead_code)]
#[deriving(Show)]
pub enum Opcode {
    Nop    = 0x00,
    Push   = 0x01,
    Pop    = 0x03,
    Jump   = 0x04,
    Call   = 0x06,
    Return = 0x07,
    Fetch  = 0x08,
    Branch = 0x09,
    Store  = 0x0A,
    Alloc  = 0x0C,
    Tcall  = 0x0D,
}

#[repr(u8)]
#[allow(dead_code)]
pub enum Type {
    Unit    = 0x00,
    Bool    = 0x01,
    Int     = 0x02,
    Sym     = 0x05,
    Fun     = 0x08,
    Prim    = 0x09
}

#[inline(always)]
pub fn base(pc: u64) -> u32 {
    ((pc & 0xFFFF0000) >> 32) as u32
}

#[inline(always)]
pub fn off(pc: u64) -> u32 {
    (pc & 0xFFFF) as u32
}
