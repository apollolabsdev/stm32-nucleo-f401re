#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use panic_halt as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    bind_interrupts!(struct Irqs {
        USART2 => usart::InterruptHandler<peripherals::USART2>;
    });

    let mut usart = Uart::new(
        p.USART2,
        p.PA3,
        p.PA2,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH5,
        Config::default(),
    )
    .unwrap();

    usart.write(b"Starting Echo\r\n").await.unwrap();

    let mut msg: [u8; 8] = [0; 8];

    loop {
        usart.read(&mut msg).await.unwrap();
        usart.write(&msg).await.unwrap();
    }
}
