#[doc = "Register `pucr_hw` reader"]
pub struct R(crate::R<PUCR_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCR_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCR_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pucr_hw` writer"]
pub struct W(crate::W<PUCR_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR_HW_SPEC>;
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
impl From<crate::W<PUCR_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCR_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rst_fbdv_hw` reader - "]
pub struct RST_FBDV_HW_R(crate::FieldReader<bool, bool>);
impl RST_FBDV_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_FBDV_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_FBDV_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rst_fbdv_hw` writer - "]
pub struct RST_FBDV_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_FBDV_HW_W<'a> {
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
#[doc = "Field `rst_lotpm_hfp_hw` reader - "]
pub struct RST_LOTPM_HFP_HW_R(crate::FieldReader<bool, bool>);
impl RST_LOTPM_HFP_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_LOTPM_HFP_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_LOTPM_HFP_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rst_lotpm_hfp_hw` writer - "]
pub struct RST_LOTPM_HFP_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_LOTPM_HFP_HW_W<'a> {
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
#[doc = "Field `rst_adpll_hw` reader - "]
pub struct RST_ADPLL_HW_R(crate::FieldReader<bool, bool>);
impl RST_ADPLL_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_ADPLL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_ADPLL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rst_adpll_hw` writer - "]
pub struct RST_ADPLL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_ADPLL_HW_W<'a> {
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
#[doc = "Field `pa_seri_cap_en_hw` reader - "]
pub struct PA_SERI_CAP_EN_HW_R(crate::FieldReader<bool, bool>);
impl PA_SERI_CAP_EN_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PA_SERI_CAP_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_SERI_CAP_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_seri_cap_en_hw` writer - "]
pub struct PA_SERI_CAP_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_SERI_CAP_EN_HW_W<'a> {
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
#[doc = "Field `rx_bypass_en_hw` reader - "]
pub struct RX_BYPASS_EN_HW_R(crate::FieldReader<bool, bool>);
impl RX_BYPASS_EN_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_BYPASS_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BYPASS_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_bypass_en_hw` writer - "]
pub struct RX_BYPASS_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BYPASS_EN_HW_W<'a> {
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
#[doc = "Field `pu_pa_hw` reader - "]
pub struct PU_PA_HW_R(crate::FieldReader<bool, bool>);
impl PU_PA_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_PA_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_PA_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_pa_hw` writer - "]
pub struct PU_PA_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PA_HW_W<'a> {
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
#[doc = "Field `pu_lna_hw` reader - "]
pub struct PU_LNA_HW_R(crate::FieldReader<bool, bool>);
impl PU_LNA_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_LNA_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_LNA_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_lna_hw` writer - "]
pub struct PU_LNA_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LNA_HW_W<'a> {
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
#[doc = "Field `pu_rmx_hw` reader - "]
pub struct PU_RMX_HW_R(crate::FieldReader<bool, bool>);
impl PU_RMX_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_RMX_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RMX_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rmx_hw` writer - "]
pub struct PU_RMX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RMX_HW_W<'a> {
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
#[doc = "Field `pu_rbb_hw` reader - "]
pub struct PU_RBB_HW_R(crate::FieldReader<bool, bool>);
impl PU_RBB_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_RBB_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RBB_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rbb_hw` writer - "]
pub struct PU_RBB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RBB_HW_W<'a> {
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
#[doc = "Field `pu_rbb_pkdet_hw` reader - "]
pub struct PU_RBB_PKDET_HW_R(crate::FieldReader<bool, bool>);
impl PU_RBB_PKDET_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_RBB_PKDET_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RBB_PKDET_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rbb_pkdet_hw` writer - "]
pub struct PU_RBB_PKDET_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RBB_PKDET_HW_W<'a> {
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
#[doc = "Field `pu_rosdac_hw` reader - "]
pub struct PU_ROSDAC_HW_R(crate::FieldReader<bool, bool>);
impl PU_ROSDAC_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_ROSDAC_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_ROSDAC_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rosdac_hw` writer - "]
pub struct PU_ROSDAC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ROSDAC_HW_W<'a> {
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
#[doc = "Field `pu_rxadc_hw` reader - "]
pub struct PU_RXADC_HW_R(crate::FieldReader<bool, bool>);
impl PU_RXADC_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_RXADC_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RXADC_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rxadc_hw` writer - "]
pub struct PU_RXADC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RXADC_HW_W<'a> {
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
#[doc = "Field `rxadc_clk_en_hw` reader - "]
pub struct RXADC_CLK_EN_HW_R(crate::FieldReader<bool, bool>);
impl RXADC_CLK_EN_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXADC_CLK_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXADC_CLK_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxadc_clk_en_hw` writer - "]
pub struct RXADC_CLK_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RXADC_CLK_EN_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `pu_lodist_body_bias_hw` reader - "]
pub struct PU_LODIST_BODY_BIAS_HW_R(crate::FieldReader<bool, bool>);
impl PU_LODIST_BODY_BIAS_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_LODIST_BODY_BIAS_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_LODIST_BODY_BIAS_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_lodist_body_bias_hw` writer - "]
pub struct PU_LODIST_BODY_BIAS_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LODIST_BODY_BIAS_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `pu_vco_ldo_hw` reader - "]
pub struct PU_VCO_LDO_HW_R(crate::FieldReader<bool, bool>);
impl PU_VCO_LDO_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_VCO_LDO_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_VCO_LDO_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_vco_ldo_hw` writer - "]
pub struct PU_VCO_LDO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_VCO_LDO_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `pu_vco_hw` reader - "]
pub struct PU_VCO_HW_R(crate::FieldReader<bool, bool>);
impl PU_VCO_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_VCO_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_VCO_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_vco_hw` writer - "]
pub struct PU_VCO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_VCO_HW_W<'a> {
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
#[doc = "Field `pu_fbdv_hw` reader - "]
pub struct PU_FBDV_HW_R(crate::FieldReader<bool, bool>);
impl PU_FBDV_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_FBDV_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_FBDV_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_fbdv_hw` writer - "]
pub struct PU_FBDV_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_FBDV_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `pu_fbdv_buf_hw` reader - "]
pub struct PU_FBDV_BUF_HW_R(crate::FieldReader<bool, bool>);
impl PU_FBDV_BUF_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_FBDV_BUF_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_FBDV_BUF_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_fbdv_buf_hw` writer - "]
pub struct PU_FBDV_BUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_FBDV_BUF_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `lotpm_hfp_clk_en_hw` reader - "]
pub struct LOTPM_HFP_CLK_EN_HW_R(crate::FieldReader<bool, bool>);
impl LOTPM_HFP_CLK_EN_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOTPM_HFP_CLK_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_HFP_CLK_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_hfp_clk_en_hw` writer - "]
pub struct LOTPM_HFP_CLK_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_HFP_CLK_EN_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `lotpm_lfp_bypass_hw` reader - "]
pub struct LOTPM_LFP_BYPASS_HW_R(crate::FieldReader<bool, bool>);
impl LOTPM_LFP_BYPASS_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOTPM_LFP_BYPASS_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_LFP_BYPASS_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_lfp_bypass_hw` writer - "]
pub struct LOTPM_LFP_BYPASS_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_LFP_BYPASS_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `lotpm_hfp_bypass_hw` reader - "]
pub struct LOTPM_HFP_BYPASS_HW_R(crate::FieldReader<bool, bool>);
impl LOTPM_HFP_BYPASS_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOTPM_HFP_BYPASS_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_HFP_BYPASS_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_hfp_bypass_hw` writer - "]
pub struct LOTPM_HFP_BYPASS_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_HFP_BYPASS_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `adpll_clk_en_hw` reader - "]
pub struct ADPLL_CLK_EN_HW_R(crate::FieldReader<bool, bool>);
impl ADPLL_CLK_EN_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_CLK_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_CLK_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_clk_en_hw` writer - "]
pub struct ADPLL_CLK_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_CLK_EN_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `pu_adpll_adc_hw` reader - "]
pub struct PU_ADPLL_ADC_HW_R(crate::FieldReader<bool, bool>);
impl PU_ADPLL_ADC_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_ADPLL_ADC_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_ADPLL_ADC_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_adpll_adc_hw` writer - "]
pub struct PU_ADPLL_ADC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ADPLL_ADC_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `pu_adpll_sfreg_hw` reader - "]
pub struct PU_ADPLL_SFREG_HW_R(crate::FieldReader<bool, bool>);
impl PU_ADPLL_SFREG_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_ADPLL_SFREG_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_ADPLL_SFREG_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_adpll_sfreg_hw` writer - "]
pub struct PU_ADPLL_SFREG_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ADPLL_SFREG_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `pu_dtc_hw` reader - "]
pub struct PU_DTC_HW_R(crate::FieldReader<bool, bool>);
impl PU_DTC_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_DTC_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_DTC_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_dtc_hw` writer - "]
pub struct PU_DTC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DTC_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `pu_rxbuf_hw` reader - "]
pub struct PU_RXBUF_HW_R(crate::FieldReader<bool, bool>);
impl PU_RXBUF_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_RXBUF_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RXBUF_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rxbuf_hw` writer - "]
pub struct PU_RXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RXBUF_HW_W<'a> {
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
#[doc = "Field `pu_txbuf_hw` reader - "]
pub struct PU_TXBUF_HW_R(crate::FieldReader<bool, bool>);
impl PU_TXBUF_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_TXBUF_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_TXBUF_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_txbuf_hw` writer - "]
pub struct PU_TXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TXBUF_HW_W<'a> {
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
#[doc = "Field `lodist_tx_en_hw` reader - "]
pub struct LODIST_TX_EN_HW_R(crate::FieldReader<bool, bool>);
impl LODIST_TX_EN_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LODIST_TX_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIST_TX_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lodist_tx_en_hw` writer - "]
pub struct LODIST_TX_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIST_TX_EN_HW_W<'a> {
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
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rst_fbdv_hw(&self) -> RST_FBDV_HW_R {
        RST_FBDV_HW_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rst_lotpm_hfp_hw(&self) -> RST_LOTPM_HFP_HW_R {
        RST_LOTPM_HFP_HW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rst_adpll_hw(&self) -> RST_ADPLL_HW_R {
        RST_ADPLL_HW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pa_seri_cap_en_hw(&self) -> PA_SERI_CAP_EN_HW_R {
        PA_SERI_CAP_EN_HW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_bypass_en_hw(&self) -> RX_BYPASS_EN_HW_R {
        RX_BYPASS_EN_HW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pa_hw(&self) -> PU_PA_HW_R {
        PU_PA_HW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_lna_hw(&self) -> PU_LNA_HW_R {
        PU_LNA_HW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_rmx_hw(&self) -> PU_RMX_HW_R {
        PU_RMX_HW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_rbb_hw(&self) -> PU_RBB_HW_R {
        PU_RBB_HW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_rbb_pkdet_hw(&self) -> PU_RBB_PKDET_HW_R {
        PU_RBB_PKDET_HW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_rosdac_hw(&self) -> PU_ROSDAC_HW_R {
        PU_ROSDAC_HW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_rxadc_hw(&self) -> PU_RXADC_HW_R {
        PU_RXADC_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxadc_clk_en_hw(&self) -> RXADC_CLK_EN_HW_R {
        RXADC_CLK_EN_HW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_lodist_body_bias_hw(&self) -> PU_LODIST_BODY_BIAS_HW_R {
        PU_LODIST_BODY_BIAS_HW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pu_vco_ldo_hw(&self) -> PU_VCO_LDO_HW_R {
        PU_VCO_LDO_HW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_vco_hw(&self) -> PU_VCO_HW_R {
        PU_VCO_HW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_fbdv_hw(&self) -> PU_FBDV_HW_R {
        PU_FBDV_HW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_fbdv_buf_hw(&self) -> PU_FBDV_BUF_HW_R {
        PU_FBDV_BUF_HW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lotpm_hfp_clk_en_hw(&self) -> LOTPM_HFP_CLK_EN_HW_R {
        LOTPM_HFP_CLK_EN_HW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lotpm_lfp_bypass_hw(&self) -> LOTPM_LFP_BYPASS_HW_R {
        LOTPM_LFP_BYPASS_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lotpm_hfp_bypass_hw(&self) -> LOTPM_HFP_BYPASS_HW_R {
        LOTPM_HFP_BYPASS_HW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_clk_en_hw(&self) -> ADPLL_CLK_EN_HW_R {
        ADPLL_CLK_EN_HW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_adpll_adc_hw(&self) -> PU_ADPLL_ADC_HW_R {
        PU_ADPLL_ADC_HW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_adpll_sfreg_hw(&self) -> PU_ADPLL_SFREG_HW_R {
        PU_ADPLL_SFREG_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_dtc_hw(&self) -> PU_DTC_HW_R {
        PU_DTC_HW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_rxbuf_hw(&self) -> PU_RXBUF_HW_R {
        PU_RXBUF_HW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_txbuf_hw(&self) -> PU_TXBUF_HW_R {
        PU_TXBUF_HW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lodist_tx_en_hw(&self) -> LODIST_TX_EN_HW_R {
        LODIST_TX_EN_HW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rst_fbdv_hw(&mut self) -> RST_FBDV_HW_W {
        RST_FBDV_HW_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rst_lotpm_hfp_hw(&mut self) -> RST_LOTPM_HFP_HW_W {
        RST_LOTPM_HFP_HW_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rst_adpll_hw(&mut self) -> RST_ADPLL_HW_W {
        RST_ADPLL_HW_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pa_seri_cap_en_hw(&mut self) -> PA_SERI_CAP_EN_HW_W {
        PA_SERI_CAP_EN_HW_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_bypass_en_hw(&mut self) -> RX_BYPASS_EN_HW_W {
        RX_BYPASS_EN_HW_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pa_hw(&mut self) -> PU_PA_HW_W {
        PU_PA_HW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_lna_hw(&mut self) -> PU_LNA_HW_W {
        PU_LNA_HW_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_rmx_hw(&mut self) -> PU_RMX_HW_W {
        PU_RMX_HW_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_rbb_hw(&mut self) -> PU_RBB_HW_W {
        PU_RBB_HW_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_rbb_pkdet_hw(&mut self) -> PU_RBB_PKDET_HW_W {
        PU_RBB_PKDET_HW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_rosdac_hw(&mut self) -> PU_ROSDAC_HW_W {
        PU_ROSDAC_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_rxadc_hw(&mut self) -> PU_RXADC_HW_W {
        PU_RXADC_HW_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxadc_clk_en_hw(&mut self) -> RXADC_CLK_EN_HW_W {
        RXADC_CLK_EN_HW_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_lodist_body_bias_hw(&mut self) -> PU_LODIST_BODY_BIAS_HW_W {
        PU_LODIST_BODY_BIAS_HW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pu_vco_ldo_hw(&mut self) -> PU_VCO_LDO_HW_W {
        PU_VCO_LDO_HW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_vco_hw(&mut self) -> PU_VCO_HW_W {
        PU_VCO_HW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_fbdv_hw(&mut self) -> PU_FBDV_HW_W {
        PU_FBDV_HW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_fbdv_buf_hw(&mut self) -> PU_FBDV_BUF_HW_W {
        PU_FBDV_BUF_HW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lotpm_hfp_clk_en_hw(&mut self) -> LOTPM_HFP_CLK_EN_HW_W {
        LOTPM_HFP_CLK_EN_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lotpm_lfp_bypass_hw(&mut self) -> LOTPM_LFP_BYPASS_HW_W {
        LOTPM_LFP_BYPASS_HW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lotpm_hfp_bypass_hw(&mut self) -> LOTPM_HFP_BYPASS_HW_W {
        LOTPM_HFP_BYPASS_HW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_clk_en_hw(&mut self) -> ADPLL_CLK_EN_HW_W {
        ADPLL_CLK_EN_HW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_adpll_adc_hw(&mut self) -> PU_ADPLL_ADC_HW_W {
        PU_ADPLL_ADC_HW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_adpll_sfreg_hw(&mut self) -> PU_ADPLL_SFREG_HW_W {
        PU_ADPLL_SFREG_HW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_dtc_hw(&mut self) -> PU_DTC_HW_W {
        PU_DTC_HW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_rxbuf_hw(&mut self) -> PU_RXBUF_HW_W {
        PU_RXBUF_HW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_txbuf_hw(&mut self) -> PU_TXBUF_HW_W {
        PU_TXBUF_HW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lodist_tx_en_hw(&mut self) -> LODIST_TX_EN_HW_W {
        LODIST_TX_EN_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware read back of power up signals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr_hw](index.html) module"]
pub struct PUCR_HW_SPEC;
impl crate::RegisterSpec for PUCR_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr_hw::R](R) reader structure"]
impl crate::Readable for PUCR_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr_hw::W](W) writer structure"]
impl crate::Writable for PUCR_HW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pucr_hw to value 0"]
impl crate::Resettable for PUCR_HW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
