#[doc = "Register `SR1` reader"]
pub struct R(crate::R<SR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR1` writer"]
pub struct W(crate::W<SR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SB` reader - Start bit (Master mode)"]
pub type SB_R = crate::BitReader<bool>;
#[doc = "Field `ADDR` reader - Address sent (master mode)/matched (slave mode)"]
pub type ADDR_R = crate::BitReader<bool>;
#[doc = "Field `BTF` reader - Byte transfer finished"]
pub type BTF_R = crate::BitReader<bool>;
#[doc = "Field `ADD10` reader - 10-bit header sent (Master mode)"]
pub type ADD10_R = crate::BitReader<bool>;
#[doc = "Field `STOPF` reader - Stop detection (slave mode)"]
pub type STOPF_R = crate::BitReader<bool>;
#[doc = "Field `RxNE` reader - Data register not empty (receivers)"]
pub type RX_NE_R = crate::BitReader<bool>;
#[doc = "Field `TxE` reader - Data register empty (transmitters)"]
pub type TX_E_R = crate::BitReader<bool>;
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader<bool>;
#[doc = "Field `BERR` writer - Bus error"]
pub type BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `ARLO` reader - Arbitration lost (master mode)"]
pub type ARLO_R = crate::BitReader<bool>;
#[doc = "Field `ARLO` writer - Arbitration lost (master mode)"]
pub type ARLO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `AF` reader - Acknowledge failure"]
pub type AF_R = crate::BitReader<bool>;
#[doc = "Field `AF` writer - Acknowledge failure"]
pub type AF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `OVR` reader - Overrun/Underrun"]
pub type OVR_R = crate::BitReader<bool>;
#[doc = "Field `OVR` writer - Overrun/Underrun"]
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `PECERR` reader - PEC Error in reception"]
pub type PECERR_R = crate::BitReader<bool>;
#[doc = "Field `PECERR` writer - PEC Error in reception"]
pub type PECERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` reader - Timeout or Tlow error"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` writer - Timeout or Tlow error"]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
#[doc = "Field `SMBALERT` reader - SMBus alert"]
pub type SMBALERT_R = crate::BitReader<bool>;
#[doc = "Field `SMBALERT` writer - SMBus alert"]
pub type SMBALERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transfer finished"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 10-bit header sent (Master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Data register not empty (receivers)"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RX_NE_R {
        RX_NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data register empty (transmitters)"]
    #[inline(always)]
    pub fn tx_e(&self) -> TX_E_R {
        TX_E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<8> {
        BERR_W::new(self)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn arlo(&mut self) -> ARLO_W<9> {
        ARLO_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    #[must_use]
    pub fn af(&mut self) -> AF_W<10> {
        AF_W::new(self)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<11> {
        OVR_W::new(self)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    #[must_use]
    pub fn pecerr(&mut self) -> PECERR_W<12> {
        PECERR_W::new(self)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<14> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    #[must_use]
    pub fn smbalert(&mut self) -> SMBALERT_W<15> {
        SMBALERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr1](index.html) module"]
pub struct SR1_SPEC;
impl crate::RegisterSpec for SR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr1::R](R) reader structure"]
impl crate::Readable for SR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr1::W](W) writer structure"]
impl crate::Writable for SR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
