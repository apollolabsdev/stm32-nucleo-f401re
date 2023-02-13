#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
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
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLM0` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM0_R = crate::BitReader<bool>;
#[doc = "Field `PLLM0` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLM1` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM1_R = crate::BitReader<bool>;
#[doc = "Field `PLLM1` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLM2` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM2_R = crate::BitReader<bool>;
#[doc = "Field `PLLM2` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLM3` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM3_R = crate::BitReader<bool>;
#[doc = "Field `PLLM3` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLM4` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM4_R = crate::BitReader<bool>;
#[doc = "Field `PLLM4` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLM5` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM5_R = crate::BitReader<bool>;
#[doc = "Field `PLLM5` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLM5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLN0` reader - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN0_R = crate::BitReader<bool>;
#[doc = "Field `PLLN0` writer - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLN1` reader - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN1_R = crate::BitReader<bool>;
#[doc = "Field `PLLN1` writer - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLN2` reader - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN2_R = crate::BitReader<bool>;
#[doc = "Field `PLLN2` writer - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLN3` reader - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN3_R = crate::BitReader<bool>;
#[doc = "Field `PLLN3` writer - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLN4` reader - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN4_R = crate::BitReader<bool>;
#[doc = "Field `PLLN4` writer - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLN5` reader - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN5_R = crate::BitReader<bool>;
#[doc = "Field `PLLN5` writer - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLN6` reader - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN6_R = crate::BitReader<bool>;
#[doc = "Field `PLLN6` writer - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLN7` reader - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN7_R = crate::BitReader<bool>;
#[doc = "Field `PLLN7` writer - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLN8` reader - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN8_R = crate::BitReader<bool>;
#[doc = "Field `PLLN8` writer - Main PLL (PLL) multiplication factor for VCO"]
pub type PLLN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLP0` reader - Main PLL (PLL) division factor for main system clock"]
pub type PLLP0_R = crate::BitReader<bool>;
#[doc = "Field `PLLP0` writer - Main PLL (PLL) division factor for main system clock"]
pub type PLLP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLP1` reader - Main PLL (PLL) division factor for main system clock"]
pub type PLLP1_R = crate::BitReader<bool>;
#[doc = "Field `PLLP1` writer - Main PLL (PLL) division factor for main system clock"]
pub type PLLP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLSRC` reader - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
pub type PLLSRC_R = crate::BitReader<bool>;
#[doc = "Field `PLLSRC` writer - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
pub type PLLSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLQ0` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub type PLLQ0_R = crate::BitReader<bool>;
#[doc = "Field `PLLQ0` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub type PLLQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLQ1` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub type PLLQ1_R = crate::BitReader<bool>;
#[doc = "Field `PLLQ1` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub type PLLQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLQ2` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub type PLLQ2_R = crate::BitReader<bool>;
#[doc = "Field `PLLQ2` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub type PLLQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
#[doc = "Field `PLLQ3` reader - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub type PLLQ3_R = crate::BitReader<bool>;
#[doc = "Field `PLLQ3` writer - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
pub type PLLQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm0(&self) -> PLLM0_R {
        PLLM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm1(&self) -> PLLM1_R {
        PLLM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm2(&self) -> PLLM2_R {
        PLLM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm3(&self) -> PLLM3_R {
        PLLM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm4(&self) -> PLLM4_R {
        PLLM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn pllm5(&self) -> PLLM5_R {
        PLLM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln0(&self) -> PLLN0_R {
        PLLN0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln1(&self) -> PLLN1_R {
        PLLN1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln2(&self) -> PLLN2_R {
        PLLN2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln3(&self) -> PLLN3_R {
        PLLN3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln4(&self) -> PLLN4_R {
        PLLN4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln5(&self) -> PLLN5_R {
        PLLN5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln6(&self) -> PLLN6_R {
        PLLN6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln7(&self) -> PLLN7_R {
        PLLN7_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    pub fn plln8(&self) -> PLLN8_R {
        PLLN8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp0(&self) -> PLLP0_R {
        PLLP0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    pub fn pllp1(&self) -> PLLP1_R {
        PLLP1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq0(&self) -> PLLQ0_R {
        PLLQ0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq1(&self) -> PLLQ1_R {
        PLLQ1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq2(&self) -> PLLQ2_R {
        PLLQ2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    pub fn pllq3(&self) -> PLLQ3_R {
        PLLQ3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm0(&mut self) -> PLLM0_W<0> {
        PLLM0_W::new(self)
    }
    #[doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm1(&mut self) -> PLLM1_W<1> {
        PLLM1_W::new(self)
    }
    #[doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm2(&mut self) -> PLLM2_W<2> {
        PLLM2_W::new(self)
    }
    #[doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm3(&mut self) -> PLLM3_W<3> {
        PLLM3_W::new(self)
    }
    #[doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm4(&mut self) -> PLLM4_W<4> {
        PLLM4_W::new(self)
    }
    #[doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllm5(&mut self) -> PLLM5_W<5> {
        PLLM5_W::new(self)
    }
    #[doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln0(&mut self) -> PLLN0_W<6> {
        PLLN0_W::new(self)
    }
    #[doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln1(&mut self) -> PLLN1_W<7> {
        PLLN1_W::new(self)
    }
    #[doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln2(&mut self) -> PLLN2_W<8> {
        PLLN2_W::new(self)
    }
    #[doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln3(&mut self) -> PLLN3_W<9> {
        PLLN3_W::new(self)
    }
    #[doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln4(&mut self) -> PLLN4_W<10> {
        PLLN4_W::new(self)
    }
    #[doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln5(&mut self) -> PLLN5_W<11> {
        PLLN5_W::new(self)
    }
    #[doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln6(&mut self) -> PLLN6_W<12> {
        PLLN6_W::new(self)
    }
    #[doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln7(&mut self) -> PLLN7_W<13> {
        PLLN7_W::new(self)
    }
    #[doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO"]
    #[inline(always)]
    #[must_use]
    pub fn plln8(&mut self) -> PLLN8_W<14> {
        PLLN8_W::new(self)
    }
    #[doc = "Bit 16 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllp0(&mut self) -> PLLP0_W<16> {
        PLLP0_W::new(self)
    }
    #[doc = "Bit 17 - Main PLL (PLL) division factor for main system clock"]
    #[inline(always)]
    #[must_use]
    pub fn pllp1(&mut self) -> PLLP1_W<17> {
        PLLP1_W::new(self)
    }
    #[doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<22> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    #[must_use]
    pub fn pllq0(&mut self) -> PLLQ0_W<24> {
        PLLQ0_W::new(self)
    }
    #[doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    #[must_use]
    pub fn pllq1(&mut self) -> PLLQ1_W<25> {
        PLLQ1_W::new(self)
    }
    #[doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    #[must_use]
    pub fn pllq2(&mut self) -> PLLQ2_W<26> {
        PLLQ2_W::new(self)
    }
    #[doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
    #[inline(always)]
    #[must_use]
    pub fn pllq3(&mut self) -> PLLQ3_W<27> {
        PLLQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x2400_3010"]
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2400_3010;
}
