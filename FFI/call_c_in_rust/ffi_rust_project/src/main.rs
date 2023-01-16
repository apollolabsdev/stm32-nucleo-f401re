#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{self, pac, prelude::*};
// Include bindings.rs that contains the extern function declatrations
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

const GPIOA: *mut GPIO_TypeDef = GPIOA_BASE as *mut GPIO_TypeDef;
// alternative declaration
// const GPIOA: *mut GPIO_TypeDef = GPIOA_BASE as *mut _;
const GPIO_PIN_5: u16 = 0x0020;

#[entry]
fn main() -> ! {
    // Application Loop
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    let mut del = dp.TIM1.delay_ms(&clocks);

    let gpioa = dp.GPIOA.split();
    let _led = gpioa.pa5.into_push_pull_output();

    loop {
        // Call C function in bindings.rs that toggles pin
        unsafe {
            HAL_GPIO_TogglePin(GPIOA, GPIO_PIN_5);
        }
        del.delay_ms(1000_u32);
    }
}
