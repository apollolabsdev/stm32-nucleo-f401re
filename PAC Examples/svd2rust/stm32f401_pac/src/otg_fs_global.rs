#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"]
    pub fs_gotgctl: FS_GOTGCTL,
    #[doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"]
    pub fs_gotgint: FS_GOTGINT,
    #[doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
    pub fs_gahbcfg: FS_GAHBCFG,
    #[doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
    pub fs_gusbcfg: FS_GUSBCFG,
    #[doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"]
    pub fs_grstctl: FS_GRSTCTL,
    #[doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
    pub fs_gintsts: FS_GINTSTS,
    #[doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
    pub fs_gintmsk: FS_GINTMSK,
    _reserved_7_fs_grxstsr: [u8; 0x04],
    _reserved8: [u8; 0x04],
    #[doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
    pub fs_grxfsiz: FS_GRXFSIZ,
    _reserved_9_fs_gnptxfsiz: [u8; 0x04],
    #[doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
    pub fs_gnptxsts: FS_GNPTXSTS,
    _reserved11: [u8; 0x08],
    #[doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"]
    pub fs_gccfg: FS_GCCFG,
    #[doc = "0x3c - core ID register"]
    pub fs_cid: FS_CID,
    _reserved13: [u8; 0xc0],
    #[doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
    pub fs_hptxfsiz: FS_HPTXFSIZ,
    #[doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
    pub fs_dieptxf1: FS_DIEPTXF1,
    #[doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
    pub fs_dieptxf2: FS_DIEPTXF2,
    #[doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
    pub fs_dieptxf3: FS_DIEPTXF3,
}
impl RegisterBlock {
    #[doc = "0x1c - OTG_FS Receive status debug read(Host mode)"]
    #[inline(always)]
    pub const fn fs_grxstsr_host(&self) -> &FS_GRXSTSR_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x1c - OTG_FS Receive status debug read(Device mode)"]
    #[inline(always)]
    pub const fn fs_grxstsr_device(&self) -> &FS_GRXSTSR_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(28usize).cast() }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)"]
    #[inline(always)]
    pub const fn fs_gnptxfsiz_host(&self) -> &FS_GNPTXFSIZ_HOST {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
    #[doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)"]
    #[inline(always)]
    pub const fn fs_gnptxfsiz_device(&self) -> &FS_GNPTXFSIZ_DEVICE {
        unsafe { &*(self as *const Self).cast::<u8>().add(40usize).cast() }
    }
}
#[doc = "FS_GOTGCTL (rw) register accessor: an alias for `Reg<FS_GOTGCTL_SPEC>`"]
pub type FS_GOTGCTL = crate::Reg<fs_gotgctl::FS_GOTGCTL_SPEC>;
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"]
pub mod fs_gotgctl;
#[doc = "FS_GOTGINT (rw) register accessor: an alias for `Reg<FS_GOTGINT_SPEC>`"]
pub type FS_GOTGINT = crate::Reg<fs_gotgint::FS_GOTGINT_SPEC>;
#[doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"]
pub mod fs_gotgint;
#[doc = "FS_GAHBCFG (rw) register accessor: an alias for `Reg<FS_GAHBCFG_SPEC>`"]
pub type FS_GAHBCFG = crate::Reg<fs_gahbcfg::FS_GAHBCFG_SPEC>;
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"]
pub mod fs_gahbcfg;
#[doc = "FS_GUSBCFG (rw) register accessor: an alias for `Reg<FS_GUSBCFG_SPEC>`"]
pub type FS_GUSBCFG = crate::Reg<fs_gusbcfg::FS_GUSBCFG_SPEC>;
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"]
pub mod fs_gusbcfg;
#[doc = "FS_GRSTCTL (rw) register accessor: an alias for `Reg<FS_GRSTCTL_SPEC>`"]
pub type FS_GRSTCTL = crate::Reg<fs_grstctl::FS_GRSTCTL_SPEC>;
#[doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"]
pub mod fs_grstctl;
#[doc = "FS_GINTSTS (rw) register accessor: an alias for `Reg<FS_GINTSTS_SPEC>`"]
pub type FS_GINTSTS = crate::Reg<fs_gintsts::FS_GINTSTS_SPEC>;
#[doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"]
pub mod fs_gintsts;
#[doc = "FS_GINTMSK (rw) register accessor: an alias for `Reg<FS_GINTMSK_SPEC>`"]
pub type FS_GINTMSK = crate::Reg<fs_gintmsk::FS_GINTMSK_SPEC>;
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"]
pub mod fs_gintmsk;
#[doc = "FS_GRXSTSR_Device (r) register accessor: an alias for `Reg<FS_GRXSTSR_DEVICE_SPEC>`"]
pub type FS_GRXSTSR_DEVICE = crate::Reg<fs_grxstsr_device::FS_GRXSTSR_DEVICE_SPEC>;
#[doc = "OTG_FS Receive status debug read(Device mode)"]
pub mod fs_grxstsr_device;
#[doc = "FS_GRXSTSR_Host (r) register accessor: an alias for `Reg<FS_GRXSTSR_HOST_SPEC>`"]
pub type FS_GRXSTSR_HOST = crate::Reg<fs_grxstsr_host::FS_GRXSTSR_HOST_SPEC>;
#[doc = "OTG_FS Receive status debug read(Host mode)"]
pub mod fs_grxstsr_host;
#[doc = "FS_GRXFSIZ (rw) register accessor: an alias for `Reg<FS_GRXFSIZ_SPEC>`"]
pub type FS_GRXFSIZ = crate::Reg<fs_grxfsiz::FS_GRXFSIZ_SPEC>;
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"]
pub mod fs_grxfsiz;
#[doc = "FS_GNPTXFSIZ_Device (rw) register accessor: an alias for `Reg<FS_GNPTXFSIZ_DEVICE_SPEC>`"]
pub type FS_GNPTXFSIZ_DEVICE = crate::Reg<fs_gnptxfsiz_device::FS_GNPTXFSIZ_DEVICE_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)"]
pub mod fs_gnptxfsiz_device;
#[doc = "FS_GNPTXFSIZ_Host (rw) register accessor: an alias for `Reg<FS_GNPTXFSIZ_HOST_SPEC>`"]
pub type FS_GNPTXFSIZ_HOST = crate::Reg<fs_gnptxfsiz_host::FS_GNPTXFSIZ_HOST_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)"]
pub mod fs_gnptxfsiz_host;
#[doc = "FS_GNPTXSTS (r) register accessor: an alias for `Reg<FS_GNPTXSTS_SPEC>`"]
pub type FS_GNPTXSTS = crate::Reg<fs_gnptxsts::FS_GNPTXSTS_SPEC>;
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"]
pub mod fs_gnptxsts;
#[doc = "FS_GCCFG (rw) register accessor: an alias for `Reg<FS_GCCFG_SPEC>`"]
pub type FS_GCCFG = crate::Reg<fs_gccfg::FS_GCCFG_SPEC>;
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"]
pub mod fs_gccfg;
#[doc = "FS_CID (rw) register accessor: an alias for `Reg<FS_CID_SPEC>`"]
pub type FS_CID = crate::Reg<fs_cid::FS_CID_SPEC>;
#[doc = "core ID register"]
pub mod fs_cid;
#[doc = "FS_HPTXFSIZ (rw) register accessor: an alias for `Reg<FS_HPTXFSIZ_SPEC>`"]
pub type FS_HPTXFSIZ = crate::Reg<fs_hptxfsiz::FS_HPTXFSIZ_SPEC>;
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"]
pub mod fs_hptxfsiz;
#[doc = "FS_DIEPTXF1 (rw) register accessor: an alias for `Reg<FS_DIEPTXF1_SPEC>`"]
pub type FS_DIEPTXF1 = crate::Reg<fs_dieptxf1::FS_DIEPTXF1_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)"]
pub mod fs_dieptxf1;
#[doc = "FS_DIEPTXF2 (rw) register accessor: an alias for `Reg<FS_DIEPTXF2_SPEC>`"]
pub type FS_DIEPTXF2 = crate::Reg<fs_dieptxf2::FS_DIEPTXF2_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)"]
pub mod fs_dieptxf2;
#[doc = "FS_DIEPTXF3 (rw) register accessor: an alias for `Reg<FS_DIEPTXF3_SPEC>`"]
pub type FS_DIEPTXF3 = crate::Reg<fs_dieptxf3::FS_DIEPTXF3_SPEC>;
#[doc = "OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)"]
pub mod fs_dieptxf3;
