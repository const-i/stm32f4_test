
Blink example using the HAL (Hardware Abstraction Layer) for
the STM32F407-Discovery Board: stm32f4xx-hal
Based on: https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/blinky.rs

## Requirements

$ openocd --version
Open On-Chip Debugger 0.12.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html


$ arm-none-eabi-gdb --version
GNU gdb (Gentoo 16.3 vanilla) 16.3
Copyright (C) 2024 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.


## Flash

### Start openocd
openocd -f interface/stlink-v2.cfg -f target/stm32f4x.cfg

### Start gdb
arm-none-eabi-gdb -q target/thumbv7em-none-eabihf/debug/blink_02

Connect to openocd
(gdb) target extended-remote :3333

Flash
(gdb) load

Detach and quit
(gdb) detach
(gdb) quit
