#![no_std]
use stm32f401_pac as pac;

pub struct Config {
    pub freq: u32,
    pub baud: u32,
}
pub struct Uart2Tx;

impl Uart2Tx {
    pub fn init(clocks: &pac::RCC, usart: &pac::USART2, cnfg: Config) {
        // Enable Clock to USART2
        clocks.apb1enr.write(|w| w.usart2en().set_bit());

        // Enable USART2 by setting the UE bit in USART_CR1 register
        usart.cr1.reset();
        usart.cr1.modify(|_, w| {
            w.ue().set_bit() // USART enabled
        });

        // Program the UART Baud Rate
        usart
            .brr
            .write(|w| unsafe { w.bits(cnfg.freq / cnfg.baud) });

        // Enable the Transmitter
        usart.cr1.modify(|_, w| w.te().set_bit());

        // Wait until TXE flag is set
        while usart.sr.read().txe().bit_is_clear() {}
    }

    pub fn blocking_write(usart: &pac::USART2, data: u16) {
        // Put Data in Data Register
        usart.dr.write(|w| unsafe { w.dr().bits(data) });
        // Wait for data to get transmitted
        while usart.sr.read().tc().bit_is_clear() {}
    }
}
