#[doc = "Register `AHB1LPENR` reader"]
pub struct R(crate::R<AHB1LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB1LPENR` writer"]
pub struct W(crate::W<AHB1LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1LPENR_SPEC>;
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
impl From<crate::W<AHB1LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOALPEN` reader - IO port A clock enable during sleep mode"]
pub type GPIOALPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOALPEN` writer - IO port A clock enable during sleep mode"]
pub type GPIOALPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `GPIOBLPEN` reader - IO port B clock enable during Sleep mode"]
pub type GPIOBLPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode"]
pub type GPIOBLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `GPIOCLPEN` reader - IO port C clock enable during Sleep mode"]
pub type GPIOCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode"]
pub type GPIOCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `GPIODLPEN` reader - IO port D clock enable during Sleep mode"]
pub type GPIODLPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode"]
pub type GPIODLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `GPIOELPEN` reader - IO port E clock enable during Sleep mode"]
pub type GPIOELPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode"]
pub type GPIOELPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode"]
pub type GPIOHLPEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode"]
pub type GPIOHLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during Sleep mode"]
pub type CRCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during Sleep mode"]
pub type CRCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `FLITFLPEN` reader - Flash interface clock enable during Sleep mode"]
pub type FLITFLPEN_R = crate::BitReader<bool>;
#[doc = "Field `FLITFLPEN` writer - Flash interface clock enable during Sleep mode"]
pub type FLITFLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `SRAM1LPEN` reader - SRAM 1interface clock enable during Sleep mode"]
pub type SRAM1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SRAM1LPEN` writer - SRAM 1interface clock enable during Sleep mode"]
pub type SRAM1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode"]
pub type DMA1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode"]
pub type DMA1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
#[doc = "Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode"]
pub type DMA2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode"]
pub type DMA2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1LPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&self) -> FLITFLPEN_R {
        FLITFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> SRAM1LPEN_R {
        SRAM1LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> DMA1LPEN_R {
        DMA1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> DMA2LPEN_R {
        DMA2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<0> {
        GPIOALPEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<1> {
        GPIOBLPEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<2> {
        GPIOCLPEN_W::new(self)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<3> {
        GPIODLPEN_W::new(self)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<4> {
        GPIOELPEN_W::new(self)
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<7> {
        GPIOHLPEN_W::new(self)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn crclpen(&mut self) -> CRCLPEN_W<12> {
        CRCLPEN_W::new(self)
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn flitflpen(&mut self) -> FLITFLPEN_W<15> {
        FLITFLPEN_W::new(self)
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sram1lpen(&mut self) -> SRAM1LPEN_W<16> {
        SRAM1LPEN_W::new(self)
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma1lpen(&mut self) -> DMA1LPEN_W<21> {
        DMA1LPEN_W::new(self)
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dma2lpen(&mut self) -> DMA2LPEN_W<22> {
        DMA2LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB1 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb1lpenr](index.html) module"]
pub struct AHB1LPENR_SPEC;
impl crate::RegisterSpec for AHB1LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb1lpenr::R](R) reader structure"]
impl crate::Readable for AHB1LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb1lpenr::W](W) writer structure"]
impl crate::Writable for AHB1LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB1LPENR to value 0x7e67_91ff"]
impl crate::Resettable for AHB1LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x7e67_91ff;
}
