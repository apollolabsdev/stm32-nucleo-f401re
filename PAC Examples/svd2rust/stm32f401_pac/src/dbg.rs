#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IDCODE"]
    pub dbgmcu_idcode: DBGMCU_IDCODE,
    #[doc = "0x04 - Control Register"]
    pub dbgmcu_cr: DBGMCU_CR,
    #[doc = "0x08 - Debug MCU APB1 Freeze registe"]
    pub dbgmcu_apb1_fz: DBGMCU_APB1_FZ,
    #[doc = "0x0c - Debug MCU APB2 Freeze registe"]
    pub dbgmcu_apb2_fz: DBGMCU_APB2_FZ,
}
#[doc = "DBGMCU_IDCODE (r) register accessor: an alias for `Reg<DBGMCU_IDCODE_SPEC>`"]
pub type DBGMCU_IDCODE = crate::Reg<dbgmcu_idcode::DBGMCU_IDCODE_SPEC>;
#[doc = "IDCODE"]
pub mod dbgmcu_idcode;
#[doc = "DBGMCU_CR (rw) register accessor: an alias for `Reg<DBGMCU_CR_SPEC>`"]
pub type DBGMCU_CR = crate::Reg<dbgmcu_cr::DBGMCU_CR_SPEC>;
#[doc = "Control Register"]
pub mod dbgmcu_cr;
#[doc = "DBGMCU_APB1_FZ (rw) register accessor: an alias for `Reg<DBGMCU_APB1_FZ_SPEC>`"]
pub type DBGMCU_APB1_FZ = crate::Reg<dbgmcu_apb1_fz::DBGMCU_APB1_FZ_SPEC>;
#[doc = "Debug MCU APB1 Freeze registe"]
pub mod dbgmcu_apb1_fz;
#[doc = "DBGMCU_APB2_FZ (rw) register accessor: an alias for `Reg<DBGMCU_APB2_FZ_SPEC>`"]
pub type DBGMCU_APB2_FZ = crate::Reg<dbgmcu_apb2_fz::DBGMCU_APB2_FZ_SPEC>;
#[doc = "Debug MCU APB2 Freeze registe"]
pub mod dbgmcu_apb2_fz;
