#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS host configuration register (OTG_FS_HCFG)"]
    pub fs_hcfg: FS_HCFG,
    #[doc = "0x04 - OTG_FS Host frame interval register"]
    pub hfir: HFIR,
    #[doc = "0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
    pub fs_hfnum: FS_HFNUM,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
    pub fs_hptxsts: FS_HPTXSTS,
    #[doc = "0x14 - OTG_FS Host all channels interrupt register"]
    pub haint: HAINT,
    #[doc = "0x18 - OTG_FS host all channels interrupt mask register"]
    pub haintmsk: HAINTMSK,
    _reserved6: [u8; 0x24],
    #[doc = "0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)"]
    pub fs_hprt: FS_HPRT,
    _reserved7: [u8; 0xbc],
    #[doc = "0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
    pub fs_hcchar0: FS_HCCHAR0,
    _reserved8: [u8; 0x04],
    #[doc = "0x108 - OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
    pub fs_hcint0: FS_HCINT0,
    #[doc = "0x10c - OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
    pub fs_hcintmsk0: FS_HCINTMSK0,
    #[doc = "0x110 - OTG_FS host channel-0 transfer size register"]
    pub fs_hctsiz0: FS_HCTSIZ0,
    _reserved11: [u8; 0x0c],
    #[doc = "0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
    pub fs_hcchar1: FS_HCCHAR1,
    _reserved12: [u8; 0x04],
    #[doc = "0x128 - OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
    pub fs_hcint1: FS_HCINT1,
    #[doc = "0x12c - OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
    pub fs_hcintmsk1: FS_HCINTMSK1,
    #[doc = "0x130 - OTG_FS host channel-1 transfer size register"]
    pub fs_hctsiz1: FS_HCTSIZ1,
    _reserved15: [u8; 0x0c],
    #[doc = "0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
    pub fs_hcchar2: FS_HCCHAR2,
    _reserved16: [u8; 0x04],
    #[doc = "0x148 - OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
    pub fs_hcint2: FS_HCINT2,
    #[doc = "0x14c - OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
    pub fs_hcintmsk2: FS_HCINTMSK2,
    #[doc = "0x150 - OTG_FS host channel-2 transfer size register"]
    pub fs_hctsiz2: FS_HCTSIZ2,
    _reserved19: [u8; 0x0c],
    #[doc = "0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
    pub fs_hcchar3: FS_HCCHAR3,
    _reserved20: [u8; 0x04],
    #[doc = "0x168 - OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
    pub fs_hcint3: FS_HCINT3,
    #[doc = "0x16c - OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
    pub fs_hcintmsk3: FS_HCINTMSK3,
    #[doc = "0x170 - OTG_FS host channel-3 transfer size register"]
    pub fs_hctsiz3: FS_HCTSIZ3,
    _reserved23: [u8; 0x0c],
    #[doc = "0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
    pub fs_hcchar4: FS_HCCHAR4,
    _reserved24: [u8; 0x04],
    #[doc = "0x188 - OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
    pub fs_hcint4: FS_HCINT4,
    #[doc = "0x18c - OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
    pub fs_hcintmsk4: FS_HCINTMSK4,
    #[doc = "0x190 - OTG_FS host channel-x transfer size register"]
    pub fs_hctsiz4: FS_HCTSIZ4,
    _reserved27: [u8; 0x0c],
    #[doc = "0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
    pub fs_hcchar5: FS_HCCHAR5,
    _reserved28: [u8; 0x04],
    #[doc = "0x1a8 - OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
    pub fs_hcint5: FS_HCINT5,
    #[doc = "0x1ac - OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
    pub fs_hcintmsk5: FS_HCINTMSK5,
    #[doc = "0x1b0 - OTG_FS host channel-5 transfer size register"]
    pub fs_hctsiz5: FS_HCTSIZ5,
    _reserved31: [u8; 0x0c],
    #[doc = "0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
    pub fs_hcchar6: FS_HCCHAR6,
    _reserved32: [u8; 0x04],
    #[doc = "0x1c8 - OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
    pub fs_hcint6: FS_HCINT6,
    #[doc = "0x1cc - OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
    pub fs_hcintmsk6: FS_HCINTMSK6,
    #[doc = "0x1d0 - OTG_FS host channel-6 transfer size register"]
    pub fs_hctsiz6: FS_HCTSIZ6,
    _reserved35: [u8; 0x0c],
    #[doc = "0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
    pub fs_hcchar7: FS_HCCHAR7,
    _reserved36: [u8; 0x04],
    #[doc = "0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
    pub fs_hcint7: FS_HCINT7,
    #[doc = "0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
    pub fs_hcintmsk7: FS_HCINTMSK7,
    #[doc = "0x1f0 - OTG_FS host channel-7 transfer size register"]
    pub fs_hctsiz7: FS_HCTSIZ7,
}
#[doc = "FS_HCFG (rw) register accessor: an alias for `Reg<FS_HCFG_SPEC>`"]
pub type FS_HCFG = crate::Reg<fs_hcfg::FS_HCFG_SPEC>;
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)"]
pub mod fs_hcfg;
#[doc = "HFIR (rw) register accessor: an alias for `Reg<HFIR_SPEC>`"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "OTG_FS Host frame interval register"]
pub mod hfir;
#[doc = "FS_HFNUM (r) register accessor: an alias for `Reg<FS_HFNUM_SPEC>`"]
pub type FS_HFNUM = crate::Reg<fs_hfnum::FS_HFNUM_SPEC>;
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)"]
pub mod fs_hfnum;
#[doc = "FS_HPTXSTS (rw) register accessor: an alias for `Reg<FS_HPTXSTS_SPEC>`"]
pub type FS_HPTXSTS = crate::Reg<fs_hptxsts::FS_HPTXSTS_SPEC>;
#[doc = "OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)"]
pub mod fs_hptxsts;
#[doc = "HAINT (r) register accessor: an alias for `Reg<HAINT_SPEC>`"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "OTG_FS Host all channels interrupt register"]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: an alias for `Reg<HAINTMSK_SPEC>`"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "OTG_FS host all channels interrupt mask register"]
pub mod haintmsk;
#[doc = "FS_HPRT (rw) register accessor: an alias for `Reg<FS_HPRT_SPEC>`"]
pub type FS_HPRT = crate::Reg<fs_hprt::FS_HPRT_SPEC>;
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)"]
pub mod fs_hprt;
#[doc = "FS_HCCHAR0 (rw) register accessor: an alias for `Reg<FS_HCCHAR0_SPEC>`"]
pub type FS_HCCHAR0 = crate::Reg<fs_hcchar0::FS_HCCHAR0_SPEC>;
#[doc = "OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)"]
pub mod fs_hcchar0;
#[doc = "FS_HCCHAR1 (rw) register accessor: an alias for `Reg<FS_HCCHAR1_SPEC>`"]
pub type FS_HCCHAR1 = crate::Reg<fs_hcchar1::FS_HCCHAR1_SPEC>;
#[doc = "OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)"]
pub mod fs_hcchar1;
#[doc = "FS_HCCHAR2 (rw) register accessor: an alias for `Reg<FS_HCCHAR2_SPEC>`"]
pub type FS_HCCHAR2 = crate::Reg<fs_hcchar2::FS_HCCHAR2_SPEC>;
#[doc = "OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)"]
pub mod fs_hcchar2;
#[doc = "FS_HCCHAR3 (rw) register accessor: an alias for `Reg<FS_HCCHAR3_SPEC>`"]
pub type FS_HCCHAR3 = crate::Reg<fs_hcchar3::FS_HCCHAR3_SPEC>;
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)"]
pub mod fs_hcchar3;
#[doc = "FS_HCCHAR4 (rw) register accessor: an alias for `Reg<FS_HCCHAR4_SPEC>`"]
pub type FS_HCCHAR4 = crate::Reg<fs_hcchar4::FS_HCCHAR4_SPEC>;
#[doc = "OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)"]
pub mod fs_hcchar4;
#[doc = "FS_HCCHAR5 (rw) register accessor: an alias for `Reg<FS_HCCHAR5_SPEC>`"]
pub type FS_HCCHAR5 = crate::Reg<fs_hcchar5::FS_HCCHAR5_SPEC>;
#[doc = "OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)"]
pub mod fs_hcchar5;
#[doc = "FS_HCCHAR6 (rw) register accessor: an alias for `Reg<FS_HCCHAR6_SPEC>`"]
pub type FS_HCCHAR6 = crate::Reg<fs_hcchar6::FS_HCCHAR6_SPEC>;
#[doc = "OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)"]
pub mod fs_hcchar6;
#[doc = "FS_HCCHAR7 (rw) register accessor: an alias for `Reg<FS_HCCHAR7_SPEC>`"]
pub type FS_HCCHAR7 = crate::Reg<fs_hcchar7::FS_HCCHAR7_SPEC>;
#[doc = "OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)"]
pub mod fs_hcchar7;
#[doc = "FS_HCINT0 (rw) register accessor: an alias for `Reg<FS_HCINT0_SPEC>`"]
pub type FS_HCINT0 = crate::Reg<fs_hcint0::FS_HCINT0_SPEC>;
#[doc = "OTG_FS host channel-0 interrupt register (OTG_FS_HCINT0)"]
pub mod fs_hcint0;
#[doc = "FS_HCINT1 (rw) register accessor: an alias for `Reg<FS_HCINT1_SPEC>`"]
pub type FS_HCINT1 = crate::Reg<fs_hcint1::FS_HCINT1_SPEC>;
#[doc = "OTG_FS host channel-1 interrupt register (OTG_FS_HCINT1)"]
pub mod fs_hcint1;
#[doc = "FS_HCINT2 (rw) register accessor: an alias for `Reg<FS_HCINT2_SPEC>`"]
pub type FS_HCINT2 = crate::Reg<fs_hcint2::FS_HCINT2_SPEC>;
#[doc = "OTG_FS host channel-2 interrupt register (OTG_FS_HCINT2)"]
pub mod fs_hcint2;
#[doc = "FS_HCINT3 (rw) register accessor: an alias for `Reg<FS_HCINT3_SPEC>`"]
pub type FS_HCINT3 = crate::Reg<fs_hcint3::FS_HCINT3_SPEC>;
#[doc = "OTG_FS host channel-3 interrupt register (OTG_FS_HCINT3)"]
pub mod fs_hcint3;
#[doc = "FS_HCINT4 (rw) register accessor: an alias for `Reg<FS_HCINT4_SPEC>`"]
pub type FS_HCINT4 = crate::Reg<fs_hcint4::FS_HCINT4_SPEC>;
#[doc = "OTG_FS host channel-4 interrupt register (OTG_FS_HCINT4)"]
pub mod fs_hcint4;
#[doc = "FS_HCINT5 (rw) register accessor: an alias for `Reg<FS_HCINT5_SPEC>`"]
pub type FS_HCINT5 = crate::Reg<fs_hcint5::FS_HCINT5_SPEC>;
#[doc = "OTG_FS host channel-5 interrupt register (OTG_FS_HCINT5)"]
pub mod fs_hcint5;
#[doc = "FS_HCINT6 (rw) register accessor: an alias for `Reg<FS_HCINT6_SPEC>`"]
pub type FS_HCINT6 = crate::Reg<fs_hcint6::FS_HCINT6_SPEC>;
#[doc = "OTG_FS host channel-6 interrupt register (OTG_FS_HCINT6)"]
pub mod fs_hcint6;
#[doc = "FS_HCINT7 (rw) register accessor: an alias for `Reg<FS_HCINT7_SPEC>`"]
pub type FS_HCINT7 = crate::Reg<fs_hcint7::FS_HCINT7_SPEC>;
#[doc = "OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)"]
pub mod fs_hcint7;
#[doc = "FS_HCINTMSK0 (rw) register accessor: an alias for `Reg<FS_HCINTMSK0_SPEC>`"]
pub type FS_HCINTMSK0 = crate::Reg<fs_hcintmsk0::FS_HCINTMSK0_SPEC>;
#[doc = "OTG_FS host channel-0 mask register (OTG_FS_HCINTMSK0)"]
pub mod fs_hcintmsk0;
#[doc = "FS_HCINTMSK1 (rw) register accessor: an alias for `Reg<FS_HCINTMSK1_SPEC>`"]
pub type FS_HCINTMSK1 = crate::Reg<fs_hcintmsk1::FS_HCINTMSK1_SPEC>;
#[doc = "OTG_FS host channel-1 mask register (OTG_FS_HCINTMSK1)"]
pub mod fs_hcintmsk1;
#[doc = "FS_HCINTMSK2 (rw) register accessor: an alias for `Reg<FS_HCINTMSK2_SPEC>`"]
pub type FS_HCINTMSK2 = crate::Reg<fs_hcintmsk2::FS_HCINTMSK2_SPEC>;
#[doc = "OTG_FS host channel-2 mask register (OTG_FS_HCINTMSK2)"]
pub mod fs_hcintmsk2;
#[doc = "FS_HCINTMSK3 (rw) register accessor: an alias for `Reg<FS_HCINTMSK3_SPEC>`"]
pub type FS_HCINTMSK3 = crate::Reg<fs_hcintmsk3::FS_HCINTMSK3_SPEC>;
#[doc = "OTG_FS host channel-3 mask register (OTG_FS_HCINTMSK3)"]
pub mod fs_hcintmsk3;
#[doc = "FS_HCINTMSK4 (rw) register accessor: an alias for `Reg<FS_HCINTMSK4_SPEC>`"]
pub type FS_HCINTMSK4 = crate::Reg<fs_hcintmsk4::FS_HCINTMSK4_SPEC>;
#[doc = "OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)"]
pub mod fs_hcintmsk4;
#[doc = "FS_HCINTMSK5 (rw) register accessor: an alias for `Reg<FS_HCINTMSK5_SPEC>`"]
pub type FS_HCINTMSK5 = crate::Reg<fs_hcintmsk5::FS_HCINTMSK5_SPEC>;
#[doc = "OTG_FS host channel-5 mask register (OTG_FS_HCINTMSK5)"]
pub mod fs_hcintmsk5;
#[doc = "FS_HCINTMSK6 (rw) register accessor: an alias for `Reg<FS_HCINTMSK6_SPEC>`"]
pub type FS_HCINTMSK6 = crate::Reg<fs_hcintmsk6::FS_HCINTMSK6_SPEC>;
#[doc = "OTG_FS host channel-6 mask register (OTG_FS_HCINTMSK6)"]
pub mod fs_hcintmsk6;
#[doc = "FS_HCINTMSK7 (rw) register accessor: an alias for `Reg<FS_HCINTMSK7_SPEC>`"]
pub type FS_HCINTMSK7 = crate::Reg<fs_hcintmsk7::FS_HCINTMSK7_SPEC>;
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)"]
pub mod fs_hcintmsk7;
#[doc = "FS_HCTSIZ0 (rw) register accessor: an alias for `Reg<FS_HCTSIZ0_SPEC>`"]
pub type FS_HCTSIZ0 = crate::Reg<fs_hctsiz0::FS_HCTSIZ0_SPEC>;
#[doc = "OTG_FS host channel-0 transfer size register"]
pub mod fs_hctsiz0;
#[doc = "FS_HCTSIZ1 (rw) register accessor: an alias for `Reg<FS_HCTSIZ1_SPEC>`"]
pub type FS_HCTSIZ1 = crate::Reg<fs_hctsiz1::FS_HCTSIZ1_SPEC>;
#[doc = "OTG_FS host channel-1 transfer size register"]
pub mod fs_hctsiz1;
#[doc = "FS_HCTSIZ2 (rw) register accessor: an alias for `Reg<FS_HCTSIZ2_SPEC>`"]
pub type FS_HCTSIZ2 = crate::Reg<fs_hctsiz2::FS_HCTSIZ2_SPEC>;
#[doc = "OTG_FS host channel-2 transfer size register"]
pub mod fs_hctsiz2;
#[doc = "FS_HCTSIZ3 (rw) register accessor: an alias for `Reg<FS_HCTSIZ3_SPEC>`"]
pub type FS_HCTSIZ3 = crate::Reg<fs_hctsiz3::FS_HCTSIZ3_SPEC>;
#[doc = "OTG_FS host channel-3 transfer size register"]
pub mod fs_hctsiz3;
#[doc = "FS_HCTSIZ4 (rw) register accessor: an alias for `Reg<FS_HCTSIZ4_SPEC>`"]
pub type FS_HCTSIZ4 = crate::Reg<fs_hctsiz4::FS_HCTSIZ4_SPEC>;
#[doc = "OTG_FS host channel-x transfer size register"]
pub mod fs_hctsiz4;
#[doc = "FS_HCTSIZ5 (rw) register accessor: an alias for `Reg<FS_HCTSIZ5_SPEC>`"]
pub type FS_HCTSIZ5 = crate::Reg<fs_hctsiz5::FS_HCTSIZ5_SPEC>;
#[doc = "OTG_FS host channel-5 transfer size register"]
pub mod fs_hctsiz5;
#[doc = "FS_HCTSIZ6 (rw) register accessor: an alias for `Reg<FS_HCTSIZ6_SPEC>`"]
pub type FS_HCTSIZ6 = crate::Reg<fs_hctsiz6::FS_HCTSIZ6_SPEC>;
#[doc = "OTG_FS host channel-6 transfer size register"]
pub mod fs_hctsiz6;
#[doc = "FS_HCTSIZ7 (rw) register accessor: an alias for `Reg<FS_HCTSIZ7_SPEC>`"]
pub type FS_HCTSIZ7 = crate::Reg<fs_hctsiz7::FS_HCTSIZ7_SPEC>;
#[doc = "OTG_FS host channel-7 transfer size register"]
pub mod fs_hctsiz7;
