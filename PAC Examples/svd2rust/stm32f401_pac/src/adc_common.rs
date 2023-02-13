#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Common status register"]
    pub csr: CSR,
    #[doc = "0x04 - ADC common control register"]
    pub ccr: CCR,
}
#[doc = "CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "ADC Common status register"]
pub mod csr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "ADC common control register"]
pub mod ccr;
