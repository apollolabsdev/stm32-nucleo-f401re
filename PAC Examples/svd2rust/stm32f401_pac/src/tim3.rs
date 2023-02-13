#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - slave mode control register"]
    pub smcr: SMCR,
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dier: DIER,
    #[doc = "0x10 - status register"]
    pub sr: SR,
    #[doc = "0x14 - event generation register"]
    pub egr: EGR,
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    #[doc = "0x20 - capture/compare enable register"]
    pub ccer: CCER,
    #[doc = "0x24 - counter"]
    pub cnt: CNT,
    #[doc = "0x28 - prescaler"]
    pub psc: PSC,
    #[doc = "0x2c - auto-reload register"]
    pub arr: ARR,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - capture/compare register 1"]
    pub ccr1: CCR1,
    #[doc = "0x38 - capture/compare register 2"]
    pub ccr2: CCR2,
    #[doc = "0x3c - capture/compare register 3"]
    pub ccr3: CCR3,
    #[doc = "0x40 - capture/compare register 4"]
    pub ccr4: CCR4,
    _reserved16: [u8; 0x04],
    #[doc = "0x48 - DMA control register"]
    pub dcr: DCR,
    #[doc = "0x4c - DMA address for full transfer"]
    pub dmar: DMAR,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(24usize).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub const fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub const fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "slave mode control register"]
pub mod smcr;
#[doc = "DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: an alias for `Reg<EGR_SPEC>`"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "event generation register"]
pub mod egr;
#[doc = "CCMR1_Output (rw) register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`"]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "CCMR1_Input (rw) register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`"]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccmr1_input;
#[doc = "CCMR2_Output (rw) register accessor: an alias for `Reg<CCMR2_OUTPUT_SPEC>`"]
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr2_output;
#[doc = "CCMR2_Input (rw) register accessor: an alias for `Reg<CCMR2_INPUT_SPEC>`"]
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>;
#[doc = "capture/compare mode register 2 (input mode)"]
pub mod ccmr2_input;
#[doc = "CCER (rw) register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "CCR1 (rw) register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "capture/compare register 1"]
pub mod ccr1;
#[doc = "CCR2 (rw) register accessor: an alias for `Reg<CCR2_SPEC>`"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "capture/compare register 2"]
pub mod ccr2;
#[doc = "CCR3 (rw) register accessor: an alias for `Reg<CCR3_SPEC>`"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "capture/compare register 3"]
pub mod ccr3;
#[doc = "CCR4 (rw) register accessor: an alias for `Reg<CCR4_SPEC>`"]
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
#[doc = "capture/compare register 4"]
pub mod ccr4;
#[doc = "DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "DMA control register"]
pub mod dcr;
#[doc = "DMAR (rw) register accessor: an alias for `Reg<DMAR_SPEC>`"]
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
#[doc = "DMA address for full transfer"]
pub mod dmar;
