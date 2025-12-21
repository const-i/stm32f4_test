#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_halt as _;

use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;
use cortex_m::asm;


#[entry]
fn main() -> ! {

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
        for _ in 0..10_000{
        	asm::nop();
        }
        
        // Switch off led
		led_red.set_low();
		led_green.set_high();
		
		// Do nops
        for _ in 0..10_000{
        	asm::nop();
        }
        
	}

}
