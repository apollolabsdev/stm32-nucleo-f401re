#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE` reader - Peripheral enable"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - Peripheral enable"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SMBUS` reader - SMBus mode"]
pub type SMBUS_R = crate::BitReader<bool>;
#[doc = "Field `SMBUS` writer - SMBus mode"]
pub type SMBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SMBTYPE` reader - SMBus type"]
pub type SMBTYPE_R = crate::BitReader<bool>;
#[doc = "Field `SMBTYPE` writer - SMBus type"]
pub type SMBTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ENARP` reader - ARP enable"]
pub type ENARP_R = crate::BitReader<bool>;
#[doc = "Field `ENARP` writer - ARP enable"]
pub type ENARP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ENPEC` reader - PEC enable"]
pub type ENPEC_R = crate::BitReader<bool>;
#[doc = "Field `ENPEC` writer - PEC enable"]
pub type ENPEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ENGC` reader - General call enable"]
pub type ENGC_R = crate::BitReader<bool>;
#[doc = "Field `ENGC` writer - General call enable"]
pub type ENGC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable (Slave mode)"]
pub type NOSTRETCH_R = crate::BitReader<bool>;
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable (Slave mode)"]
pub type NOSTRETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `START` reader - Start generation"]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Start generation"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Stop generation"]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Stop generation"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ACK` reader - Acknowledge enable"]
pub type ACK_R = crate::BitReader<bool>;
#[doc = "Field `ACK` writer - Acknowledge enable"]
pub type ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `POS` reader - Acknowledge/PEC Position (for data reception)"]
pub type POS_R = crate::BitReader<bool>;
#[doc = "Field `POS` writer - Acknowledge/PEC Position (for data reception)"]
pub type POS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `PEC` reader - Packet error checking"]
pub type PEC_R = crate::BitReader<bool>;
#[doc = "Field `PEC` writer - Packet error checking"]
pub type PEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `ALERT` reader - SMBus alert"]
pub type ALERT_R = crate::BitReader<bool>;
#[doc = "Field `ALERT` writer - SMBus alert"]
pub type ALERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
#[doc = "Field `SWRST` reader - Software reset"]
pub type SWRST_R = crate::BitReader<bool>;
#[doc = "Field `SWRST` writer - Software reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    pub fn smbus(&self) -> SMBUS_R {
        SMBUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    pub fn smbtype(&self) -> SMBTYPE_R {
        SMBTYPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    pub fn enarp(&self) -> ENARP_R {
        ENARP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn enpec(&self) -> ENPEC_R {
        ENPEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn engc(&self) -> ENGC_R {
        ENGC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<0> {
        PE_W::new(self)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    #[must_use]
    pub fn smbus(&mut self) -> SMBUS_W<1> {
        SMBUS_W::new(self)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    #[must_use]
    pub fn smbtype(&mut self) -> SMBTYPE_W<3> {
        SMBTYPE_W::new(self)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    #[must_use]
    pub fn enarp(&mut self) -> ENARP_W<4> {
        ENARP_W::new(self)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    #[must_use]
    pub fn enpec(&mut self) -> ENPEC_W<5> {
        ENPEC_W::new(self)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn engc(&mut self) -> ENGC_W<6> {
        ENGC_W::new(self)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<7> {
        NOSTRETCH_W::new(self)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<8> {
        START_W::new(self)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<9> {
        STOP_W::new(self)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<10> {
        ACK_W::new(self)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    #[must_use]
    pub fn pos(&mut self) -> POS_W<11> {
        POS_W::new(self)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    #[must_use]
    pub fn pec(&mut self) -> PEC_W<12> {
        PEC_W::new(self)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    #[must_use]
    pub fn alert(&mut self) -> ALERT_W<13> {
        ALERT_W::new(self)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<15> {
        SWRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
