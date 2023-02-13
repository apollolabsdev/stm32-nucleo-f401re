#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MR0` reader - Interrupt Mask on line 0"]
pub type MR0_R = crate::BitReader<bool>;
#[doc = "Field `MR0` writer - Interrupt Mask on line 0"]
pub type MR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR1` reader - Interrupt Mask on line 1"]
pub type MR1_R = crate::BitReader<bool>;
#[doc = "Field `MR1` writer - Interrupt Mask on line 1"]
pub type MR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR2` reader - Interrupt Mask on line 2"]
pub type MR2_R = crate::BitReader<bool>;
#[doc = "Field `MR2` writer - Interrupt Mask on line 2"]
pub type MR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR3` reader - Interrupt Mask on line 3"]
pub type MR3_R = crate::BitReader<bool>;
#[doc = "Field `MR3` writer - Interrupt Mask on line 3"]
pub type MR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR4` reader - Interrupt Mask on line 4"]
pub type MR4_R = crate::BitReader<bool>;
#[doc = "Field `MR4` writer - Interrupt Mask on line 4"]
pub type MR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR5` reader - Interrupt Mask on line 5"]
pub type MR5_R = crate::BitReader<bool>;
#[doc = "Field `MR5` writer - Interrupt Mask on line 5"]
pub type MR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR6` reader - Interrupt Mask on line 6"]
pub type MR6_R = crate::BitReader<bool>;
#[doc = "Field `MR6` writer - Interrupt Mask on line 6"]
pub type MR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR7` reader - Interrupt Mask on line 7"]
pub type MR7_R = crate::BitReader<bool>;
#[doc = "Field `MR7` writer - Interrupt Mask on line 7"]
pub type MR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR8` reader - Interrupt Mask on line 8"]
pub type MR8_R = crate::BitReader<bool>;
#[doc = "Field `MR8` writer - Interrupt Mask on line 8"]
pub type MR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR9` reader - Interrupt Mask on line 9"]
pub type MR9_R = crate::BitReader<bool>;
#[doc = "Field `MR9` writer - Interrupt Mask on line 9"]
pub type MR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR10` reader - Interrupt Mask on line 10"]
pub type MR10_R = crate::BitReader<bool>;
#[doc = "Field `MR10` writer - Interrupt Mask on line 10"]
pub type MR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR11` reader - Interrupt Mask on line 11"]
pub type MR11_R = crate::BitReader<bool>;
#[doc = "Field `MR11` writer - Interrupt Mask on line 11"]
pub type MR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR12` reader - Interrupt Mask on line 12"]
pub type MR12_R = crate::BitReader<bool>;
#[doc = "Field `MR12` writer - Interrupt Mask on line 12"]
pub type MR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR13` reader - Interrupt Mask on line 13"]
pub type MR13_R = crate::BitReader<bool>;
#[doc = "Field `MR13` writer - Interrupt Mask on line 13"]
pub type MR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR14` reader - Interrupt Mask on line 14"]
pub type MR14_R = crate::BitReader<bool>;
#[doc = "Field `MR14` writer - Interrupt Mask on line 14"]
pub type MR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR15` reader - Interrupt Mask on line 15"]
pub type MR15_R = crate::BitReader<bool>;
#[doc = "Field `MR15` writer - Interrupt Mask on line 15"]
pub type MR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR16` reader - Interrupt Mask on line 16"]
pub type MR16_R = crate::BitReader<bool>;
#[doc = "Field `MR16` writer - Interrupt Mask on line 16"]
pub type MR16_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR17` reader - Interrupt Mask on line 17"]
pub type MR17_R = crate::BitReader<bool>;
#[doc = "Field `MR17` writer - Interrupt Mask on line 17"]
pub type MR17_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR18` reader - Interrupt Mask on line 18"]
pub type MR18_R = crate::BitReader<bool>;
#[doc = "Field `MR18` writer - Interrupt Mask on line 18"]
pub type MR18_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR19` reader - Interrupt Mask on line 19"]
pub type MR19_R = crate::BitReader<bool>;
#[doc = "Field `MR19` writer - Interrupt Mask on line 19"]
pub type MR19_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR20` reader - Interrupt Mask on line 20"]
pub type MR20_R = crate::BitReader<bool>;
#[doc = "Field `MR20` writer - Interrupt Mask on line 20"]
pub type MR20_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR21` reader - Interrupt Mask on line 21"]
pub type MR21_R = crate::BitReader<bool>;
#[doc = "Field `MR21` writer - Interrupt Mask on line 21"]
pub type MR21_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `MR22` reader - Interrupt Mask on line 22"]
pub type MR22_R = crate::BitReader<bool>;
#[doc = "Field `MR22` writer - Interrupt Mask on line 22"]
pub type MR22_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    pub fn mr0(&self) -> MR0_R {
        MR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    pub fn mr1(&self) -> MR1_R {
        MR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    pub fn mr2(&self) -> MR2_R {
        MR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    pub fn mr3(&self) -> MR3_R {
        MR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    pub fn mr4(&self) -> MR4_R {
        MR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    pub fn mr5(&self) -> MR5_R {
        MR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    pub fn mr6(&self) -> MR6_R {
        MR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    pub fn mr7(&self) -> MR7_R {
        MR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    pub fn mr8(&self) -> MR8_R {
        MR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    pub fn mr9(&self) -> MR9_R {
        MR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    pub fn mr10(&self) -> MR10_R {
        MR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    pub fn mr11(&self) -> MR11_R {
        MR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    pub fn mr12(&self) -> MR12_R {
        MR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    pub fn mr13(&self) -> MR13_R {
        MR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    pub fn mr14(&self) -> MR14_R {
        MR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    pub fn mr15(&self) -> MR15_R {
        MR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    pub fn mr16(&self) -> MR16_R {
        MR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    pub fn mr17(&self) -> MR17_R {
        MR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    pub fn mr18(&self) -> MR18_R {
        MR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Mask on line 19"]
    #[inline(always)]
    pub fn mr19(&self) -> MR19_R {
        MR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Mask on line 20"]
    #[inline(always)]
    pub fn mr20(&self) -> MR20_R {
        MR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Mask on line 21"]
    #[inline(always)]
    pub fn mr21(&self) -> MR21_R {
        MR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Mask on line 22"]
    #[inline(always)]
    pub fn mr22(&self) -> MR22_R {
        MR22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Mask on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn mr0(&mut self) -> MR0_W<0> {
        MR0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Mask on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn mr1(&mut self) -> MR1_W<1> {
        MR1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Mask on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn mr2(&mut self) -> MR2_W<2> {
        MR2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Mask on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn mr3(&mut self) -> MR3_W<3> {
        MR3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Mask on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn mr4(&mut self) -> MR4_W<4> {
        MR4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Mask on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn mr5(&mut self) -> MR5_W<5> {
        MR5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Mask on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn mr6(&mut self) -> MR6_W<6> {
        MR6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Mask on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn mr7(&mut self) -> MR7_W<7> {
        MR7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt Mask on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn mr8(&mut self) -> MR8_W<8> {
        MR8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Mask on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn mr9(&mut self) -> MR9_W<9> {
        MR9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt Mask on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn mr10(&mut self) -> MR10_W<10> {
        MR10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt Mask on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn mr11(&mut self) -> MR11_W<11> {
        MR11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt Mask on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn mr12(&mut self) -> MR12_W<12> {
        MR12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Mask on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn mr13(&mut self) -> MR13_W<13> {
        MR13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt Mask on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn mr14(&mut self) -> MR14_W<14> {
        MR14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt Mask on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn mr15(&mut self) -> MR15_W<15> {
        MR15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Mask on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn mr16(&mut self) -> MR16_W<16> {
        MR16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt Mask on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn mr17(&mut self) -> MR17_W<17> {
        MR17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt Mask on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn mr18(&mut self) -> MR18_W<18> {
        MR18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt Mask on line 19"]
    #[inline(always)]
    #[must_use]
    pub fn mr19(&mut self) -> MR19_W<19> {
        MR19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt Mask on line 20"]
    #[inline(always)]
    #[must_use]
    pub fn mr20(&mut self) -> MR20_W<20> {
        MR20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt Mask on line 21"]
    #[inline(always)]
    #[must_use]
    pub fn mr21(&mut self) -> MR21_W<21> {
        MR21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt Mask on line 22"]
    #[inline(always)]
    #[must_use]
    pub fn mr22(&mut self) -> MR22_W<22> {
        MR22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register (EXTI_IMR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
