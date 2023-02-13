#[doc = "Register `DBGMCU_CR` reader"]
pub struct R(crate::R<DBGMCU_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGMCU_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGMCU_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGMCU_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGMCU_CR` writer"]
pub struct W(crate::W<DBGMCU_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGMCU_CR_SPEC>;
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
impl From<crate::W<DBGMCU_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGMCU_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBG_SLEEP` reader - DBG_SLEEP"]
pub type DBG_SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_SLEEP` writer - DBG_SLEEP"]
pub type DBG_SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGMCU_CR_SPEC, bool, O>;
#[doc = "Field `DBG_STOP` reader - DBG_STOP"]
pub type DBG_STOP_R = crate::BitReader<bool>;
#[doc = "Field `DBG_STOP` writer - DBG_STOP"]
pub type DBG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGMCU_CR_SPEC, bool, O>;
#[doc = "Field `DBG_STANDBY` reader - DBG_STANDBY"]
pub type DBG_STANDBY_R = crate::BitReader<bool>;
#[doc = "Field `DBG_STANDBY` writer - DBG_STANDBY"]
pub type DBG_STANDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGMCU_CR_SPEC, bool, O>;
#[doc = "Field `TRACE_IOEN` reader - TRACE_IOEN"]
pub type TRACE_IOEN_R = crate::BitReader<bool>;
#[doc = "Field `TRACE_IOEN` writer - TRACE_IOEN"]
pub type TRACE_IOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DBGMCU_CR_SPEC, bool, O>;
#[doc = "Field `TRACE_MODE` reader - TRACE_MODE"]
pub type TRACE_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRACE_MODE` writer - TRACE_MODE"]
pub type TRACE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBGMCU_CR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<0> {
        DBG_SLEEP_W::new(self)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<1> {
        DBG_STOP_W::new(self)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    #[must_use]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<2> {
        DBG_STANDBY_W::new(self)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    #[must_use]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<5> {
        TRACE_IOEN_W::new(self)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    #[must_use]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<6> {
        TRACE_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgmcu_cr](index.html) module"]
pub struct DBGMCU_CR_SPEC;
impl crate::RegisterSpec for DBGMCU_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgmcu_cr::R](R) reader structure"]
impl crate::Readable for DBGMCU_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgmcu_cr::W](W) writer structure"]
impl crate::Writable for DBGMCU_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGMCU_CR to value 0"]
impl crate::Resettable for DBGMCU_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
