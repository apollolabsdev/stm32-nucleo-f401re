#![no_std]
use panic_halt as _;
use stm32f4::stm32f401::gpioa::{afrh, afrl, bsrr, idr, lckr, moder, odr, ospeedr, otyper, pupdr};

pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub moder: stm32f4::Reg<moder::MODER_SPEC>,
    #[doc = "0x04 - GPIO port output type register"]
    pub otyper: stm32f4::Reg<otyper::OTYPER_SPEC>,
    #[doc = "0x08 - GPIO port output speed register"]
    pub ospeedr: stm32f4::Reg<ospeedr::OSPEEDR_SPEC>,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub pupdr: stm32f4::Reg<pupdr::PUPDR_SPEC>,
    #[doc = "0x10 - GPIO port input data register"]
    pub idr: stm32f4::Reg<idr::IDR_SPEC>,
    #[doc = "0x14 - GPIO port output data register"]
    pub odr: stm32f4::Reg<odr::ODR_SPEC>,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub bsrr: stm32f4::Reg<bsrr::BSRR_SPEC>,
    #[doc = "0x1c - GPIO port configuration lock register"]
    pub lckr: stm32f4::Reg<lckr::LCKR_SPEC>,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub afrl: stm32f4::Reg<afrl::AFRL_SPEC>,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub afrh: stm32f4::Reg<afrh::AFRH_SPEC>,
}

#[no_mangle]
pub extern "C" fn togglePin(p: &mut RegisterBlock) {
    // Toggle the ODR5 bit, all the other bits will remain untouched
    p.odr.modify(|r, w| w.odr5().bit(!r.odr5().bit()));
}
