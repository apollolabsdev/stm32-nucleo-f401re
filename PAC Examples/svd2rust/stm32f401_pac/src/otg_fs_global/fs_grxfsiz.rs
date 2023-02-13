#[doc = "Register `FS_GRXFSIZ` reader"]
pub struct R(crate::R<FS_GRXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_GRXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_GRXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_GRXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_GRXFSIZ` writer"]
pub struct W(crate::W<FS_GRXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_GRXFSIZ_SPEC>;
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
impl From<crate::W<FS_GRXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_GRXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFD` reader - RxFIFO depth"]
pub type RXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXFD` writer - RxFIFO depth"]
pub type RXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FS_GRXFSIZ_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&self) -> RXFD_R {
        RXFD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn rxfd(&mut self) -> RXFD_W<0> {
        RXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_grxfsiz](index.html) module"]
pub struct FS_GRXFSIZ_SPEC;
impl crate::RegisterSpec for FS_GRXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_grxfsiz::R](R) reader structure"]
impl crate::Readable for FS_GRXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_grxfsiz::W](W) writer structure"]
impl crate::Writable for FS_GRXFSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_GRXFSIZ to value 0x0200"]
impl crate::Resettable for FS_GRXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
