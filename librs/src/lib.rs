

// http://blackforrest-embedded.de/2024/05/01/rust-and-vendor-sdksii/
// see also https://github.com/rust-lang/miri/issues/3498
#![no_std]

extern crate panic_itm;
use cortex_m::{iprintln, peripheral};
use core::ffi::c_int;

//extern "C" { pub fn HAL_Delay(mil :u32); }
extern "C" { pub fn osDelay(mil :u32) -> c_int; }

#[no_mangle]
fn rs_main() -> !{
	
	
    loop {
		iprintln!(itm(), "hop");
	    unsafe {
		    osDelay(1000);
	    }
	}
}

/*
fn test_panic() -> Option<()> {
	None
}*/


fn itm() -> &'static mut peripheral::itm::Stim {
    unsafe { &mut (*peripheral::ITM::PTR).stim[0] }
}
/*
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
*/