#[doc = "Register `OAR1` reader"]
pub struct R(crate::R<OAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OAR1` writer"]
pub struct W(crate::W<OAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OAR1_SPEC>;
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
impl From<crate::W<OAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADD0` reader - Interface address"]
pub type ADD0_R = crate::BitReader<bool>;
#[doc = "Field `ADD0` writer - Interface address"]
pub type ADD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, bool, O>;
#[doc = "Field `ADD7` reader - Interface address"]
pub type ADD7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD7` writer - Interface address"]
pub type ADD7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OAR1_SPEC, u8, u8, 7, O>;
#[doc = "Field `ADD10` reader - Interface address"]
pub type ADD10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD10` writer - Interface address"]
pub type ADD10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OAR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADDMODE` reader - Addressing mode (slave mode)"]
pub type ADDMODE_R = crate::BitReader<bool>;
#[doc = "Field `ADDMODE` writer - Addressing mode (slave mode)"]
pub type ADDMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn add0(&self) -> ADD0_R {
        ADD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add7(&self) -> ADD7_R {
        ADD7_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add0(&mut self) -> ADD0_W<0> {
        ADD0_W::new(self)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add7(&mut self) -> ADD7_W<1> {
        ADD7_W::new(self)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<8> {
        ADD10_W::new(self)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn addmode(&mut self) -> ADDMODE_W<15> {
        ADDMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Own address register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oar1](index.html) module"]
pub struct OAR1_SPEC;
impl crate::RegisterSpec for OAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oar1::R](R) reader structure"]
impl crate::Readable for OAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oar1::W](W) writer structure"]
impl crate::Writable for OAR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for OAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
