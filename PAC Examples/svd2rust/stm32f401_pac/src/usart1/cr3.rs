#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader<bool>;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IREN_R = crate::BitReader<bool>;
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IRLP_R = crate::BitReader<bool>;
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IRLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HDSEL_R = crate::BitReader<bool>;
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `NACK` reader - Smartcard NACK enable"]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `NACK` writer - Smartcard NACK enable"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type SCEN_R = crate::BitReader<bool>;
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type SCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DMAR_R = crate::BitReader<bool>;
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DMAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DMAT_R = crate::BitReader<bool>;
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RTSE_R = crate::BitReader<bool>;
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CTSE_R = crate::BitReader<bool>;
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader<bool>;
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `ONEBIT` reader - One sample bit method enable"]
pub type ONEBIT_R = crate::BitReader<bool>;
#[doc = "Field `ONEBIT` writer - One sample bit method enable"]
pub type ONEBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<0> {
        EIE_W::new(self)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<1> {
        IREN_W::new(self)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IRLP_W<2> {
        IRLP_W::new(self)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<3> {
        HDSEL_W::new(self)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<4> {
        NACK_W::new(self)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<5> {
        SCEN_W::new(self)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DMAR_W<6> {
        DMAR_W::new(self)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn dmat(&mut self) -> DMAT_W<7> {
        DMAT_W::new(self)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtse(&mut self) -> RTSE_W<8> {
        RTSE_W::new(self)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CTSE_W<9> {
        CTSE_W::new(self)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CTSIE_W<10> {
        CTSIE_W::new(self)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    #[must_use]
    pub fn onebit(&mut self) -> ONEBIT_W<11> {
        ONEBIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
