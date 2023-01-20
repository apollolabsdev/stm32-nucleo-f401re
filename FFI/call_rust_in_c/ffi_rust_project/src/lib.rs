#![no_std]
use panic_halt as _;
// use core::fmt::Error;

// use stm32f4xx_hal::gpio::gpioa::PA5;
// use stm32f4xx_hal::gpio::{Output, Pin};
use stm32f4xx_hal::pac::gpioa::RegisterBlock as PORTA;
// use stm32f4xx_hal::prelude::*;
// use embedded_hal::prelude::_embedded_hal_digital_OutputPin as OutputPin;

// #[no_mangle]
// pub unsafe extern "C" fn togglePin(pin: &mut dyn OutputPin<Error = Error>) {
//     pin.set_high();
// }

// #[no_mangle]
// pub unsafe extern "C" fn togglePin(pin: &mut dyn OutputPin) {
//     pin.set_high();
// }

#[no_mangle]
pub unsafe extern "C" fn togglePin(p: &PORTA) {
    // i2c1.cr2.modify(|r, w| w.stop().bit(!r.stop().bit()));
    // Toggle the ODR5 bit, all the other bits will remain untouched
    p.odr.modify(|r, w| w.odr5().bit(!r.odr5().bit()));
}
