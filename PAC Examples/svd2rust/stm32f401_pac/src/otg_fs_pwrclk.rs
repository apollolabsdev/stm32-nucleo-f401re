#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS power and clock gating control register"]
    pub fs_pcgcctl: FS_PCGCCTL,
}
#[doc = "FS_PCGCCTL (rw) register accessor: an alias for `Reg<FS_PCGCCTL_SPEC>`"]
pub type FS_PCGCCTL = crate::Reg<fs_pcgcctl::FS_PCGCCTL_SPEC>;
#[doc = "OTG_FS power and clock gating control register"]
pub mod fs_pcgcctl;
