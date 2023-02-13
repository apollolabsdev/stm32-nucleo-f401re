#[doc = "Register `JDR2` reader"]
pub struct R(crate::R<JDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATA` reader - Injected data"]
pub type JDATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Injected data"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "injected data register x\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jdr2](index.html) module"]
pub struct JDR2_SPEC;
impl crate::RegisterSpec for JDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jdr2::R](R) reader structure"]
impl crate::Readable for JDR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JDR2 to value 0"]
impl crate::Resettable for JDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
