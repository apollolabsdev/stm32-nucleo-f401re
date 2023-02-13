#[doc = "Register `FS_HCFG` reader"]
pub struct R(crate::R<FS_HCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_HCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_HCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_HCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_HCFG` writer"]
pub struct W(crate::W<FS_HCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_HCFG_SPEC>;
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
impl From<crate::W<FS_HCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_HCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSLSPCS` reader - FS/LS PHY clock select"]
pub type FSLSPCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSLSPCS` writer - FS/LS PHY clock select"]
pub type FSLSPCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FS_HCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `FSLSS` reader - FS- and LS-only support"]
pub type FSLSS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-only support"]
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    #[must_use]
    pub fn fslspcs(&mut self) -> FSLSPCS_W<0> {
        FSLSPCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hcfg](index.html) module"]
pub struct FS_HCFG_SPEC;
impl crate::RegisterSpec for FS_HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_hcfg::R](R) reader structure"]
impl crate::Readable for FS_HCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_hcfg::W](W) writer structure"]
impl crate::Writable for FS_HCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_HCFG to value 0"]
impl crate::Resettable for FS_HCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
