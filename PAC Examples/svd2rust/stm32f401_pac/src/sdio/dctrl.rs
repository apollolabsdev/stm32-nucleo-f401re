#[doc = "Register `DCTRL` reader"]
pub struct R(crate::R<DCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCTRL` writer"]
pub struct W(crate::W<DCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTRL_SPEC>;
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
impl From<crate::W<DCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTEN` reader - DTEN"]
pub type DTEN_R = crate::BitReader<bool>;
#[doc = "Field `DTEN` writer - DTEN"]
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
#[doc = "Field `DTDIR` reader - Data transfer direction selection"]
pub type DTDIR_R = crate::BitReader<bool>;
#[doc = "Field `DTDIR` writer - Data transfer direction selection"]
pub type DTDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
#[doc = "Field `DTMODE` reader - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
pub type DTMODE_R = crate::BitReader<bool>;
#[doc = "Field `DTMODE` writer - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
pub type DTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
#[doc = "Field `DMAEN` reader - DMA enable bit"]
pub type DMAEN_R = crate::BitReader<bool>;
#[doc = "Field `DMAEN` writer - DMA enable bit"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
#[doc = "Field `DBLOCKSIZE` reader - Data block size"]
pub type DBLOCKSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBLOCKSIZE` writer - Data block size"]
pub type DBLOCKSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RWSTART` reader - Read wait start"]
pub type RWSTART_R = crate::BitReader<bool>;
#[doc = "Field `RWSTART` writer - Read wait start"]
pub type RWSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
#[doc = "Field `RWSTOP` reader - Read wait stop"]
pub type RWSTOP_R = crate::BitReader<bool>;
#[doc = "Field `RWSTOP` writer - Read wait stop"]
pub type RWSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
#[doc = "Field `RWMOD` reader - Read wait mode"]
pub type RWMOD_R = crate::BitReader<bool>;
#[doc = "Field `RWMOD` writer - Read wait mode"]
pub type RWMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
#[doc = "Field `SDIOEN` reader - SD I/O enable functions"]
pub type SDIOEN_R = crate::BitReader<bool>;
#[doc = "Field `SDIOEN` writer - SD I/O enable functions"]
pub type SDIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction selection"]
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Read wait start"]
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read wait mode"]
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O enable functions"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<0> {
        DTEN_W::new(self)
    }
    #[doc = "Bit 1 - Data transfer direction selection"]
    #[inline(always)]
    #[must_use]
    pub fn dtdir(&mut self) -> DTDIR_W<1> {
        DTDIR_W::new(self)
    }
    #[doc = "Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer."]
    #[inline(always)]
    #[must_use]
    pub fn dtmode(&mut self) -> DTMODE_W<2> {
        DTMODE_W::new(self)
    }
    #[doc = "Bit 3 - DMA enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<3> {
        DMAEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Data block size"]
    #[inline(always)]
    #[must_use]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<4> {
        DBLOCKSIZE_W::new(self)
    }
    #[doc = "Bit 8 - Read wait start"]
    #[inline(always)]
    #[must_use]
    pub fn rwstart(&mut self) -> RWSTART_W<8> {
        RWSTART_W::new(self)
    }
    #[doc = "Bit 9 - Read wait stop"]
    #[inline(always)]
    #[must_use]
    pub fn rwstop(&mut self) -> RWSTOP_W<9> {
        RWSTOP_W::new(self)
    }
    #[doc = "Bit 10 - Read wait mode"]
    #[inline(always)]
    #[must_use]
    pub fn rwmod(&mut self) -> RWMOD_W<10> {
        RWMOD_W::new(self)
    }
    #[doc = "Bit 11 - SD I/O enable functions"]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<11> {
        SDIOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctrl](index.html) module"]
pub struct DCTRL_SPEC;
impl crate::RegisterSpec for DCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctrl::R](R) reader structure"]
impl crate::Readable for DCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctrl::W](W) writer structure"]
impl crate::Writable for DCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCTRL to value 0"]
impl crate::Resettable for DCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
