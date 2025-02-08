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

//use core::{ptr, slice};

use cortex_m::peripheral::itm::Stim;
use cortex_m::peripheral::ITM;

//use cortex_m::iprintln;


static MASK :u16 = 0;

fn write_all(port: &mut Stim, buffer: &[u8]) {
    for c in buffer {
        while !port.is_fifo_ready() {}
        port.write_u8(*c);
    }       
}

fn prt3(msg: &str, _a: i32, _b: i32, _c: i32, _n: u8) {
    let itm = unsafe { &mut *ITM::PTR };
    let port0 = &mut itm.stim[0];

    write_all(port0, msg.as_bytes());
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








pub fn t3() {
    itm_debug3!(1, "test", 42, 44, 55);
}
