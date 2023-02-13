#[doc = "Register `CCMR2_Output` reader"]
pub struct R(crate::R<CCMR2_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR2_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR2_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR2_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR2_Output` writer"]
pub struct W(crate::W<CCMR2_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR2_OUTPUT_SPEC>;
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
impl From<crate::W<CCMR2_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR2_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC3S` reader - Capture/Compare 3 selection"]
pub type CC3S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC3S` writer - Capture/Compare 3 selection"]
pub type CC3S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_OUTPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `OC3FE` reader - Output compare 3 fast enable"]
pub type OC3FE_R = crate::BitReader<bool>;
#[doc = "Field `OC3FE` writer - Output compare 3 fast enable"]
pub type OC3FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC3PE` reader - Output compare 3 preload enable"]
pub type OC3PE_R = crate::BitReader<bool>;
#[doc = "Field `OC3PE` writer - Output compare 3 preload enable"]
pub type OC3PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC3M` reader - Output compare 3 mode"]
pub type OC3M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC3M` writer - Output compare 3 mode"]
pub type OC3M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_OUTPUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `OC3CE` reader - Output compare 3 clear enable"]
pub type OC3CE_R = crate::BitReader<bool>;
#[doc = "Field `OC3CE` writer - Output compare 3 clear enable"]
pub type OC3CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
#[doc = "Field `CC4S` reader - Capture/Compare 4 selection"]
pub type CC4S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CC4S` writer - Capture/Compare 4 selection"]
pub type CC4S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_OUTPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `OC4FE` reader - Output compare 4 fast enable"]
pub type OC4FE_R = crate::BitReader<bool>;
#[doc = "Field `OC4FE` writer - Output compare 4 fast enable"]
pub type OC4FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC4PE` reader - Output compare 4 preload enable"]
pub type OC4PE_R = crate::BitReader<bool>;
#[doc = "Field `OC4PE` writer - Output compare 4 preload enable"]
pub type OC4PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
#[doc = "Field `OC4M` reader - Output compare 4 mode"]
pub type OC4M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OC4M` writer - Output compare 4 mode"]
pub type OC4M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCMR2_OUTPUT_SPEC, u8, u8, 3, O>;
#[doc = "Field `OC4CE` reader - Output compare 4 clear enable"]
pub type OC4CE_R = crate::BitReader<bool>;
#[doc = "Field `OC4CE` writer - Output compare 4 clear enable"]
pub type OC4CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCMR2_OUTPUT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fe(&self) -> OC3FE_R {
        OC3FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pe(&self) -> OC3PE_R {
        OC3PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline(always)]
    pub fn oc3m(&self) -> OC3M_R {
        OC3M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3ce(&self) -> OC3CE_R {
        OC3CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fe(&self) -> OC4FE_R {
        OC4FE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pe(&self) -> OC4PE_R {
        OC4PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline(always)]
    pub fn oc4m(&self) -> OC4M_R {
        OC4M_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4ce(&self) -> OC4CE_R {
        OC4CE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 3 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc3s(&mut self) -> CC3S_W<0> {
        CC3S_W::new(self)
    }
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc3fe(&mut self) -> OC3FE_W<2> {
        OC3FE_W::new(self)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc3pe(&mut self) -> OC3PE_W<3> {
        OC3PE_W::new(self)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc3m(&mut self) -> OC3M_W<4> {
        OC3M_W::new(self)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc3ce(&mut self) -> OC3CE_W<7> {
        OC3CE_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/Compare 4 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc4s(&mut self) -> CC4S_W<8> {
        CC4S_W::new(self)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc4fe(&mut self) -> OC4FE_W<10> {
        OC4FE_W::new(self)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc4pe(&mut self) -> OC4PE_W<11> {
        OC4PE_W::new(self)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline(always)]
    #[must_use]
    pub fn oc4m(&mut self) -> OC4M_W<12> {
        OC4M_W::new(self)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    #[must_use]
    pub fn oc4ce(&mut self) -> OC4CE_W<15> {
        OC4CE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 2 (output mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr2_output](index.html) module"]
pub struct CCMR2_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR2_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr2_output::R](R) reader structure"]
impl crate::Readable for CCMR2_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr2_output::W](W) writer structure"]
impl crate::Writable for CCMR2_OUTPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCMR2_Output to value 0"]
impl crate::Resettable for CCMR2_OUTPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
