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
#[doc = "Field `ten_mbg` reader - "]
pub struct TEN_MBG_R(crate::FieldReader<bool, bool>);
impl TEN_MBG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_MBG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_MBG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_mbg` writer - "]
pub struct TEN_MBG_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_MBG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `ten_dll` reader - "]
pub struct TEN_DLL_R(crate::FieldReader<bool, bool>);
impl TEN_DLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_DLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_DLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_dll` writer - "]
pub struct TEN_DLL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DLL_W<'a> {
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
#[doc = "Field `ten_lodist` reader - "]
pub struct TEN_LODIST_R(crate::FieldReader<bool, bool>);
impl TEN_LODIST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_LODIST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_LODIST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_lodist` writer - "]
pub struct TEN_LODIST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_LODIST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `ten_pa_0` reader - "]
pub struct TEN_PA_0_R(crate::FieldReader<bool, bool>);
impl TEN_PA_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_PA_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_PA_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_pa_0` writer - "]
pub struct TEN_PA_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_PA_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `ten_pa_1` reader - "]
pub struct TEN_PA_1_R(crate::FieldReader<bool, bool>);
impl TEN_PA_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_PA_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_PA_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_pa_1` writer - "]
pub struct TEN_PA_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_PA_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `ten_rrf0` reader - "]
pub struct TEN_RRF0_R(crate::FieldReader<bool, bool>);
impl TEN_RRF0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_RRF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_RRF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_rrf0` writer - "]
pub struct TEN_RRF0_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_RRF0_W<'a> {
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
#[doc = "Field `ten_rrf1` reader - "]
pub struct TEN_RRF1_R(crate::FieldReader<bool, bool>);
impl TEN_RRF1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_RRF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_RRF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_rrf1` writer - "]
pub struct TEN_RRF1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_RRF1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `ten_rxadc` reader - "]
pub struct TEN_RXADC_R(crate::FieldReader<bool, bool>);
impl TEN_RXADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_RXADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_RXADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_rxadc` writer - "]
pub struct TEN_RXADC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_RXADC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `ten_vco` reader - "]
pub struct TEN_VCO_R(crate::FieldReader<bool, bool>);
impl TEN_VCO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_VCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_VCO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_vco` writer - "]
pub struct TEN_VCO_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_VCO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `ten_adpll_adc` reader - "]
pub struct TEN_ADPLL_ADC_R(crate::FieldReader<bool, bool>);
impl TEN_ADPLL_ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_ADPLL_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_ADPLL_ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_adpll_adc` writer - "]
pub struct TEN_ADPLL_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_ADPLL_ADC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `ten_rbb_actest` reader - "]
pub struct TEN_RBB_ACTEST_R(crate::FieldReader<bool, bool>);
impl TEN_RBB_ACTEST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_RBB_ACTEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_RBB_ACTEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_rbb_actest` writer - "]
pub struct TEN_RBB_ACTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_RBB_ACTEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `ten_rbb` reader - "]
pub struct TEN_RBB_R(crate::FieldReader<bool, bool>);
impl TEN_RBB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_RBB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_RBB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_rbb` writer - "]
pub struct TEN_RBB_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_RBB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `ten_dtc` reader - "]
pub struct TEN_DTC_R(crate::FieldReader<bool, bool>);
impl TEN_DTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_DTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_DTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_dtc` writer - "]
pub struct TEN_DTC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `atest_out_en` reader - "]
pub struct ATEST_OUT_EN_R(crate::FieldReader<u8, u8>);
impl ATEST_OUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ATEST_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATEST_OUT_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `atest_out_en` writer - "]
pub struct ATEST_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATEST_OUT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `dc_tp_out_en` reader - "]
pub struct DC_TP_OUT_EN_R(crate::FieldReader<u8, u8>);
impl DC_TP_OUT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DC_TP_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_TP_OUT_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dc_tp_out_en` writer - "]
pub struct DC_TP_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_TP_OUT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ten_mbg(&self) -> TEN_MBG_R {
        TEN_MBG_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ten_dll(&self) -> TEN_DLL_R {
        TEN_DLL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&self) -> TEN_LODIST_R {
        TEN_LODIST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_pa_0(&self) -> TEN_PA_0_R {
        TEN_PA_0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pa_1(&self) -> TEN_PA_1_R {
        TEN_PA_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_rrf0(&self) -> TEN_RRF0_R {
        TEN_RRF0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ten_rrf1(&self) -> TEN_RRF1_R {
        TEN_RRF1_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_rxadc(&self) -> TEN_RXADC_R {
        TEN_RXADC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_vco(&self) -> TEN_VCO_R {
        TEN_VCO_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adpll_adc(&self) -> TEN_ADPLL_ADC_R {
        TEN_ADPLL_ADC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_rbb_actest(&self) -> TEN_RBB_ACTEST_R {
        TEN_RBB_ACTEST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_rbb(&self) -> TEN_RBB_R {
        TEN_RBB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_dtc(&self) -> TEN_DTC_R {
        TEN_DTC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn atest_out_en(&self) -> ATEST_OUT_EN_R {
        ATEST_OUT_EN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dc_tp_out_en(&self) -> DC_TP_OUT_EN_R {
        DC_TP_OUT_EN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ten_mbg(&mut self) -> TEN_MBG_W {
        TEN_MBG_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ten_dll(&mut self) -> TEN_DLL_W {
        TEN_DLL_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&mut self) -> TEN_LODIST_W {
        TEN_LODIST_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_pa_0(&mut self) -> TEN_PA_0_W {
        TEN_PA_0_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pa_1(&mut self) -> TEN_PA_1_W {
        TEN_PA_1_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_rrf0(&mut self) -> TEN_RRF0_W {
        TEN_RRF0_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ten_rrf1(&mut self) -> TEN_RRF1_W {
        TEN_RRF1_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_rxadc(&mut self) -> TEN_RXADC_W {
        TEN_RXADC_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_vco(&mut self) -> TEN_VCO_W {
        TEN_VCO_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adpll_adc(&mut self) -> TEN_ADPLL_ADC_W {
        TEN_ADPLL_ADC_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_rbb_actest(&mut self) -> TEN_RBB_ACTEST_W {
        TEN_RBB_ACTEST_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_rbb(&mut self) -> TEN_RBB_W {
        TEN_RBB_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_dtc(&mut self) -> TEN_DTC_W {
        TEN_DTC_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn atest_out_en(&mut self) -> ATEST_OUT_EN_W {
        ATEST_OUT_EN_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn dc_tp_out_en(&mut self) -> DC_TP_OUT_EN_W {
        DC_TP_OUT_EN_W { w: self }
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
