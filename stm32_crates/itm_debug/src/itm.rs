//! Instrumentation Trace Macrocell debug msg with low overhead
//!
//! **NOTE** This module is only available on ARMv7-M and newer.
//! We do **not** provide fmt as this has a high memory footprint
//! (both flash and stack), similary to C sprintf()
//!
//! rather we only provide possibility to write a short string
//! (a longer one would add more time overhead)
//! and three i32
//!
//! we also provide several msg domains, because we don't want to
//! activate all messages at the same time


#[cfg(not(test))]
use cortex_m::peripheral::itm::Stim;
#[cfg(not(test))]
use cortex_m::peripheral::ITM;


#[cfg(not(test))]
static MASK :u16 = 0;

struct IntToCharIter {
    num: i32,
    divisor: i32,
    neg: bool,
}


impl IntToCharIter {
     fn new(n: i32) -> Self {
        if n == 0 {
            return Self { num: 0, divisor: 1, neg: false };
        }
        let mut num = n;
        let mut neg = false;
        if num < 0 {
            neg = true;
            num = -num;
        }

        let mut divisor = 1;
        while num / divisor >= 10 {
            divisor *= 10;
        }

        Self { num, divisor, neg }
    }
}
impl Iterator for IntToCharIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor == 0 {
            return None;
        }
        if self.neg {
            self.neg = false;
            return Some('-' as u8);
        }

        let digit = (self.num / self.divisor) % 10;
        self.divisor /= 10;

        Some((b'0' + digit as u8) as u8)
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
   
    #[test]
    fn test_pos() {
        let mut it = IntToCharIter::new(1234);
        assert_eq!(it.next(), Some('1' as u8));
        assert_eq!(it.next(), Some('2' as u8));
        assert_eq!(it.next(), Some('3' as u8));
        assert_eq!(it.next(), Some('4' as u8));
        assert_eq!(it.next(), None);
    }
    #[test]
    fn test_zero() {
        let mut it = IntToCharIter::new(0);
        assert_eq!(it.next(), Some('0' as u8));
        assert_eq!(it.next(), None);
    }
    #[test]
    fn test_neg() {
        let mut it = IntToCharIter::new(-1234);
        assert_eq!(it.next(), Some('-' as u8));
        assert_eq!(it.next(), Some('1' as u8));
        assert_eq!(it.next(), Some('2' as u8));
        assert_eq!(it.next(), Some('3' as u8));
        assert_eq!(it.next(), Some('4' as u8));
        assert_eq!(it.next(), None);
    }
    #[test]
    fn test_itloop() {
        let it = IntToCharIter::new(4287234);
        let mut v: Vec<u8> = Vec::new();
        for x in it {
            v.push(x);
        }
        let r = ['4' as u8,'2' as u8,'8' as u8,'7' as u8,'2' as u8,'3' as u8,'4' as u8];
        let rv = r.to_vec();
        assert_eq!(v, rv);
    }
}



#[cfg(not(test))]
fn write_all(port: &mut Stim, buffer: &[u8]) {
    for c in buffer {
        while !port.is_fifo_ready() {}
        port.write_u8(*c);
    }       
}

#[cfg(not(test))]
fn write_int(port: &mut Stim, v: i32)
{
    let it = IntToCharIter::new(v);
    for ch in it {
        while !port.is_fifo_ready() {}
        port.write_u8(ch);
    }
}

#[cfg(not(test))]
fn prt3(msg: &str, a: i32, b: i32, c: i32, n: u8) {
    let itm = unsafe { &mut *ITM::PTR };
    let port0 = &mut itm.stim[0];

    write_all(port0, msg.as_bytes());
    if n>0 { write_int(port0, a) }
    if n>1 { write_int(port0, b) }
    if n>2 { write_int(port0, c) }
}



#[macro_export]
macro_rules! itm_debug3 {
    ($flags:expr, $msg:expr, $a:expr, $b:expr, $c:expr) => {
        if 0 != ($flags & MASK) {
            prt3($msg, $a, $b, $c, 3);
        }
    };
}
#[macro_export]
macro_rules! itm_debug2 {
    ($flags:expr, $msg:expr, $a:expr, $b:expr) => {
        if 0 != ($flags & MASK) {
            prt3($msg, $a, $b, 0, 2);
        }
    };
}
#[macro_export]
macro_rules! itm_debug1 {
    ($flags:expr, $msg:expr, $a:expr) => {
        if 0 != ($flags & MASK) {
            prt3($msg, $a, 0, 0, 1);
        }
    };
}
#[macro_export]
macro_rules! itm_debug0 {
    ($flags:expr, $msg:expr) => {
        if 0 != ($flags & MASK) {
            prt3($msg, 0,0,0, 0);
        }
    };
}








#[cfg(not(test))]
pub fn t3() {
    itm_debug3!(1, "test", 42, 44, 55);
}
