#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSION` reader - Internal low-speed oscillator enable"]
pub type LSION_R = crate::BitReader<bool>;
#[doc = "Field `LSION` writer - Internal low-speed oscillator enable"]
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `LSIRDY` reader - Internal low-speed oscillator ready"]
pub type LSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader<bool>;
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `BORRSTF` reader - BOR reset flag"]
pub type BORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `BORRSTF` writer - BOR reset flag"]
pub type BORRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `PADRSTF` reader - PIN reset flag"]
pub type PADRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PADRSTF` writer - PIN reset flag"]
pub type PADRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub type PORRSTF_R = crate::BitReader<bool>;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub type PORRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SFTRSTF_R = crate::BitReader<bool>;
#[doc = "Field `SFTRSTF` writer - Software reset flag"]
pub type SFTRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `WDGRSTF` reader - Independent watchdog reset flag"]
pub type WDGRSTF_R = crate::BitReader<bool>;
#[doc = "Field `WDGRSTF` writer - Independent watchdog reset flag"]
pub type WDGRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WWDGRSTF_R = crate::BitReader<bool>;
#[doc = "Field `WWDGRSTF` writer - Window watchdog reset flag"]
pub type WWDGRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub type LPWRRSTF_R = crate::BitReader<bool>;
#[doc = "Field `LPWRRSTF` writer - Low-power reset flag"]
pub type LPWRRSTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low-speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn wdgrstf(&self) -> WDGRSTF_R {
        WDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<24> {
        RMVF_W::new(self)
    }
    #[doc = "Bit 25 - BOR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn borrstf(&mut self) -> BORRSTF_W<25> {
        BORRSTF_W::new(self)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn padrstf(&mut self) -> PADRSTF_W<26> {
        PADRSTF_W::new(self)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn porrstf(&mut self) -> PORRSTF_W<27> {
        PORRSTF_W::new(self)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<28> {
        SFTRSTF_W::new(self)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wdgrstf(&mut self) -> WDGRSTF_W<29> {
        WDGRSTF_W::new(self)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<30> {
        WWDGRSTF_W::new(self)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    #[must_use]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<31> {
        LPWRRSTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clock control & status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0x0e00_0000"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e00_0000;
}
