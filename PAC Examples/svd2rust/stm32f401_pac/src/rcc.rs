#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - clock control register"]
    pub cr: CR,
    #[doc = "0x04 - PLL configuration register"]
    pub pllcfgr: PLLCFGR,
    #[doc = "0x08 - clock configuration register"]
    pub cfgr: CFGR,
    #[doc = "0x0c - clock interrupt register"]
    pub cir: CIR,
    #[doc = "0x10 - AHB1 peripheral reset register"]
    pub ahb1rstr: AHB1RSTR,
    #[doc = "0x14 - AHB2 peripheral reset register"]
    pub ahb2rstr: AHB2RSTR,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - APB1 peripheral reset register"]
    pub apb1rstr: APB1RSTR,
    #[doc = "0x24 - APB2 peripheral reset register"]
    pub apb2rstr: APB2RSTR,
    _reserved8: [u8; 0x08],
    #[doc = "0x30 - AHB1 peripheral clock register"]
    pub ahb1enr: AHB1ENR,
    #[doc = "0x34 - AHB2 peripheral clock enable register"]
    pub ahb2enr: AHB2ENR,
    _reserved10: [u8; 0x08],
    #[doc = "0x40 - APB1 peripheral clock enable register"]
    pub apb1enr: APB1ENR,
    #[doc = "0x44 - APB2 peripheral clock enable register"]
    pub apb2enr: APB2ENR,
    _reserved12: [u8; 0x08],
    #[doc = "0x50 - AHB1 peripheral clock enable in low power mode register"]
    pub ahb1lpenr: AHB1LPENR,
    #[doc = "0x54 - AHB2 peripheral clock enable in low power mode register"]
    pub ahb2lpenr: AHB2LPENR,
    _reserved14: [u8; 0x08],
    #[doc = "0x60 - APB1 peripheral clock enable in low power mode register"]
    pub apb1lpenr: APB1LPENR,
    #[doc = "0x64 - APB2 peripheral clock enabled in low power mode register"]
    pub apb2lpenr: APB2LPENR,
    _reserved16: [u8; 0x08],
    #[doc = "0x70 - Backup domain control register"]
    pub bdcr: BDCR,
    #[doc = "0x74 - clock control & status register"]
    pub csr: CSR,
    _reserved18: [u8; 0x08],
    #[doc = "0x80 - spread spectrum clock generation register"]
    pub sscgr: SSCGR,
    #[doc = "0x84 - PLLI2S configuration register"]
    pub plli2scfgr: PLLI2SCFGR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "clock control register"]
pub mod cr;
#[doc = "PLLCFGR (rw) register accessor: an alias for `Reg<PLLCFGR_SPEC>`"]
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
#[doc = "PLL configuration register"]
pub mod pllcfgr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "clock configuration register"]
pub mod cfgr;
#[doc = "CIR (rw) register accessor: an alias for `Reg<CIR_SPEC>`"]
pub type CIR = crate::Reg<cir::CIR_SPEC>;
#[doc = "clock interrupt register"]
pub mod cir;
#[doc = "AHB1RSTR (rw) register accessor: an alias for `Reg<AHB1RSTR_SPEC>`"]
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
#[doc = "AHB1 peripheral reset register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: an alias for `Reg<AHB2RSTR_SPEC>`"]
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
#[doc = "AHB2 peripheral reset register"]
pub mod ahb2rstr;
#[doc = "APB1RSTR (rw) register accessor: an alias for `Reg<APB1RSTR_SPEC>`"]
pub type APB1RSTR = crate::Reg<apb1rstr::APB1RSTR_SPEC>;
#[doc = "APB1 peripheral reset register"]
pub mod apb1rstr;
#[doc = "APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`"]
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
#[doc = "APB2 peripheral reset register"]
pub mod apb2rstr;
#[doc = "AHB1ENR (rw) register accessor: an alias for `Reg<AHB1ENR_SPEC>`"]
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
#[doc = "AHB1 peripheral clock register"]
pub mod ahb1enr;
#[doc = "AHB2ENR (rw) register accessor: an alias for `Reg<AHB2ENR_SPEC>`"]
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
#[doc = "AHB2 peripheral clock enable register"]
pub mod ahb2enr;
#[doc = "APB1ENR (rw) register accessor: an alias for `Reg<APB1ENR_SPEC>`"]
pub type APB1ENR = crate::Reg<apb1enr::APB1ENR_SPEC>;
#[doc = "APB1 peripheral clock enable register"]
pub mod apb1enr;
#[doc = "APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`"]
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
#[doc = "APB2 peripheral clock enable register"]
pub mod apb2enr;
#[doc = "AHB1LPENR (rw) register accessor: an alias for `Reg<AHB1LPENR_SPEC>`"]
pub type AHB1LPENR = crate::Reg<ahb1lpenr::AHB1LPENR_SPEC>;
#[doc = "AHB1 peripheral clock enable in low power mode register"]
pub mod ahb1lpenr;
#[doc = "AHB2LPENR (rw) register accessor: an alias for `Reg<AHB2LPENR_SPEC>`"]
pub type AHB2LPENR = crate::Reg<ahb2lpenr::AHB2LPENR_SPEC>;
#[doc = "AHB2 peripheral clock enable in low power mode register"]
pub mod ahb2lpenr;
#[doc = "APB1LPENR (rw) register accessor: an alias for `Reg<APB1LPENR_SPEC>`"]
pub type APB1LPENR = crate::Reg<apb1lpenr::APB1LPENR_SPEC>;
#[doc = "APB1 peripheral clock enable in low power mode register"]
pub mod apb1lpenr;
#[doc = "APB2LPENR (rw) register accessor: an alias for `Reg<APB2LPENR_SPEC>`"]
pub type APB2LPENR = crate::Reg<apb2lpenr::APB2LPENR_SPEC>;
#[doc = "APB2 peripheral clock enabled in low power mode register"]
pub mod apb2lpenr;
#[doc = "BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`"]
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
#[doc = "Backup domain control register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "clock control & status register"]
pub mod csr;
#[doc = "SSCGR (rw) register accessor: an alias for `Reg<SSCGR_SPEC>`"]
pub type SSCGR = crate::Reg<sscgr::SSCGR_SPEC>;
#[doc = "spread spectrum clock generation register"]
pub mod sscgr;
#[doc = "PLLI2SCFGR (rw) register accessor: an alias for `Reg<PLLI2SCFGR_SPEC>`"]
pub type PLLI2SCFGR = crate::Reg<plli2scfgr::PLLI2SCFGR_SPEC>;
#[doc = "PLLI2S configuration register"]
pub mod plli2scfgr;
