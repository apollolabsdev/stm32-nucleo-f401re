#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AWD1` reader - Analog watchdog flag of ADC 1"]
pub type AWD1_R = crate::BitReader<bool>;
#[doc = "Field `EOC1` reader - End of conversion of ADC 1"]
pub type EOC1_R = crate::BitReader<bool>;
#[doc = "Field `JEOC1` reader - Injected channel end of conversion of ADC 1"]
pub type JEOC1_R = crate::BitReader<bool>;
#[doc = "Field `JSTRT1` reader - Injected channel Start flag of ADC 1"]
pub type JSTRT1_R = crate::BitReader<bool>;
#[doc = "Field `STRT1` reader - Regular channel Start flag of ADC 1"]
pub type STRT1_R = crate::BitReader<bool>;
#[doc = "Field `OVR1` reader - Overrun flag of ADC 1"]
pub type OVR1_R = crate::BitReader<bool>;
#[doc = "Field `AWD2` reader - Analog watchdog flag of ADC 2"]
pub type AWD2_R = crate::BitReader<bool>;
#[doc = "Field `EOC2` reader - End of conversion of ADC 2"]
pub type EOC2_R = crate::BitReader<bool>;
#[doc = "Field `JEOC2` reader - Injected channel end of conversion of ADC 2"]
pub type JEOC2_R = crate::BitReader<bool>;
#[doc = "Field `JSTRT2` reader - Injected channel Start flag of ADC 2"]
pub type JSTRT2_R = crate::BitReader<bool>;
#[doc = "Field `STRT2` reader - Regular channel Start flag of ADC 2"]
pub type STRT2_R = crate::BitReader<bool>;
#[doc = "Field `OVR2` reader - Overrun flag of ADC 2"]
pub type OVR2_R = crate::BitReader<bool>;
#[doc = "Field `AWD3` reader - Analog watchdog flag of ADC 3"]
pub type AWD3_R = crate::BitReader<bool>;
#[doc = "Field `EOC3` reader - End of conversion of ADC 3"]
pub type EOC3_R = crate::BitReader<bool>;
#[doc = "Field `JEOC3` reader - Injected channel end of conversion of ADC 3"]
pub type JEOC3_R = crate::BitReader<bool>;
#[doc = "Field `JSTRT3` reader - Injected channel Start flag of ADC 3"]
pub type JSTRT3_R = crate::BitReader<bool>;
#[doc = "Field `STRT3` reader - Regular channel Start flag of ADC 3"]
pub type STRT3_R = crate::BitReader<bool>;
#[doc = "Field `OVR3` reader - Overrun flag of ADC3"]
pub type OVR3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Analog watchdog flag of ADC 1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of conversion of ADC 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion of ADC 1"]
    #[inline(always)]
    pub fn jeoc1(&self) -> JEOC1_R {
        JEOC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Injected channel Start flag of ADC 1"]
    #[inline(always)]
    pub fn jstrt1(&self) -> JSTRT1_R {
        JSTRT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular channel Start flag of ADC 1"]
    #[inline(always)]
    pub fn strt1(&self) -> STRT1_R {
        STRT1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun flag of ADC 1"]
    #[inline(always)]
    pub fn ovr1(&self) -> OVR1_R {
        OVR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Analog watchdog flag of ADC 2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - End of conversion of ADC 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Injected channel end of conversion of ADC 2"]
    #[inline(always)]
    pub fn jeoc2(&self) -> JEOC2_R {
        JEOC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Injected channel Start flag of ADC 2"]
    #[inline(always)]
    pub fn jstrt2(&self) -> JSTRT2_R {
        JSTRT2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Regular channel Start flag of ADC 2"]
    #[inline(always)]
    pub fn strt2(&self) -> STRT2_R {
        STRT2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Overrun flag of ADC 2"]
    #[inline(always)]
    pub fn ovr2(&self) -> OVR2_R {
        OVR2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Analog watchdog flag of ADC 3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - End of conversion of ADC 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Injected channel end of conversion of ADC 3"]
    #[inline(always)]
    pub fn jeoc3(&self) -> JEOC3_R {
        JEOC3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Injected channel Start flag of ADC 3"]
    #[inline(always)]
    pub fn jstrt3(&self) -> JSTRT3_R {
        JSTRT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Regular channel Start flag of ADC 3"]
    #[inline(always)]
    pub fn strt3(&self) -> STRT3_R {
        STRT3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Overrun flag of ADC3"]
    #[inline(always)]
    pub fn ovr3(&self) -> OVR3_R {
        OVR3_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "ADC Common status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
