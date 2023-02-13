#[doc = "Register `SSCGR` reader"]
pub struct R(crate::R<SSCGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSCGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSCGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSCGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSCGR` writer"]
pub struct W(crate::W<SSCGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSCGR_SPEC>;
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
impl From<crate::W<SSCGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSCGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODPER` reader - Modulation period"]
pub type MODPER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MODPER` writer - Modulation period"]
pub type MODPER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSCGR_SPEC, u16, u16, 13, O>;
#[doc = "Field `INCSTEP` reader - Incrementation step"]
pub type INCSTEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INCSTEP` writer - Incrementation step"]
pub type INCSTEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSCGR_SPEC, u16, u16, 15, O>;
#[doc = "Field `SPREADSEL` reader - Spread Select"]
pub type SPREADSEL_R = crate::BitReader<bool>;
#[doc = "Field `SPREADSEL` writer - Spread Select"]
pub type SPREADSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCGR_SPEC, bool, O>;
#[doc = "Field `SSCGEN` reader - Spread spectrum modulation enable"]
pub type SSCGEN_R = crate::BitReader<bool>;
#[doc = "Field `SSCGEN` writer - Spread spectrum modulation enable"]
pub type SSCGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SSCGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&self) -> MODPER_R {
        MODPER_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&self) -> INCSTEP_R {
        INCSTEP_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&self) -> SPREADSEL_R {
        SPREADSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&self) -> SSCGEN_R {
        SSCGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    #[must_use]
    pub fn modper(&mut self) -> MODPER_W<0> {
        MODPER_W::new(self)
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    #[must_use]
    pub fn incstep(&mut self) -> INCSTEP_W<13> {
        INCSTEP_W::new(self)
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    #[must_use]
    pub fn spreadsel(&mut self) -> SPREADSEL_W<30> {
        SPREADSEL_W::new(self)
    }
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    #[must_use]
    pub fn sscgen(&mut self) -> SSCGEN_W<31> {
        SSCGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spread spectrum clock generation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sscgr](index.html) module"]
pub struct SSCGR_SPEC;
impl crate::RegisterSpec for SSCGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sscgr::R](R) reader structure"]
impl crate::Readable for SSCGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sscgr::W](W) writer structure"]
impl crate::Writable for SSCGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSCGR to value 0"]
impl crate::Resettable for SSCGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
