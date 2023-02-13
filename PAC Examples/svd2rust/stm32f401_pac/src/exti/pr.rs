#[doc = "Register `PR` reader"]
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR` writer"]
pub struct W(crate::W<PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_SPEC>;
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
impl From<crate::W<PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR0` reader - Pending bit 0"]
pub type PR0_R = crate::BitReader<bool>;
#[doc = "Field `PR0` writer - Pending bit 0"]
pub type PR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR1` reader - Pending bit 1"]
pub type PR1_R = crate::BitReader<bool>;
#[doc = "Field `PR1` writer - Pending bit 1"]
pub type PR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR2` reader - Pending bit 2"]
pub type PR2_R = crate::BitReader<bool>;
#[doc = "Field `PR2` writer - Pending bit 2"]
pub type PR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR3` reader - Pending bit 3"]
pub type PR3_R = crate::BitReader<bool>;
#[doc = "Field `PR3` writer - Pending bit 3"]
pub type PR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR4` reader - Pending bit 4"]
pub type PR4_R = crate::BitReader<bool>;
#[doc = "Field `PR4` writer - Pending bit 4"]
pub type PR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR5` reader - Pending bit 5"]
pub type PR5_R = crate::BitReader<bool>;
#[doc = "Field `PR5` writer - Pending bit 5"]
pub type PR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR6` reader - Pending bit 6"]
pub type PR6_R = crate::BitReader<bool>;
#[doc = "Field `PR6` writer - Pending bit 6"]
pub type PR6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR7` reader - Pending bit 7"]
pub type PR7_R = crate::BitReader<bool>;
#[doc = "Field `PR7` writer - Pending bit 7"]
pub type PR7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR8` reader - Pending bit 8"]
pub type PR8_R = crate::BitReader<bool>;
#[doc = "Field `PR8` writer - Pending bit 8"]
pub type PR8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR9` reader - Pending bit 9"]
pub type PR9_R = crate::BitReader<bool>;
#[doc = "Field `PR9` writer - Pending bit 9"]
pub type PR9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR10` reader - Pending bit 10"]
pub type PR10_R = crate::BitReader<bool>;
#[doc = "Field `PR10` writer - Pending bit 10"]
pub type PR10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR11` reader - Pending bit 11"]
pub type PR11_R = crate::BitReader<bool>;
#[doc = "Field `PR11` writer - Pending bit 11"]
pub type PR11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR12` reader - Pending bit 12"]
pub type PR12_R = crate::BitReader<bool>;
#[doc = "Field `PR12` writer - Pending bit 12"]
pub type PR12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR13` reader - Pending bit 13"]
pub type PR13_R = crate::BitReader<bool>;
#[doc = "Field `PR13` writer - Pending bit 13"]
pub type PR13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR14` reader - Pending bit 14"]
pub type PR14_R = crate::BitReader<bool>;
#[doc = "Field `PR14` writer - Pending bit 14"]
pub type PR14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR15` reader - Pending bit 15"]
pub type PR15_R = crate::BitReader<bool>;
#[doc = "Field `PR15` writer - Pending bit 15"]
pub type PR15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR16` reader - Pending bit 16"]
pub type PR16_R = crate::BitReader<bool>;
#[doc = "Field `PR16` writer - Pending bit 16"]
pub type PR16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR17` reader - Pending bit 17"]
pub type PR17_R = crate::BitReader<bool>;
#[doc = "Field `PR17` writer - Pending bit 17"]
pub type PR17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR18` reader - Pending bit 18"]
pub type PR18_R = crate::BitReader<bool>;
#[doc = "Field `PR18` writer - Pending bit 18"]
pub type PR18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR19` reader - Pending bit 19"]
pub type PR19_R = crate::BitReader<bool>;
#[doc = "Field `PR19` writer - Pending bit 19"]
pub type PR19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR20` reader - Pending bit 20"]
pub type PR20_R = crate::BitReader<bool>;
#[doc = "Field `PR20` writer - Pending bit 20"]
pub type PR20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR21` reader - Pending bit 21"]
pub type PR21_R = crate::BitReader<bool>;
#[doc = "Field `PR21` writer - Pending bit 21"]
pub type PR21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
#[doc = "Field `PR22` reader - Pending bit 22"]
pub type PR22_R = crate::BitReader<bool>;
#[doc = "Field `PR22` writer - Pending bit 22"]
pub type PR22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    pub fn pr0(&self) -> PR0_R {
        PR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    pub fn pr1(&self) -> PR1_R {
        PR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    pub fn pr2(&self) -> PR2_R {
        PR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    pub fn pr3(&self) -> PR3_R {
        PR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    pub fn pr4(&self) -> PR4_R {
        PR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    pub fn pr5(&self) -> PR5_R {
        PR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    pub fn pr6(&self) -> PR6_R {
        PR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    pub fn pr7(&self) -> PR7_R {
        PR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    pub fn pr8(&self) -> PR8_R {
        PR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    pub fn pr9(&self) -> PR9_R {
        PR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    pub fn pr10(&self) -> PR10_R {
        PR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    pub fn pr11(&self) -> PR11_R {
        PR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    pub fn pr12(&self) -> PR12_R {
        PR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    pub fn pr13(&self) -> PR13_R {
        PR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    pub fn pr14(&self) -> PR14_R {
        PR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    pub fn pr15(&self) -> PR15_R {
        PR15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    pub fn pr16(&self) -> PR16_R {
        PR16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    pub fn pr17(&self) -> PR17_R {
        PR17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    pub fn pr18(&self) -> PR18_R {
        PR18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pending bit 19"]
    #[inline(always)]
    pub fn pr19(&self) -> PR19_R {
        PR19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pending bit 20"]
    #[inline(always)]
    pub fn pr20(&self) -> PR20_R {
        PR20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    pub fn pr21(&self) -> PR21_R {
        PR21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    pub fn pr22(&self) -> PR22_R {
        PR22_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pr0(&mut self) -> PR0_W<0> {
        PR0_W::new(self)
    }
    #[doc = "Bit 1 - Pending bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pr1(&mut self) -> PR1_W<1> {
        PR1_W::new(self)
    }
    #[doc = "Bit 2 - Pending bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pr2(&mut self) -> PR2_W<2> {
        PR2_W::new(self)
    }
    #[doc = "Bit 3 - Pending bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pr3(&mut self) -> PR3_W<3> {
        PR3_W::new(self)
    }
    #[doc = "Bit 4 - Pending bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pr4(&mut self) -> PR4_W<4> {
        PR4_W::new(self)
    }
    #[doc = "Bit 5 - Pending bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pr5(&mut self) -> PR5_W<5> {
        PR5_W::new(self)
    }
    #[doc = "Bit 6 - Pending bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn pr6(&mut self) -> PR6_W<6> {
        PR6_W::new(self)
    }
    #[doc = "Bit 7 - Pending bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn pr7(&mut self) -> PR7_W<7> {
        PR7_W::new(self)
    }
    #[doc = "Bit 8 - Pending bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn pr8(&mut self) -> PR8_W<8> {
        PR8_W::new(self)
    }
    #[doc = "Bit 9 - Pending bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn pr9(&mut self) -> PR9_W<9> {
        PR9_W::new(self)
    }
    #[doc = "Bit 10 - Pending bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pr10(&mut self) -> PR10_W<10> {
        PR10_W::new(self)
    }
    #[doc = "Bit 11 - Pending bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pr11(&mut self) -> PR11_W<11> {
        PR11_W::new(self)
    }
    #[doc = "Bit 12 - Pending bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pr12(&mut self) -> PR12_W<12> {
        PR12_W::new(self)
    }
    #[doc = "Bit 13 - Pending bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pr13(&mut self) -> PR13_W<13> {
        PR13_W::new(self)
    }
    #[doc = "Bit 14 - Pending bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pr14(&mut self) -> PR14_W<14> {
        PR14_W::new(self)
    }
    #[doc = "Bit 15 - Pending bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pr15(&mut self) -> PR15_W<15> {
        PR15_W::new(self)
    }
    #[doc = "Bit 16 - Pending bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pr16(&mut self) -> PR16_W<16> {
        PR16_W::new(self)
    }
    #[doc = "Bit 17 - Pending bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pr17(&mut self) -> PR17_W<17> {
        PR17_W::new(self)
    }
    #[doc = "Bit 18 - Pending bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pr18(&mut self) -> PR18_W<18> {
        PR18_W::new(self)
    }
    #[doc = "Bit 19 - Pending bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pr19(&mut self) -> PR19_W<19> {
        PR19_W::new(self)
    }
    #[doc = "Bit 20 - Pending bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pr20(&mut self) -> PR20_W<20> {
        PR20_W::new(self)
    }
    #[doc = "Bit 21 - Pending bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pr21(&mut self) -> PR21_W<21> {
        PR21_W::new(self)
    }
    #[doc = "Bit 22 - Pending bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pr22(&mut self) -> PR22_W<22> {
        PR22_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending register (EXTI_PR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](index.html) module"]
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr::R](R) reader structure"]
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr::W](W) writer structure"]
impl crate::Writable for PR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
