#[doc = "Register `IDR` reader"]
pub struct R(crate::R<IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDR0` reader - Port input data (y = 0..15)"]
pub type IDR0_R = crate::BitReader<bool>;
#[doc = "Field `IDR1` reader - Port input data (y = 0..15)"]
pub type IDR1_R = crate::BitReader<bool>;
#[doc = "Field `IDR2` reader - Port input data (y = 0..15)"]
pub type IDR2_R = crate::BitReader<bool>;
#[doc = "Field `IDR3` reader - Port input data (y = 0..15)"]
pub type IDR3_R = crate::BitReader<bool>;
#[doc = "Field `IDR4` reader - Port input data (y = 0..15)"]
pub type IDR4_R = crate::BitReader<bool>;
#[doc = "Field `IDR5` reader - Port input data (y = 0..15)"]
pub type IDR5_R = crate::BitReader<bool>;
#[doc = "Field `IDR6` reader - Port input data (y = 0..15)"]
pub type IDR6_R = crate::BitReader<bool>;
#[doc = "Field `IDR7` reader - Port input data (y = 0..15)"]
pub type IDR7_R = crate::BitReader<bool>;
#[doc = "Field `IDR8` reader - Port input data (y = 0..15)"]
pub type IDR8_R = crate::BitReader<bool>;
#[doc = "Field `IDR9` reader - Port input data (y = 0..15)"]
pub type IDR9_R = crate::BitReader<bool>;
#[doc = "Field `IDR10` reader - Port input data (y = 0..15)"]
pub type IDR10_R = crate::BitReader<bool>;
#[doc = "Field `IDR11` reader - Port input data (y = 0..15)"]
pub type IDR11_R = crate::BitReader<bool>;
#[doc = "Field `IDR12` reader - Port input data (y = 0..15)"]
pub type IDR12_R = crate::BitReader<bool>;
#[doc = "Field `IDR13` reader - Port input data (y = 0..15)"]
pub type IDR13_R = crate::BitReader<bool>;
#[doc = "Field `IDR14` reader - Port input data (y = 0..15)"]
pub type IDR14_R = crate::BitReader<bool>;
#[doc = "Field `IDR15` reader - Port input data (y = 0..15)"]
pub type IDR15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr0(&self) -> IDR0_R {
        IDR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr1(&self) -> IDR1_R {
        IDR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr2(&self) -> IDR2_R {
        IDR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr3(&self) -> IDR3_R {
        IDR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr4(&self) -> IDR4_R {
        IDR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr5(&self) -> IDR5_R {
        IDR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr6(&self) -> IDR6_R {
        IDR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr7(&self) -> IDR7_R {
        IDR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr8(&self) -> IDR8_R {
        IDR8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr9(&self) -> IDR9_R {
        IDR9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr10(&self) -> IDR10_R {
        IDR10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr11(&self) -> IDR11_R {
        IDR11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr12(&self) -> IDR12_R {
        IDR12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr13(&self) -> IDR13_R {
        IDR13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr14(&self) -> IDR14_R {
        IDR14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port input data (y = 0..15)"]
    #[inline(always)]
    pub fn idr15(&self) -> IDR15_R {
        IDR15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "GPIO port input data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idr::R](R) reader structure"]
impl crate::Readable for IDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
