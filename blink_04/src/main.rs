#![no_main]
#![no_std]

use defmt_rtt as _; // global logger

use panic_probe as _;

use cortex_m::asm::nop;

use stm32f4xx_hal as hal; // memory layout
use hal::{pac, prelude::*};

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    semihosting::process::exit(0);
}

/// Hardfault handler.
///
/// Terminates the application and makes a semihosting-capable debug tool exit
/// with an error. This seems better than the default, which is to spin in a
/// loop.
#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    semihosting::process::exit(1);
}


#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");
    
    // Get the peripherals
	let p = pac::Peripherals::take().unwrap();
	
	// Use the Reset and Clock Control to Contraint the clock
	let mut rcc = p.RCC.constrain();
	
	// Get the Led
	let gpiod = p.GPIOD.split(&mut rcc);
	let mut led_red = gpiod.pd14.into_push_pull_output();
	let mut led_green = gpiod.pd12.into_push_pull_output();
	
	loop {
		
		// Switch on led
		led_red.set_high();
		led_green.set_low();
		
		// Do nops
        for _ in 0..100_000{
        	nop();
        }
        
        // Switch off led
		led_red.set_low();
		led_green.set_high();
		
		// Do nops
        for _ in 0..100_000{
        	nop();
        }
        
	}
    
    // Should not reach here
    // exit()
}
