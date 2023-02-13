#[doc = "Register `SMCR` reader"]
pub struct R(crate::R<SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCR` writer"]
pub struct W(crate::W<SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCR_SPEC>;
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
impl From<crate::W<SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMS` reader - Slave mode selection"]
pub type SMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMS` writer - Slave mode selection"]
pub type SMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TS` reader - Trigger selection"]
pub type TS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TS` writer - Trigger selection"]
pub type TS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader<bool>;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
#[doc = "Field `ETF` reader - External trigger filter"]
pub type ETF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETF` writer - External trigger filter"]
pub type ETF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ETPS` reader - External trigger prescaler"]
pub type ETPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETPS` writer - External trigger prescaler"]
pub type ETPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `ECE` reader - External clock enable"]
pub type ECE_R = crate::BitReader<bool>;
#[doc = "Field `ECE` writer - External clock enable"]
pub type ECE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type ETP_R = crate::BitReader<bool>;
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type ETP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<0> {
        SMS_W::new(self)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<4> {
        TS_W::new(self)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<7> {
        MSM_W::new(self)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<8> {
        ETF_W::new(self)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<12> {
        ETPS_W::new(self)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<14> {
        ECE_W::new(self)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<15> {
        ETP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcr](index.html) module"]
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smcr::R](R) reader structure"]
impl crate::Readable for SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcr::W](W) writer structure"]
impl crate::Writable for SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
