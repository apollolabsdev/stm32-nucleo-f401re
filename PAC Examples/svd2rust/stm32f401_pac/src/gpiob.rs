#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO port mode register"]
    pub moder: MODER,
    #[doc = "0x04 - GPIO port output type register"]
    pub otyper: OTYPER,
    #[doc = "0x08 - GPIO port output speed register"]
    pub ospeedr: OSPEEDR,
    #[doc = "0x0c - GPIO port pull-up/pull-down register"]
    pub pupdr: PUPDR,
    #[doc = "0x10 - GPIO port input data register"]
    pub idr: IDR,
    #[doc = "0x14 - GPIO port output data register"]
    pub odr: ODR,
    #[doc = "0x18 - GPIO port bit set/reset register"]
    pub bsrr: BSRR,
    #[doc = "0x1c - GPIO port configuration lock register"]
    pub lckr: LCKR,
    #[doc = "0x20 - GPIO alternate function low register"]
    pub afrl: AFRL,
    #[doc = "0x24 - GPIO alternate function high register"]
    pub afrh: AFRH,
}
#[doc = "MODER (rw) register accessor: an alias for `Reg<MODER_SPEC>`"]
pub type MODER = crate::Reg<moder::MODER_SPEC>;
#[doc = "GPIO port mode register"]
pub mod moder;
#[doc = "OTYPER (rw) register accessor: an alias for `Reg<OTYPER_SPEC>`"]
pub type OTYPER = crate::Reg<otyper::OTYPER_SPEC>;
#[doc = "GPIO port output type register"]
pub mod otyper;
#[doc = "OSPEEDR (rw) register accessor: an alias for `Reg<OSPEEDR_SPEC>`"]
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDR_SPEC>;
#[doc = "GPIO port output speed register"]
pub mod ospeedr;
#[doc = "PUPDR (rw) register accessor: an alias for `Reg<PUPDR_SPEC>`"]
pub type PUPDR = crate::Reg<pupdr::PUPDR_SPEC>;
#[doc = "GPIO port pull-up/pull-down register"]
pub mod pupdr;
#[doc = "IDR (r) register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "GPIO port input data register"]
pub mod idr;
#[doc = "ODR (rw) register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "GPIO port output data register"]
pub mod odr;
#[doc = "BSRR (w) register accessor: an alias for `Reg<BSRR_SPEC>`"]
pub type BSRR = crate::Reg<bsrr::BSRR_SPEC>;
#[doc = "GPIO port bit set/reset register"]
pub mod bsrr;
#[doc = "LCKR (rw) register accessor: an alias for `Reg<LCKR_SPEC>`"]
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
#[doc = "GPIO port configuration lock register"]
pub mod lckr;
#[doc = "AFRL (rw) register accessor: an alias for `Reg<AFRL_SPEC>`"]
pub type AFRL = crate::Reg<afrl::AFRL_SPEC>;
#[doc = "GPIO alternate function low register"]
pub mod afrl;
#[doc = "AFRH (rw) register accessor: an alias for `Reg<AFRH_SPEC>`"]
pub type AFRH = crate::Reg<afrh::AFRH_SPEC>;
#[doc = "GPIO alternate function high register"]
pub mod afrh;
