#[doc = "Register `pucr_reg` reader"]
pub struct R(crate::R<PUCR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pucr_reg` writer"]
pub struct W(crate::W<PUCR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR_REG_SPEC>;
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
impl From<crate::W<PUCR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rst_fbdv` reader - "]
pub struct RST_FBDV_R(crate::FieldReader<bool, bool>);
impl RST_FBDV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_FBDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_FBDV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rst_fbdv` writer - "]
pub struct RST_FBDV_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_FBDV_W<'a> {
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
#[doc = "Field `rst_lotpm_hfp` reader - "]
pub struct RST_LOTPM_HFP_R(crate::FieldReader<bool, bool>);
impl RST_LOTPM_HFP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_LOTPM_HFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_LOTPM_HFP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rst_lotpm_hfp` writer - "]
pub struct RST_LOTPM_HFP_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_LOTPM_HFP_W<'a> {
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
#[doc = "Field `rst_adpll` reader - "]
pub struct RST_ADPLL_R(crate::FieldReader<bool, bool>);
impl RST_ADPLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_ADPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_ADPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rst_adpll` writer - "]
pub struct RST_ADPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_ADPLL_W<'a> {
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
#[doc = "Field `pa_seri_cap_en` reader - "]
pub struct PA_SERI_CAP_EN_R(crate::FieldReader<bool, bool>);
impl PA_SERI_CAP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PA_SERI_CAP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_SERI_CAP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_seri_cap_en` writer - "]
pub struct PA_SERI_CAP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_SERI_CAP_EN_W<'a> {
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
#[doc = "Field `rx_bypass_en` reader - "]
pub struct RX_BYPASS_EN_R(crate::FieldReader<bool, bool>);
impl RX_BYPASS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_BYPASS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_BYPASS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_bypass_en` writer - "]
pub struct RX_BYPASS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BYPASS_EN_W<'a> {
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
#[doc = "Field `pu_pa` reader - "]
pub struct PU_PA_R(crate::FieldReader<bool, bool>);
impl PU_PA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_PA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_PA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_pa` writer - "]
pub struct PU_PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PA_W<'a> {
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
#[doc = "Field `pu_lna` reader - "]
pub struct PU_LNA_R(crate::FieldReader<bool, bool>);
impl PU_LNA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_LNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_LNA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_lna` writer - "]
pub struct PU_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LNA_W<'a> {
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
#[doc = "Field `pu_rmx` reader - "]
pub struct PU_RMX_R(crate::FieldReader<bool, bool>);
impl PU_RMX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_RMX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RMX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rmx` writer - "]
pub struct PU_RMX_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RMX_W<'a> {
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
#[doc = "Field `pu_rbb` reader - "]
pub struct PU_RBB_R(crate::FieldReader<bool, bool>);
impl PU_RBB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_RBB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RBB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rbb` writer - "]
pub struct PU_RBB_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RBB_W<'a> {
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
#[doc = "Field `pu_rbb_pkdet` reader - "]
pub struct PU_RBB_PKDET_R(crate::FieldReader<bool, bool>);
impl PU_RBB_PKDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_RBB_PKDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RBB_PKDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rbb_pkdet` writer - "]
pub struct PU_RBB_PKDET_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RBB_PKDET_W<'a> {
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
#[doc = "Field `pu_rosdac` reader - "]
pub struct PU_ROSDAC_R(crate::FieldReader<bool, bool>);
impl PU_ROSDAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_ROSDAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_ROSDAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rosdac` writer - "]
pub struct PU_ROSDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ROSDAC_W<'a> {
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
#[doc = "Field `pu_rxadc` reader - "]
pub struct PU_RXADC_R(crate::FieldReader<bool, bool>);
impl PU_RXADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_RXADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RXADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rxadc` writer - "]
pub struct PU_RXADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RXADC_W<'a> {
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
#[doc = "Field `rxadc_clk_en` reader - "]
pub struct RXADC_CLK_EN_R(crate::FieldReader<bool, bool>);
impl RXADC_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXADC_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXADC_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxadc_clk_en` writer - "]
pub struct RXADC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXADC_CLK_EN_W<'a> {
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
#[doc = "Field `pu_lodist_body_bias` reader - "]
pub struct PU_LODIST_BODY_BIAS_R(crate::FieldReader<bool, bool>);
impl PU_LODIST_BODY_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_LODIST_BODY_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_LODIST_BODY_BIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_lodist_body_bias` writer - "]
pub struct PU_LODIST_BODY_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LODIST_BODY_BIAS_W<'a> {
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
#[doc = "Field `pu_vco_ldo` reader - "]
pub struct PU_VCO_LDO_R(crate::FieldReader<bool, bool>);
impl PU_VCO_LDO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_VCO_LDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_VCO_LDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_vco_ldo` writer - "]
pub struct PU_VCO_LDO_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_VCO_LDO_W<'a> {
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
#[doc = "Field `pu_vco` reader - "]
pub struct PU_VCO_R(crate::FieldReader<bool, bool>);
impl PU_VCO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_VCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_VCO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_vco` writer - "]
pub struct PU_VCO_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_VCO_W<'a> {
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
#[doc = "Field `pu_fbdv` reader - "]
pub struct PU_FBDV_R(crate::FieldReader<bool, bool>);
impl PU_FBDV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_FBDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_FBDV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_fbdv` writer - "]
pub struct PU_FBDV_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_FBDV_W<'a> {
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
#[doc = "Field `pu_fbdv_buf` reader - "]
pub struct PU_FBDV_BUF_R(crate::FieldReader<bool, bool>);
impl PU_FBDV_BUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_FBDV_BUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_FBDV_BUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_fbdv_buf` writer - "]
pub struct PU_FBDV_BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_FBDV_BUF_W<'a> {
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
#[doc = "Field `lotpm_hfp_clk_en` reader - "]
pub struct LOTPM_HFP_CLK_EN_R(crate::FieldReader<bool, bool>);
impl LOTPM_HFP_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOTPM_HFP_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_HFP_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_hfp_clk_en` writer - "]
pub struct LOTPM_HFP_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_HFP_CLK_EN_W<'a> {
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
#[doc = "Field `lotpm_lfp_bypass` reader - "]
pub struct LOTPM_LFP_BYPASS_R(crate::FieldReader<bool, bool>);
impl LOTPM_LFP_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOTPM_LFP_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_LFP_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_lfp_bypass` writer - "]
pub struct LOTPM_LFP_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_LFP_BYPASS_W<'a> {
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
#[doc = "Field `lotpm_hfp_bypass` reader - "]
pub struct LOTPM_HFP_BYPASS_R(crate::FieldReader<bool, bool>);
impl LOTPM_HFP_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOTPM_HFP_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_HFP_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_hfp_bypass` writer - "]
pub struct LOTPM_HFP_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_HFP_BYPASS_W<'a> {
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
#[doc = "Field `adpll_clk_en` reader - "]
pub struct ADPLL_CLK_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_clk_en` writer - "]
pub struct ADPLL_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_CLK_EN_W<'a> {
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
#[doc = "Field `pu_adpll_adc` reader - "]
pub struct PU_ADPLL_ADC_R(crate::FieldReader<bool, bool>);
impl PU_ADPLL_ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_ADPLL_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_ADPLL_ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_adpll_adc` writer - "]
pub struct PU_ADPLL_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ADPLL_ADC_W<'a> {
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
#[doc = "Field `pu_adpll_sfreg` reader - "]
pub struct PU_ADPLL_SFREG_R(crate::FieldReader<bool, bool>);
impl PU_ADPLL_SFREG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_ADPLL_SFREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_ADPLL_SFREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_adpll_sfreg` writer - "]
pub struct PU_ADPLL_SFREG_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ADPLL_SFREG_W<'a> {
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
#[doc = "Field `pu_dtc` reader - "]
pub struct PU_DTC_R(crate::FieldReader<bool, bool>);
impl PU_DTC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_DTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_DTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_dtc` writer - "]
pub struct PU_DTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DTC_W<'a> {
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
#[doc = "Field `pu_rxbuf` reader - "]
pub struct PU_RXBUF_R(crate::FieldReader<bool, bool>);
impl PU_RXBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_RXBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RXBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rxbuf` writer - "]
pub struct PU_RXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RXBUF_W<'a> {
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
#[doc = "Field `pu_txbuf` reader - "]
pub struct PU_TXBUF_R(crate::FieldReader<bool, bool>);
impl PU_TXBUF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_TXBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_TXBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_txbuf` writer - "]
pub struct PU_TXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TXBUF_W<'a> {
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
#[doc = "Field `lodist_tx_en` reader - "]
pub struct LODIST_TX_EN_R(crate::FieldReader<bool, bool>);
impl LODIST_TX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LODIST_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIST_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lodist_tx_en` writer - "]
pub struct LODIST_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIST_TX_EN_W<'a> {
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
    pub fn rst_fbdv(&self) -> RST_FBDV_R {
        RST_FBDV_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rst_lotpm_hfp(&self) -> RST_LOTPM_HFP_R {
        RST_LOTPM_HFP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rst_adpll(&self) -> RST_ADPLL_R {
        RST_ADPLL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pa_seri_cap_en(&self) -> PA_SERI_CAP_EN_R {
        PA_SERI_CAP_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_bypass_en(&self) -> RX_BYPASS_EN_R {
        RX_BYPASS_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pa(&self) -> PU_PA_R {
        PU_PA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_lna(&self) -> PU_LNA_R {
        PU_LNA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_rmx(&self) -> PU_RMX_R {
        PU_RMX_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_rbb(&self) -> PU_RBB_R {
        PU_RBB_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_rbb_pkdet(&self) -> PU_RBB_PKDET_R {
        PU_RBB_PKDET_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_rosdac(&self) -> PU_ROSDAC_R {
        PU_ROSDAC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_rxadc(&self) -> PU_RXADC_R {
        PU_RXADC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxadc_clk_en(&self) -> RXADC_CLK_EN_R {
        RXADC_CLK_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_lodist_body_bias(&self) -> PU_LODIST_BODY_BIAS_R {
        PU_LODIST_BODY_BIAS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pu_vco_ldo(&self) -> PU_VCO_LDO_R {
        PU_VCO_LDO_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_vco(&self) -> PU_VCO_R {
        PU_VCO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_fbdv(&self) -> PU_FBDV_R {
        PU_FBDV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_fbdv_buf(&self) -> PU_FBDV_BUF_R {
        PU_FBDV_BUF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lotpm_hfp_clk_en(&self) -> LOTPM_HFP_CLK_EN_R {
        LOTPM_HFP_CLK_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lotpm_lfp_bypass(&self) -> LOTPM_LFP_BYPASS_R {
        LOTPM_LFP_BYPASS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lotpm_hfp_bypass(&self) -> LOTPM_HFP_BYPASS_R {
        LOTPM_HFP_BYPASS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_clk_en(&self) -> ADPLL_CLK_EN_R {
        ADPLL_CLK_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_adpll_adc(&self) -> PU_ADPLL_ADC_R {
        PU_ADPLL_ADC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_adpll_sfreg(&self) -> PU_ADPLL_SFREG_R {
        PU_ADPLL_SFREG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_dtc(&self) -> PU_DTC_R {
        PU_DTC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_rxbuf(&self) -> PU_RXBUF_R {
        PU_RXBUF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_txbuf(&self) -> PU_TXBUF_R {
        PU_TXBUF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lodist_tx_en(&self) -> LODIST_TX_EN_R {
        LODIST_TX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rst_fbdv(&mut self) -> RST_FBDV_W {
        RST_FBDV_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rst_lotpm_hfp(&mut self) -> RST_LOTPM_HFP_W {
        RST_LOTPM_HFP_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rst_adpll(&mut self) -> RST_ADPLL_W {
        RST_ADPLL_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pa_seri_cap_en(&mut self) -> PA_SERI_CAP_EN_W {
        PA_SERI_CAP_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_bypass_en(&mut self) -> RX_BYPASS_EN_W {
        RX_BYPASS_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pa(&mut self) -> PU_PA_W {
        PU_PA_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_lna(&mut self) -> PU_LNA_W {
        PU_LNA_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_rmx(&mut self) -> PU_RMX_W {
        PU_RMX_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_rbb(&mut self) -> PU_RBB_W {
        PU_RBB_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_rbb_pkdet(&mut self) -> PU_RBB_PKDET_W {
        PU_RBB_PKDET_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_rosdac(&mut self) -> PU_ROSDAC_W {
        PU_ROSDAC_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_rxadc(&mut self) -> PU_RXADC_W {
        PU_RXADC_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxadc_clk_en(&mut self) -> RXADC_CLK_EN_W {
        RXADC_CLK_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_lodist_body_bias(&mut self) -> PU_LODIST_BODY_BIAS_W {
        PU_LODIST_BODY_BIAS_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pu_vco_ldo(&mut self) -> PU_VCO_LDO_W {
        PU_VCO_LDO_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_vco(&mut self) -> PU_VCO_W {
        PU_VCO_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_fbdv(&mut self) -> PU_FBDV_W {
        PU_FBDV_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_fbdv_buf(&mut self) -> PU_FBDV_BUF_W {
        PU_FBDV_BUF_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lotpm_hfp_clk_en(&mut self) -> LOTPM_HFP_CLK_EN_W {
        LOTPM_HFP_CLK_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lotpm_lfp_bypass(&mut self) -> LOTPM_LFP_BYPASS_W {
        LOTPM_LFP_BYPASS_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lotpm_hfp_bypass(&mut self) -> LOTPM_HFP_BYPASS_W {
        LOTPM_HFP_BYPASS_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_clk_en(&mut self) -> ADPLL_CLK_EN_W {
        ADPLL_CLK_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_adpll_adc(&mut self) -> PU_ADPLL_ADC_W {
        PU_ADPLL_ADC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_adpll_sfreg(&mut self) -> PU_ADPLL_SFREG_W {
        PU_ADPLL_SFREG_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_dtc(&mut self) -> PU_DTC_W {
        PU_DTC_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_rxbuf(&mut self) -> PU_RXBUF_W {
        PU_RXBUF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_txbuf(&mut self) -> PU_TXBUF_W {
        PU_TXBUF_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lodist_tx_en(&mut self) -> LODIST_TX_EN_W {
        LODIST_TX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register control of power up signals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr_reg](index.html) module"]
pub struct PUCR_REG_SPEC;
impl crate::RegisterSpec for PUCR_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr_reg::R](R) reader structure"]
impl crate::Readable for PUCR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr_reg::W](W) writer structure"]
impl crate::Writable for PUCR_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pucr_reg to value 0"]
impl crate::Resettable for PUCR_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
