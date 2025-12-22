#![no_main]
#![no_std]

use emb_book_01 as _; // global logger + panicking-behavior + memory layout

use stm32f4xx_hal::{pac, prelude::*};
use cortex_m::asm;

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
        	asm::nop();
        }
        
        // Switch off led
		led_red.set_low();
		led_green.set_high();
		
		// Do nops
        for _ in 0..100_000{
        	asm::nop();
        }
        
	}
	
	// Should not reach here
    emb_book_01::exit()
}
