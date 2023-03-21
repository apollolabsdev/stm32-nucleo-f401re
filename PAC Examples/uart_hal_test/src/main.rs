#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use panic_halt as _;
use stm32_uart_hal as hal;
use stm32f401_pac as pac;

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Enable HSI Oscillator
    dp.RCC.cr.modify(|_, w| w.hsion().set_bit());

    // Wait for HSI clock to become ready
    while dp.RCC.cr.read().hsirdy().bit() {}

    // Enable Clock to GPIOA
    dp.RCC.ahb1enr.write(|w| w.gpioaen().set_bit());

    // Select Alternate Function for PA2
    dp.GPIOA.afrl.modify(|_, w| unsafe { w.afrl2().bits(7) });

    // Configure PA2 as Alternate Output
    dp.GPIOA.moder.modify(|_, w| unsafe { w.moder2().bits(2) });

    // Configure USART Parameters
    let usartcfg = hal::Config {
        freq: 16_000_000,
        baud: 115_200,
    };

    // Initialize USART
    hal::Uart2Tx::init(&dp.RCC, &dp.USART2, usartcfg);

    loop {
        hal::Uart2Tx::blocking_write(&dp.USART2, b'A' as u16);
    }
}
