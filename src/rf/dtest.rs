#[doc = "Register `dtest` reader"]
pub struct R(crate::R<DTEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dtest` writer"]
pub struct W(crate::W<DTEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTEST_SPEC>;
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
impl From<crate::W<DTEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dtest_pulldown` reader - "]
pub type DTEST_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `dtest_pulldown` writer - "]
pub type DTEST_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTEST_SPEC, bool, O>;
#[doc = "Field `dtest_en_rxadc_q` reader - "]
pub type DTEST_EN_RXADC_Q_R = crate::BitReader<bool>;
#[doc = "Field `dtest_en_rxadc_q` writer - "]
pub type DTEST_EN_RXADC_Q_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTEST_SPEC, bool, O>;
#[doc = "Field `dtest_en_rxadc_i` reader - "]
pub type DTEST_EN_RXADC_I_R = crate::BitReader<bool>;
#[doc = "Field `dtest_en_rxadc_i` writer - "]
pub type DTEST_EN_RXADC_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTEST_SPEC, bool, O>;
#[doc = "Field `dtest_en_adpll_adc` reader - "]
pub type DTEST_EN_ADPLL_ADC_R = crate::BitReader<bool>;
#[doc = "Field `dtest_en_adpll_adc` writer - "]
pub type DTEST_EN_ADPLL_ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTEST_SPEC, bool, O>;
#[doc = "Field `dtest_en_mod4` reader - "]
pub type DTEST_EN_MOD4_R = crate::BitReader<bool>;
#[doc = "Field `dtest_en_mod4` writer - "]
pub type DTEST_EN_MOD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTEST_SPEC, bool, O>;
#[doc = "Field `dtest_en_fref` reader - "]
pub type DTEST_EN_FREF_R = crate::BitReader<bool>;
#[doc = "Field `dtest_en_fref` writer - "]
pub type DTEST_EN_FREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTEST_SPEC, bool, O>;
#[doc = "Field `dtest_en_dtc_out` reader - "]
pub type DTEST_EN_DTC_OUT_R = crate::BitReader<bool>;
#[doc = "Field `dtest_en_dtc_out` writer - "]
pub type DTEST_EN_DTC_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTEST_SPEC, bool, O>;
#[doc = "Field `dtest_en_dtc_in` reader - "]
pub type DTEST_EN_DTC_IN_R = crate::BitReader<bool>;
#[doc = "Field `dtest_en_dtc_in` writer - "]
pub type DTEST_EN_DTC_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTEST_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dtest_pulldown(&self) -> DTEST_PULLDOWN_R {
        DTEST_PULLDOWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dtest_en_rxadc_q(&self) -> DTEST_EN_RXADC_Q_R {
        DTEST_EN_RXADC_Q_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dtest_en_rxadc_i(&self) -> DTEST_EN_RXADC_I_R {
        DTEST_EN_RXADC_I_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dtest_en_adpll_adc(&self) -> DTEST_EN_ADPLL_ADC_R {
        DTEST_EN_ADPLL_ADC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dtest_en_mod4(&self) -> DTEST_EN_MOD4_R {
        DTEST_EN_MOD4_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dtest_en_fref(&self) -> DTEST_EN_FREF_R {
        DTEST_EN_FREF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dtest_en_dtc_out(&self) -> DTEST_EN_DTC_OUT_R {
        DTEST_EN_DTC_OUT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dtest_en_dtc_in(&self) -> DTEST_EN_DTC_IN_R {
        DTEST_EN_DTC_IN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dtest_pulldown(&mut self) -> DTEST_PULLDOWN_W<0> {
        DTEST_PULLDOWN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dtest_en_rxadc_q(&mut self) -> DTEST_EN_RXADC_Q_W<23> {
        DTEST_EN_RXADC_Q_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn dtest_en_rxadc_i(&mut self) -> DTEST_EN_RXADC_I_W<24> {
        DTEST_EN_RXADC_I_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dtest_en_adpll_adc(&mut self) -> DTEST_EN_ADPLL_ADC_W<25> {
        DTEST_EN_ADPLL_ADC_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dtest_en_mod4(&mut self) -> DTEST_EN_MOD4_W<26> {
        DTEST_EN_MOD4_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dtest_en_fref(&mut self) -> DTEST_EN_FREF_W<27> {
        DTEST_EN_FREF_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dtest_en_dtc_out(&mut self) -> DTEST_EN_DTC_OUT_W<28> {
        DTEST_EN_DTC_OUT_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dtest_en_dtc_in(&mut self) -> DTEST_EN_DTC_IN_W<29> {
        DTEST_EN_DTC_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LO Bias Mode registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtest](index.html) module"]
pub struct DTEST_SPEC;
impl crate::RegisterSpec for DTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtest::R](R) reader structure"]
impl crate::Readable for DTEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtest::W](W) writer structure"]
impl crate::Writable for DTEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dtest to value 0"]
impl crate::Resettable for DTEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
