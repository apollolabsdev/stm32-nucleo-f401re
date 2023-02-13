#[doc = "Register `LISR` reader"]
pub struct R(crate::R<LISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FEIF0` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF0_R = crate::BitReader<bool>;
#[doc = "Field `DMEIF0` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF0_R = crate::BitReader<bool>;
#[doc = "Field `TEIF0` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF0_R = crate::BitReader<bool>;
#[doc = "Field `HTIF0` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF0_R = crate::BitReader<bool>;
#[doc = "Field `TCIF0` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF0_R = crate::BitReader<bool>;
#[doc = "Field `FEIF1` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF1_R = crate::BitReader<bool>;
#[doc = "Field `DMEIF1` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF1_R = crate::BitReader<bool>;
#[doc = "Field `TEIF1` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF1_R = crate::BitReader<bool>;
#[doc = "Field `HTIF1` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF1_R = crate::BitReader<bool>;
#[doc = "Field `TCIF1` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF1_R = crate::BitReader<bool>;
#[doc = "Field `FEIF2` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF2_R = crate::BitReader<bool>;
#[doc = "Field `DMEIF2` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF2_R = crate::BitReader<bool>;
#[doc = "Field `TEIF2` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF2_R = crate::BitReader<bool>;
#[doc = "Field `HTIF2` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF2_R = crate::BitReader<bool>;
#[doc = "Field `TCIF2` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF2_R = crate::BitReader<bool>;
#[doc = "Field `FEIF3` reader - Stream x FIFO error interrupt flag (x=3..0)"]
pub type FEIF3_R = crate::BitReader<bool>;
#[doc = "Field `DMEIF3` reader - Stream x direct mode error interrupt flag (x=3..0)"]
pub type DMEIF3_R = crate::BitReader<bool>;
#[doc = "Field `TEIF3` reader - Stream x transfer error interrupt flag (x=3..0)"]
pub type TEIF3_R = crate::BitReader<bool>;
#[doc = "Field `HTIF3` reader - Stream x half transfer interrupt flag (x=3..0)"]
pub type HTIF3_R = crate::BitReader<bool>;
#[doc = "Field `TCIF3` reader - Stream x transfer complete interrupt flag (x = 3..0)"]
pub type TCIF3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif0(&self) -> FEIF0_R {
        FEIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif0(&self) -> DMEIF0_R {
        DMEIF0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif0(&self) -> HTIF0_R {
        HTIF0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif1(&self) -> FEIF1_R {
        FEIF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif1(&self) -> DMEIF1_R {
        DMEIF1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif1(&self) -> HTIF1_R {
        HTIF1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif2(&self) -> FEIF2_R {
        FEIF2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif2(&self) -> DMEIF2_R {
        DMEIF2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif2(&self) -> HTIF2_R {
        HTIF2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream x FIFO error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn feif3(&self) -> FEIF3_R {
        FEIF3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream x direct mode error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn dmeif3(&self) -> DMEIF3_R {
        DMEIF3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream x transfer error interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream x half transfer interrupt flag (x=3..0)"]
    #[inline(always)]
    pub fn htif3(&self) -> HTIF3_R {
        HTIF3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream x transfer complete interrupt flag (x = 3..0)"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "low interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lisr](index.html) module"]
pub struct LISR_SPEC;
impl crate::RegisterSpec for LISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lisr::R](R) reader structure"]
impl crate::Readable for LISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LISR to value 0"]
impl crate::Resettable for LISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
