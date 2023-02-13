#[doc = "Register `PLLI2SCFGR` reader"]
pub struct R(crate::R<PLLI2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLI2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLI2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLI2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLI2SCFGR` writer"]
pub struct W(crate::W<PLLI2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLI2SCFGR_SPEC>;
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
impl From<crate::W<PLLI2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLI2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLI2SNx` reader - PLLI2S multiplication factor for VCO"]
pub type PLLI2SNX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PLLI2SNx` writer - PLLI2S multiplication factor for VCO"]
pub type PLLI2SNX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLI2SCFGR_SPEC, u16, u16, 9, O>;
#[doc = "Field `PLLI2SRx` reader - PLLI2S division factor for I2S clocks"]
pub type PLLI2SRX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLI2SRx` writer - PLLI2S division factor for I2S clocks"]
pub type PLLI2SRX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLI2SCFGR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2snx(&self) -> PLLI2SNX_R {
        PLLI2SNX_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2srx(&self) -> PLLI2SRX_R {
        PLLI2SRX_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plli2snx(&mut self) -> PLLI2SNX_W<6> {
        PLLI2SNX_W::new(self)
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    #[must_use]
    pub fn plli2srx(&mut self) -> PLLI2SRX_W<28> {
        PLLI2SRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLI2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plli2scfgr](index.html) module"]
pub struct PLLI2SCFGR_SPEC;
impl crate::RegisterSpec for PLLI2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plli2scfgr::R](R) reader structure"]
impl crate::Readable for PLLI2SCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plli2scfgr::W](W) writer structure"]
impl crate::Writable for PLLI2SCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLI2SCFGR to value 0x2000_3000"]
impl crate::Resettable for PLLI2SCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_3000;
}
