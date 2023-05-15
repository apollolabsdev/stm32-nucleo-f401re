#![no_std]
use embedded_hal as ehal;
use nb;
use stm32f401_pac as pac;

pub struct Config {
    pub freq: u32,
    pub baud: u32,
}
pub struct SerUart<USART> {
    usart: USART,
}

pub enum Errors {}

impl ehal::serial::Write<u8> for SerUart<pac::USART2> {
    type Error = Errors;
    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        // Put Data in Data Register
        self.usart.dr.write(|w| unsafe { w.dr().bits(word as u16) });
        // Wait for data to get transmitted
        while self.usart.sr.read().tc().bit_is_clear() {}
        Ok(())
    }
    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        Ok(())
    }
}

impl SerUart<pac::USART2> {
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
}
