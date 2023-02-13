#[doc = "Register `CALR` reader"]
pub struct R(crate::R<CALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALR` writer"]
pub struct W(crate::W<CALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALR_SPEC>;
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
impl From<crate::W<CALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALM` reader - Calibration minus"]
pub type CALM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CALM` writer - Calibration minus"]
pub type CALM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALR_SPEC, u16, u16, 9, O>;
#[doc = "Field `CALW16` reader - Use a 16-second calibration cycle period"]
pub type CALW16_R = crate::BitReader<bool>;
#[doc = "Field `CALW16` writer - Use a 16-second calibration cycle period"]
pub type CALW16_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, bool, O>;
#[doc = "Field `CALW8` reader - Use an 8-second calibration cycle period"]
pub type CALW8_R = crate::BitReader<bool>;
#[doc = "Field `CALW8` writer - Use an 8-second calibration cycle period"]
pub type CALW8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, bool, O>;
#[doc = "Field `CALP` reader - Increase frequency of RTC by 488.5 ppm"]
pub type CALP_R = crate::BitReader<bool>;
#[doc = "Field `CALP` writer - Increase frequency of RTC by 488.5 ppm"]
pub type CALP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CALR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8 - Calibration minus"]
    #[inline(always)]
    pub fn calm(&self) -> CALM_R {
        CALM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period"]
    #[inline(always)]
    pub fn calw16(&self) -> CALW16_R {
        CALW16_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period"]
    #[inline(always)]
    pub fn calw8(&self) -> CALW8_R {
        CALW8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm"]
    #[inline(always)]
    pub fn calp(&self) -> CALP_R {
        CALP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration minus"]
    #[inline(always)]
    #[must_use]
    pub fn calm(&mut self) -> CALM_W<0> {
        CALM_W::new(self)
    }
    #[doc = "Bit 13 - Use a 16-second calibration cycle period"]
    #[inline(always)]
    #[must_use]
    pub fn calw16(&mut self) -> CALW16_W<13> {
        CALW16_W::new(self)
    }
    #[doc = "Bit 14 - Use an 8-second calibration cycle period"]
    #[inline(always)]
    #[must_use]
    pub fn calw8(&mut self) -> CALW8_W<14> {
        CALW8_W::new(self)
    }
    #[doc = "Bit 15 - Increase frequency of RTC by 488.5 ppm"]
    #[inline(always)]
    #[must_use]
    pub fn calp(&mut self) -> CALP_W<15> {
        CALP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calr](index.html) module"]
pub struct CALR_SPEC;
impl crate::RegisterSpec for CALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calr::R](R) reader structure"]
impl crate::Readable for CALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calr::W](W) writer structure"]
impl crate::Writable for CALR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALR to value 0"]
impl crate::Resettable for CALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
