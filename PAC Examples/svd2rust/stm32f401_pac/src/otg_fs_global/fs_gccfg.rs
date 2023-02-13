#[doc = "Register `FS_GCCFG` reader"]
pub struct R(crate::R<FS_GCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FS_GCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FS_GCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FS_GCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FS_GCCFG` writer"]
pub struct W(crate::W<FS_GCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FS_GCCFG_SPEC>;
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
impl From<crate::W<FS_GCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FS_GCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - Power down"]
pub type PWRDWN_R = crate::BitReader<bool>;
#[doc = "Field `PWRDWN` writer - Power down"]
pub type PWRDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_GCCFG_SPEC, bool, O>;
#[doc = "Field `VBUSASEN` reader - Enable the VBUS sensing device"]
pub type VBUSASEN_R = crate::BitReader<bool>;
#[doc = "Field `VBUSASEN` writer - Enable the VBUS sensing device"]
pub type VBUSASEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_GCCFG_SPEC, bool, O>;
#[doc = "Field `VBUSBSEN` reader - Enable the VBUS sensing device"]
pub type VBUSBSEN_R = crate::BitReader<bool>;
#[doc = "Field `VBUSBSEN` writer - Enable the VBUS sensing device"]
pub type VBUSBSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_GCCFG_SPEC, bool, O>;
#[doc = "Field `SOFOUTEN` reader - SOF output enable"]
pub type SOFOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `SOFOUTEN` writer - SOF output enable"]
pub type SOFOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FS_GCCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable the VBUS sensing device"]
    #[inline(always)]
    pub fn vbusasen(&self) -> VBUSASEN_R {
        VBUSASEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable the VBUS sensing device"]
    #[inline(always)]
    pub fn vbusbsen(&self) -> VBUSBSEN_R {
        VBUSBSEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    pub fn sofouten(&self) -> SOFOUTEN_R {
        SOFOUTEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pwrdwn(&mut self) -> PWRDWN_W<16> {
        PWRDWN_W::new(self)
    }
    #[doc = "Bit 18 - Enable the VBUS sensing device"]
    #[inline(always)]
    #[must_use]
    pub fn vbusasen(&mut self) -> VBUSASEN_W<18> {
        VBUSASEN_W::new(self)
    }
    #[doc = "Bit 19 - Enable the VBUS sensing device"]
    #[inline(always)]
    #[must_use]
    pub fn vbusbsen(&mut self) -> VBUSBSEN_W<19> {
        VBUSBSEN_W::new(self)
    }
    #[doc = "Bit 20 - SOF output enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofouten(&mut self) -> SOFOUTEN_W<20> {
        SOFOUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fs_gccfg](index.html) module"]
pub struct FS_GCCFG_SPEC;
impl crate::RegisterSpec for FS_GCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fs_gccfg::R](R) reader structure"]
impl crate::Readable for FS_GCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fs_gccfg::W](W) writer structure"]
impl crate::Writable for FS_GCCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FS_GCCFG to value 0"]
impl crate::Resettable for FS_GCCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
