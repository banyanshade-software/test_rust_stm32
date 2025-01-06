

// http://blackforrest-embedded.de/2024/05/01/rust-and-vendor-sdksii/
// see also https://github.com/rust-lang/miri/issues/3498

// see also https://jonathanklimt.de/electronics/programming/embedded-rust/rust-STM32F103-blink/
#![no_std]

//extern crate panic_itm;
extern crate critical_section;

//use cortex_m::{peripheral};
use core::ffi::c_int;
use stm32g4_staging::stm32g491;

//extern "C" { pub fn HAL_Delay(mil :u32); }
extern "C" { 
	pub fn osDelay(mil :u32) -> c_int;
	pub fn notifWait() -> u32;
}


#[no_mangle]
fn rs_main() -> !{

	// green led : PA5
	//let peripherals = stm32g491::Peripherals::take().unwrap();
	let peripherals = unsafe { stm32g491::Peripherals::steal() };
    let gpioa = &peripherals.GPIOA;
	
	let sometext = "\
Longtemps je me suis couche de bonne heure  \
Parfois a peine ma bougie eteinte  mes yeux se fermaient si vite \
que je n avais pas le temps de me dire  \
je m endors  Et une demi heure apres  la pensee qu il etait temps de chercher \
le someil m eveillait Je voulais poser le volume que je croyais avoir encore \
dans les mains et souffler ma lumière  je n avais ps cesse en dormant de faire des \
reflexions sur ce que je venais de lire mais ces reflexions avaient pris un tour \
un peu particulier  il me semblait que j etais moi meme ce dont parlait l ouvrage";
	
	loop {
  		let mut mi = sometext.chars().flat_map(|k| morse(k).chars())
                                     .flat_map(|m| to_onoff(m).chars()); 
  		
    	loop {
            // notifWait() is simply a wrapper around xTaskNotifyWait()
            // Timer IRQ will send a notification every 100ms
			let n = unsafe { notifWait() };
			if n & 0x0000_0001 == 0 { continue; }
				
			let k = mi.next();
			match k {
				None => break,
				// we have only ' ' or 'x' as defined in to_onoff()
                // GPIO PA5 is connected to LED on G491 Nucleo board
				Some(' ') => gpioa.brr().write( |w| w.br5().set_bit()),
				_         => gpioa.bsrr().write(|w| w.bs5().set_bit()),
				//_ => (),
			};
	    }
	}
}


fn morse(ch:char) -> &'static str
{
	//iprintln!(itm(), "morsing {}", ch);
	let c = ch.to_ascii_lowercase();
	if c == ' ' { return "  "; }
	if c<'a' || c > 'z' {
		return "";
	}
	let conv :[&'static str;26] = [
		 /* a */ ".- ",
    /* b */ "-... ",
    /* c */ "-.-. ",
    /* d */ "-.. ",
    /* e */ ". ",
    /* f */ "..-. ",
    /* g */ "--. ",
    /* h */ ".... ",
    /* i */ ".. ",
    /* j */ ".--- ",
    /* k */ "-.- ",
    /* l */ ".-.. ",
    /* m */ "-- ",
    /* n */ "-. ",
    /* o */ "--- ",
    /* p */ ".--. ",
    /* q */ "--.- ",
    /* r */ ".-. ",
    /* s */ "... ",
    /* t */ "- ",
    /* u */ "..- ",
    /* v */ "...- ",
    /* w */ ".-- ",
    /* x */ "-..- ",
    /* y */ "-.-- ",
    /* z */ "--.. "
	];
	let i  = c as usize - 'a' as usize;
	conv[i]
}

fn to_onoff(m :char) -> &'static str {
	let r = match m {
		' ' => "   ",
		'-' => "xxx ",
		'.' => "x ",
		_ => ""
	};
	r
}



// stack 844 B
// 364B with opt


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

