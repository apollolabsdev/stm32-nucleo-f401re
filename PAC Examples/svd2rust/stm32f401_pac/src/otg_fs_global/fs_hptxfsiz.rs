#[doc = "Register `FS_HPTXFSIZ` reader"]
pub struct R(crate::R<FS_HPTXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_HPTXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_HPTXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_HPTXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_HPTXFSIZ` writer"]
pub struct W(crate::W<FS_HPTXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_HPTXFSIZ_SPEC>;
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
impl From<crate::W<FS_HPTXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_HPTXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTXSA` reader - Host periodic TxFIFO start address"]
pub type PTXSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTXSA` writer - Host periodic TxFIFO start address"]
pub type PTXSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FS_HPTXFSIZ_SPEC, u16, u16, 16, O>;
#[doc = "Field `PTXFSIZ` reader - Host periodic TxFIFO depth"]
pub type PTXFSIZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTXFSIZ` writer - Host periodic TxFIFO depth"]
pub type PTXFSIZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FS_HPTXFSIZ_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfsiz(&self) -> PTXFSIZ_R {
        PTXFSIZ_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    #[must_use]
    pub fn ptxsa(&mut self) -> PTXSA_W<0> {
        PTXSA_W::new(self)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfsiz(&mut self) -> PTXFSIZ_W<16> {
        PTXFSIZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_hptxfsiz](index.html) module"]
pub struct FS_HPTXFSIZ_SPEC;
impl crate::RegisterSpec for FS_HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_hptxfsiz::R](R) reader structure"]
impl crate::Readable for FS_HPTXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_hptxfsiz::W](W) writer structure"]
impl crate::Writable for FS_HPTXFSIZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_HPTXFSIZ to value 0x0200_0600"]
impl crate::Resettable for FS_HPTXFSIZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0600;
}
