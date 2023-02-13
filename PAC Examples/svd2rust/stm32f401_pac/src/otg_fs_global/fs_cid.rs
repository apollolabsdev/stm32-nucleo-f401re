#[doc = "Register `FS_CID` reader"]
pub struct R(crate::R<FS_CID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_CID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_CID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_CID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_CID` writer"]
pub struct W(crate::W<FS_CID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_CID_SPEC>;
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
impl From<crate::W<FS_CID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_CID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRODUCT_ID` reader - Product ID field"]
pub type PRODUCT_ID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRODUCT_ID` writer - Product ID field"]
pub type PRODUCT_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FS_CID_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    pub fn product_id(&self) -> PRODUCT_ID_R {
        PRODUCT_ID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Product ID field"]
    #[inline(always)]
    #[must_use]
    pub fn product_id(&mut self) -> PRODUCT_ID_W<0> {
        PRODUCT_ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core ID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_cid](index.html) module"]
pub struct FS_CID_SPEC;
impl crate::RegisterSpec for FS_CID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_cid::R](R) reader structure"]
impl crate::Readable for FS_CID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_cid::W](W) writer structure"]
impl crate::Writable for FS_CID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_CID to value 0x1000"]
impl crate::Resettable for FS_CID_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
