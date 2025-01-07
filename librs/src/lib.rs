

// http://blackforrest-embedded.de/2024/05/01/rust-and-vendor-sdksii/
// see also https://github.com/rust-lang/miri/issues/3498

// see also https://jonathanklimt.de/electronics/programming/embedded-rust/rust-STM32F103-blink/

//#![no_std]
#![cfg_attr(not(test), no_std)]
#[cfg(test)]
extern crate std;


//extern crate panic_itm;
#[cfg(not(test))]
extern crate critical_section;

//use cortex_m::{peripheral};
use core::ffi::c_int;
#[cfg(not(test))]
use stm32g4_staging::stm32g491;

//extern "C" { pub fn HAL_Delay(mil :u32); }

extern "C" { 
	pub fn osDelay(mil :u32) -> c_int;
	pub fn notifWait() -> u32;
}

const SOMETEXT :&str = "\
Longtemps je me suis couche de bonne heure  \
Parfois a peine ma bougie eteinte  mes yeux se fermaient si vite \
que je n avais pas le temps de me dire  \
je m endors  Et une demi heure apres  la pensee qu il etait temps de chercher \
le someil m eveillait Je voulais poser le volume que je croyais avoir encore \
dans les mains et souffler ma lumière  je n avais ps cesse en dormant de faire des \
reflexions sur ce que je venais de lire mais ces reflexions avaient pris un tour \
un peu particulier  il me semblait que j etais moi meme ce dont parlait l ouvrage";
	
#[cfg(not(test))]
#[no_mangle]
fn rs_main() -> !{
	

	// let peripherals = stm32g491::Peripherals::take().unwrap(); << dependecies problem
	let peripherals = unsafe { stm32g491::Peripherals::steal() };
    let gpioa = &peripherals.GPIOA;  // Nucleo green LED is on PA5
	
	loop {
        // all the string to morse conversion is on the 3 following lines,
        // which obviously can easyly be tested on host, separately from the MCU stuffs

  		let mut mi = morse_iterator(SOMETEXT);
    	loop {
			let _ = unsafe { notifWait() }; // notifWait() is simply a wrapper around xTaskNotifyWait()
			// TIM7 IRQ (configured as timebase src timer) sends notification every 100ms
			
			let k = mi.next(); // iterators are lazy so actual call to morse() and to_onoff()
                               // occurs here "on demand"
                               // around 200 bytes stack are used on iterator next() call 
                               // (several call levels) which can be significant in RTOS tasks
			match k {
				None => break,
                // GPIO PA5 is connected to LED on G491 Nucleo board
				Some(' ') => gpioa.brr() .write(|w| w.br5().set_bit()),
				_         => gpioa.bsrr().write(|w| w.bs5().set_bit()),
			};
	    }
	}
}

fn morse_iterator(txt:&str) -> impl Iterator<Item=char> + use<'_> {
  		let mi = txt.chars()                           // eg: "abc" 
  		                     .flat_map(|k| morse(k).chars())    // eg: ".- -... -.-. ",
                             .flat_map(|m| to_onoff(m).chars());// eg  "x xxx   xxx x x x   xxx x xxx x
        mi
}

fn morse(ch:char) -> &'static str
{
	//iprintln!(itm(), "morsing {}", ch);
	let c = ch.to_ascii_lowercase();
	if c == ' ' { return "  "; }
	if c<'a' || c > 'z' {
		return "";
	}
	// do NOT declare conv with a let, otherwise
	// it will allocate the array on the stack (364 bytes instead of 156)
	// strangely static also seems to allocate space on stack, which is strange
	const  CONV :[&'static str;26] = [
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
	CONV[i]
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    
    #[test]
    fn test_onoff() {
        assert_eq!(to_onoff(' '), "   ");
        assert_eq!(to_onoff('-'), "xxx ");
        assert_eq!(to_onoff('.'), "x ");
        assert_eq!(to_onoff('?'), "");
    }
    
     #[test]
    fn test_morse() {
        assert_eq!(morse('z'), "--.. ");
        assert_eq!(morse('a'), ".- ");
    }
    
    #[test]
    fn test_iter() {
		let m  = morse_iterator("sos");
		let s:String = m.collect();
		assert_eq!(s, "x x x    xxx xxx xxx    x x x    ");
	}
}


// stack 844 B
// 364B with opt

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

