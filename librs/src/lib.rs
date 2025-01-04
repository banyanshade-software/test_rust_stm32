

// http://blackforrest-embedded.de/2024/05/01/rust-and-vendor-sdksii/
// see also https://github.com/rust-lang/miri/issues/3498

// see also https://jonathanklimt.de/electronics/programming/embedded-rust/rust-STM32F103-blink/
#![no_std]

extern crate panic_itm;
extern crate critical_section;

use cortex_m::{iprintln, peripheral};
use core::ffi::c_int;
use stm32g4_staging::stm32g491;

//extern "C" { pub fn HAL_Delay(mil :u32); }
extern "C" { pub fn osDelay(mil :u32) -> c_int; }

#[no_mangle]
fn rs_main() -> !{
	// green led : PA5
	//let peripherals = stm32g491::Peripherals::take().unwrap();
	let peripherals = unsafe { stm32g491::Peripherals::steal() };
    let gpioa = &peripherals.GPIOA;
	
    loop {
		iprintln!(itm(), "hop");
		gpioa.bsrr().write(|w| w.bs5().set_bit());
	    unsafe {  osDelay(1000);  }
	    gpioa.brr().write(|w| w.br5().set_bit());
	    unsafe {  osDelay(200);  }
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