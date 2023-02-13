#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMDINDEX` reader - Command index"]
pub type CMDINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDINDEX` writer - Command index"]
pub type CMDINDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 6, O>;
#[doc = "Field `WAITRESP` reader - Wait for response bits"]
pub type WAITRESP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAITRESP` writer - Wait for response bits"]
pub type WAITRESP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 2, O>;
#[doc = "Field `WAITINT` reader - CPSM waits for interrupt request"]
pub type WAITINT_R = crate::BitReader<bool>;
#[doc = "Field `WAITINT` writer - CPSM waits for interrupt request"]
pub type WAITINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `WAITPEND` reader - CPSM Waits for ends of data transfer (CmdPend internal signal)."]
pub type WAITPEND_R = crate::BitReader<bool>;
#[doc = "Field `WAITPEND` writer - CPSM Waits for ends of data transfer (CmdPend internal signal)."]
pub type WAITPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CPSMEN` reader - Command path state machine (CPSM) Enable bit"]
pub type CPSMEN_R = crate::BitReader<bool>;
#[doc = "Field `CPSMEN` writer - Command path state machine (CPSM) Enable bit"]
pub type CPSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `SDIOSuspend` reader - SD I/O suspend command"]
pub type SDIOSUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `SDIOSuspend` writer - SD I/O suspend command"]
pub type SDIOSUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `ENCMDcompl` reader - Enable CMD completion"]
pub type ENCMDCOMPL_R = crate::BitReader<bool>;
#[doc = "Field `ENCMDcompl` writer - Enable CMD completion"]
pub type ENCMDCOMPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `nIEN` reader - not Interrupt Enable"]
pub type N_IEN_R = crate::BitReader<bool>;
#[doc = "Field `nIEN` writer - not Interrupt Enable"]
pub type N_IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CE_ATACMD` reader - CE-ATA command"]
pub type CE_ATACMD_R = crate::BitReader<bool>;
#[doc = "Field `CE_ATACMD` writer - CE-ATA command"]
pub type CE_ATACMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    pub fn cmdindex(&self) -> CMDINDEX_R {
        CMDINDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Wait for response bits"]
    #[inline(always)]
    pub fn waitresp(&self) -> WAITRESP_R {
        WAITRESP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - CPSM waits for interrupt request"]
    #[inline(always)]
    pub fn waitint(&self) -> WAITINT_R {
        WAITINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)."]
    #[inline(always)]
    pub fn waitpend(&self) -> WAITPEND_R {
        WAITPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    pub fn cpsmen(&self) -> CPSMEN_R {
        CPSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    pub fn sdiosuspend(&self) -> SDIOSUSPEND_R {
        SDIOSUSPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable CMD completion"]
    #[inline(always)]
    pub fn encmdcompl(&self) -> ENCMDCOMPL_R {
        ENCMDCOMPL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - not Interrupt Enable"]
    #[inline(always)]
    pub fn n_ien(&self) -> N_IEN_R {
        N_IEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CE-ATA command"]
    #[inline(always)]
    pub fn ce_atacmd(&self) -> CE_ATACMD_R {
        CE_ATACMD_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index"]
    #[inline(always)]
    #[must_use]
    pub fn cmdindex(&mut self) -> CMDINDEX_W<0> {
        CMDINDEX_W::new(self)
    }
    #[doc = "Bits 6:7 - Wait for response bits"]
    #[inline(always)]
    #[must_use]
    pub fn waitresp(&mut self) -> WAITRESP_W<6> {
        WAITRESP_W::new(self)
    }
    #[doc = "Bit 8 - CPSM waits for interrupt request"]
    #[inline(always)]
    #[must_use]
    pub fn waitint(&mut self) -> WAITINT_W<8> {
        WAITINT_W::new(self)
    }
    #[doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)."]
    #[inline(always)]
    #[must_use]
    pub fn waitpend(&mut self) -> WAITPEND_W<9> {
        WAITPEND_W::new(self)
    }
    #[doc = "Bit 10 - Command path state machine (CPSM) Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpsmen(&mut self) -> CPSMEN_W<10> {
        CPSMEN_W::new(self)
    }
    #[doc = "Bit 11 - SD I/O suspend command"]
    #[inline(always)]
    #[must_use]
    pub fn sdiosuspend(&mut self) -> SDIOSUSPEND_W<11> {
        SDIOSUSPEND_W::new(self)
    }
    #[doc = "Bit 12 - Enable CMD completion"]
    #[inline(always)]
    #[must_use]
    pub fn encmdcompl(&mut self) -> ENCMDCOMPL_W<12> {
        ENCMDCOMPL_W::new(self)
    }
    #[doc = "Bit 13 - not Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn n_ien(&mut self) -> N_IEN_W<13> {
        N_IEN_W::new(self)
    }
    #[doc = "Bit 14 - CE-ATA command"]
    #[inline(always)]
    #[must_use]
    pub fn ce_atacmd(&mut self) -> CE_ATACMD_W<14> {
        CE_ATACMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
