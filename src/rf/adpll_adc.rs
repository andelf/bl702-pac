#[doc = "Register `adpll_adc` reader"]
pub struct R(crate::R<ADPLL_ADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_ADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_ADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_ADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_adc` writer"]
pub struct W(crate::W<ADPLL_ADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_ADC_SPEC>;
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
impl From<crate::W<ADPLL_ADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_ADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_adc_vth_bias_mode` reader - "]
pub type ADPLL_ADC_VTH_BIAS_MODE_R = crate::BitReader<bool>;
#[doc = "Field `adpll_adc_vth_bias_mode` writer - "]
pub type ADPLL_ADC_VTH_BIAS_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_ADC_SPEC, bool, O>;
#[doc = "Field `adpll_adc_vth_en` reader - "]
pub type ADPLL_ADC_VTH_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_adc_vth_en` writer - "]
pub type ADPLL_ADC_VTH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_ADC_SPEC, bool, O>;
#[doc = "Field `adpll_adc_data_sign_sel` reader - "]
pub type ADPLL_ADC_DATA_SIGN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_adc_data_sign_sel` writer - "]
pub type ADPLL_ADC_DATA_SIGN_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_ADC_SPEC, bool, O>;
#[doc = "Field `adpll_adc_vref_fine` reader - "]
pub type ADPLL_ADC_VREF_FINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_adc_vref_fine` writer - "]
pub type ADPLL_ADC_VREF_FINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_ADC_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_adc_vref_coarse` reader - "]
pub type ADPLL_ADC_VREF_COARSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_adc_vref_coarse` writer - "]
pub type ADPLL_ADC_VREF_COARSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_ADC_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_adc_oscal_en` reader - "]
pub type ADPLL_ADC_OSCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_adc_oscal_en` writer - "]
pub type ADPLL_ADC_OSCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_ADC_SPEC, bool, O>;
#[doc = "Field `adpll_adc_clk_sync_inv` reader - "]
pub type ADPLL_ADC_CLK_SYNC_INV_R = crate::BitReader<bool>;
#[doc = "Field `adpll_adc_clk_sync_inv` writer - "]
pub type ADPLL_ADC_CLK_SYNC_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_ADC_SPEC, bool, O>;
#[doc = "Field `adpll_adc_clk_div_sel` reader - "]
pub type ADPLL_ADC_CLK_DIV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_adc_clk_div_sel` writer - "]
pub type ADPLL_ADC_CLK_DIV_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_ADC_SPEC, bool, O>;
#[doc = "Field `adpll_adc_clk_inv` reader - "]
pub type ADPLL_ADC_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `adpll_adc_clk_inv` writer - "]
pub type ADPLL_ADC_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_ADC_SPEC, bool, O>;
#[doc = "Field `adpll_adc_clk_en` reader - "]
pub type ADPLL_ADC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_adc_clk_en` writer - "]
pub type ADPLL_ADC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_ADC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_adc_vth_bias_mode(&self) -> ADPLL_ADC_VTH_BIAS_MODE_R {
        ADPLL_ADC_VTH_BIAS_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adpll_adc_vth_en(&self) -> ADPLL_ADC_VTH_EN_R {
        ADPLL_ADC_VTH_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adpll_adc_data_sign_sel(&self) -> ADPLL_ADC_DATA_SIGN_SEL_R {
        ADPLL_ADC_DATA_SIGN_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adpll_adc_vref_fine(&self) -> ADPLL_ADC_VREF_FINE_R {
        ADPLL_ADC_VREF_FINE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_adc_vref_coarse(&self) -> ADPLL_ADC_VREF_COARSE_R {
        ADPLL_ADC_VREF_COARSE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_adc_oscal_en(&self) -> ADPLL_ADC_OSCAL_EN_R {
        ADPLL_ADC_OSCAL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adpll_adc_clk_sync_inv(&self) -> ADPLL_ADC_CLK_SYNC_INV_R {
        ADPLL_ADC_CLK_SYNC_INV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_adc_clk_div_sel(&self) -> ADPLL_ADC_CLK_DIV_SEL_R {
        ADPLL_ADC_CLK_DIV_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adpll_adc_clk_inv(&self) -> ADPLL_ADC_CLK_INV_R {
        ADPLL_ADC_CLK_INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adpll_adc_clk_en(&self) -> ADPLL_ADC_CLK_EN_R {
        ADPLL_ADC_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_adc_vth_bias_mode(&mut self) -> ADPLL_ADC_VTH_BIAS_MODE_W<0> {
        ADPLL_ADC_VTH_BIAS_MODE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adpll_adc_vth_en(&mut self) -> ADPLL_ADC_VTH_EN_W<1> {
        ADPLL_ADC_VTH_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adpll_adc_data_sign_sel(&mut self) -> ADPLL_ADC_DATA_SIGN_SEL_W<2> {
        ADPLL_ADC_DATA_SIGN_SEL_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adpll_adc_vref_fine(&mut self) -> ADPLL_ADC_VREF_FINE_W<4> {
        ADPLL_ADC_VREF_FINE_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_adc_vref_coarse(&mut self) -> ADPLL_ADC_VREF_COARSE_W<8> {
        ADPLL_ADC_VREF_COARSE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_adc_oscal_en(&mut self) -> ADPLL_ADC_OSCAL_EN_W<12> {
        ADPLL_ADC_OSCAL_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adpll_adc_clk_sync_inv(&mut self) -> ADPLL_ADC_CLK_SYNC_INV_W<16> {
        ADPLL_ADC_CLK_SYNC_INV_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_adc_clk_div_sel(&mut self) -> ADPLL_ADC_CLK_DIV_SEL_W<24> {
        ADPLL_ADC_CLK_DIV_SEL_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adpll_adc_clk_inv(&mut self) -> ADPLL_ADC_CLK_INV_W<28> {
        ADPLL_ADC_CLK_INV_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adpll_adc_clk_en(&mut self) -> ADPLL_ADC_CLK_EN_W<29> {
        ADPLL_ADC_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_adc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_adc](index.html) module"]
pub struct ADPLL_ADC_SPEC;
impl crate::RegisterSpec for ADPLL_ADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_adc::R](R) reader structure"]
impl crate::Readable for ADPLL_ADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_adc::W](W) writer structure"]
impl crate::Writable for ADPLL_ADC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_adc to value 0"]
impl crate::Resettable for ADPLL_ADC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
