#[doc = "Register `HIFCR` writer"]
pub struct W(crate::W<HIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIFCR_SPEC>;
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
impl From<crate::W<HIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFEIF4` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CDMEIF4` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF4` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF4` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF4` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CFEIF5` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CDMEIF5` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF5` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF5` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF5` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CFEIF6` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CDMEIF6` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF6` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF6` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF6` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CFEIF7` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CDMEIF7` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF7` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF7` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF7` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HIFCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif4(&mut self) -> CFEIF4_W<0> {
        CFEIF4_W::new(self)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif4(&mut self) -> CDMEIF4_W<2> {
        CDMEIF4_W::new(self)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> CTEIF4_W<3> {
        CTEIF4_W::new(self)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> CHTIF4_W<4> {
        CHTIF4_W::new(self)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> CTCIF4_W<5> {
        CTCIF4_W::new(self)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif5(&mut self) -> CFEIF5_W<6> {
        CFEIF5_W::new(self)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif5(&mut self) -> CDMEIF5_W<8> {
        CDMEIF5_W::new(self)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> CTEIF5_W<9> {
        CTEIF5_W::new(self)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> CHTIF5_W<10> {
        CHTIF5_W::new(self)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> CTCIF5_W<11> {
        CTCIF5_W::new(self)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif6(&mut self) -> CFEIF6_W<16> {
        CFEIF6_W::new(self)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif6(&mut self) -> CDMEIF6_W<18> {
        CDMEIF6_W::new(self)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> CTEIF6_W<19> {
        CTEIF6_W::new(self)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif6(&mut self) -> CHTIF6_W<20> {
        CHTIF6_W::new(self)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif6(&mut self) -> CTCIF6_W<21> {
        CTCIF6_W::new(self)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif7(&mut self) -> CFEIF7_W<22> {
        CFEIF7_W::new(self)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif7(&mut self) -> CDMEIF7_W<24> {
        CDMEIF7_W::new(self)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn cteif7(&mut self) -> CTEIF7_W<25> {
        CTEIF7_W::new(self)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn chtif7(&mut self) -> CHTIF7_W<26> {
        CHTIF7_W::new(self)
    }
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif7(&mut self) -> CTCIF7_W<27> {
        CTCIF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "high interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hifcr](index.html) module"]
pub struct HIFCR_SPEC;
impl crate::RegisterSpec for HIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hifcr::W](W) writer structure"]
impl crate::Writable for HIFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HIFCR to value 0"]
impl crate::Resettable for HIFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
