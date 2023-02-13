#[doc = "Register `CLKCR` reader"]
pub struct R(crate::R<CLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCR` writer"]
pub struct W(crate::W<CLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCR_SPEC>;
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
impl From<crate::W<CLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - Clock divide factor"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - Clock divide factor"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLKEN` reader - Clock enable bit"]
pub type CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN` writer - Clock enable bit"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `PWRSAV` reader - Power saving configuration bit"]
pub type PWRSAV_R = crate::BitReader<bool>;
#[doc = "Field `PWRSAV` writer - Power saving configuration bit"]
pub type PWRSAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `BYPASS` reader - Clock divider bypass enable bit"]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Clock divider bypass enable bit"]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `WIDBUS` reader - Wide bus mode enable bit"]
pub type WIDBUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WIDBUS` writer - Wide bus mode enable bit"]
pub type WIDBUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NEGEDGE` reader - SDIO_CK dephasing selection bit"]
pub type NEGEDGE_R = crate::BitReader<bool>;
#[doc = "Field `NEGEDGE` writer - SDIO_CK dephasing selection bit"]
pub type NEGEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
#[doc = "Field `HWFC_EN` reader - HW Flow Control enable"]
pub type HWFC_EN_R = crate::BitReader<bool>;
#[doc = "Field `HWFC_EN` writer - HW Flow Control enable"]
pub type HWFC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divide factor"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 8 - Clock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<8> {
        CLKEN_W::new(self)
    }
    #[doc = "Bit 9 - Power saving configuration bit"]
    #[inline(always)]
    #[must_use]
    pub fn pwrsav(&mut self) -> PWRSAV_W<9> {
        PWRSAV_W::new(self)
    }
    #[doc = "Bit 10 - Clock divider bypass enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<10> {
        BYPASS_W::new(self)
    }
    #[doc = "Bits 11:12 - Wide bus mode enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn widbus(&mut self) -> WIDBUS_W<11> {
        WIDBUS_W::new(self)
    }
    #[doc = "Bit 13 - SDIO_CK dephasing selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn negedge(&mut self) -> NEGEDGE_W<13> {
        NEGEDGE_W::new(self)
    }
    #[doc = "Bit 14 - HW Flow Control enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<14> {
        HWFC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDI clock control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcr](index.html) module"]
pub struct CLKCR_SPEC;
impl crate::RegisterSpec for CLKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkcr::R](R) reader structure"]
impl crate::Readable for CLKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcr::W](W) writer structure"]
impl crate::Writable for CLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKCR to value 0"]
impl crate::Resettable for CLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
