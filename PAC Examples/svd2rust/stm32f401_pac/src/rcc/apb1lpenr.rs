#[doc = "Register `APB1LPENR` reader"]
pub struct R(crate::R<APB1LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1LPENR` writer"]
pub struct W(crate::W<APB1LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1LPENR_SPEC>;
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
impl From<crate::W<APB1LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2LPEN` reader - TIM2 clock enable during Sleep mode"]
pub type TIM2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM2LPEN` writer - TIM2 clock enable during Sleep mode"]
pub type TIM2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `TIM3LPEN` reader - TIM3 clock enable during Sleep mode"]
pub type TIM3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM3LPEN` writer - TIM3 clock enable during Sleep mode"]
pub type TIM3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `TIM4LPEN` reader - TIM4 clock enable during Sleep mode"]
pub type TIM4LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM4LPEN` writer - TIM4 clock enable during Sleep mode"]
pub type TIM4LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `TIM5LPEN` reader - TIM5 clock enable during Sleep mode"]
pub type TIM5LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM5LPEN` writer - TIM5 clock enable during Sleep mode"]
pub type TIM5LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode"]
pub type WWDGLPEN_R = crate::BitReader<bool>;
#[doc = "Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode"]
pub type WWDGLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during Sleep mode"]
pub type SPI2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during Sleep mode"]
pub type SPI2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during Sleep mode"]
pub type SPI3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during Sleep mode"]
pub type SPI3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during Sleep mode"]
pub type USART2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during Sleep mode"]
pub type USART2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `I2C1LPEN` reader - I2C1 clock enable during Sleep mode"]
pub type I2C1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C1LPEN` writer - I2C1 clock enable during Sleep mode"]
pub type I2C1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `I2C2LPEN` reader - I2C2 clock enable during Sleep mode"]
pub type I2C2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C2LPEN` writer - I2C2 clock enable during Sleep mode"]
pub type I2C2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `I2C3LPEN` reader - I2C3 clock enable during Sleep mode"]
pub type I2C3LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C3LPEN` writer - I2C3 clock enable during Sleep mode"]
pub type I2C3LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
#[doc = "Field `PWRLPEN` reader - Power interface clock enable during Sleep mode"]
pub type PWRLPEN_R = crate::BitReader<bool>;
#[doc = "Field `PWRLPEN` writer - Power interface clock enable during Sleep mode"]
pub type PWRLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1LPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<0> {
        TIM2LPEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<1> {
        TIM3LPEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<2> {
        TIM4LPEN_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<3> {
        TIM5LPEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<11> {
        WWDGLPEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<14> {
        SPI2LPEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<15> {
        SPI3LPEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<17> {
        USART2LPEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<21> {
        I2C1LPEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<22> {
        I2C2LPEN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W<23> {
        I2C3LPEN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W<28> {
        PWRLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lpenr](index.html) module"]
pub struct APB1LPENR_SPEC;
impl crate::RegisterSpec for APB1LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1lpenr::R](R) reader structure"]
impl crate::Readable for APB1LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1lpenr::W](W) writer structure"]
impl crate::Writable for APB1LPENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB1LPENR to value 0x36fe_c9ff"]
impl crate::Resettable for APB1LPENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x36fe_c9ff;
}
