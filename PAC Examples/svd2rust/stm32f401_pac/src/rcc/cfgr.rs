#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW0` reader - System clock switch"]
pub type SW0_R = crate::BitReader<bool>;
#[doc = "Field `SW0` writer - System clock switch"]
pub type SW0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SW1` reader - System clock switch"]
pub type SW1_R = crate::BitReader<bool>;
#[doc = "Field `SW1` writer - System clock switch"]
pub type SW1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `SWS0` reader - System clock switch status"]
pub type SWS0_R = crate::BitReader<bool>;
#[doc = "Field `SWS1` reader - System clock switch status"]
pub type SWS1_R = crate::BitReader<bool>;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PPRE1` reader - APB Low speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPRE1` writer - APB Low speed prescaler (APB1)"]
pub type PPRE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `PPRE2` reader - APB high-speed prescaler (APB2)"]
pub type PPRE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPRE2` writer - APB high-speed prescaler (APB2)"]
pub type PPRE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RTCPRE` reader - HSE division factor for RTC clock"]
pub type RTCPRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCPRE` writer - HSE division factor for RTC clock"]
pub type RTCPRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 5, O>;
#[doc = "Field `MCO1` reader - Microcontroller clock output 1"]
pub type MCO1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO1` writer - Microcontroller clock output 1"]
pub type MCO1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `I2SSRC` reader - I2S clock selection"]
pub type I2SSRC_R = crate::BitReader<bool>;
#[doc = "Field `I2SSRC` writer - I2S clock selection"]
pub type I2SSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
#[doc = "Field `MCO1PRE` reader - MCO1 prescaler"]
pub type MCO1PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO1PRE` writer - MCO1 prescaler"]
pub type MCO1PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MCO2PRE` reader - MCO2 prescaler"]
pub type MCO2PRE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO2PRE` writer - MCO2 prescaler"]
pub type MCO2PRE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MCO2` reader - Microcontroller clock output 2"]
pub type MCO2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO2` writer - Microcontroller clock output 2"]
pub type MCO2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - System clock switch"]
    #[inline(always)]
    pub fn sw0(&self) -> SW0_R {
        SW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System clock switch"]
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System clock switch status"]
    #[inline(always)]
    pub fn sws0(&self) -> SWS0_R {
        SWS0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System clock switch status"]
    #[inline(always)]
    pub fn sws1(&self) -> SWS1_R {
        SWS1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw0(&mut self) -> SW0_W<0> {
        SW0_W::new(self)
    }
    #[doc = "Bit 1 - System clock switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw1(&mut self) -> SW1_W<1> {
        SW1_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<4> {
        HPRE_W::new(self)
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<10> {
        PPRE1_W::new(self)
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<13> {
        PPRE2_W::new(self)
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline(always)]
    #[must_use]
    pub fn rtcpre(&mut self) -> RTCPRE_W<16> {
        RTCPRE_W::new(self)
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline(always)]
    #[must_use]
    pub fn mco1(&mut self) -> MCO1_W<21> {
        MCO1_W::new(self)
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2ssrc(&mut self) -> I2SSRC_W<23> {
        I2SSRC_W::new(self)
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<24> {
        MCO1PRE_W::new(self)
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<27> {
        MCO2PRE_W::new(self)
    }
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline(always)]
    #[must_use]
    pub fn mco2(&mut self) -> MCO2_W<30> {
        MCO2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
