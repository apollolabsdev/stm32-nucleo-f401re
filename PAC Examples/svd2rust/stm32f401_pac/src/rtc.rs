#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - time register"]
    pub tr: TR,
    #[doc = "0x04 - date register"]
    pub dr: DR,
    #[doc = "0x08 - control register"]
    pub cr: CR,
    #[doc = "0x0c - initialization and status register"]
    pub isr: ISR,
    #[doc = "0x10 - prescaler register"]
    pub prer: PRER,
    #[doc = "0x14 - wakeup timer register"]
    pub wutr: WUTR,
    #[doc = "0x18 - calibration register"]
    pub calibr: CALIBR,
    #[doc = "0x1c - alarm A register"]
    pub alrmar: ALRMAR,
    #[doc = "0x20 - alarm B register"]
    pub alrmbr: ALRMBR,
    #[doc = "0x24 - write protection register"]
    pub wpr: WPR,
    #[doc = "0x28 - sub second register"]
    pub ssr: SSR,
    #[doc = "0x2c - shift control register"]
    pub shiftr: SHIFTR,
    #[doc = "0x30 - time stamp time register"]
    pub tstr: TSTR,
    #[doc = "0x34 - time stamp date register"]
    pub tsdr: TSDR,
    #[doc = "0x38 - timestamp sub second register"]
    pub tsssr: TSSSR,
    #[doc = "0x3c - calibration register"]
    pub calr: CALR,
    #[doc = "0x40 - tamper and alternate function configuration register"]
    pub tafcr: TAFCR,
    #[doc = "0x44 - alarm A sub second register"]
    pub alrmassr: ALRMASSR,
    #[doc = "0x48 - alarm B sub second register"]
    pub alrmbssr: ALRMBSSR,
    _reserved19: [u8; 0x04],
    #[doc = "0x50 - backup register"]
    pub bkp0r: BKP0R,
    #[doc = "0x54 - backup register"]
    pub bkp1r: BKP1R,
    #[doc = "0x58 - backup register"]
    pub bkp2r: BKP2R,
    #[doc = "0x5c - backup register"]
    pub bkp3r: BKP3R,
    #[doc = "0x60 - backup register"]
    pub bkp4r: BKP4R,
    #[doc = "0x64 - backup register"]
    pub bkp5r: BKP5R,
    #[doc = "0x68 - backup register"]
    pub bkp6r: BKP6R,
    #[doc = "0x6c - backup register"]
    pub bkp7r: BKP7R,
    #[doc = "0x70 - backup register"]
    pub bkp8r: BKP8R,
    #[doc = "0x74 - backup register"]
    pub bkp9r: BKP9R,
    #[doc = "0x78 - backup register"]
    pub bkp10r: BKP10R,
    #[doc = "0x7c - backup register"]
    pub bkp11r: BKP11R,
    #[doc = "0x80 - backup register"]
    pub bkp12r: BKP12R,
    #[doc = "0x84 - backup register"]
    pub bkp13r: BKP13R,
    #[doc = "0x88 - backup register"]
    pub bkp14r: BKP14R,
    #[doc = "0x8c - backup register"]
    pub bkp15r: BKP15R,
    #[doc = "0x90 - backup register"]
    pub bkp16r: BKP16R,
    #[doc = "0x94 - backup register"]
    pub bkp17r: BKP17R,
    #[doc = "0x98 - backup register"]
    pub bkp18r: BKP18R,
    #[doc = "0x9c - backup register"]
    pub bkp19r: BKP19R,
}
#[doc = "TR (rw) register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "time register"]
pub mod tr;
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "date register"]
pub mod dr;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "initialization and status register"]
pub mod isr;
#[doc = "PRER (rw) register accessor: an alias for `Reg<PRER_SPEC>`"]
pub type PRER = crate::Reg<prer::PRER_SPEC>;
#[doc = "prescaler register"]
pub mod prer;
#[doc = "WUTR (rw) register accessor: an alias for `Reg<WUTR_SPEC>`"]
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
#[doc = "wakeup timer register"]
pub mod wutr;
#[doc = "CALIBR (rw) register accessor: an alias for `Reg<CALIBR_SPEC>`"]
pub type CALIBR = crate::Reg<calibr::CALIBR_SPEC>;
#[doc = "calibration register"]
pub mod calibr;
#[doc = "ALRMAR (rw) register accessor: an alias for `Reg<ALRMAR_SPEC>`"]
pub type ALRMAR = crate::Reg<alrmar::ALRMAR_SPEC>;
#[doc = "alarm A register"]
pub mod alrmar;
#[doc = "ALRMBR (rw) register accessor: an alias for `Reg<ALRMBR_SPEC>`"]
pub type ALRMBR = crate::Reg<alrmbr::ALRMBR_SPEC>;
#[doc = "alarm B register"]
pub mod alrmbr;
#[doc = "WPR (w) register accessor: an alias for `Reg<WPR_SPEC>`"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "write protection register"]
pub mod wpr;
#[doc = "SSR (r) register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "sub second register"]
pub mod ssr;
#[doc = "SHIFTR (w) register accessor: an alias for `Reg<SHIFTR_SPEC>`"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
#[doc = "shift control register"]
pub mod shiftr;
#[doc = "TSTR (r) register accessor: an alias for `Reg<TSTR_SPEC>`"]
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
#[doc = "time stamp time register"]
pub mod tstr;
#[doc = "TSDR (r) register accessor: an alias for `Reg<TSDR_SPEC>`"]
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
#[doc = "time stamp date register"]
pub mod tsdr;
#[doc = "TSSSR (r) register accessor: an alias for `Reg<TSSSR_SPEC>`"]
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
#[doc = "timestamp sub second register"]
pub mod tsssr;
#[doc = "CALR (rw) register accessor: an alias for `Reg<CALR_SPEC>`"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "calibration register"]
pub mod calr;
#[doc = "TAFCR (rw) register accessor: an alias for `Reg<TAFCR_SPEC>`"]
pub type TAFCR = crate::Reg<tafcr::TAFCR_SPEC>;
#[doc = "tamper and alternate function configuration register"]
pub mod tafcr;
#[doc = "ALRMASSR (rw) register accessor: an alias for `Reg<ALRMASSR_SPEC>`"]
pub type ALRMASSR = crate::Reg<alrmassr::ALRMASSR_SPEC>;
#[doc = "alarm A sub second register"]
pub mod alrmassr;
#[doc = "ALRMBSSR (rw) register accessor: an alias for `Reg<ALRMBSSR_SPEC>`"]
pub type ALRMBSSR = crate::Reg<alrmbssr::ALRMBSSR_SPEC>;
#[doc = "alarm B sub second register"]
pub mod alrmbssr;
#[doc = "BKP0R (rw) register accessor: an alias for `Reg<BKP0R_SPEC>`"]
pub type BKP0R = crate::Reg<bkp0r::BKP0R_SPEC>;
#[doc = "backup register"]
pub mod bkp0r;
#[doc = "BKP1R (rw) register accessor: an alias for `Reg<BKP1R_SPEC>`"]
pub type BKP1R = crate::Reg<bkp1r::BKP1R_SPEC>;
#[doc = "backup register"]
pub mod bkp1r;
#[doc = "BKP2R (rw) register accessor: an alias for `Reg<BKP2R_SPEC>`"]
pub type BKP2R = crate::Reg<bkp2r::BKP2R_SPEC>;
#[doc = "backup register"]
pub mod bkp2r;
#[doc = "BKP3R (rw) register accessor: an alias for `Reg<BKP3R_SPEC>`"]
pub type BKP3R = crate::Reg<bkp3r::BKP3R_SPEC>;
#[doc = "backup register"]
pub mod bkp3r;
#[doc = "BKP4R (rw) register accessor: an alias for `Reg<BKP4R_SPEC>`"]
pub type BKP4R = crate::Reg<bkp4r::BKP4R_SPEC>;
#[doc = "backup register"]
pub mod bkp4r;
#[doc = "BKP5R (rw) register accessor: an alias for `Reg<BKP5R_SPEC>`"]
pub type BKP5R = crate::Reg<bkp5r::BKP5R_SPEC>;
#[doc = "backup register"]
pub mod bkp5r;
#[doc = "BKP6R (rw) register accessor: an alias for `Reg<BKP6R_SPEC>`"]
pub type BKP6R = crate::Reg<bkp6r::BKP6R_SPEC>;
#[doc = "backup register"]
pub mod bkp6r;
#[doc = "BKP7R (rw) register accessor: an alias for `Reg<BKP7R_SPEC>`"]
pub type BKP7R = crate::Reg<bkp7r::BKP7R_SPEC>;
#[doc = "backup register"]
pub mod bkp7r;
#[doc = "BKP8R (rw) register accessor: an alias for `Reg<BKP8R_SPEC>`"]
pub type BKP8R = crate::Reg<bkp8r::BKP8R_SPEC>;
#[doc = "backup register"]
pub mod bkp8r;
#[doc = "BKP9R (rw) register accessor: an alias for `Reg<BKP9R_SPEC>`"]
pub type BKP9R = crate::Reg<bkp9r::BKP9R_SPEC>;
#[doc = "backup register"]
pub mod bkp9r;
#[doc = "BKP10R (rw) register accessor: an alias for `Reg<BKP10R_SPEC>`"]
pub type BKP10R = crate::Reg<bkp10r::BKP10R_SPEC>;
#[doc = "backup register"]
pub mod bkp10r;
#[doc = "BKP11R (rw) register accessor: an alias for `Reg<BKP11R_SPEC>`"]
pub type BKP11R = crate::Reg<bkp11r::BKP11R_SPEC>;
#[doc = "backup register"]
pub mod bkp11r;
#[doc = "BKP12R (rw) register accessor: an alias for `Reg<BKP12R_SPEC>`"]
pub type BKP12R = crate::Reg<bkp12r::BKP12R_SPEC>;
#[doc = "backup register"]
pub mod bkp12r;
#[doc = "BKP13R (rw) register accessor: an alias for `Reg<BKP13R_SPEC>`"]
pub type BKP13R = crate::Reg<bkp13r::BKP13R_SPEC>;
#[doc = "backup register"]
pub mod bkp13r;
#[doc = "BKP14R (rw) register accessor: an alias for `Reg<BKP14R_SPEC>`"]
pub type BKP14R = crate::Reg<bkp14r::BKP14R_SPEC>;
#[doc = "backup register"]
pub mod bkp14r;
#[doc = "BKP15R (rw) register accessor: an alias for `Reg<BKP15R_SPEC>`"]
pub type BKP15R = crate::Reg<bkp15r::BKP15R_SPEC>;
#[doc = "backup register"]
pub mod bkp15r;
#[doc = "BKP16R (rw) register accessor: an alias for `Reg<BKP16R_SPEC>`"]
pub type BKP16R = crate::Reg<bkp16r::BKP16R_SPEC>;
#[doc = "backup register"]
pub mod bkp16r;
#[doc = "BKP17R (rw) register accessor: an alias for `Reg<BKP17R_SPEC>`"]
pub type BKP17R = crate::Reg<bkp17r::BKP17R_SPEC>;
#[doc = "backup register"]
pub mod bkp17r;
#[doc = "BKP18R (rw) register accessor: an alias for `Reg<BKP18R_SPEC>`"]
pub type BKP18R = crate::Reg<bkp18r::BKP18R_SPEC>;
#[doc = "backup register"]
pub mod bkp18r;
#[doc = "BKP19R (rw) register accessor: an alias for `Reg<BKP19R_SPEC>`"]
pub type BKP19R = crate::Reg<bkp19r::BKP19R_SPEC>;
#[doc = "backup register"]
pub mod bkp19r;
