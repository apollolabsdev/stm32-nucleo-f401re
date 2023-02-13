#[doc = "Register `JOFR1` reader"]
pub struct R(crate::R<JOFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JOFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JOFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JOFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JOFR1` writer"]
pub struct W(crate::W<JOFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JOFR1_SPEC>;
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
impl From<crate::W<JOFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JOFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JOFFSET1` reader - Data offset for injected channel x"]
pub type JOFFSET1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `JOFFSET1` writer - Data offset for injected channel x"]
pub type JOFFSET1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JOFR1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    pub fn joffset1(&self) -> JOFFSET1_R {
        JOFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Data offset for injected channel x"]
    #[inline(always)]
    #[must_use]
    pub fn joffset1(&mut self) -> JOFFSET1_W<0> {
        JOFFSET1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected channel data offset register x\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jofr1](index.html) module"]
pub struct JOFR1_SPEC;
impl crate::RegisterSpec for JOFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jofr1::R](R) reader structure"]
impl crate::Readable for JOFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jofr1::W](W) writer structure"]
impl crate::Writable for JOFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JOFR1 to value 0"]
impl crate::Resettable for JOFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
