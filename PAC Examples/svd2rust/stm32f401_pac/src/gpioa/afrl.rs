#[doc = "Register `AFRL` reader"]
pub struct R(crate::R<AFRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AFRL` writer"]
pub struct W(crate::W<AFRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRL_SPEC>;
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
impl From<crate::W<AFRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFRL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRL2` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRL2` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRL4` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRL4` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRL5` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRL5` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRL6` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRL6` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `AFRL7` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AFRL7` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl0(&self) -> AFRL0_R {
        AFRL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl1(&self) -> AFRL1_R {
        AFRL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl2(&self) -> AFRL2_R {
        AFRL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&self) -> AFRL3_R {
        AFRL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl4(&self) -> AFRL4_R {
        AFRL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl5(&self) -> AFRL5_R {
        AFRL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl6(&self) -> AFRL6_R {
        AFRL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl7(&self) -> AFRL7_R {
        AFRL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afrl0(&mut self) -> AFRL0_W<0> {
        AFRL0_W::new(self)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afrl1(&mut self) -> AFRL1_W<4> {
        AFRL1_W::new(self)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afrl2(&mut self) -> AFRL2_W<8> {
        AFRL2_W::new(self)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afrl3(&mut self) -> AFRL3_W<12> {
        AFRL3_W::new(self)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afrl4(&mut self) -> AFRL4_W<16> {
        AFRL4_W::new(self)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afrl5(&mut self) -> AFRL5_W<20> {
        AFRL5_W::new(self)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afrl6(&mut self) -> AFRL6_W<24> {
        AFRL6_W::new(self)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    #[must_use]
    pub fn afrl7(&mut self) -> AFRL7_W<28> {
        AFRL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO alternate function low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrl](index.html) module"]
pub struct AFRL_SPEC;
impl crate::RegisterSpec for AFRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [afrl::R](R) reader structure"]
impl crate::Readable for AFRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [afrl::W](W) writer structure"]
impl crate::Writable for AFRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
