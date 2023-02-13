#[doc = "Register `ALRMBR` reader"]
pub struct R(crate::R<ALRMBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALRMBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALRMBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALRMBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALRMBR` writer"]
pub struct W(crate::W<ALRMBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRMBR_SPEC>;
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
impl From<crate::W<ALRMBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRMBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SU` reader - Second units in BCD format"]
pub type SU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SU` writer - Second units in BCD format"]
pub type SU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMBR_SPEC, u8, u8, 4, O>;
#[doc = "Field `ST` reader - Second tens in BCD format"]
pub type ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST` writer - Second tens in BCD format"]
pub type ST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMBR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MSK1` reader - Alarm B seconds mask"]
pub type MSK1_R = crate::BitReader<bool>;
#[doc = "Field `MSK1` writer - Alarm B seconds mask"]
pub type MSK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMBR_SPEC, bool, O>;
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MNU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MNU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMBR_SPEC, u8, u8, 4, O>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMBR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MSK2` reader - Alarm B minutes mask"]
pub type MSK2_R = crate::BitReader<bool>;
#[doc = "Field `MSK2` writer - Alarm B minutes mask"]
pub type MSK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMBR_SPEC, bool, O>;
#[doc = "Field `HU` reader - Hour units in BCD format"]
pub type HU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HU` writer - Hour units in BCD format"]
pub type HU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMBR_SPEC, u8, u8, 4, O>;
#[doc = "Field `HT` reader - Hour tens in BCD format"]
pub type HT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HT` writer - Hour tens in BCD format"]
pub type HT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMBR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PM_R = crate::BitReader<bool>;
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMBR_SPEC, bool, O>;
#[doc = "Field `MSK3` reader - Alarm B hours mask"]
pub type MSK3_R = crate::BitReader<bool>;
#[doc = "Field `MSK3` writer - Alarm B hours mask"]
pub type MSK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMBR_SPEC, bool, O>;
#[doc = "Field `DU` reader - Date units or day in BCD format"]
pub type DU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DU` writer - Date units or day in BCD format"]
pub type DU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMBR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALRMBR_SPEC, u8, u8, 2, O>;
#[doc = "Field `WDSEL` reader - Week day selection"]
pub type WDSEL_R = crate::BitReader<bool>;
#[doc = "Field `WDSEL` writer - Week day selection"]
pub type WDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMBR_SPEC, bool, O>;
#[doc = "Field `MSK4` reader - Alarm B date mask"]
pub type MSK4_R = crate::BitReader<bool>;
#[doc = "Field `MSK4` writer - Alarm B date mask"]
pub type MSK4_W<'a, const O: u8> = crate::BitWriter<'a, u32, ALRMBR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm B seconds mask"]
    #[inline(always)]
    pub fn msk1(&self) -> MSK1_R {
        MSK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm B minutes mask"]
    #[inline(always)]
    pub fn msk2(&self) -> MSK2_R {
        MSK2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Alarm B hours mask"]
    #[inline(always)]
    pub fn msk3(&self) -> MSK3_R {
        MSK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&self) -> WDSEL_R {
        WDSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm B date mask"]
    #[inline(always)]
    pub fn msk4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<0> {
        SU_W::new(self)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<4> {
        ST_W::new(self)
    }
    #[doc = "Bit 7 - Alarm B seconds mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk1(&mut self) -> MSK1_W<7> {
        MSK1_W::new(self)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MNU_W<8> {
        MNU_W::new(self)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MNT_W<12> {
        MNT_W::new(self)
    }
    #[doc = "Bit 15 - Alarm B minutes mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk2(&mut self) -> MSK2_W<15> {
        MSK2_W::new(self)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<16> {
        HU_W::new(self)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<20> {
        HT_W::new(self)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<22> {
        PM_W::new(self)
    }
    #[doc = "Bit 23 - Alarm B hours mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk3(&mut self) -> MSK3_W<23> {
        MSK3_W::new(self)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<24> {
        DU_W::new(self)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<28> {
        DT_W::new(self)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    #[must_use]
    pub fn wdsel(&mut self) -> WDSEL_W<30> {
        WDSEL_W::new(self)
    }
    #[doc = "Bit 31 - Alarm B date mask"]
    #[inline(always)]
    #[must_use]
    pub fn msk4(&mut self) -> MSK4_W<31> {
        MSK4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "alarm B register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrmbr](index.html) module"]
pub struct ALRMBR_SPEC;
impl crate::RegisterSpec for ALRMBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alrmbr::R](R) reader structure"]
impl crate::Readable for ALRMBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alrmbr::W](W) writer structure"]
impl crate::Writable for ALRMBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALRMBR to value 0"]
impl crate::Resettable for ALRMBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
