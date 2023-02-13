#[doc = "Register `AFRH` reader"]
pub struct R(crate::R<AFRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFRH` writer"]
pub struct W(crate::W<AFRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRH_SPEC>;
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
impl From<crate::W<AFRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFRH8` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRH8` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRH9` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRH9` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRH10` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRH10` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRH11` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRH11` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRH12` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRH12` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRH13` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRH13` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRH14` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRH14` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRH15` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRH15` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh8(&self) -> AFRH8_R {
        AFRH8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh9(&self) -> AFRH9_R {
        AFRH9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh10(&self) -> AFRH10_R {
        AFRH10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh11(&self) -> AFRH11_R {
        AFRH11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh12(&self) -> AFRH12_R {
        AFRH12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh13(&self) -> AFRH13_R {
        AFRH13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh14(&self) -> AFRH14_R {
        AFRH14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh15(&self) -> AFRH15_R {
        AFRH15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh8(&mut self) -> AFRH8_W<0> {
        AFRH8_W::new(self)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh9(&mut self) -> AFRH9_W<4> {
        AFRH9_W::new(self)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh10(&mut self) -> AFRH10_W<8> {
        AFRH10_W::new(self)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh11(&mut self) -> AFRH11_W<12> {
        AFRH11_W::new(self)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh12(&mut self) -> AFRH12_W<16> {
        AFRH12_W::new(self)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh13(&mut self) -> AFRH13_W<20> {
        AFRH13_W::new(self)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh14(&mut self) -> AFRH14_W<24> {
        AFRH14_W::new(self)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    #[must_use]
    pub fn afrh15(&mut self) -> AFRH15_W<28> {
        AFRH15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function high register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrh](index.html) module"]
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afrh::R](R) reader structure"]
impl crate::Readable for AFRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afrh::W](W) writer structure"]
impl crate::Writable for AFRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AFRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
