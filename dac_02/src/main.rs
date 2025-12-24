#![no_main]
#![no_std]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::dac::{DacCh1, Value};
use embassy_stm32::mode::Blocking;
use embassy_stm32::peripherals::DAC1;
use embassy_stm32::rcc::{Hse, HseMode};
use embassy_stm32::time::Hertz;
use embassy_time::{Duration, Ticker, TICK_HZ};
use micromath::F32Ext;

// Constants for the 440Hz wave at 44.1kHz sample rate
const SAMPLE_RATE_HZ: u64 = 44_100;
const FREQUENCY_HZ: f32 = 440.0;


fn clock_config() -> embassy_stm32::Config {
    let mut config = embassy_stm32::Config::default();

    // Configure to use the high speed internal oscillator (HSI).
    config.rcc.hsi = true;
    config.rcc.hse = Some(Hse {
            freq: Hertz(8_000_000), // 8 MHz external crystal frequency
            mode: HseMode::Oscillator, // Use a crystal oscillator (not bypass)
        });

    config
}

#[embassy_executor::task]
async fn dac_task(mut dac: DacCh1<'static, DAC1, Blocking>) {

	let mut ticker = Ticker::every(Duration::from_hz(SAMPLE_RATE_HZ));
	
	let mut sample_index: f32 = 0.0;
    
    loop {
        // Calculate the next sine value
        // wave = sin(2 * PI * freq * t)
        let phase = 2.0 * core::f32::consts::PI * FREQUENCY_HZ * (sample_index / SAMPLE_RATE_HZ as f32);
        let sine_val = (phase.sin() + 1.0) * 2047.5; // Scale to 0..4095
        
        // Update the DAC (12-bit value)
        dac.set(Value::Bit12Right(sine_val as u16));
        
        // Increment and wrap the index to prevent float precision loss over time
        sample_index += 1.0;
        if sample_index >= (SAMPLE_RATE_HZ as f32 / FREQUENCY_HZ) {
            sample_index = 0.0;
        }

        // Wait for the next tick
        ticker.next().await;
    }

}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    //let p = embassy_stm32::init(Default::default());
	let p = embassy_stm32::init(clock_config());

    info!("Hello World! System Tick: {}", TICK_HZ);
    
    // Get the DAC peripheral
    let mut dac = DacCh1::new_blocking(p.DAC1, p.PA4);
    dac.enable();

    // Spawn the high-priority signal generation task
    defmt::unwrap!(spawner.spawn(dac_task(dac)));

    loop {
        embassy_time::Timer::after_secs(1).await;
    }
}
