#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    pac::{self},
    prelude::*,
    spi::{Mode, NoMiso, Phase, Polarity, Spi},
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
    let mut cs = gpioa.pa6.into_push_pull_output();

    // Initialize SPI
    // Theres also two other methods to instantiate 'new' and 'init'!
    let mut spi = dp.SPI1.spi(
        (sclk, NoMiso {}, mosi),
        Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        },
        2.MHz(),
        &clocks,
    );

    // OR you can do something like this
    // let mut spi = Spi::new(
    //     dp.SPI1,
    //     (sclk, NoMiso {}, mosi),
    //     Mode {
    //         polarity: Polarity::IdleLow,
    //         phase: Phase::CaptureOnFirstTransition,
    //     },
    //     2.MHz(),
    //     &clocks,
    // );

    // Application Loop

    // 1) Initalizing Matrix Display

    // 1.a) Power Up Device

    // - Prepare Data to be Sent
    // 8-bit Data/Command Corresponding to Matrix Power Up
    let data: u8 = 0x01;
    // 4-bit Address of Shutdown Mode Command
    let addr: u8 = 0x0C;
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [addr, data];

    // - Send Data
    // Set CS to low to shift/clock bits into max7219 (datasheet requirement)
    cs.set_low();
    // Shift in 16 bits by passing send_array (bits will be shifted MSB first)
    spi.write(&send_array).unwrap();
    // Set CS to high to latch shifted bits into max7219 (datasheet requirement)
    cs.set_high();

    // 1.b) Set up Decode Mode

    // - Prepare Information to be Sent
    // 8-bit Data/Command Corresponding to No Decode Mode
    let data: u8 = 0x00;
    // 4-bit Address of Decode Mode Command
    let addr: u8 = 0x09;
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [addr, data];

    // - Send Data
    // Set CS to low to shift/clock bits into max7219 (datasheet requirement)
    cs.set_low();
    // Shift in 16 bits by passing send_array (bits will be shifted MSB first)
    spi.write(&send_array).unwrap();
    // Set CS to high to latch shifted bits into max7219 (datasheet requirement)
    cs.set_high();

    // 1.c) Configure Scan Limit

    // - Prepare Information to be Sent
    // 8-bit Data/Command Corresponding to Scan Limit Displaying all digits
    let data: u8 = 0x07;
    // 4-bit Address of Scan Limit Command
    let addr: u8 = 0x0B;
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [addr, data];

    // - Send Data
    // Set CS to low to shift/clock bits into max7219 (datasheet requirement)
    cs.set_low();
    // Shift in 16 bits by passing send_array (bits will be shifted MSB first)
    spi.write(&send_array).unwrap();
    // Set CS to high to latch shifted bits into max7219 (datasheet requirement)
    cs.set_high();

    // 1.c) Configure Intensity

    // - Prepare Information to be Sent
    // 8-bit Data/Command Corresponding to (15/32 Duty Cycle) Medium Intensity
    let data: u8 = 0x07;
    // 4-bit Address of Intensity Control Command
    let addr: u8 = 0x0A;
    // Package into array to pass to SPI write method
    // Write method will grab array and send all data in it
    let send_array: [u8; 2] = [addr, data];

    // - Send Data
    // Set CS to low to shift/clock bits into max7219 (datasheet requirement)
    cs.set_low();
    // Shift in 16 bits by passing send_array (bits will be shifted MSB first)
    spi.write(&send_array).unwrap();
    // Set CS to high to latch shifted bits into max7219 (datasheet requirement)
    cs.set_high();

    let mut delay = dp.TIM2.delay_ms(&clocks);

    loop {
        let mut data: u8 = 1;
        // Iterate over all rows of LED matrix
        for addr in 1..9 {
            // addr refrences the row data will be sent to
            let send_array: [u8; 2] = [addr, data];
            // Shift a 1 with evey loop
            data = data << 1;

            // Send data just like earlier
            cs.set_low();
            spi.write(&send_array).unwrap();
            cs.set_high();

            // Delay for 500ms to show effect
            delay.delay_ms(500_u32);
        }

        // Clear the LED matrix row by row with 500ms delay in between
        for addr in 1..9 {
            let send_array: [u8; 2] = [addr, data];
            cs.set_low();
            spi.write(&send_array).unwrap();
            cs.set_high();
            delay.delay_ms(500_u32);
        }
    }
}
