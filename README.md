# test_rust_stm32

This repo is an experiment of RUST on STM32 target (STM32G491 Nucleo Board)
Several experimentations are performed here, on several branches

* 01_minimal_setup

  this setup is based on http://blackforrest-embedded.de/2024/05/01/rust-and-vendor-sdksii/
  Rust is only a static library, with (for now) no specific stm32 crates, called from C


* 02_test_panic

  in this setup we generate a panic, and check our handler is called (through hw breakpoint)
  
* 03_measure_footprint

  we build with and without rust to compare footprint (with FreeRTOS, CMSY 1)
  With Rust and librs.a :        Without Rust
  	RAM     1.69 KB              RAM : 3.23 KB
  	FLASH   13.55 KB             FLASH : 17,2 KB
  	
  	we have a few core function (core::str:::count 1.2kB, core==option::, core::fmt 518B)
  	
* 04_ITM

  we add itm support, through cortex-m crate. The ITM part has a problem (sending 4 chars per 
  u32 cell, while SWV viewer expect a single char), see https://github.com/rust-embedded/cortex-m/issues/571
  This is fixed by forking and adding "onechar_itm" feature.
  
  Footprint significntly increased, with FLASH : 26,77 KB (+9KB) with format method, (but also encode_utf8)

* 05_GPIO

  we now call main_rs() from FreeRTOS default task, and from rust we call osDelay()
  we use use stm32g4_staging::stm32g491 for a low level (too low ?) access to GPIO

 FLASH  usage increased to 27.62 K (mostly du to ITM, see above)

* 06_FreeRtosNotif
  We start RTOS integration by using a very simple wrapper around xTaskNotifyWait()
  and a "tick" notification sent to our task

* 07_more_iterators
  We're starting todo real things here, and we want to take advantage of
  lazy iterators: we define simple text to morse code transformation
  using iterator map(). 
  Because map() is evaluated leazily, the actual transformation occurs only
  when needed, and the input text could be arbitrary long. It could even
  be characters input from a keyboard actually.

* 08_unittest
  we setup unit tests running on host (here on mac) through an
  alias 'test_mac' defined in .cargo/config.toml

* **09_rustdoc**
  we add rust_doc, and we configure (see .cargo/config.toml and src/docs-header.html) 
  it to be able to include LaTeX comments, because it's fun and it can be
  *very* convenient when some maths are part of the process !
