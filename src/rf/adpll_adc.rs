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
#[doc = "Field `adpll_adc_clk_en` reader - "]
pub struct ADPLL_ADC_CLK_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_ADC_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_ADC_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_ADC_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_adc_clk_en` writer - "]
pub struct ADPLL_ADC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_ADC_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `adpll_adc_clk_inv` reader - "]
pub struct ADPLL_ADC_CLK_INV_R(crate::FieldReader<bool, bool>);
impl ADPLL_ADC_CLK_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_ADC_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_ADC_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_adc_clk_inv` writer - "]
pub struct ADPLL_ADC_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_ADC_CLK_INV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `adpll_adc_clk_div_sel` reader - "]
pub struct ADPLL_ADC_CLK_DIV_SEL_R(crate::FieldReader<bool, bool>);
impl ADPLL_ADC_CLK_DIV_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_ADC_CLK_DIV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_ADC_CLK_DIV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_adc_clk_div_sel` writer - "]
pub struct ADPLL_ADC_CLK_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_ADC_CLK_DIV_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `adpll_adc_clk_sync_inv` reader - "]
pub struct ADPLL_ADC_CLK_SYNC_INV_R(crate::FieldReader<bool, bool>);
impl ADPLL_ADC_CLK_SYNC_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_ADC_CLK_SYNC_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_ADC_CLK_SYNC_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_adc_clk_sync_inv` writer - "]
pub struct ADPLL_ADC_CLK_SYNC_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_ADC_CLK_SYNC_INV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `adpll_adc_oscal_en` reader - "]
pub struct ADPLL_ADC_OSCAL_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_ADC_OSCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_ADC_OSCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_ADC_OSCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_adc_oscal_en` writer - "]
pub struct ADPLL_ADC_OSCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_ADC_OSCAL_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `adpll_adc_vref_coarse` reader - "]
pub struct ADPLL_ADC_VREF_COARSE_R(crate::FieldReader<u8, u8>);
impl ADPLL_ADC_VREF_COARSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_ADC_VREF_COARSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_ADC_VREF_COARSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_adc_vref_coarse` writer - "]
pub struct ADPLL_ADC_VREF_COARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_ADC_VREF_COARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `adpll_adc_vref_fine` reader - "]
pub struct ADPLL_ADC_VREF_FINE_R(crate::FieldReader<u8, u8>);
impl ADPLL_ADC_VREF_FINE_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_ADC_VREF_FINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_ADC_VREF_FINE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_adc_vref_fine` writer - "]
pub struct ADPLL_ADC_VREF_FINE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_ADC_VREF_FINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `adpll_adc_data_sign_sel` reader - "]
pub struct ADPLL_ADC_DATA_SIGN_SEL_R(crate::FieldReader<bool, bool>);
impl ADPLL_ADC_DATA_SIGN_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_ADC_DATA_SIGN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_ADC_DATA_SIGN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_adc_data_sign_sel` writer - "]
pub struct ADPLL_ADC_DATA_SIGN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_ADC_DATA_SIGN_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `adpll_adc_vth_en` reader - "]
pub struct ADPLL_ADC_VTH_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_ADC_VTH_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_ADC_VTH_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_ADC_VTH_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_adc_vth_en` writer - "]
pub struct ADPLL_ADC_VTH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_ADC_VTH_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `adpll_adc_vth_bias_mode` reader - "]
pub struct ADPLL_ADC_VTH_BIAS_MODE_R(crate::FieldReader<bool, bool>);
impl ADPLL_ADC_VTH_BIAS_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_ADC_VTH_BIAS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_ADC_VTH_BIAS_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_adc_vth_bias_mode` writer - "]
pub struct ADPLL_ADC_VTH_BIAS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_ADC_VTH_BIAS_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adpll_adc_clk_en(&self) -> ADPLL_ADC_CLK_EN_R {
        ADPLL_ADC_CLK_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adpll_adc_clk_inv(&self) -> ADPLL_ADC_CLK_INV_R {
        ADPLL_ADC_CLK_INV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_adc_clk_div_sel(&self) -> ADPLL_ADC_CLK_DIV_SEL_R {
        ADPLL_ADC_CLK_DIV_SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adpll_adc_clk_sync_inv(&self) -> ADPLL_ADC_CLK_SYNC_INV_R {
        ADPLL_ADC_CLK_SYNC_INV_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_adc_oscal_en(&self) -> ADPLL_ADC_OSCAL_EN_R {
        ADPLL_ADC_OSCAL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_adc_vref_coarse(&self) -> ADPLL_ADC_VREF_COARSE_R {
        ADPLL_ADC_VREF_COARSE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adpll_adc_vref_fine(&self) -> ADPLL_ADC_VREF_FINE_R {
        ADPLL_ADC_VREF_FINE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adpll_adc_data_sign_sel(&self) -> ADPLL_ADC_DATA_SIGN_SEL_R {
        ADPLL_ADC_DATA_SIGN_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adpll_adc_vth_en(&self) -> ADPLL_ADC_VTH_EN_R {
        ADPLL_ADC_VTH_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_adc_vth_bias_mode(&self) -> ADPLL_ADC_VTH_BIAS_MODE_R {
        ADPLL_ADC_VTH_BIAS_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adpll_adc_clk_en(&mut self) -> ADPLL_ADC_CLK_EN_W {
        ADPLL_ADC_CLK_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adpll_adc_clk_inv(&mut self) -> ADPLL_ADC_CLK_INV_W {
        ADPLL_ADC_CLK_INV_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_adc_clk_div_sel(&mut self) -> ADPLL_ADC_CLK_DIV_SEL_W {
        ADPLL_ADC_CLK_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adpll_adc_clk_sync_inv(&mut self) -> ADPLL_ADC_CLK_SYNC_INV_W {
        ADPLL_ADC_CLK_SYNC_INV_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_adc_oscal_en(&mut self) -> ADPLL_ADC_OSCAL_EN_W {
        ADPLL_ADC_OSCAL_EN_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_adc_vref_coarse(&mut self) -> ADPLL_ADC_VREF_COARSE_W {
        ADPLL_ADC_VREF_COARSE_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adpll_adc_vref_fine(&mut self) -> ADPLL_ADC_VREF_FINE_W {
        ADPLL_ADC_VREF_FINE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adpll_adc_data_sign_sel(&mut self) -> ADPLL_ADC_DATA_SIGN_SEL_W {
        ADPLL_ADC_DATA_SIGN_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adpll_adc_vth_en(&mut self) -> ADPLL_ADC_VTH_EN_W {
        ADPLL_ADC_VTH_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_adc_vth_bias_mode(&mut self) -> ADPLL_ADC_VTH_BIAS_MODE_W {
        ADPLL_ADC_VTH_BIAS_MODE_W { w: self }
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
