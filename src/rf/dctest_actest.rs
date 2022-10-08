#[doc = "Register `dctest_actest` reader"]
pub struct R(crate::R<DCTEST_ACTEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTEST_ACTEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTEST_ACTEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTEST_ACTEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dctest_actest` writer"]
pub struct W(crate::W<DCTEST_ACTEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTEST_ACTEST_SPEC>;
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
impl From<crate::W<DCTEST_ACTEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTEST_ACTEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dc_tp_out_en` reader - "]
pub type DC_TP_OUT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dc_tp_out_en` writer - "]
pub type DC_TP_OUT_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCTEST_ACTEST_SPEC, u8, u8, 2, O>;
#[doc = "Field `atest_out_en` reader - "]
pub type ATEST_OUT_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `atest_out_en` writer - "]
pub type ATEST_OUT_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCTEST_ACTEST_SPEC, u8, u8, 2, O>;
#[doc = "Field `ten_dtc` reader - "]
pub type TEN_DTC_R = crate::BitReader<bool>;
#[doc = "Field `ten_dtc` writer - "]
pub type TEN_DTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_rbb` reader - "]
pub type TEN_RBB_R = crate::BitReader<bool>;
#[doc = "Field `ten_rbb` writer - "]
pub type TEN_RBB_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_rbb_actest` reader - "]
pub type TEN_RBB_ACTEST_R = crate::BitReader<bool>;
#[doc = "Field `ten_rbb_actest` writer - "]
pub type TEN_RBB_ACTEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_adpll_adc` reader - "]
pub type TEN_ADPLL_ADC_R = crate::BitReader<bool>;
#[doc = "Field `ten_adpll_adc` writer - "]
pub type TEN_ADPLL_ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_vco` reader - "]
pub type TEN_VCO_R = crate::BitReader<bool>;
#[doc = "Field `ten_vco` writer - "]
pub type TEN_VCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_rxadc` reader - "]
pub type TEN_RXADC_R = crate::BitReader<bool>;
#[doc = "Field `ten_rxadc` writer - "]
pub type TEN_RXADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_rrf1` reader - "]
pub type TEN_RRF1_R = crate::BitReader<bool>;
#[doc = "Field `ten_rrf1` writer - "]
pub type TEN_RRF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_rrf0` reader - "]
pub type TEN_RRF0_R = crate::BitReader<bool>;
#[doc = "Field `ten_rrf0` writer - "]
pub type TEN_RRF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_pa_1` reader - "]
pub type TEN_PA_1_R = crate::BitReader<bool>;
#[doc = "Field `ten_pa_1` writer - "]
pub type TEN_PA_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_pa_0` reader - "]
pub type TEN_PA_0_R = crate::BitReader<bool>;
#[doc = "Field `ten_pa_0` writer - "]
pub type TEN_PA_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_lodist` reader - "]
pub type TEN_LODIST_R = crate::BitReader<bool>;
#[doc = "Field `ten_lodist` writer - "]
pub type TEN_LODIST_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_dll` reader - "]
pub type TEN_DLL_R = crate::BitReader<bool>;
#[doc = "Field `ten_dll` writer - "]
pub type TEN_DLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
#[doc = "Field `ten_mbg` reader - "]
pub type TEN_MBG_R = crate::BitReader<bool>;
#[doc = "Field `ten_mbg` writer - "]
pub type TEN_MBG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCTEST_ACTEST_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dc_tp_out_en(&self) -> DC_TP_OUT_EN_R {
        DC_TP_OUT_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn atest_out_en(&self) -> ATEST_OUT_EN_R {
        ATEST_OUT_EN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_dtc(&self) -> TEN_DTC_R {
        TEN_DTC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_rbb(&self) -> TEN_RBB_R {
        TEN_RBB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_rbb_actest(&self) -> TEN_RBB_ACTEST_R {
        TEN_RBB_ACTEST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adpll_adc(&self) -> TEN_ADPLL_ADC_R {
        TEN_ADPLL_ADC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_vco(&self) -> TEN_VCO_R {
        TEN_VCO_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_rxadc(&self) -> TEN_RXADC_R {
        TEN_RXADC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ten_rrf1(&self) -> TEN_RRF1_R {
        TEN_RRF1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_rrf0(&self) -> TEN_RRF0_R {
        TEN_RRF0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pa_1(&self) -> TEN_PA_1_R {
        TEN_PA_1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_pa_0(&self) -> TEN_PA_0_R {
        TEN_PA_0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&self) -> TEN_LODIST_R {
        TEN_LODIST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ten_dll(&self) -> TEN_DLL_R {
        TEN_DLL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ten_mbg(&self) -> TEN_MBG_R {
        TEN_MBG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dc_tp_out_en(&mut self) -> DC_TP_OUT_EN_W<4> {
        DC_TP_OUT_EN_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn atest_out_en(&mut self) -> ATEST_OUT_EN_W<6> {
        ATEST_OUT_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_dtc(&mut self) -> TEN_DTC_W<17> {
        TEN_DTC_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_rbb(&mut self) -> TEN_RBB_W<18> {
        TEN_RBB_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_rbb_actest(&mut self) -> TEN_RBB_ACTEST_W<19> {
        TEN_RBB_ACTEST_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adpll_adc(&mut self) -> TEN_ADPLL_ADC_W<20> {
        TEN_ADPLL_ADC_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_vco(&mut self) -> TEN_VCO_W<21> {
        TEN_VCO_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_rxadc(&mut self) -> TEN_RXADC_W<22> {
        TEN_RXADC_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ten_rrf1(&mut self) -> TEN_RRF1_W<23> {
        TEN_RRF1_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_rrf0(&mut self) -> TEN_RRF0_W<24> {
        TEN_RRF0_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pa_1(&mut self) -> TEN_PA_1_W<25> {
        TEN_PA_1_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_pa_0(&mut self) -> TEN_PA_0_W<26> {
        TEN_PA_0_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&mut self) -> TEN_LODIST_W<27> {
        TEN_LODIST_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ten_dll(&mut self) -> TEN_DLL_W<28> {
        TEN_DLL_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ten_mbg(&mut self) -> TEN_MBG_W<31> {
        TEN_MBG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DC Test register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctest_actest](index.html) module"]
pub struct DCTEST_ACTEST_SPEC;
impl crate::RegisterSpec for DCTEST_ACTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctest_actest::R](R) reader structure"]
impl crate::Readable for DCTEST_ACTEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctest_actest::W](W) writer structure"]
impl crate::Writable for DCTEST_ACTEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dctest_actest to value 0"]
impl crate::Resettable for DCTEST_ACTEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
