#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - low interrupt status register"]
    pub lisr: LISR,
    #[doc = "0x04 - high interrupt status register"]
    pub hisr: HISR,
    #[doc = "0x08 - low interrupt flag clear register"]
    pub lifcr: LIFCR,
    #[doc = "0x0c - high interrupt flag clear register"]
    pub hifcr: HIFCR,
    #[doc = "0x10 - stream x configuration register"]
    pub s0cr: S0CR,
    #[doc = "0x14 - stream x number of data register"]
    pub s0ndtr: S0NDTR,
    #[doc = "0x18 - stream x peripheral address register"]
    pub s0par: S0PAR,
    #[doc = "0x1c - stream x memory 0 address register"]
    pub s0m0ar: S0M0AR,
    #[doc = "0x20 - stream x memory 1 address register"]
    pub s0m1ar: S0M1AR,
    #[doc = "0x24 - stream x FIFO control register"]
    pub s0fcr: S0FCR,
    #[doc = "0x28 - stream x configuration register"]
    pub s1cr: S1CR,
    #[doc = "0x2c - stream x number of data register"]
    pub s1ndtr: S1NDTR,
    #[doc = "0x30 - stream x peripheral address register"]
    pub s1par: S1PAR,
    #[doc = "0x34 - stream x memory 0 address register"]
    pub s1m0ar: S1M0AR,
    #[doc = "0x38 - stream x memory 1 address register"]
    pub s1m1ar: S1M1AR,
    #[doc = "0x3c - stream x FIFO control register"]
    pub s1fcr: S1FCR,
    #[doc = "0x40 - stream x configuration register"]
    pub s2cr: S2CR,
    #[doc = "0x44 - stream x number of data register"]
    pub s2ndtr: S2NDTR,
    #[doc = "0x48 - stream x peripheral address register"]
    pub s2par: S2PAR,
    #[doc = "0x4c - stream x memory 0 address register"]
    pub s2m0ar: S2M0AR,
    #[doc = "0x50 - stream x memory 1 address register"]
    pub s2m1ar: S2M1AR,
    #[doc = "0x54 - stream x FIFO control register"]
    pub s2fcr: S2FCR,
    #[doc = "0x58 - stream x configuration register"]
    pub s3cr: S3CR,
    #[doc = "0x5c - stream x number of data register"]
    pub s3ndtr: S3NDTR,
    #[doc = "0x60 - stream x peripheral address register"]
    pub s3par: S3PAR,
    #[doc = "0x64 - stream x memory 0 address register"]
    pub s3m0ar: S3M0AR,
    #[doc = "0x68 - stream x memory 1 address register"]
    pub s3m1ar: S3M1AR,
    #[doc = "0x6c - stream x FIFO control register"]
    pub s3fcr: S3FCR,
    #[doc = "0x70 - stream x configuration register"]
    pub s4cr: S4CR,
    #[doc = "0x74 - stream x number of data register"]
    pub s4ndtr: S4NDTR,
    #[doc = "0x78 - stream x peripheral address register"]
    pub s4par: S4PAR,
    #[doc = "0x7c - stream x memory 0 address register"]
    pub s4m0ar: S4M0AR,
    #[doc = "0x80 - stream x memory 1 address register"]
    pub s4m1ar: S4M1AR,
    #[doc = "0x84 - stream x FIFO control register"]
    pub s4fcr: S4FCR,
    #[doc = "0x88 - stream x configuration register"]
    pub s5cr: S5CR,
    #[doc = "0x8c - stream x number of data register"]
    pub s5ndtr: S5NDTR,
    #[doc = "0x90 - stream x peripheral address register"]
    pub s5par: S5PAR,
    #[doc = "0x94 - stream x memory 0 address register"]
    pub s5m0ar: S5M0AR,
    #[doc = "0x98 - stream x memory 1 address register"]
    pub s5m1ar: S5M1AR,
    #[doc = "0x9c - stream x FIFO control register"]
    pub s5fcr: S5FCR,
    #[doc = "0xa0 - stream x configuration register"]
    pub s6cr: S6CR,
    #[doc = "0xa4 - stream x number of data register"]
    pub s6ndtr: S6NDTR,
    #[doc = "0xa8 - stream x peripheral address register"]
    pub s6par: S6PAR,
    #[doc = "0xac - stream x memory 0 address register"]
    pub s6m0ar: S6M0AR,
    #[doc = "0xb0 - stream x memory 1 address register"]
    pub s6m1ar: S6M1AR,
    #[doc = "0xb4 - stream x FIFO control register"]
    pub s6fcr: S6FCR,
    #[doc = "0xb8 - stream x configuration register"]
    pub s7cr: S7CR,
    #[doc = "0xbc - stream x number of data register"]
    pub s7ndtr: S7NDTR,
    #[doc = "0xc0 - stream x peripheral address register"]
    pub s7par: S7PAR,
    #[doc = "0xc4 - stream x memory 0 address register"]
    pub s7m0ar: S7M0AR,
    #[doc = "0xc8 - stream x memory 1 address register"]
    pub s7m1ar: S7M1AR,
    #[doc = "0xcc - stream x FIFO control register"]
    pub s7fcr: S7FCR,
}
#[doc = "LISR (r) register accessor: an alias for `Reg<LISR_SPEC>`"]
pub type LISR = crate::Reg<lisr::LISR_SPEC>;
#[doc = "low interrupt status register"]
pub mod lisr;
#[doc = "HISR (r) register accessor: an alias for `Reg<HISR_SPEC>`"]
pub type HISR = crate::Reg<hisr::HISR_SPEC>;
#[doc = "high interrupt status register"]
pub mod hisr;
#[doc = "LIFCR (w) register accessor: an alias for `Reg<LIFCR_SPEC>`"]
pub type LIFCR = crate::Reg<lifcr::LIFCR_SPEC>;
#[doc = "low interrupt flag clear register"]
pub mod lifcr;
#[doc = "HIFCR (w) register accessor: an alias for `Reg<HIFCR_SPEC>`"]
pub type HIFCR = crate::Reg<hifcr::HIFCR_SPEC>;
#[doc = "high interrupt flag clear register"]
pub mod hifcr;
#[doc = "S0CR (rw) register accessor: an alias for `Reg<S0CR_SPEC>`"]
pub type S0CR = crate::Reg<s0cr::S0CR_SPEC>;
#[doc = "stream x configuration register"]
pub mod s0cr;
#[doc = "S0NDTR (rw) register accessor: an alias for `Reg<S0NDTR_SPEC>`"]
pub type S0NDTR = crate::Reg<s0ndtr::S0NDTR_SPEC>;
#[doc = "stream x number of data register"]
pub mod s0ndtr;
#[doc = "S0PAR (rw) register accessor: an alias for `Reg<S0PAR_SPEC>`"]
pub type S0PAR = crate::Reg<s0par::S0PAR_SPEC>;
#[doc = "stream x peripheral address register"]
pub mod s0par;
#[doc = "S0M0AR (rw) register accessor: an alias for `Reg<S0M0AR_SPEC>`"]
pub type S0M0AR = crate::Reg<s0m0ar::S0M0AR_SPEC>;
#[doc = "stream x memory 0 address register"]
pub mod s0m0ar;
#[doc = "S0M1AR (rw) register accessor: an alias for `Reg<S0M1AR_SPEC>`"]
pub type S0M1AR = crate::Reg<s0m1ar::S0M1AR_SPEC>;
#[doc = "stream x memory 1 address register"]
pub mod s0m1ar;
#[doc = "S0FCR (rw) register accessor: an alias for `Reg<S0FCR_SPEC>`"]
pub type S0FCR = crate::Reg<s0fcr::S0FCR_SPEC>;
#[doc = "stream x FIFO control register"]
pub mod s0fcr;
#[doc = "S1CR (rw) register accessor: an alias for `Reg<S1CR_SPEC>`"]
pub type S1CR = crate::Reg<s1cr::S1CR_SPEC>;
#[doc = "stream x configuration register"]
pub mod s1cr;
#[doc = "S1NDTR (rw) register accessor: an alias for `Reg<S1NDTR_SPEC>`"]
pub type S1NDTR = crate::Reg<s1ndtr::S1NDTR_SPEC>;
#[doc = "stream x number of data register"]
pub mod s1ndtr;
#[doc = "S1PAR (rw) register accessor: an alias for `Reg<S1PAR_SPEC>`"]
pub type S1PAR = crate::Reg<s1par::S1PAR_SPEC>;
#[doc = "stream x peripheral address register"]
pub mod s1par;
#[doc = "S1M0AR (rw) register accessor: an alias for `Reg<S1M0AR_SPEC>`"]
pub type S1M0AR = crate::Reg<s1m0ar::S1M0AR_SPEC>;
#[doc = "stream x memory 0 address register"]
pub mod s1m0ar;
#[doc = "S1M1AR (rw) register accessor: an alias for `Reg<S1M1AR_SPEC>`"]
pub type S1M1AR = crate::Reg<s1m1ar::S1M1AR_SPEC>;
#[doc = "stream x memory 1 address register"]
pub mod s1m1ar;
#[doc = "S1FCR (rw) register accessor: an alias for `Reg<S1FCR_SPEC>`"]
pub type S1FCR = crate::Reg<s1fcr::S1FCR_SPEC>;
#[doc = "stream x FIFO control register"]
pub mod s1fcr;
#[doc = "S2CR (rw) register accessor: an alias for `Reg<S2CR_SPEC>`"]
pub type S2CR = crate::Reg<s2cr::S2CR_SPEC>;
#[doc = "stream x configuration register"]
pub mod s2cr;
#[doc = "S2NDTR (rw) register accessor: an alias for `Reg<S2NDTR_SPEC>`"]
pub type S2NDTR = crate::Reg<s2ndtr::S2NDTR_SPEC>;
#[doc = "stream x number of data register"]
pub mod s2ndtr;
#[doc = "S2PAR (rw) register accessor: an alias for `Reg<S2PAR_SPEC>`"]
pub type S2PAR = crate::Reg<s2par::S2PAR_SPEC>;
#[doc = "stream x peripheral address register"]
pub mod s2par;
#[doc = "S2M0AR (rw) register accessor: an alias for `Reg<S2M0AR_SPEC>`"]
pub type S2M0AR = crate::Reg<s2m0ar::S2M0AR_SPEC>;
#[doc = "stream x memory 0 address register"]
pub mod s2m0ar;
#[doc = "S2M1AR (rw) register accessor: an alias for `Reg<S2M1AR_SPEC>`"]
pub type S2M1AR = crate::Reg<s2m1ar::S2M1AR_SPEC>;
#[doc = "stream x memory 1 address register"]
pub mod s2m1ar;
#[doc = "S2FCR (rw) register accessor: an alias for `Reg<S2FCR_SPEC>`"]
pub type S2FCR = crate::Reg<s2fcr::S2FCR_SPEC>;
#[doc = "stream x FIFO control register"]
pub mod s2fcr;
#[doc = "S3CR (rw) register accessor: an alias for `Reg<S3CR_SPEC>`"]
pub type S3CR = crate::Reg<s3cr::S3CR_SPEC>;
#[doc = "stream x configuration register"]
pub mod s3cr;
#[doc = "S3NDTR (rw) register accessor: an alias for `Reg<S3NDTR_SPEC>`"]
pub type S3NDTR = crate::Reg<s3ndtr::S3NDTR_SPEC>;
#[doc = "stream x number of data register"]
pub mod s3ndtr;
#[doc = "S3PAR (rw) register accessor: an alias for `Reg<S3PAR_SPEC>`"]
pub type S3PAR = crate::Reg<s3par::S3PAR_SPEC>;
#[doc = "stream x peripheral address register"]
pub mod s3par;
#[doc = "S3M0AR (rw) register accessor: an alias for `Reg<S3M0AR_SPEC>`"]
pub type S3M0AR = crate::Reg<s3m0ar::S3M0AR_SPEC>;
#[doc = "stream x memory 0 address register"]
pub mod s3m0ar;
#[doc = "S3M1AR (rw) register accessor: an alias for `Reg<S3M1AR_SPEC>`"]
pub type S3M1AR = crate::Reg<s3m1ar::S3M1AR_SPEC>;
#[doc = "stream x memory 1 address register"]
pub mod s3m1ar;
#[doc = "S3FCR (rw) register accessor: an alias for `Reg<S3FCR_SPEC>`"]
pub type S3FCR = crate::Reg<s3fcr::S3FCR_SPEC>;
#[doc = "stream x FIFO control register"]
pub mod s3fcr;
#[doc = "S4CR (rw) register accessor: an alias for `Reg<S4CR_SPEC>`"]
pub type S4CR = crate::Reg<s4cr::S4CR_SPEC>;
#[doc = "stream x configuration register"]
pub mod s4cr;
#[doc = "S4NDTR (rw) register accessor: an alias for `Reg<S4NDTR_SPEC>`"]
pub type S4NDTR = crate::Reg<s4ndtr::S4NDTR_SPEC>;
#[doc = "stream x number of data register"]
pub mod s4ndtr;
#[doc = "S4PAR (rw) register accessor: an alias for `Reg<S4PAR_SPEC>`"]
pub type S4PAR = crate::Reg<s4par::S4PAR_SPEC>;
#[doc = "stream x peripheral address register"]
pub mod s4par;
#[doc = "S4M0AR (rw) register accessor: an alias for `Reg<S4M0AR_SPEC>`"]
pub type S4M0AR = crate::Reg<s4m0ar::S4M0AR_SPEC>;
#[doc = "stream x memory 0 address register"]
pub mod s4m0ar;
#[doc = "S4M1AR (rw) register accessor: an alias for `Reg<S4M1AR_SPEC>`"]
pub type S4M1AR = crate::Reg<s4m1ar::S4M1AR_SPEC>;
#[doc = "stream x memory 1 address register"]
pub mod s4m1ar;
#[doc = "S4FCR (rw) register accessor: an alias for `Reg<S4FCR_SPEC>`"]
pub type S4FCR = crate::Reg<s4fcr::S4FCR_SPEC>;
#[doc = "stream x FIFO control register"]
pub mod s4fcr;
#[doc = "S5CR (rw) register accessor: an alias for `Reg<S5CR_SPEC>`"]
pub type S5CR = crate::Reg<s5cr::S5CR_SPEC>;
#[doc = "stream x configuration register"]
pub mod s5cr;
#[doc = "S5NDTR (rw) register accessor: an alias for `Reg<S5NDTR_SPEC>`"]
pub type S5NDTR = crate::Reg<s5ndtr::S5NDTR_SPEC>;
#[doc = "stream x number of data register"]
pub mod s5ndtr;
#[doc = "S5PAR (rw) register accessor: an alias for `Reg<S5PAR_SPEC>`"]
pub type S5PAR = crate::Reg<s5par::S5PAR_SPEC>;
#[doc = "stream x peripheral address register"]
pub mod s5par;
#[doc = "S5M0AR (rw) register accessor: an alias for `Reg<S5M0AR_SPEC>`"]
pub type S5M0AR = crate::Reg<s5m0ar::S5M0AR_SPEC>;
#[doc = "stream x memory 0 address register"]
pub mod s5m0ar;
#[doc = "S5M1AR (rw) register accessor: an alias for `Reg<S5M1AR_SPEC>`"]
pub type S5M1AR = crate::Reg<s5m1ar::S5M1AR_SPEC>;
#[doc = "stream x memory 1 address register"]
pub mod s5m1ar;
#[doc = "S5FCR (rw) register accessor: an alias for `Reg<S5FCR_SPEC>`"]
pub type S5FCR = crate::Reg<s5fcr::S5FCR_SPEC>;
#[doc = "stream x FIFO control register"]
pub mod s5fcr;
#[doc = "S6CR (rw) register accessor: an alias for `Reg<S6CR_SPEC>`"]
pub type S6CR = crate::Reg<s6cr::S6CR_SPEC>;
#[doc = "stream x configuration register"]
pub mod s6cr;
#[doc = "S6NDTR (rw) register accessor: an alias for `Reg<S6NDTR_SPEC>`"]
pub type S6NDTR = crate::Reg<s6ndtr::S6NDTR_SPEC>;
#[doc = "stream x number of data register"]
pub mod s6ndtr;
#[doc = "S6PAR (rw) register accessor: an alias for `Reg<S6PAR_SPEC>`"]
pub type S6PAR = crate::Reg<s6par::S6PAR_SPEC>;
#[doc = "stream x peripheral address register"]
pub mod s6par;
#[doc = "S6M0AR (rw) register accessor: an alias for `Reg<S6M0AR_SPEC>`"]
pub type S6M0AR = crate::Reg<s6m0ar::S6M0AR_SPEC>;
#[doc = "stream x memory 0 address register"]
pub mod s6m0ar;
#[doc = "S6M1AR (rw) register accessor: an alias for `Reg<S6M1AR_SPEC>`"]
pub type S6M1AR = crate::Reg<s6m1ar::S6M1AR_SPEC>;
#[doc = "stream x memory 1 address register"]
pub mod s6m1ar;
#[doc = "S6FCR (rw) register accessor: an alias for `Reg<S6FCR_SPEC>`"]
pub type S6FCR = crate::Reg<s6fcr::S6FCR_SPEC>;
#[doc = "stream x FIFO control register"]
pub mod s6fcr;
#[doc = "S7CR (rw) register accessor: an alias for `Reg<S7CR_SPEC>`"]
pub type S7CR = crate::Reg<s7cr::S7CR_SPEC>;
#[doc = "stream x configuration register"]
pub mod s7cr;
#[doc = "S7NDTR (rw) register accessor: an alias for `Reg<S7NDTR_SPEC>`"]
pub type S7NDTR = crate::Reg<s7ndtr::S7NDTR_SPEC>;
#[doc = "stream x number of data register"]
pub mod s7ndtr;
#[doc = "S7PAR (rw) register accessor: an alias for `Reg<S7PAR_SPEC>`"]
pub type S7PAR = crate::Reg<s7par::S7PAR_SPEC>;
#[doc = "stream x peripheral address register"]
pub mod s7par;
#[doc = "S7M0AR (rw) register accessor: an alias for `Reg<S7M0AR_SPEC>`"]
pub type S7M0AR = crate::Reg<s7m0ar::S7M0AR_SPEC>;
#[doc = "stream x memory 0 address register"]
pub mod s7m0ar;
#[doc = "S7M1AR (rw) register accessor: an alias for `Reg<S7M1AR_SPEC>`"]
pub type S7M1AR = crate::Reg<s7m1ar::S7M1AR_SPEC>;
#[doc = "stream x memory 1 address register"]
pub mod s7m1ar;
#[doc = "S7FCR (rw) register accessor: an alias for `Reg<S7FCR_SPEC>`"]
pub type S7FCR = crate::Reg<s7fcr::S7FCR_SPEC>;
#[doc = "stream x FIFO control register"]
pub mod s7fcr;
