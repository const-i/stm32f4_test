#![no_main]
#![no_std]

use panic_halt as _;

// Using this seprately works when we dont pull in stm32f407g-disc
// Once we have that there is a version clash
// use cortex_m_rt::entry;
use stm32f407g_disc::entry;

use stm32f407g_disc::hal::pac;
use stm32f407g_disc::hal::prelude::*;	// This is for split() method
use stm32f407g_disc::led::{LedColor, Leds};

use stm32f407g_disc::asm;


#[entry]
fn main() -> ! {

	// Get the peripherals
	let p = pac::Peripherals::take().unwrap();
	
	// Get the GPIO - D parts - thats where the leds are
	let gpiod = p.GPIOD.split();

	// Initialize the leds
	let mut leds = Leds::new(gpiod);
	
	// Loops Led
    loop {
        
        // Switch on led
        leds[LedColor::Red].on();
        
        // Do nops
        for _ in 0..10_000{
        	asm::nop();
        }
        
        // Switch off led
        leds[LedColor::Red].off();
        
        // Do nops
        for _ in 0..10_000{
        	asm::nop();
        }
        
    }
}
