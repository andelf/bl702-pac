#[doc = "Register `adpll_lms` reader"]
pub struct R(crate::R<ADPLL_LMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_LMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_LMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_LMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_lms` writer"]
pub struct W(crate::W<ADPLL_LMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_LMS_SPEC>;
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
impl From<crate::W<ADPLL_LMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_LMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_fref_div2_en` reader - "]
pub struct ADPLL_FREF_DIV2_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_FREF_DIV2_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_FREF_DIV2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_FREF_DIV2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_fref_div2_en` writer - "]
pub struct ADPLL_FREF_DIV2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_FREF_DIV2_EN_W<'a> {
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
#[doc = "Field `adpll_lms_ext_value_en` reader - "]
pub struct ADPLL_LMS_EXT_VALUE_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_LMS_EXT_VALUE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_LMS_EXT_VALUE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LMS_EXT_VALUE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lms_ext_value_en` writer - "]
pub struct ADPLL_LMS_EXT_VALUE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LMS_EXT_VALUE_EN_W<'a> {
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
#[doc = "Field `adpll_lms_ext_value` reader - "]
pub struct ADPLL_LMS_EXT_VALUE_R(crate::FieldReader<u16, u16>);
impl ADPLL_LMS_EXT_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        ADPLL_LMS_EXT_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LMS_EXT_VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lms_ext_value` writer - "]
pub struct ADPLL_LMS_EXT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LMS_EXT_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 20)) | ((value as u32 & 0x01ff) << 20);
        self.w
    }
}
#[doc = "Field `adpll_sdm_dither_en` reader - "]
pub struct ADPLL_SDM_DITHER_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_SDM_DITHER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_SDM_DITHER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SDM_DITHER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_sdm_dither_en` writer - "]
pub struct ADPLL_SDM_DITHER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SDM_DITHER_EN_W<'a> {
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
#[doc = "Field `adpll_sdm_dither_en_ctrl_hw` reader - "]
pub struct ADPLL_SDM_DITHER_EN_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl ADPLL_SDM_DITHER_EN_CTRL_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_SDM_DITHER_EN_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SDM_DITHER_EN_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_sdm_dither_en_ctrl_hw` writer - "]
pub struct ADPLL_SDM_DITHER_EN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SDM_DITHER_EN_CTRL_HW_W<'a> {
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
#[doc = "Field `adpll_lms_step` reader - "]
pub struct ADPLL_LMS_STEP_R(crate::FieldReader<u8, u8>);
impl ADPLL_LMS_STEP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_LMS_STEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LMS_STEP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lms_step` writer - "]
pub struct ADPLL_LMS_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LMS_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `adpll_sdm_dither_prbs_en` reader - "]
pub struct ADPLL_SDM_DITHER_PRBS_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_SDM_DITHER_PRBS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_SDM_DITHER_PRBS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SDM_DITHER_PRBS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_sdm_dither_prbs_en` writer - "]
pub struct ADPLL_SDM_DITHER_PRBS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SDM_DITHER_PRBS_EN_W<'a> {
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
#[doc = "Field `adpll_pha_dem_en` reader - "]
pub struct ADPLL_PHA_DEM_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_PHA_DEM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_PHA_DEM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_PHA_DEM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_pha_dem_en` writer - "]
pub struct ADPLL_PHA_DEM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_PHA_DEM_EN_W<'a> {
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
#[doc = "Field `adpll_pha_dither_en` reader - "]
pub struct ADPLL_PHA_DITHER_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_PHA_DITHER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_PHA_DITHER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_PHA_DITHER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_pha_dither_en` writer - "]
pub struct ADPLL_PHA_DITHER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_PHA_DITHER_EN_W<'a> {
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
#[doc = "Field `adpll_lms_step_enlarge` reader - "]
pub struct ADPLL_LMS_STEP_ENLARGE_R(crate::FieldReader<bool, bool>);
impl ADPLL_LMS_STEP_ENLARGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_LMS_STEP_ENLARGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LMS_STEP_ENLARGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lms_step_enlarge` writer - "]
pub struct ADPLL_LMS_STEP_ENLARGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LMS_STEP_ENLARGE_W<'a> {
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
#[doc = "Field `adpll_pha_prbs_sel` reader - "]
pub struct ADPLL_PHA_PRBS_SEL_R(crate::FieldReader<u8, u8>);
impl ADPLL_PHA_PRBS_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_PHA_PRBS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_PHA_PRBS_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_pha_prbs_sel` writer - "]
pub struct ADPLL_PHA_PRBS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_PHA_PRBS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `adpll_lms_q_delay` reader - "]
pub struct ADPLL_LMS_Q_DELAY_R(crate::FieldReader<u8, u8>);
impl ADPLL_LMS_Q_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_LMS_Q_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LMS_Q_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lms_q_delay` writer - "]
pub struct ADPLL_LMS_Q_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LMS_Q_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `adpll_pha_cancel_en` reader - "]
pub struct ADPLL_PHA_CANCEL_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_PHA_CANCEL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_PHA_CANCEL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_PHA_CANCEL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_pha_cancel_en` writer - "]
pub struct ADPLL_PHA_CANCEL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_PHA_CANCEL_EN_W<'a> {
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
#[doc = "Field `adpll_pha_cancel_delay` reader - "]
pub struct ADPLL_PHA_CANCEL_DELAY_R(crate::FieldReader<u8, u8>);
impl ADPLL_PHA_CANCEL_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_PHA_CANCEL_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_PHA_CANCEL_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_pha_cancel_delay` writer - "]
pub struct ADPLL_PHA_CANCEL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_PHA_CANCEL_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adpll_fref_div2_en(&self) -> ADPLL_FREF_DIV2_EN_R {
        ADPLL_FREF_DIV2_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adpll_lms_ext_value_en(&self) -> ADPLL_LMS_EXT_VALUE_EN_R {
        ADPLL_LMS_EXT_VALUE_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn adpll_lms_ext_value(&self) -> ADPLL_LMS_EXT_VALUE_R {
        ADPLL_LMS_EXT_VALUE_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en(&self) -> ADPLL_SDM_DITHER_EN_R {
        ADPLL_SDM_DITHER_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en_ctrl_hw(&self) -> ADPLL_SDM_DITHER_EN_CTRL_HW_R {
        ADPLL_SDM_DITHER_EN_CTRL_HW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adpll_lms_step(&self) -> ADPLL_LMS_STEP_R {
        ADPLL_LMS_STEP_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adpll_sdm_dither_prbs_en(&self) -> ADPLL_SDM_DITHER_PRBS_EN_R {
        ADPLL_SDM_DITHER_PRBS_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_pha_dem_en(&self) -> ADPLL_PHA_DEM_EN_R {
        ADPLL_PHA_DEM_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_pha_dither_en(&self) -> ADPLL_PHA_DITHER_EN_R {
        ADPLL_PHA_DITHER_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_lms_step_enlarge(&self) -> ADPLL_LMS_STEP_ENLARGE_R {
        ADPLL_LMS_STEP_ENLARGE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adpll_pha_prbs_sel(&self) -> ADPLL_PHA_PRBS_SEL_R {
        ADPLL_PHA_PRBS_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_lms_q_delay(&self) -> ADPLL_LMS_Q_DELAY_R {
        ADPLL_LMS_Q_DELAY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adpll_pha_cancel_en(&self) -> ADPLL_PHA_CANCEL_EN_R {
        ADPLL_PHA_CANCEL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adpll_pha_cancel_delay(&self) -> ADPLL_PHA_CANCEL_DELAY_R {
        ADPLL_PHA_CANCEL_DELAY_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adpll_fref_div2_en(&mut self) -> ADPLL_FREF_DIV2_EN_W {
        ADPLL_FREF_DIV2_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adpll_lms_ext_value_en(&mut self) -> ADPLL_LMS_EXT_VALUE_EN_W {
        ADPLL_LMS_EXT_VALUE_EN_W { w: self }
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn adpll_lms_ext_value(&mut self) -> ADPLL_LMS_EXT_VALUE_W {
        ADPLL_LMS_EXT_VALUE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en(&mut self) -> ADPLL_SDM_DITHER_EN_W {
        ADPLL_SDM_DITHER_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en_ctrl_hw(&mut self) -> ADPLL_SDM_DITHER_EN_CTRL_HW_W {
        ADPLL_SDM_DITHER_EN_CTRL_HW_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adpll_lms_step(&mut self) -> ADPLL_LMS_STEP_W {
        ADPLL_LMS_STEP_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adpll_sdm_dither_prbs_en(&mut self) -> ADPLL_SDM_DITHER_PRBS_EN_W {
        ADPLL_SDM_DITHER_PRBS_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_pha_dem_en(&mut self) -> ADPLL_PHA_DEM_EN_W {
        ADPLL_PHA_DEM_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_pha_dither_en(&mut self) -> ADPLL_PHA_DITHER_EN_W {
        ADPLL_PHA_DITHER_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_lms_step_enlarge(&mut self) -> ADPLL_LMS_STEP_ENLARGE_W {
        ADPLL_LMS_STEP_ENLARGE_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adpll_pha_prbs_sel(&mut self) -> ADPLL_PHA_PRBS_SEL_W {
        ADPLL_PHA_PRBS_SEL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_lms_q_delay(&mut self) -> ADPLL_LMS_Q_DELAY_W {
        ADPLL_LMS_Q_DELAY_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adpll_pha_cancel_en(&mut self) -> ADPLL_PHA_CANCEL_EN_W {
        ADPLL_PHA_CANCEL_EN_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adpll_pha_cancel_delay(&mut self) -> ADPLL_PHA_CANCEL_DELAY_W {
        ADPLL_PHA_CANCEL_DELAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_lms.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_lms](index.html) module"]
pub struct ADPLL_LMS_SPEC;
impl crate::RegisterSpec for ADPLL_LMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_lms::R](R) reader structure"]
impl crate::Readable for ADPLL_LMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_lms::W](W) writer structure"]
impl crate::Writable for ADPLL_LMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_lms to value 0"]
impl crate::Resettable for ADPLL_LMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
