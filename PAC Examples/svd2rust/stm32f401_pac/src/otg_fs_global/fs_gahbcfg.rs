#[doc = "Register `FS_GAHBCFG` reader"]
pub struct R(crate::R<FS_GAHBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_GAHBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_GAHBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_GAHBCFG` writer"]
pub struct W(crate::W<FS_GAHBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_GAHBCFG_SPEC>;
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
impl From<crate::W<FS_GAHBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_GAHBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GINT` reader - Global interrupt mask"]
pub type GINT_R = crate::BitReader<bool>;
#[doc = "Field `GINT` writer - Global interrupt mask"]
pub type GINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_GAHBCFG_SPEC, bool, O>;
#[doc = "Field `TXFELVL` reader - TxFIFO empty level"]
pub type TXFELVL_R = crate::BitReader<bool>;
#[doc = "Field `TXFELVL` writer - TxFIFO empty level"]
pub type TXFELVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_GAHBCFG_SPEC, bool, O>;
#[doc = "Field `PTXFELVL` reader - Periodic TxFIFO empty level"]
pub type PTXFELVL_R = crate::BitReader<bool>;
#[doc = "Field `PTXFELVL` writer - Periodic TxFIFO empty level"]
pub type PTXFELVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_GAHBCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn gint(&mut self) -> GINT_W<0> {
        GINT_W::new(self)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn txfelvl(&mut self) -> TXFELVL_W<7> {
        TXFELVL_W::new(self)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<8> {
        PTXFELVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gahbcfg](index.html) module"]
pub struct FS_GAHBCFG_SPEC;
impl crate::RegisterSpec for FS_GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_gahbcfg::R](R) reader structure"]
impl crate::Readable for FS_GAHBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_gahbcfg::W](W) writer structure"]
impl crate::Writable for FS_GAHBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_GAHBCFG to value 0"]
impl crate::Resettable for FS_GAHBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
