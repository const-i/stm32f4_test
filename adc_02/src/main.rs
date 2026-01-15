#![no_main]
#![no_std]

use core::cell::RefCell;

use {defmt_rtt as _, panic_probe as _};

use defmt::*;

use embassy_executor::Spawner;
use embassy_stm32::Config;
use embassy_stm32::adc::{Adc, AdcChannel, RegularConversionMode, RingBufferedAdc, SampleTime};
use embassy_stm32::peripherals::ADC1;
use embassy_stm32::time::Hertz;
use embassy_stm32::rcc::{Hse, HseMode};
use embassy_time::Timer;






const ADC_BUF_SIZE: usize = 32;
const SAMPLE_RATE_HZ: u32 = 44_100;

#[unsafe(link_section = ".ram_d3")]
static mut DMA_BUF: [u16; ADC_BUF_SIZE] = [0; ADC_BUF_SIZE];


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");
    
    let mut config = Config::default(); 
    config.rcc.hse = Some(Hse {
            freq: Hertz::mhz(25),
            mode: HseMode::Oscillator,
    });


    let p = embassy_stm32::init(config);

    
    let adc = Adc::new_with_config(p.ADC1, Default::default());
    
    let mut adc_buf = unsafe { &mut DMA_BUF[..] };
    
    let mut adc: RingBufferedAdc<embassy_stm32::peripherals::ADC1> = adc.into_ring_buffered(
        p.DMA2_CH0,
        adc_buf,
        [
            (p.PA0.degrade_adc(), SampleTime::CYCLES480),
        ]
        .into_iter(),
        RegularConversionMode::Continuous,
    );
    
    spawner.spawn(adc_dma_task(adc)).unwrap();
    
}


#[embassy_executor::task] 
async fn adc_dma_task(mut adc: RingBufferedAdc<'static, ADC1>) {

	let mut measurements = [0u16; ADC_BUF_SIZE / 2];
	loop { 
		
		match adc.read(&mut measurements).await {
        	Ok(_) => {
            	defmt::info!("adc1: {}", measurements);
        	}
        	Err(e) => {
            	defmt::warn!("Error: {:?}", e);
        	}
        }
    }
}
