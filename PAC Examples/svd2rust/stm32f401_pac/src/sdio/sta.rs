#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CCRCFAIL` reader - Command response received (CRC check failed)"]
pub type CCRCFAIL_R = crate::BitReader<bool>;
#[doc = "Field `DCRCFAIL` reader - Data block sent/received (CRC check failed)"]
pub type DCRCFAIL_R = crate::BitReader<bool>;
#[doc = "Field `CTIMEOUT` reader - Command response timeout"]
pub type CTIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `DTIMEOUT` reader - Data timeout"]
pub type DTIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TXUNDERR` reader - Transmit FIFO underrun error"]
pub type TXUNDERR_R = crate::BitReader<bool>;
#[doc = "Field `RXOVERR` reader - Received FIFO overrun error"]
pub type RXOVERR_R = crate::BitReader<bool>;
#[doc = "Field `CMDREND` reader - Command response received (CRC check passed)"]
pub type CMDREND_R = crate::BitReader<bool>;
#[doc = "Field `CMDSENT` reader - Command sent (no response required)"]
pub type CMDSENT_R = crate::BitReader<bool>;
#[doc = "Field `DATAEND` reader - Data end (data counter, SDIDCOUNT, is zero)"]
pub type DATAEND_R = crate::BitReader<bool>;
#[doc = "Field `STBITERR` reader - Start bit not detected on all data signals in wide bus mode"]
pub type STBITERR_R = crate::BitReader<bool>;
#[doc = "Field `DBCKEND` reader - Data block sent/received (CRC check passed)"]
pub type DBCKEND_R = crate::BitReader<bool>;
#[doc = "Field `CMDACT` reader - Command transfer in progress"]
pub type CMDACT_R = crate::BitReader<bool>;
#[doc = "Field `TXACT` reader - Data transmit in progress"]
pub type TXACT_R = crate::BitReader<bool>;
#[doc = "Field `RXACT` reader - Data receive in progress"]
pub type RXACT_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOHE` reader - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
pub type TXFIFOHE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOHF` reader - Receive FIFO half full: there are at least 8 words in the FIFO"]
pub type RXFIFOHF_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOF` reader - Transmit FIFO full"]
pub type TXFIFOF_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOF` reader - Receive FIFO full"]
pub type RXFIFOF_R = crate::BitReader<bool>;
#[doc = "Field `TXFIFOE` reader - Transmit FIFO empty"]
pub type TXFIFOE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOE` reader - Receive FIFO empty"]
pub type RXFIFOE_R = crate::BitReader<bool>;
#[doc = "Field `TXDAVL` reader - Data available in transmit FIFO"]
pub type TXDAVL_R = crate::BitReader<bool>;
#[doc = "Field `RXDAVL` reader - Data available in receive FIFO"]
pub type RXDAVL_R = crate::BitReader<bool>;
#[doc = "Field `SDIOIT` reader - SDIO interrupt received"]
pub type SDIOIT_R = crate::BitReader<bool>;
#[doc = "Field `CEATAEND` reader - CE-ATA command completion signal received for CMD61"]
pub type CEATAEND_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Command response received (CRC check failed)"]
    #[inline(always)]
    pub fn ccrcfail(&self) -> CCRCFAIL_R {
        CCRCFAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data block sent/received (CRC check failed)"]
    #[inline(always)]
    pub fn dcrcfail(&self) -> DCRCFAIL_R {
        DCRCFAIL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command response timeout"]
    #[inline(always)]
    pub fn ctimeout(&self) -> CTIMEOUT_R {
        CTIMEOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dtimeout(&self) -> DTIMEOUT_R {
        DTIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error"]
    #[inline(always)]
    pub fn txunderr(&self) -> TXUNDERR_R {
        TXUNDERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error"]
    #[inline(always)]
    pub fn rxoverr(&self) -> RXOVERR_R {
        RXOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received (CRC check passed)"]
    #[inline(always)]
    pub fn cmdrend(&self) -> CMDREND_R {
        CMDREND_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent (no response required)"]
    #[inline(always)]
    pub fn cmdsent(&self) -> CMDSENT_R {
        CMDSENT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)"]
    #[inline(always)]
    pub fn dataend(&self) -> DATAEND_R {
        DATAEND_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode"]
    #[inline(always)]
    pub fn stbiterr(&self) -> STBITERR_R {
        STBITERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent/received (CRC check passed)"]
    #[inline(always)]
    pub fn dbckend(&self) -> DBCKEND_R {
        DBCKEND_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress"]
    #[inline(always)]
    pub fn cmdact(&self) -> CMDACT_R {
        CMDACT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress"]
    #[inline(always)]
    pub fn txact(&self) -> TXACT_R {
        TXACT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress"]
    #[inline(always)]
    pub fn rxact(&self) -> RXACT_R {
        RXACT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO"]
    #[inline(always)]
    pub fn txfifohe(&self) -> TXFIFOHE_R {
        TXFIFOHE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO"]
    #[inline(always)]
    pub fn rxfifohf(&self) -> RXFIFOHF_R {
        RXFIFOHF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full"]
    #[inline(always)]
    pub fn txfifof(&self) -> TXFIFOF_R {
        TXFIFOF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxfifof(&self) -> RXFIFOF_R {
        RXFIFOF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty"]
    #[inline(always)]
    pub fn txfifoe(&self) -> TXFIFOE_R {
        TXFIFOE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty"]
    #[inline(always)]
    pub fn rxfifoe(&self) -> RXFIFOE_R {
        RXFIFOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data available in transmit FIFO"]
    #[inline(always)]
    pub fn txdavl(&self) -> TXDAVL_R {
        TXDAVL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data available in receive FIFO"]
    #[inline(always)]
    pub fn rxdavl(&self) -> RXDAVL_R {
        RXDAVL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO interrupt received"]
    #[inline(always)]
    pub fn sdioit(&self) -> SDIOIT_R {
        SDIOIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received for CMD61"]
    #[inline(always)]
    pub fn ceataend(&self) -> CEATAEND_R {
        CEATAEND_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
