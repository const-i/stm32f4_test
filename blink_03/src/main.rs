
#![no_main]
#![no_std]

use panic_halt as _;

use stm32f4xx_hal as hal;

use crate::hal::{
	gpio::{self, Output, PushPull},
	rcc::Config,
	pac::{interrupt, Interrupt, Peripherals, TIM2},
	prelude::*,
    timer::{CounterUs, Event},
};

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;


// Type definition for GPIO pin for the led: PD14 - Red Led
type LedPin = gpio::PD14<Output<PushPull>>;

// Make LED pin globally available
static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));

// Make timer interrupt registers globally available
static G_TIM: Mutex<RefCell<Option<CounterUs<TIM2>>>> = Mutex::new(RefCell::new(None));


// Define an interrupt handler, i.e. function to call when interrupt occurs.
// This specific interrupt will "trip" when the timer TIM2 times out
#[interrupt]
fn TIM2() {
    static mut LED: Option<LedPin> = None;
    static mut TIM: Option<CounterUs<TIM2>> = None;

    let led = LED.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move LED pin here, leaving a None in its place
            G_LED.borrow(cs).replace(None).unwrap()
        })
    });

    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move LED pin here, leaving a None in its place
            G_TIM.borrow(cs).replace(None).unwrap()
        })
    });

    led.toggle();
    let _ = tim.wait();
}



#[entry]
fn main() -> ! {

	// Get the peripherals
	let dp = Peripherals::take().unwrap();
	
	// Set up the system clock. We want to run at 48MHz for this one.
    let mut rcc = dp.RCC.freeze(Config::hsi().sysclk(48.MHz()));
	
	// Configure the led
    let gpiod = dp.GPIOD.split(&mut rcc);
    let mut led = gpiod.pd14.into_push_pull_output();
    led.set_high(); // Turn on
    
    // Move the pin into our global storage
    cortex_m::interrupt::free(|cs| *G_LED.borrow(cs).borrow_mut() = Some(led));
    
    // Set up a timer expiring after 1s
    let mut timer = dp.TIM2.counter(&mut rcc);
    timer.start(500.millis()).unwrap();
    
    // Generate an interrupt when the timer expires
    timer.listen(Event::Update);

    // Move the timer into our global storage
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));

    //enable TIM2 interrupt
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }
	
	loop {
		continue;
	}
	
}
