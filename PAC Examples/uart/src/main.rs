#![no_std]
#![no_main]

// Imports
use cortex_m_rt::entry;
use pac::interrupt;
use panic_halt as _;
use stm32f401_pac as pac;

const FREQ: u32 = 16_000_000;
const BAUD: u32 = 115_200;

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

    // Enable Clock to USART2
    dp.RCC.apb1enr.write(|w| w.usart2en().set_bit());

    // Select Alternate Function for PA2
    dp.GPIOA.afrl.modify(|_, w| unsafe { w.afrl2().bits(7) });

    // Configure PA2 as Alternate Output
    dp.GPIOA.moder.modify(|_, w| unsafe { w.moder2().bits(2) });

    // Enable USART2 by setting the UE bit in USART_CR1 register
    dp.USART2.cr1.reset();
    dp.USART2.cr1.modify(|_, w| {
        w.ue().set_bit() // USART enabled
    });

    // Program the UART Baud Rate
    dp.USART2.brr.write(|w| unsafe { w.bits(FREQ / BAUD) });

    // Enable the Transmitter
    dp.USART2.cr1.modify(|_, w| w.te().set_bit());

    // Wait until TXE flag is set
    while dp.USART2.sr.read().txe().bit_is_clear() {}

    loop {
        // Put Data in Data Register
        dp.USART2.dr.write(|w| unsafe { w.dr().bits(b'A' as u16) });
        // Wait for data to get transmitted
        while dp.USART2.sr.read().tc().bit_is_clear() {}
    }
}

#[interrupt]
fn EXT13() {}
