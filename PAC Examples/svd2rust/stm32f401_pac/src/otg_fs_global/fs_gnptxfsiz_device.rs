#[doc = "Register `FS_GNPTXFSIZ_Device` reader"]
pub struct R(crate::R<FS_GNPTXFSIZ_DEVICE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_GNPTXFSIZ_DEVICE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_GNPTXFSIZ_DEVICE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_GNPTXFSIZ_DEVICE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_GNPTXFSIZ_Device` writer"]
pub struct W(crate::W<FS_GNPTXFSIZ_DEVICE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_GNPTXFSIZ_DEVICE_SPEC>;
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
impl From<crate::W<FS_GNPTXFSIZ_DEVICE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_GNPTXFSIZ_DEVICE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FS_GNPTXFSIZ_DEVICE_SPEC, u16, u16, 16, O>;
#[doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"]
pub type TX0FD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"]
pub type TX0FD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FS_GNPTXFSIZ_DEVICE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<0> {
        TX0FSA_W::new(self)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fd(&mut self) -> TX0FD_W<16> {
        TX0FD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gnptxfsiz_device](index.html) module"]
pub struct FS_GNPTXFSIZ_DEVICE_SPEC;
impl crate::RegisterSpec for FS_GNPTXFSIZ_DEVICE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_gnptxfsiz_device::R](R) reader structure"]
impl crate::Readable for FS_GNPTXFSIZ_DEVICE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_gnptxfsiz_device::W](W) writer structure"]
impl crate::Writable for FS_GNPTXFSIZ_DEVICE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_GNPTXFSIZ_Device to value 0x0200"]
impl crate::Resettable for FS_GNPTXFSIZ_DEVICE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
