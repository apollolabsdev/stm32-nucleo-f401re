#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_stm32::interrupt;
use embassy_stm32::usart::{Config, Uart};
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let irq = interrupt::take!(USART2);
    let mut usart = Uart::new(
        p.USART2,
        p.PA3,
        p.PA2,
        irq,
        p.DMA1_CH6,
        p.DMA1_CH5,
        Config::default(),
    );

    usart.write(b"Starting Echo\r\n").await.unwrap();

    let mut msg: [u8; 8] = [0; 8];

    loop {
        usart.read(&mut msg).await.unwrap();
        usart.write(&msg).await.unwrap();
    }
}
