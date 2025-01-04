

// http://blackforrest-embedded.de/2024/05/01/rust-and-vendor-sdksii/
// see also https://github.com/rust-lang/miri/issues/3498
#![no_std]

#[no_mangle]
fn rs_main() -> !{
	
	let _ = test_panic().expect("foo");
    loop {}
}

fn test_panic() -> Option<()> {
	None
}


/// add breakpoint on panic handler to check it is called
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
