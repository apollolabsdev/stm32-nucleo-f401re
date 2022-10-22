#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Triangle},
    text::Text,
};
use max7219_driver::{self, MAX7219LedMat};
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

    // BUFLEN value is number of pixels single 8x8 display results in a 64 element buffer
    // COUNT value reflects the number of displays connected
    let mut max7219: MAX7219LedMat<_, _, 64, 1> = MAX7219LedMat::new(spi, cs).unwrap();

    // Initialize Display
    max7219.init_display(true);

    // Below are a few Exmaples for drawing Different Shapes
    // Uncomment the Code for the desired shape to be drawn

    // Example Drawing a Circle
    // Circle::new(Point::new(0, 0), 4)
    //     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    //     .draw(&mut max7219)
    //     .unwrap();

    // Example Drawing a Point
    // Pixel(Point::new(0, 1), BinaryColor::On)
    //     .draw(&mut max7219)
    //     .unwrap();

    // Example Drawing a Triangle
    // Triangle::new(Point::new(3, 0), Point::new(0, 4), Point::new(7, 4))
    //     .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    //     .draw(&mut max7219)
    //     .unwrap();

    // Example Drawing a Character
    let txtstyle = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Y", Point::new(0, 7), txtstyle)
        .draw(&mut max7219)
        .unwrap();

    // Update the Display
    max7219.flush().unwrap();

    loop {}
}
