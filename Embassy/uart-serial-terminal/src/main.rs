#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;
use core::sync::atomic::{AtomicU32, Ordering};

use heapless::String;

use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed};
use embassy_stm32::usart::{Config, UartTx};
use embassy_time::{Duration, Timer};
use panic_halt as _;

static BLINK_MS: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
async fn led_task(led: AnyPin) {
    // Configure the LED pin as a low -speed output and obtain a handler
    // Initialize LED output to Low
    // On the Nucleo FR401 theres an on-board LED connected to pin PA5
    let mut led = Output::new(led, Level::Low, Speed::Low);

    loop {
        let del = BLINK_MS.load(Ordering::Relaxed);
        Timer::after(Duration::from_millis(del.into())).await;
        led.toggle();
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize and create handle for devicer peripherals
    let p = embassy_stm32::init(Default::default());

    // Configure the button pin (if needed) and obtain handler.
    // On the Nucleo FR401 there is a button connected to pin PC13.
    let button = Input::new(p.PC13, Pull::None);
    let mut button = ExtiInput::new(button, p.EXTI13);

    //Configure UART
    let mut usart = UartTx::new(p.USART2, p.PA2, NoDma, Config::default());

    // Create and initialize a delay variable to manage delay loop
    let mut del_var = 2000;

    // Publish blink duration value to global context
    BLINK_MS.store(del_var, Ordering::Relaxed);

    // Spawn LED blinking task
    spawner.spawn(led_task(p.PA5.degrade())).unwrap();

    // Variable to keep track of how many button presses occured
    let mut value: u8 = 0;

    // Create empty String for message
    let mut msg: String<8> = String::new();

    loop {
        // Check if button got pressed
        button.wait_for_rising_edge().await;

        // If button pressed decrease the delay value
        del_var = del_var - 300;
        // If updated delay value drops below 300 then reset it back to starting value
        if del_var < 500 {
            del_var = 2000;
        }
        // Publish updated delay value to global context
        BLINK_MS.store(del_var, Ordering::Relaxed);

        // Format Message
        core::writeln!(&mut msg, "{:02}", value).unwrap();

        // Transmit Message
        usart.blocking_write(msg.as_bytes()).unwrap();

        // Update Value Parameter
        value = value.wrapping_add(1);

        // Clear String for next message
        msg.clear();
    }
}
