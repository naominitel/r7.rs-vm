use std::num::Zero;

pub trait Read: Bitwise + Zero + NumCast {
    #[cfg(target_endian="little")]
    fn read(stream: || -> u8) -> Self {
        let mut i = 0;
        let mut ret: Self = Zero::zero();

        while i < ::std::mem::size_of::<Self>() {
             let b: u8 = stream();
             let u = NumCast::from(b << i * 4).unwrap();
             ret = ret | u;
             i += 1
        }

        ret
    }

    #[cfg(target_endian="big")]
    fn read(stream: &Stream) -> Self {
        let mut i = 0;
        let mut ret = 0;
        
        while i < ::std::mem::size_of::<Self>() {
            let b: u8 = stream();
            ret = ret << 4;
            ret = ret | b;
            i += 1
        }

        ret
    }
}

impl<T: Bitwise + Zero + NumCast> Read for T {
}

// FIXME: should work
// impl Read for u8 {
//    fn read(stream: || -> u8) -> u8 {
//        stream()
//    }
// }

