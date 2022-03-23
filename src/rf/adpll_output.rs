#[doc = "Register `adpll_output` reader"]
pub struct R(crate::R<ADPLL_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_output` writer"]
pub struct W(crate::W<ADPLL_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_OUTPUT_SPEC>;
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
impl From<crate::W<ADPLL_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_freqerr_det_done` reader - "]
pub struct ADPLL_FREQERR_DET_DONE_R(crate::FieldReader<bool, bool>);
impl ADPLL_FREQERR_DET_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_FREQERR_DET_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_FREQERR_DET_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_freqerr_det_done` writer - "]
pub struct ADPLL_FREQERR_DET_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_FREQERR_DET_DONE_W<'a> {
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
#[doc = "Field `adpll_freqerr_ou` reader - "]
pub struct ADPLL_FREQERR_OU_R(crate::FieldReader<bool, bool>);
impl ADPLL_FREQERR_OU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_FREQERR_OU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_FREQERR_OU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_freqerr_ou` writer - "]
pub struct ADPLL_FREQERR_OU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_FREQERR_OU_W<'a> {
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
#[doc = "Field `adpll_freqerr_sign` reader - "]
pub struct ADPLL_FREQERR_SIGN_R(crate::FieldReader<bool, bool>);
impl ADPLL_FREQERR_SIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_FREQERR_SIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_FREQERR_SIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_freqerr_sign` writer - "]
pub struct ADPLL_FREQERR_SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_FREQERR_SIGN_W<'a> {
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
#[doc = "Field `adpll_vctrl_det_done` reader - "]
pub struct ADPLL_VCTRL_DET_DONE_R(crate::FieldReader<bool, bool>);
impl ADPLL_VCTRL_DET_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_VCTRL_DET_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_VCTRL_DET_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_vctrl_det_done` writer - "]
pub struct ADPLL_VCTRL_DET_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_VCTRL_DET_DONE_W<'a> {
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
#[doc = "Field `adpll_capcode_ud` reader - "]
pub struct ADPLL_CAPCODE_UD_R(crate::FieldReader<bool, bool>);
impl ADPLL_CAPCODE_UD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_CAPCODE_UD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_CAPCODE_UD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_capcode_ud` writer - "]
pub struct ADPLL_CAPCODE_UD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_CAPCODE_UD_W<'a> {
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
#[doc = "Field `adpll_mom_update_total_ou` reader - "]
pub struct ADPLL_MOM_UPDATE_TOTAL_OU_R(crate::FieldReader<u8, u8>);
impl ADPLL_MOM_UPDATE_TOTAL_OU_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_MOM_UPDATE_TOTAL_OU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_MOM_UPDATE_TOTAL_OU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_mom_update_total_ou` writer - "]
pub struct ADPLL_MOM_UPDATE_TOTAL_OU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_MOM_UPDATE_TOTAL_OU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "Field `adpll_capcode_out_range` reader - "]
pub struct ADPLL_CAPCODE_OUT_RANGE_R(crate::FieldReader<bool, bool>);
impl ADPLL_CAPCODE_OUT_RANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_CAPCODE_OUT_RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_CAPCODE_OUT_RANGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_capcode_out_range` writer - "]
pub struct ADPLL_CAPCODE_OUT_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_CAPCODE_OUT_RANGE_W<'a> {
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
#[doc = "Field `adpll_fcal_done_fsm` reader - "]
pub struct ADPLL_FCAL_DONE_FSM_R(crate::FieldReader<bool, bool>);
impl ADPLL_FCAL_DONE_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_FCAL_DONE_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_FCAL_DONE_FSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_fcal_done_fsm` writer - "]
pub struct ADPLL_FCAL_DONE_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_FCAL_DONE_FSM_W<'a> {
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
#[doc = "Field `adpll_spd_lock_fsm` reader - "]
pub struct ADPLL_SPD_LOCK_FSM_R(crate::FieldReader<bool, bool>);
impl ADPLL_SPD_LOCK_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_SPD_LOCK_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SPD_LOCK_FSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_spd_lock_fsm` writer - "]
pub struct ADPLL_SPD_LOCK_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SPD_LOCK_FSM_W<'a> {
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
#[doc = "Field `adpll_spd_unlock_fsm` reader - "]
pub struct ADPLL_SPD_UNLOCK_FSM_R(crate::FieldReader<bool, bool>);
impl ADPLL_SPD_UNLOCK_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_SPD_UNLOCK_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SPD_UNLOCK_FSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_spd_unlock_fsm` writer - "]
pub struct ADPLL_SPD_UNLOCK_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SPD_UNLOCK_FSM_W<'a> {
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
#[doc = "Field `adpll_mom_update_ou_fsm` reader - "]
pub struct ADPLL_MOM_UPDATE_OU_FSM_R(crate::FieldReader<bool, bool>);
impl ADPLL_MOM_UPDATE_OU_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_MOM_UPDATE_OU_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_MOM_UPDATE_OU_FSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_mom_update_ou_fsm` writer - "]
pub struct ADPLL_MOM_UPDATE_OU_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_MOM_UPDATE_OU_FSM_W<'a> {
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
#[doc = "Field `adpll_mom_update_fail_fsm` reader - "]
pub struct ADPLL_MOM_UPDATE_FAIL_FSM_R(crate::FieldReader<bool, bool>);
impl ADPLL_MOM_UPDATE_FAIL_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_MOM_UPDATE_FAIL_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_MOM_UPDATE_FAIL_FSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_mom_update_fail_fsm` writer - "]
pub struct ADPLL_MOM_UPDATE_FAIL_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_MOM_UPDATE_FAIL_FSM_W<'a> {
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
#[doc = "Field `adpll_vctrl_out_range_fsm` reader - "]
pub struct ADPLL_VCTRL_OUT_RANGE_FSM_R(crate::FieldReader<bool, bool>);
impl ADPLL_VCTRL_OUT_RANGE_FSM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_VCTRL_OUT_RANGE_FSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_VCTRL_OUT_RANGE_FSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_vctrl_out_range_fsm` writer - "]
pub struct ADPLL_VCTRL_OUT_RANGE_FSM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_VCTRL_OUT_RANGE_FSM_W<'a> {
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
#[doc = "Field `adpll_spd_unlock_sign` reader - "]
pub struct ADPLL_SPD_UNLOCK_SIGN_R(crate::FieldReader<bool, bool>);
impl ADPLL_SPD_UNLOCK_SIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_SPD_UNLOCK_SIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SPD_UNLOCK_SIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_spd_unlock_sign` writer - "]
pub struct ADPLL_SPD_UNLOCK_SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SPD_UNLOCK_SIGN_W<'a> {
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
#[doc = "Field `adpll_fsm_state` reader - "]
pub struct ADPLL_FSM_STATE_R(crate::FieldReader<u8, u8>);
impl ADPLL_FSM_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_FSM_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_FSM_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_fsm_state` writer - "]
pub struct ADPLL_FSM_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_FSM_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | ((value as u32 & 0x0f) << 3);
        self.w
    }
}
#[doc = "Field `adpll_lo_lock` reader - "]
pub struct ADPLL_LO_LOCK_R(crate::FieldReader<bool, bool>);
impl ADPLL_LO_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_LO_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LO_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lo_lock` writer - "]
pub struct ADPLL_LO_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LO_LOCK_W<'a> {
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
#[doc = "Field `adpll_unlock_intrpt` reader - "]
pub struct ADPLL_UNLOCK_INTRPT_R(crate::FieldReader<bool, bool>);
impl ADPLL_UNLOCK_INTRPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_UNLOCK_INTRPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_UNLOCK_INTRPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_unlock_intrpt` writer - "]
pub struct ADPLL_UNLOCK_INTRPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_UNLOCK_INTRPT_W<'a> {
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
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adpll_freqerr_det_done(&self) -> ADPLL_FREQERR_DET_DONE_R {
        ADPLL_FREQERR_DET_DONE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adpll_freqerr_ou(&self) -> ADPLL_FREQERR_OU_R {
        ADPLL_FREQERR_OU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adpll_freqerr_sign(&self) -> ADPLL_FREQERR_SIGN_R {
        ADPLL_FREQERR_SIGN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adpll_vctrl_det_done(&self) -> ADPLL_VCTRL_DET_DONE_R {
        ADPLL_VCTRL_DET_DONE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adpll_capcode_ud(&self) -> ADPLL_CAPCODE_UD_R {
        ADPLL_CAPCODE_UD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn adpll_mom_update_total_ou(&self) -> ADPLL_MOM_UPDATE_TOTAL_OU_R {
        ADPLL_MOM_UPDATE_TOTAL_OU_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_capcode_out_range(&self) -> ADPLL_CAPCODE_OUT_RANGE_R {
        ADPLL_CAPCODE_OUT_RANGE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_fcal_done_fsm(&self) -> ADPLL_FCAL_DONE_FSM_R {
        ADPLL_FCAL_DONE_FSM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_spd_lock_fsm(&self) -> ADPLL_SPD_LOCK_FSM_R {
        ADPLL_SPD_LOCK_FSM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn adpll_spd_unlock_fsm(&self) -> ADPLL_SPD_UNLOCK_FSM_R {
        ADPLL_SPD_UNLOCK_FSM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adpll_mom_update_ou_fsm(&self) -> ADPLL_MOM_UPDATE_OU_FSM_R {
        ADPLL_MOM_UPDATE_OU_FSM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adpll_mom_update_fail_fsm(&self) -> ADPLL_MOM_UPDATE_FAIL_FSM_R {
        ADPLL_MOM_UPDATE_FAIL_FSM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adpll_vctrl_out_range_fsm(&self) -> ADPLL_VCTRL_OUT_RANGE_FSM_R {
        ADPLL_VCTRL_OUT_RANGE_FSM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn adpll_spd_unlock_sign(&self) -> ADPLL_SPD_UNLOCK_SIGN_R {
        ADPLL_SPD_UNLOCK_SIGN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn adpll_fsm_state(&self) -> ADPLL_FSM_STATE_R {
        ADPLL_FSM_STATE_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adpll_lo_lock(&self) -> ADPLL_LO_LOCK_R {
        ADPLL_LO_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_unlock_intrpt(&self) -> ADPLL_UNLOCK_INTRPT_R {
        ADPLL_UNLOCK_INTRPT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adpll_freqerr_det_done(&mut self) -> ADPLL_FREQERR_DET_DONE_W {
        ADPLL_FREQERR_DET_DONE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adpll_freqerr_ou(&mut self) -> ADPLL_FREQERR_OU_W {
        ADPLL_FREQERR_OU_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adpll_freqerr_sign(&mut self) -> ADPLL_FREQERR_SIGN_W {
        ADPLL_FREQERR_SIGN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adpll_vctrl_det_done(&mut self) -> ADPLL_VCTRL_DET_DONE_W {
        ADPLL_VCTRL_DET_DONE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adpll_capcode_ud(&mut self) -> ADPLL_CAPCODE_UD_W {
        ADPLL_CAPCODE_UD_W { w: self }
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn adpll_mom_update_total_ou(&mut self) -> ADPLL_MOM_UPDATE_TOTAL_OU_W {
        ADPLL_MOM_UPDATE_TOTAL_OU_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_capcode_out_range(&mut self) -> ADPLL_CAPCODE_OUT_RANGE_W {
        ADPLL_CAPCODE_OUT_RANGE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_fcal_done_fsm(&mut self) -> ADPLL_FCAL_DONE_FSM_W {
        ADPLL_FCAL_DONE_FSM_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_spd_lock_fsm(&mut self) -> ADPLL_SPD_LOCK_FSM_W {
        ADPLL_SPD_LOCK_FSM_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn adpll_spd_unlock_fsm(&mut self) -> ADPLL_SPD_UNLOCK_FSM_W {
        ADPLL_SPD_UNLOCK_FSM_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adpll_mom_update_ou_fsm(&mut self) -> ADPLL_MOM_UPDATE_OU_FSM_W {
        ADPLL_MOM_UPDATE_OU_FSM_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adpll_mom_update_fail_fsm(&mut self) -> ADPLL_MOM_UPDATE_FAIL_FSM_W {
        ADPLL_MOM_UPDATE_FAIL_FSM_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adpll_vctrl_out_range_fsm(&mut self) -> ADPLL_VCTRL_OUT_RANGE_FSM_W {
        ADPLL_VCTRL_OUT_RANGE_FSM_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn adpll_spd_unlock_sign(&mut self) -> ADPLL_SPD_UNLOCK_SIGN_W {
        ADPLL_SPD_UNLOCK_SIGN_W { w: self }
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn adpll_fsm_state(&mut self) -> ADPLL_FSM_STATE_W {
        ADPLL_FSM_STATE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adpll_lo_lock(&mut self) -> ADPLL_LO_LOCK_W {
        ADPLL_LO_LOCK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_unlock_intrpt(&mut self) -> ADPLL_UNLOCK_INTRPT_W {
        ADPLL_UNLOCK_INTRPT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_output](index.html) module"]
pub struct ADPLL_OUTPUT_SPEC;
impl crate::RegisterSpec for ADPLL_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_output::R](R) reader structure"]
impl crate::Readable for ADPLL_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_output::W](W) writer structure"]
impl crate::Writable for ADPLL_OUTPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_output to value 0"]
impl crate::Resettable for ADPLL_OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
