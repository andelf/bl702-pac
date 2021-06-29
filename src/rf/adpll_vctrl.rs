#[doc = "Register `adpll_vctrl` reader"]
pub struct R(crate::R<ADPLL_VCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_VCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_VCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_VCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_vctrl` writer"]
pub struct W(crate::W<ADPLL_VCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_VCTRL_SPEC>;
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
impl From<crate::W<ADPLL_VCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_VCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_vctrl_range_sel_ext_en` reader - "]
pub struct ADPLL_VCTRL_RANGE_SEL_EXT_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_VCTRL_RANGE_SEL_EXT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_VCTRL_RANGE_SEL_EXT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_VCTRL_RANGE_SEL_EXT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_vctrl_range_sel_ext_en` writer - "]
pub struct ADPLL_VCTRL_RANGE_SEL_EXT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_VCTRL_RANGE_SEL_EXT_EN_W<'a> {
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
#[doc = "Field `adpll_vctrl_lock_win_sel` reader - "]
pub struct ADPLL_VCTRL_LOCK_WIN_SEL_R(crate::FieldReader<bool, bool>);
impl ADPLL_VCTRL_LOCK_WIN_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_VCTRL_LOCK_WIN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_VCTRL_LOCK_WIN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_vctrl_lock_win_sel` writer - "]
pub struct ADPLL_VCTRL_LOCK_WIN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_VCTRL_LOCK_WIN_SEL_W<'a> {
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
#[doc = "Field `adpll_vctrl_moni_win_sel` reader - "]
pub struct ADPLL_VCTRL_MONI_WIN_SEL_R(crate::FieldReader<bool, bool>);
impl ADPLL_VCTRL_MONI_WIN_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_VCTRL_MONI_WIN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_VCTRL_MONI_WIN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_vctrl_moni_win_sel` writer - "]
pub struct ADPLL_VCTRL_MONI_WIN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_VCTRL_MONI_WIN_SEL_W<'a> {
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
#[doc = "Field `adpll_vctrl_det_cons_en` reader - "]
pub struct ADPLL_VCTRL_DET_CONS_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_VCTRL_DET_CONS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_VCTRL_DET_CONS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_VCTRL_DET_CONS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_vctrl_det_cons_en` writer - "]
pub struct ADPLL_VCTRL_DET_CONS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_VCTRL_DET_CONS_EN_W<'a> {
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
#[doc = "Field `adpll_mom_update_period` reader - "]
pub struct ADPLL_MOM_UPDATE_PERIOD_R(crate::FieldReader<u8, u8>);
impl ADPLL_MOM_UPDATE_PERIOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_MOM_UPDATE_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_MOM_UPDATE_PERIOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_mom_update_period` writer - "]
pub struct ADPLL_MOM_UPDATE_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_MOM_UPDATE_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `adpll_force_mom_hold` reader - "]
pub struct ADPLL_FORCE_MOM_HOLD_R(crate::FieldReader<bool, bool>);
impl ADPLL_FORCE_MOM_HOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_FORCE_MOM_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_FORCE_MOM_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_force_mom_hold` writer - "]
pub struct ADPLL_FORCE_MOM_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_FORCE_MOM_HOLD_W<'a> {
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
#[doc = "Field `adpll_dco_mash_bypass` reader - "]
pub struct ADPLL_DCO_MASH_BYPASS_R(crate::FieldReader<bool, bool>);
impl ADPLL_DCO_MASH_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_DCO_MASH_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_DCO_MASH_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_dco_mash_bypass` writer - "]
pub struct ADPLL_DCO_MASH_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_DCO_MASH_BYPASS_W<'a> {
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
#[doc = "Field `adpll_capcode_bypass` reader - "]
pub struct ADPLL_CAPCODE_BYPASS_R(crate::FieldReader<bool, bool>);
impl ADPLL_CAPCODE_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_CAPCODE_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_CAPCODE_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_capcode_bypass` writer - "]
pub struct ADPLL_CAPCODE_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_CAPCODE_BYPASS_W<'a> {
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
#[doc = "Field `sdm_order` reader - "]
pub struct SDM_ORDER_R(crate::FieldReader<bool, bool>);
impl SDM_ORDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDM_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDM_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdm_order` writer - "]
pub struct SDM_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> SDM_ORDER_W<'a> {
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
#[doc = "Field `sdm_dither` reader - "]
pub struct SDM_DITHER_R(crate::FieldReader<u8, u8>);
impl SDM_DITHER_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDM_DITHER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDM_DITHER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdm_dither` writer - "]
pub struct SDM_DITHER_W<'a> {
    w: &'a mut W,
}
impl<'a> SDM_DITHER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `sdm_bypass` reader - "]
pub struct SDM_BYPASS_R(crate::FieldReader<bool, bool>);
impl SDM_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDM_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDM_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdm_bypass` writer - "]
pub struct SDM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> SDM_BYPASS_W<'a> {
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
#[doc = "Field `sdmout_dly_sel` reader - "]
pub struct SDMOUT_DLY_SEL_R(crate::FieldReader<u8, u8>);
impl SDMOUT_DLY_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDMOUT_DLY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDMOUT_DLY_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdmout_dly_sel` writer - "]
pub struct SDMOUT_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMOUT_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_vctrl_range_sel_ext_en(&self) -> ADPLL_VCTRL_RANGE_SEL_EXT_EN_R {
        ADPLL_VCTRL_RANGE_SEL_EXT_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn adpll_vctrl_lock_win_sel(&self) -> ADPLL_VCTRL_LOCK_WIN_SEL_R {
        ADPLL_VCTRL_LOCK_WIN_SEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adpll_vctrl_moni_win_sel(&self) -> ADPLL_VCTRL_MONI_WIN_SEL_R {
        ADPLL_VCTRL_MONI_WIN_SEL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_vctrl_det_cons_en(&self) -> ADPLL_VCTRL_DET_CONS_EN_R {
        ADPLL_VCTRL_DET_CONS_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_mom_update_period(&self) -> ADPLL_MOM_UPDATE_PERIOD_R {
        ADPLL_MOM_UPDATE_PERIOD_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adpll_force_mom_hold(&self) -> ADPLL_FORCE_MOM_HOLD_R {
        ADPLL_FORCE_MOM_HOLD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adpll_dco_mash_bypass(&self) -> ADPLL_DCO_MASH_BYPASS_R {
        ADPLL_DCO_MASH_BYPASS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_capcode_bypass(&self) -> ADPLL_CAPCODE_BYPASS_R {
        ADPLL_CAPCODE_BYPASS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sdm_order(&self) -> SDM_ORDER_R {
        SDM_ORDER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sdm_dither(&self) -> SDM_DITHER_R {
        SDM_DITHER_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sdm_bypass(&self) -> SDM_BYPASS_R {
        SDM_BYPASS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sdmout_dly_sel(&self) -> SDMOUT_DLY_SEL_R {
        SDMOUT_DLY_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_vctrl_range_sel_ext_en(&mut self) -> ADPLL_VCTRL_RANGE_SEL_EXT_EN_W {
        ADPLL_VCTRL_RANGE_SEL_EXT_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn adpll_vctrl_lock_win_sel(&mut self) -> ADPLL_VCTRL_LOCK_WIN_SEL_W {
        ADPLL_VCTRL_LOCK_WIN_SEL_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adpll_vctrl_moni_win_sel(&mut self) -> ADPLL_VCTRL_MONI_WIN_SEL_W {
        ADPLL_VCTRL_MONI_WIN_SEL_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_vctrl_det_cons_en(&mut self) -> ADPLL_VCTRL_DET_CONS_EN_W {
        ADPLL_VCTRL_DET_CONS_EN_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_mom_update_period(&mut self) -> ADPLL_MOM_UPDATE_PERIOD_W {
        ADPLL_MOM_UPDATE_PERIOD_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adpll_force_mom_hold(&mut self) -> ADPLL_FORCE_MOM_HOLD_W {
        ADPLL_FORCE_MOM_HOLD_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adpll_dco_mash_bypass(&mut self) -> ADPLL_DCO_MASH_BYPASS_W {
        ADPLL_DCO_MASH_BYPASS_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_capcode_bypass(&mut self) -> ADPLL_CAPCODE_BYPASS_W {
        ADPLL_CAPCODE_BYPASS_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sdm_order(&mut self) -> SDM_ORDER_W {
        SDM_ORDER_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sdm_dither(&mut self) -> SDM_DITHER_W {
        SDM_DITHER_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sdm_bypass(&mut self) -> SDM_BYPASS_W {
        SDM_BYPASS_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sdmout_dly_sel(&mut self) -> SDMOUT_DLY_SEL_W {
        SDMOUT_DLY_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_vctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_vctrl](index.html) module"]
pub struct ADPLL_VCTRL_SPEC;
impl crate::RegisterSpec for ADPLL_VCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_vctrl::R](R) reader structure"]
impl crate::Readable for ADPLL_VCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_vctrl::W](W) writer structure"]
impl crate::Writable for ADPLL_VCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_vctrl to value 0"]
impl crate::Resettable for ADPLL_VCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
