#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRAWF` reader - Alarm A write flag"]
pub type ALRAWF_R = crate::BitReader<bool>;
#[doc = "Field `ALRBWF` reader - Alarm B write flag"]
pub type ALRBWF_R = crate::BitReader<bool>;
#[doc = "Field `WUTWF` reader - Wakeup timer write flag"]
pub type WUTWF_R = crate::BitReader<bool>;
#[doc = "Field `SHPF` reader - Shift operation pending"]
pub type SHPF_R = crate::BitReader<bool>;
#[doc = "Field `SHPF` writer - Shift operation pending"]
pub type SHPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `INITS` reader - Initialization status flag"]
pub type INITS_R = crate::BitReader<bool>;
#[doc = "Field `RSF` reader - Registers synchronization flag"]
pub type RSF_R = crate::BitReader<bool>;
#[doc = "Field `RSF` writer - Registers synchronization flag"]
pub type RSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `INITF` reader - Initialization flag"]
pub type INITF_R = crate::BitReader<bool>;
#[doc = "Field `INIT` reader - Initialization mode"]
pub type INIT_R = crate::BitReader<bool>;
#[doc = "Field `INIT` writer - Initialization mode"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `ALRAF` reader - Alarm A flag"]
pub type ALRAF_R = crate::BitReader<bool>;
#[doc = "Field `ALRAF` writer - Alarm A flag"]
pub type ALRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `ALRBF` reader - Alarm B flag"]
pub type ALRBF_R = crate::BitReader<bool>;
#[doc = "Field `ALRBF` writer - Alarm B flag"]
pub type ALRBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `WUTF` reader - Wakeup timer flag"]
pub type WUTF_R = crate::BitReader<bool>;
#[doc = "Field `WUTF` writer - Wakeup timer flag"]
pub type WUTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TSF` reader - Time-stamp flag"]
pub type TSF_R = crate::BitReader<bool>;
#[doc = "Field `TSF` writer - Time-stamp flag"]
pub type TSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TSOVF` reader - Time-stamp overflow flag"]
pub type TSOVF_R = crate::BitReader<bool>;
#[doc = "Field `TSOVF` writer - Time-stamp overflow flag"]
pub type TSOVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TAMP1F` reader - Tamper detection flag"]
pub type TAMP1F_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1F` writer - Tamper detection flag"]
pub type TAMP1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `TAMP2F` reader - TAMPER2 detection flag"]
pub type TAMP2F_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2F` writer - TAMPER2 detection flag"]
pub type TAMP2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
#[doc = "Field `RECALPF` reader - Recalibration pending Flag"]
pub type RECALPF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Alarm A write flag"]
    #[inline(always)]
    pub fn alrawf(&self) -> ALRAWF_R {
        ALRAWF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alarm B write flag"]
    #[inline(always)]
    pub fn alrbwf(&self) -> ALRBWF_R {
        ALRBWF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup timer write flag"]
    #[inline(always)]
    pub fn wutwf(&self) -> WUTWF_R {
        WUTWF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    pub fn shpf(&self) -> SHPF_R {
        SHPF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initialization status flag"]
    #[inline(always)]
    pub fn inits(&self) -> INITS_R {
        INITS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    pub fn rsf(&self) -> RSF_R {
        RSF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Initialization flag"]
    #[inline(always)]
    pub fn initf(&self) -> INITF_R {
        INITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    pub fn alraf(&self) -> ALRAF_R {
        ALRAF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    pub fn alrbf(&self) -> ALRBF_R {
        ALRBF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    pub fn wutf(&self) -> WUTF_R {
        WUTF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    pub fn tsovf(&self) -> TSOVF_R {
        TSOVF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    pub fn tamp1f(&self) -> TAMP1F_R {
        TAMP1F_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TAMPER2 detection flag"]
    #[inline(always)]
    pub fn tamp2f(&self) -> TAMP2F_R {
        TAMP2F_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Recalibration pending Flag"]
    #[inline(always)]
    pub fn recalpf(&self) -> RECALPF_R {
        RECALPF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shift operation pending"]
    #[inline(always)]
    #[must_use]
    pub fn shpf(&mut self) -> SHPF_W<3> {
        SHPF_W::new(self)
    }
    #[doc = "Bit 5 - Registers synchronization flag"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RSF_W<5> {
        RSF_W::new(self)
    }
    #[doc = "Bit 7 - Initialization mode"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<7> {
        INIT_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A flag"]
    #[inline(always)]
    #[must_use]
    pub fn alraf(&mut self) -> ALRAF_W<8> {
        ALRAF_W::new(self)
    }
    #[doc = "Bit 9 - Alarm B flag"]
    #[inline(always)]
    #[must_use]
    pub fn alrbf(&mut self) -> ALRBF_W<9> {
        ALRBF_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup timer flag"]
    #[inline(always)]
    #[must_use]
    pub fn wutf(&mut self) -> WUTF_W<10> {
        WUTF_W::new(self)
    }
    #[doc = "Bit 11 - Time-stamp flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<11> {
        TSF_W::new(self)
    }
    #[doc = "Bit 12 - Time-stamp overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsovf(&mut self) -> TSOVF_W<12> {
        TSOVF_W::new(self)
    }
    #[doc = "Bit 13 - Tamper detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1f(&mut self) -> TAMP1F_W<13> {
        TAMP1F_W::new(self)
    }
    #[doc = "Bit 14 - TAMPER2 detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2f(&mut self) -> TAMP2F_W<14> {
        TAMP2F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "initialization and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR to value 0x07"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
