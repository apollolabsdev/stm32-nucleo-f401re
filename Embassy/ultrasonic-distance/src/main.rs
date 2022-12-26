#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayUs;
use heapless::String;

use cortex_m_rt::entry;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Flex, Pull, Speed};
use embassy_stm32::usart::{Config, UartTx};
use embassy_time::{Delay, Instant};
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Initialize and create handle for devicer peripherals
    let p = embassy_stm32::init(Default::default());

    let mut echo = Flex::new(p.PA8);

    //Configure UART
    let mut usart = UartTx::new(p.USART2, p.PA2, NoDma, Config::default());

    // Create empty String for message
    let mut msg: String<64> = String::new();

    // Delay Handle
    let mut delay = Delay;

    // Algorithm
    // 1) Set pin ouput to low for 5 us to get clean low pulse
    // 2) Set pin output to high (trigger) for 10us
    // 3) Switch back to input
    // 4) Keep checking if pin goes high
    // 5) Once pin goes high start kick off counter/timer
    // 6) Wait for Pin to go low
    // 7) Obtain pulse measurement from timer
    // 8) Print out measurement on Serial
    // 9) Go back to 1)

    // Application Loop
    loop {
        // 1) Set pin ouput to low for 5 us to get clean low pulse
        echo.set_as_output(Speed::Low);
        echo.set_low();
        delay.delay_us(5_u32);

        // 2) Set pin output to high (trigger) for 10us
        echo.set_high();
        delay.delay_us(10_u32);

        // 3) Switch back to input
        echo.set_as_input(Pull::None);

        // Wait until pin goes high
        while !(echo.is_high()) {}

        // Kick off timer measurement by capturing current instant
        let inst = Instant::now();

        // Wait until pin goes low.
        while !(echo.is_low()) {}

        // Stop timer and collect elapsed time
        let duration = Instant::checked_duration_since(&Instant::now(), inst).unwrap();

        // Calculate the distance in cms using formula in datasheet
        let distance_cm = duration.as_micros() / 2 / 29;

        // Send calculated distance to serial interface
        core::writeln!(&mut msg, "Distance {:02} cm\r", distance_cm).unwrap();
        usart.blocking_write(msg.as_bytes()).unwrap();
        msg.clear();
    }
}
