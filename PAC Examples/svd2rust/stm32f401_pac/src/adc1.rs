#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - status register"]
    pub sr: SR,
    #[doc = "0x04 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x08 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x0c - sample time register 1"]
    pub smpr1: SMPR1,
    #[doc = "0x10 - sample time register 2"]
    pub smpr2: SMPR2,
    #[doc = "0x14 - injected channel data offset register x"]
    pub jofr1: JOFR1,
    #[doc = "0x18 - injected channel data offset register x"]
    pub jofr2: JOFR2,
    #[doc = "0x1c - injected channel data offset register x"]
    pub jofr3: JOFR3,
    #[doc = "0x20 - injected channel data offset register x"]
    pub jofr4: JOFR4,
    #[doc = "0x24 - watchdog higher threshold register"]
    pub htr: HTR,
    #[doc = "0x28 - watchdog lower threshold register"]
    pub ltr: LTR,
    #[doc = "0x2c - regular sequence register 1"]
    pub sqr1: SQR1,
    #[doc = "0x30 - regular sequence register 2"]
    pub sqr2: SQR2,
    #[doc = "0x34 - regular sequence register 3"]
    pub sqr3: SQR3,
    #[doc = "0x38 - injected sequence register"]
    pub jsqr: JSQR,
    #[doc = "0x3c - injected data register x"]
    pub jdr1: JDR1,
    #[doc = "0x40 - injected data register x"]
    pub jdr2: JDR2,
    #[doc = "0x44 - injected data register x"]
    pub jdr3: JDR3,
    #[doc = "0x48 - injected data register x"]
    pub jdr4: JDR4,
    #[doc = "0x4c - regular data register"]
    pub dr: DR,
}
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SMPR1 (rw) register accessor: an alias for `Reg<SMPR1_SPEC>`"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "sample time register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: an alias for `Reg<SMPR2_SPEC>`"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "sample time register 2"]
pub mod smpr2;
#[doc = "JOFR1 (rw) register accessor: an alias for `Reg<JOFR1_SPEC>`"]
pub type JOFR1 = crate::Reg<jofr1::JOFR1_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod jofr1;
#[doc = "JOFR2 (rw) register accessor: an alias for `Reg<JOFR2_SPEC>`"]
pub type JOFR2 = crate::Reg<jofr2::JOFR2_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod jofr2;
#[doc = "JOFR3 (rw) register accessor: an alias for `Reg<JOFR3_SPEC>`"]
pub type JOFR3 = crate::Reg<jofr3::JOFR3_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod jofr3;
#[doc = "JOFR4 (rw) register accessor: an alias for `Reg<JOFR4_SPEC>`"]
pub type JOFR4 = crate::Reg<jofr4::JOFR4_SPEC>;
#[doc = "injected channel data offset register x"]
pub mod jofr4;
#[doc = "HTR (rw) register accessor: an alias for `Reg<HTR_SPEC>`"]
pub type HTR = crate::Reg<htr::HTR_SPEC>;
#[doc = "watchdog higher threshold register"]
pub mod htr;
#[doc = "LTR (rw) register accessor: an alias for `Reg<LTR_SPEC>`"]
pub type LTR = crate::Reg<ltr::LTR_SPEC>;
#[doc = "watchdog lower threshold register"]
pub mod ltr;
#[doc = "SQR1 (rw) register accessor: an alias for `Reg<SQR1_SPEC>`"]
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
#[doc = "regular sequence register 1"]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: an alias for `Reg<SQR2_SPEC>`"]
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
#[doc = "regular sequence register 2"]
pub mod sqr2;
#[doc = "SQR3 (rw) register accessor: an alias for `Reg<SQR3_SPEC>`"]
pub type SQR3 = crate::Reg<sqr3::SQR3_SPEC>;
#[doc = "regular sequence register 3"]
pub mod sqr3;
#[doc = "JSQR (rw) register accessor: an alias for `Reg<JSQR_SPEC>`"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "injected sequence register"]
pub mod jsqr;
#[doc = "JDR1 (r) register accessor: an alias for `Reg<JDR1_SPEC>`"]
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
#[doc = "injected data register x"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: an alias for `Reg<JDR2_SPEC>`"]
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
#[doc = "injected data register x"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: an alias for `Reg<JDR3_SPEC>`"]
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
#[doc = "injected data register x"]
pub mod jdr3;
#[doc = "JDR4 (r) register accessor: an alias for `Reg<JDR4_SPEC>`"]
pub type JDR4 = crate::Reg<jdr4::JDR4_SPEC>;
#[doc = "injected data register x"]
pub mod jdr4;
#[doc = "DR (r) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "regular data register"]
pub mod dr;
