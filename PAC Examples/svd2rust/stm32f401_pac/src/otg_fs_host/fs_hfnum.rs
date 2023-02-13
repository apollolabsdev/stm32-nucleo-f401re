#[doc = "Register `FS_HFNUM` reader"]
pub struct R(crate::R<FS_HFNUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_HFNUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_HFNUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_HFNUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRNUM` reader - Frame number"]
pub type FRNUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FTREM` reader - Frame time remaining"]
pub type FTREM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame time remaining"]
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hfnum](index.html) module"]
pub struct FS_HFNUM_SPEC;
impl crate::RegisterSpec for FS_HFNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_hfnum::R](R) reader structure"]
impl crate::Readable for FS_HFNUM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FS_HFNUM to value 0x3fff"]
impl crate::Resettable for FS_HFNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff;
}
