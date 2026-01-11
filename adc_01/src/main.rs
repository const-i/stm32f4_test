#![no_main]
#![no_std]


use {defmt_rtt as _, panic_probe as _};

use defmt::*;

use embassy_executor::Spawner;
use embassy_stm32::adc::{Adc, SampleTime};
use embassy_stm32::dac::{DacCh1, Value};
use embassy_time::{Duration, Ticker, Instant};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");
    
    // Configure the ADC and the pin
    let mut adc = Adc::new_with_config(p.ADC1, Default::default());
    let mut pin = p.PC1;
    
    // Configure the DAC and the pin
    let mut dac = DacCh1::new_blocking(p.DAC1, p.PA4);
    
	// Configure the Ticker
    let mut ticker = Ticker::every(Duration::from_millis(1000));

    // Create an array to store values
    const BUF_LENGTH: usize = 32; 
    let mut adc_buffer: [u16; BUF_LENGTH] = [0; BUF_LENGTH];
    
    // Dont loop this
    {
    
    	// Sample the data
    	
    	let tic = Instant::now();
    	for i in 0..BUF_LENGTH {
    		// Read pin
        	let v = adc.blocking_read(&mut pin, SampleTime::CYCLES15);
        	adc_buffer[i] = v;
        }
        let toc = Instant::now();
        let dt = (toc - tic).as_micros();
        let samp_freq = 1e6/(dt as f32) * BUF_LENGTH as f32;
        info!(
        	"head: {}; tail: {}; samples: {}; dt: {}; freq: {}", 
        	adc_buffer[0..5], 
        	adc_buffer[BUF_LENGTH-5..BUF_LENGTH],
        	BUF_LENGTH,
        	dt, 
        	samp_freq,
        );
        
        // Push to DAC
        let tic = Instant::now();
        for i in 0..BUF_LENGTH {
            dac.set(Value::Bit12Left(adc_buffer[i]));
        }
        let toc = Instant::now();
        let dt = (toc - tic).as_micros();
        let samp_freq = 1e6/(dt as f32) * BUF_LENGTH as f32;
        info!(
        	"dt: {}; freq: {}", 
        	dt, 
        	samp_freq,
        );
        
        ticker.next().await;
    	
    	
    }
    
    loop {
    
    	//let tic = Instant::now();
    	let v = adc.blocking_read(&mut pin, SampleTime::CYCLES15);
    	dac.set(Value::Bit12Left(v));
    	//let toc = Instant::now();
        //let dt = (toc - tic).as_micros();
        //let samp_freq = 1e6/(dt as f32);
        //info!("v: {}; dt: {}; freq: {}", v, dt, samp_freq);
    	
    	//ticker.next().await;
    
    }
    
}

