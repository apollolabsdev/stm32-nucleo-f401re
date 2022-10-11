#![no_std]
#![no_main]

// Imports
use core::convert::TryFrom;
use cortex_m_rt::entry;
use max7219_driver::{self, DigitRowAddress, MAX7219};
use panic_halt as _;
use stm32f4xx_hal::{
    pac,
    prelude::*,
    spi::{Mode, NoMiso, Phase, Polarity},
};

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Configure the system clocks
    // - Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // - Configure system clocks
    // 8 MHz must be used for the Nucleo-F401RE board according to manual
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();

    // Configure the SPI pins
    // Don't need miso since communication is simplex (single direction)
    // PA5 and PA7 use SPI1 in the STM32F401RE
    let gpioa = dp.GPIOA.split();
    let sclk = gpioa.pa5.into_alternate();
    let mosi = gpioa.pa7.into_alternate();
    let cs = gpioa.pa6.into_push_pull_output();

    // Initialize SPI
    // Theres also two other methods to instantiate 'new' and 'init'!
    let spi = dp.SPI1.spi(
        (sclk, NoMiso {}, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        2.MHz(),
        &clocks,
    );

    let mut max7219 = MAX7219::new(spi, cs).unwrap();

    // configure Delay Handle
    let mut delay = dp.TIM2.delay_ms(&clocks);

    // Application Code
    // Initialize Display
    max7219.init_display(true);

    loop {
        let mut data: u8 = 1;
        for addr in 1..9 {
            max7219.draw_row_or_digit(DigitRowAddress::try_from(addr).unwrap(), data);
            data = data << 1;
            delay.delay_ms(500_u32);
        }

        // Clear the LED matrix row by row with 500ms delay in between
        for addr in 1..9 {
            max7219.draw_row_or_digit(DigitRowAddress::try_from(addr).unwrap(), data);
            delay.delay_ms(500_u32);
        }
    }
}
