#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt as rt;
use hal::prelude::*;
use panic_halt as _;
use stm32f4xx_hal as hal;

#[rt::entry]
fn main() -> ! {
    if let Some(peripherals) = hal::pac::Peripherals::take() {
        let gpiob = peripherals.GPIOB.split();
        let mut led = gpiob.pb7.into_push_pull_output();
        let gpioc = peripherals.GPIOC.split();
        let button = gpioc.pc13;

        loop {
            if button.is_high() {
                led.set_low();
            } else {
                led.set_high();
            }
        }
    }
}
