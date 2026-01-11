# STM32F407 - Discovery Board

Learning embedded Rust on the STM32F4-Discovery Board

blink_01 - Blink example using the BSP (Board Support Package) for the STM32F407-Discovery Board: [`stm32f407g-disc`]

[`stm32f407g-disc`]: https://crates.io/crates/stm32f407g-disc

blink_02 - Blink example using the HAL (Hardware Abstraction Layer) for the STM32F407-Discovery Board: [`stm32f4xx-hal`]

[`stm32f4xx-hal`]: https://crates.io/crates/stm32f4xx-hal

blink_03 - Same as blink_02, but using Timers and Interrupts for blinking

blink_04 - Blink using [`probe-rs`] + [`defmt`] + [`flip-link`] embedded project [`template`]

[`probe-rs`]: https://crates.io/crates/probe-rs
[`defmt`]: https://github.com/knurling-rs/defmt
[`flip-link`]: https://github.com/knurling-rs/flip-link
[`template`]: https://github.com/knurling-rs/app-template

blink_05 - Blink example from the [`Embassy Framework`]

[`Embassy Framework`]: https://github.com/embassy-rs/embassy

dac_01 - Sine wave output example from [`Embassy`]

[`Embassy`]: https://github.com/embassy-rs/embassy/blob/main/examples/stm32f4/src/bin/dac.rs

dac_02 - Sine wave of a specific freq (440 Hz) - This only gets to about 430 Hz when built with --release

acd_01 - ADC + DAC with brute force writing single values from ADC to DAC.
