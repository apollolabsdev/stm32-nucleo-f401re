#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;

use heapless::String;

use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, UartTx};
use embassy_time::{Duration, Timer};
use panic_halt as _;

static SHARED: Signal<ThreadModeRawMutex, u32> = Signal::new();

#[embassy_executor::task]
async fn async_task() {
    loop {
        SHARED.signal(5);
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize and create handle for device peripherals
    let p = embassy_stm32::init(Default::default());
    //Configure UART
    let mut usart = UartTx::new(p.USART2, p.PA2, NoDma, Config::default());
    // Create empty String for message
    let mut msg: String<16> = String::new();
    // Spawn async blinking task
    spawner.spawn(async_task()).unwrap();

    loop {
        let val = SHARED.wait().await;
        core::writeln!(&mut msg, "Signal Detected {:02}", val).unwrap();
        //Transmit Message
        usart.blocking_write(msg.as_bytes()).unwrap();
        msg.clear();
    }
}
