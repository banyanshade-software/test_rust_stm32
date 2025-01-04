

// http://blackforrest-embedded.de/2024/05/01/rust-and-vendor-sdksii/
// see also https://github.com/rust-lang/miri/issues/3498
#![no_std]

extern crate panic_itm;
use cortex_m::{iprintln,itm, peripheral};

#[no_mangle]
fn rs_main() -> !{
	let x:[u8;5] = [34,34,34,34,13];
	itm::write_all(itm(), &x);
	itm::write_all(itm(), "X".as_bytes());
	itm::write_all(itm(), "X".as_bytes());
	itm::write_all(itm(), "X".as_bytes());
	itm::write_all(itm(), "X".as_bytes());
	itm::write_all(itm(), "X".as_bytes());
	itm::write_all(itm(), "X".as_bytes());
	itm::write_all(itm(), "\n".as_bytes());
	
	itm::write_all(itm(), "A".as_bytes());
	itm::write_all(itm(), "A".as_bytes());
	itm::write_all(itm(), "A".as_bytes());
	itm::write_all(itm(), "A".as_bytes());
	itm::write_all(itm(), "A".as_bytes());
	itm::write_all(itm(), "\n".as_bytes());
	
	itm::write_all(itm(), "BC".as_bytes());
	itm::write_all(itm(), "BC".as_bytes());
	itm::write_all(itm(), "BC".as_bytes());
	itm::write_all(itm(), "\n".as_bytes());
		
	itm::write_all(itm(), "DEF".as_bytes());
	itm::write_all(itm(), "DEF".as_bytes());
	itm::write_all(itm(), "DEF".as_bytes());
	itm::write_all(itm(), "\n".as_bytes());
		
	itm::write_all(itm(), "GHIJ".as_bytes());
	itm::write_all(itm(), "GHIJ".as_bytes());
	itm::write_all(itm(), "GHIJ".as_bytes());
	itm::write_all(itm(), "\n".as_bytes());
	
	itm::write_all(itm(), "KLMNO".as_bytes());
	itm::write_all(itm(), "KLMNO".as_bytes());
	itm::write_all(itm(), "KLMNO".as_bytes());
	itm::write_all(itm(), "\n".as_bytes());
	

	itm::write_str(itm(), "opopopop\n");
	itm::write_str(itm(), "opopopop\n");
	itm::write_str(itm(), "opopopop\n");
	
	iprintln!(itm(), "[TEST] Val: {:?}", 42);
	iprintln!(itm(), "[TEST] Val: {:?}", 42);
	let _ = test_panic().expect("paf");
    loop {}
}

fn test_panic() -> Option<()> {
	None
}



fn itm() -> &'static mut peripheral::itm::Stim {
    unsafe { &mut (*peripheral::ITM::PTR).stim[0] }
}
/*
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
*/