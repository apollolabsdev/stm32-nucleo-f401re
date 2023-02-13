#[doc = "Register `S2FCR` reader"]
pub struct R(crate::R<S2FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S2FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S2FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S2FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S2FCR` writer"]
pub struct W(crate::W<S2FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S2FCR_SPEC>;
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
impl From<crate::W<S2FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S2FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTH` reader - FIFO threshold selection"]
pub type FTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTH` writer - FIFO threshold selection"]
pub type FTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, S2FCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DMDIS` reader - Direct mode disable"]
pub type DMDIS_R = crate::BitReader<bool>;
#[doc = "Field `DMDIS` writer - Direct mode disable"]
pub type DMDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, S2FCR_SPEC, bool, O>;
#[doc = "Field `FS` reader - FIFO status"]
pub type FS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEIE` reader - FIFO error interrupt enable"]
pub type FEIE_R = crate::BitReader<bool>;
#[doc = "Field `FEIE` writer - FIFO error interrupt enable"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, S2FCR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    pub fn dmdis(&self) -> DMDIS_R {
        DMDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<0> {
        FTH_W::new(self)
    }
    #[doc = "Bit 2 - Direct mode disable"]
    #[inline(always)]
    #[must_use]
    pub fn dmdis(&mut self) -> DMDIS_W<2> {
        DMDIS_W::new(self)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<7> {
        FEIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stream x FIFO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2fcr](index.html) module"]
pub struct S2FCR_SPEC;
impl crate::RegisterSpec for S2FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s2fcr::R](R) reader structure"]
impl crate::Readable for S2FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s2fcr::W](W) writer structure"]
impl crate::Writable for S2FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S2FCR to value 0x21"]
impl crate::Resettable for S2FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
