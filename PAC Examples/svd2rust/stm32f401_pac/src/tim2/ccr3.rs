#[doc = "Register `CCR3` reader"]
pub struct R(crate::R<CCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR3` writer"]
pub struct W(crate::W<CCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR3_SPEC>;
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
impl From<crate::W<CCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR3_L` reader - Low Capture/Compare value"]
pub type CCR3_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR3_L` writer - Low Capture/Compare value"]
pub type CCR3_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR3_SPEC, u16, u16, 16, O>;
#[doc = "Field `CCR3_H` reader - High Capture/Compare value"]
pub type CCR3_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR3_H` writer - High Capture/Compare value"]
pub type CCR3_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3_l(&self) -> CCR3_L_R {
        CCR3_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value"]
    #[inline(always)]
    pub fn ccr3_h(&self) -> CCR3_H_R {
        CCR3_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3_l(&mut self) -> CCR3_L_W<0> {
        CCR3_L_W::new(self)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr3_h(&mut self) -> CCR3_H_W<16> {
        CCR3_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr3](index.html) module"]
pub struct CCR3_SPEC;
impl crate::RegisterSpec for CCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr3::R](R) reader structure"]
impl crate::Readable for CCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr3::W](W) writer structure"]
impl crate::Writable for CCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR3 to value 0"]
impl crate::Resettable for CCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
