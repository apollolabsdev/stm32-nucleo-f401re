#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;
use core::sync::atomic::{AtomicU32, Ordering};

use heapless::String;

use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, UartTx};
use embassy_time::{Duration, Timer};
use panic_halt as _;

static SHARED: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
async fn async_task() {
    loop {
        // Load value from global context, modify and store
        let shared_var = SHARED.load(Ordering::Relaxed);
        SHARED.store(shared_var.wrapping_add(1), Ordering::Relaxed);
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // Initialize and create handle for devicer peripherals
    let p = embassy_stm32::init(Default::default());
    //Configure UART
    let mut usart = UartTx::new(p.USART2, p.PA2, NoDma, Config::default());
    // Create empty String for message
    let mut msg: String<8> = String::new();
    // Spawn async blinking task
    spawner.spawn(async_task()).unwrap();

    loop {
        // Load value from global context
        let shared = SHARED.load(Ordering::Relaxed);
        // Wait 1 second
        Timer::after(Duration::from_millis(1000)).await;
        // Format value for printing
        core::writeln!(&mut msg, "{:02}", shared).unwrap();
        // Transmit Message
        usart.blocking_write(msg.as_bytes()).unwrap();
        msg.clear();
    }
}
