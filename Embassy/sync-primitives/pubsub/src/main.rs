#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::pubsub::PubSubChannel;

use heapless::String;

use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::usart::{Config, UartTx};
use embassy_time::{Duration, Timer};
use panic_halt as _;

//Declare a pubsub channel with a capcity of 2 and 1 subscriber and 2 publishers
static SHARED: PubSubChannel<ThreadModeRawMutex, u32, 2, 2, 2> = PubSubChannel::new();

#[embassy_executor::task]
async fn async_task_one() {
    let pub1 = SHARED.publisher().unwrap();
    loop {
        pub1.publish_immediate(1);
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn async_task_two() {
    let pub2 = SHARED.publisher().unwrap();
    loop {
        pub2.publish_immediate(2);
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
    spawner.spawn(async_task_one()).unwrap();
    spawner.spawn(async_task_two()).unwrap();

    let mut sub = SHARED.subscriber().unwrap();

    loop {
        let val = sub.next_message_pure().await;
        core::writeln!(&mut msg, "{:02}", val).unwrap();
        //Transmit Message
        usart.blocking_write(msg.as_bytes()).unwrap();
        msg.clear();
    }
}
