#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::cell::RefCell;
use core::fmt::Write;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::blocking_mutex::Mutex;

use heapless::String;

use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, UartTx};
use embassy_time::{Duration, Timer};
use panic_halt as _;

static SHARED: Mutex<ThreadModeRawMutex, RefCell<u32>> = Mutex::new(RefCell::new(0));

#[embassy_executor::task]
async fn async_task() {
    loop {
        // Load value from global context, modify and store
        SHARED.lock(|f| {
            let val = f.borrow_mut().wrapping_add(1);
            f.replace(val);
        });
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
    let mut msg: String<8> = String::new();
    // Spawn async blinking task
    spawner.spawn(async_task()).unwrap();

    loop {
        // Wait 1 second
        Timer::after(Duration::from_millis(1000)).await;
        // Obtain updated value from global context
        let shared = SHARED.lock(|f| f.clone().into_inner());
        core::writeln!(&mut msg, "{:02}", shared).unwrap();
        // Transmit Message
        usart.blocking_write(msg.as_bytes()).unwrap();
        msg.clear();
    }
}