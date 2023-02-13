#[doc = "Register `CIR` reader"]
pub struct R(crate::R<CIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIR` writer"]
pub struct W(crate::W<CIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIR_SPEC>;
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
impl From<crate::W<CIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSIRDYF` reader - LSI ready interrupt flag"]
pub type LSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYF` reader - LSE ready interrupt flag"]
pub type LSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYF` reader - HSI ready interrupt flag"]
pub type HSIRDYF_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYF` reader - HSE ready interrupt flag"]
pub type HSERDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYF` reader - Main PLL (PLL) ready interrupt flag"]
pub type PLLRDYF_R = crate::BitReader<bool>;
#[doc = "Field `PLLI2SRDYF` reader - PLLI2S ready interrupt flag"]
pub type PLLI2SRDYF_R = crate::BitReader<bool>;
#[doc = "Field `CSSF` reader - Clock security system interrupt flag"]
pub type CSSF_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYIE` reader - LSI ready interrupt enable"]
pub type LSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSIRDYIE` writer - LSI ready interrupt enable"]
pub type LSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSERDYIE` reader - LSE ready interrupt enable"]
pub type LSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `LSERDYIE` writer - LSE ready interrupt enable"]
pub type LSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSIRDYIE` reader - HSI ready interrupt enable"]
pub type HSIRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSIRDYIE` writer - HSI ready interrupt enable"]
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSERDYIE` reader - HSE ready interrupt enable"]
pub type HSERDYIE_R = crate::BitReader<bool>;
#[doc = "Field `HSERDYIE` writer - HSE ready interrupt enable"]
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `PLLRDYIE` reader - Main PLL (PLL) ready interrupt enable"]
pub type PLLRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLRDYIE` writer - Main PLL (PLL) ready interrupt enable"]
pub type PLLRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `PLLI2SRDYIE` reader - PLLI2S ready interrupt enable"]
pub type PLLI2SRDYIE_R = crate::BitReader<bool>;
#[doc = "Field `PLLI2SRDYIE` writer - PLLI2S ready interrupt enable"]
pub type PLLI2SRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub type LSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `PLLRDYC` writer - Main PLL(PLL) ready interrupt clear"]
pub type PLLRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `PLLI2SRDYC` writer - PLLI2S ready interrupt clear"]
pub type PLLI2SRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LSI ready interrupt flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LSIRDYF_R {
        LSIRDYF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LSERDYF_R {
        LSERDYF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready interrupt flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HSIRDYF_R {
        HSIRDYF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready interrupt flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HSERDYF_R {
        HSERDYF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Main PLL (PLL) ready interrupt flag"]
    #[inline(always)]
    pub fn pllrdyf(&self) -> PLLRDYF_R {
        PLLRDYF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLLI2S ready interrupt flag"]
    #[inline(always)]
    pub fn plli2srdyf(&self) -> PLLI2SRDYF_R {
        PLLI2SRDYF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock security system interrupt flag"]
    #[inline(always)]
    pub fn cssf(&self) -> CSSF_R {
        CSSF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Main PLL (PLL) ready interrupt enable"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PLLI2S ready interrupt enable"]
    #[inline(always)]
    pub fn plli2srdyie(&self) -> PLLI2SRDYIE_R {
        PLLI2SRDYIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - LSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<8> {
        LSIRDYIE_W::new(self)
    }
    #[doc = "Bit 9 - LSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<9> {
        LSERDYIE_W::new(self)
    }
    #[doc = "Bit 10 - HSI ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<10> {
        HSIRDYIE_W::new(self)
    }
    #[doc = "Bit 11 - HSE ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<11> {
        HSERDYIE_W::new(self)
    }
    #[doc = "Bit 12 - Main PLL (PLL) ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<12> {
        PLLRDYIE_W::new(self)
    }
    #[doc = "Bit 13 - PLLI2S ready interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn plli2srdyie(&mut self) -> PLLI2SRDYIE_W<13> {
        PLLI2SRDYIE_W::new(self)
    }
    #[doc = "Bit 16 - LSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lsirdyc(&mut self) -> LSIRDYC_W<16> {
        LSIRDYC_W::new(self)
    }
    #[doc = "Bit 17 - LSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<17> {
        LSERDYC_W::new(self)
    }
    #[doc = "Bit 18 - HSI ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<18> {
        HSIRDYC_W::new(self)
    }
    #[doc = "Bit 19 - HSE ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<19> {
        HSERDYC_W::new(self)
    }
    #[doc = "Bit 20 - Main PLL(PLL) ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<20> {
        PLLRDYC_W::new(self)
    }
    #[doc = "Bit 21 - PLLI2S ready interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn plli2srdyc(&mut self) -> PLLI2SRDYC_W<21> {
        PLLI2SRDYC_W::new(self)
    }
    #[doc = "Bit 23 - Clock security system interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cssc(&mut self) -> CSSC_W<23> {
        CSSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cir](index.html) module"]
pub struct CIR_SPEC;
impl crate::RegisterSpec for CIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cir::R](R) reader structure"]
impl crate::Readable for CIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cir::W](W) writer structure"]
impl crate::Writable for CIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIR to value 0"]
impl crate::Resettable for CIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
