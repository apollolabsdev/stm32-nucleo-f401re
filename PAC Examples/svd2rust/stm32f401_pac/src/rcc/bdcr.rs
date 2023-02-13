#[doc = "Register `BDCR` reader"]
pub struct R(crate::R<BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCR` writer"]
pub struct W(crate::W<BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCR_SPEC>;
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
impl From<crate::W<BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEON` reader - External low-speed oscillator enable"]
pub type LSEON_R = crate::BitReader<bool>;
#[doc = "Field `LSEON` writer - External low-speed oscillator enable"]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `LSERDY` reader - External low-speed oscillator ready"]
pub type LSERDY_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYP` reader - External low-speed oscillator bypass"]
pub type LSEBYP_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYP` writer - External low-speed oscillator bypass"]
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `RTCSEL0` reader - RTC clock source selection"]
pub type RTCSEL0_R = crate::BitReader<bool>;
#[doc = "Field `RTCSEL0` writer - RTC clock source selection"]
pub type RTCSEL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `RTCSEL1` reader - RTC clock source selection"]
pub type RTCSEL1_R = crate::BitReader<bool>;
#[doc = "Field `RTCSEL1` writer - RTC clock source selection"]
pub type RTCSEL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
#[doc = "Field `BDRST` reader - Backup domain software reset"]
pub type BDRST_R = crate::BitReader<bool>;
#[doc = "Field `BDRST` writer - Backup domain software reset"]
pub type BDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel0(&self) -> RTCSEL0_R {
        RTCSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel1(&self) -> RTCSEL1_R {
        RTCSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    #[doc = "Bit 8 - RTC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel0(&mut self) -> RTCSEL0_W<8> {
        RTCSEL0_W::new(self)
    }
    #[doc = "Bit 9 - RTC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel1(&mut self) -> RTCSEL1_W<9> {
        RTCSEL1_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BDRST_W<16> {
        BDRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup domain control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](index.html) module"]
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdcr::R](R) reader structure"]
impl crate::Readable for BDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdcr::W](W) writer structure"]
impl crate::Writable for BDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
