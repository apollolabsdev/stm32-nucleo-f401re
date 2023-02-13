#[doc = "Register `FS_DOEPMSK` reader"]
pub struct R(crate::R<FS_DOEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_DOEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_DOEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_DOEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_DOEPMSK` writer"]
pub struct W(crate::W<FS_DOEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_DOEPMSK_SPEC>;
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
impl From<crate::W<FS_DOEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_DOEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XFRCM_R = crate::BitReader<bool>;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DOEPMSK_SPEC, bool, O>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EPDM_R = crate::BitReader<bool>;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DOEPMSK_SPEC, bool, O>;
#[doc = "Field `STUPM` reader - SETUP phase done mask"]
pub type STUPM_R = crate::BitReader<bool>;
#[doc = "Field `STUPM` writer - SETUP phase done mask"]
pub type STUPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DOEPMSK_SPEC, bool, O>;
#[doc = "Field `OTEPDM` reader - OUT token received when endpoint disabled mask"]
pub type OTEPDM_R = crate::BitReader<bool>;
#[doc = "Field `OTEPDM` writer - OUT token received when endpoint disabled mask"]
pub type OTEPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_DOEPMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<0> {
        XFRCM_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<1> {
        EPDM_W::new(self)
    }
    #[doc = "Bit 3 - SETUP phase done mask"]
    #[inline(always)]
    #[must_use]
    pub fn stupm(&mut self) -> STUPM_W<3> {
        STUPM_W::new(self)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled mask"]
    #[inline(always)]
    #[must_use]
    pub fn otepdm(&mut self) -> OTEPDM_W<4> {
        OTEPDM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_doepmsk](index.html) module"]
pub struct FS_DOEPMSK_SPEC;
impl crate::RegisterSpec for FS_DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_doepmsk::R](R) reader structure"]
impl crate::Readable for FS_DOEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_doepmsk::W](W) writer structure"]
impl crate::Writable for FS_DOEPMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_DOEPMSK to value 0"]
impl crate::Resettable for FS_DOEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
