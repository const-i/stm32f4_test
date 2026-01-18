#![no_main]
#![no_std]

use defmt::*;
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, SampleTime};
use embassy_stm32::dac::{DacCh1, Value};
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::{init, Config};
use embassy_stm32::rcc::{Hse, HseMode};
use embassy_stm32::time::Hertz;

use embassy_time::{Duration, Ticker, Timer};

const SAMPLE_RATE_HZ: u64 = 48_000;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("ADC→DAC at 48 kHz with PD14 LED heartbeat");

    // Basic clock config (adjust HSE to your board)
    let mut config = Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz::mhz(8),
        mode: HseMode::Oscillator,
    });

    let p = init(config);

    // ADC setup: single channel PC1
    let mut adc = Adc::new(p.ADC1);
    let mut channel = p.PC1;

    // DAC setup: channel 1 on PA4, blocking
    let mut dac = DacCh1::new_blocking(p.DAC1, p.PA4);

    // LED setup: PD14 as push‑pull output
    let led = Output::new(p.PD14, Level::Low, Speed::Low);

    // Spawn LED blink task
    spawner.spawn(blink_task(led)).unwrap();

    // Pace the ADC→DAC loop at exactly 48 kHz
    let mut ticker = Ticker::every(Duration::from_hz(SAMPLE_RATE_HZ));

    loop {
        // Wait for the next 48 kHz tick
        ticker.next().await;

        // Blocking ADC read with short sample time
        let sample: u16 = adc.blocking_read(&mut channel, SampleTime::CYCLES3);

        // Saturate to 12‑bit and write directly to DAC
        let val = sample.min(4095);
        dac.set(Value::Bit12Right(val));
    }
}

#[embassy_executor::task]
async fn blink_task(mut led: Output<'static>) {
    loop {
        info!("LED high");
        led.set_high();
        Timer::after_millis(300).await;

        info!("LED low");
        led.set_low();
        Timer::after_millis(300).await;
    }
}
