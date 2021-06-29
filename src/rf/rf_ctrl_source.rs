#[doc = "Register `rf_ctrl_source` reader"]
pub struct R(crate::R<RF_CTRL_SOURCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_CTRL_SOURCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_CTRL_SOURCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_CTRL_SOURCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_ctrl_source` writer"]
pub struct W(crate::W<RF_CTRL_SOURCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_CTRL_SOURCE_SPEC>;
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
impl From<crate::W<RF_CTRL_SOURCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_CTRL_SOURCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vco_idac_ctrl_hw` reader - "]
pub struct VCO_IDAC_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl VCO_IDAC_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCO_IDAC_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_IDAC_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_idac_ctrl_hw` writer - "]
pub struct VCO_IDAC_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_IDAC_CTRL_HW_W<'a> {
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
#[doc = "Field `inc_fcal_en_ctrl_hw` reader - "]
pub struct INC_FCAL_EN_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl INC_FCAL_EN_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_FCAL_EN_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_FCAL_EN_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_fcal_en_ctrl_hw` writer - "]
pub struct INC_FCAL_EN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_FCAL_EN_CTRL_HW_W<'a> {
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
#[doc = "Field `lo_fcw_ctrl_hw` reader - "]
pub struct LO_FCW_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl LO_FCW_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_FCW_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_FCW_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_fcw_ctrl_hw` writer - "]
pub struct LO_FCW_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FCW_CTRL_HW_W<'a> {
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
#[doc = "Field `rbb_bw_ctrl_hw` reader - "]
pub struct RBB_BW_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl RBB_BW_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_BW_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BW_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bw_ctrl_hw` writer - "]
pub struct RBB_BW_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BW_CTRL_HW_W<'a> {
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
#[doc = "Field `kcal_ratio_ctrl_hw` reader - "]
pub struct KCAL_RATIO_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl KCAL_RATIO_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        KCAL_RATIO_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_RATIO_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_ratio_ctrl_hw` writer - "]
pub struct KCAL_RATIO_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_RATIO_CTRL_HW_W<'a> {
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
#[doc = "Field `rosdac_ctrl_rccal` reader - "]
pub struct ROSDAC_CTRL_RCCAL_R(crate::FieldReader<bool, bool>);
impl ROSDAC_CTRL_RCCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROSDAC_CTRL_RCCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_CTRL_RCCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_ctrl_rccal` writer - "]
pub struct ROSDAC_CTRL_RCCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_CTRL_RCCAL_W<'a> {
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
#[doc = "Field `rosdac_ctrl_hw` reader - "]
pub struct ROSDAC_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl ROSDAC_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROSDAC_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_ctrl_hw` writer - "]
pub struct ROSDAC_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_CTRL_HW_W<'a> {
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
#[doc = "Field `gain_ctrl_rx_hw` reader - "]
pub struct GAIN_CTRL_RX_HW_R(crate::FieldReader<bool, bool>);
impl GAIN_CTRL_RX_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GAIN_CTRL_RX_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL_RX_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl_rx_hw` writer - "]
pub struct GAIN_CTRL_RX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL_RX_HW_W<'a> {
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
#[doc = "Field `gain_ctrl_tx_hw` reader - "]
pub struct GAIN_CTRL_TX_HW_R(crate::FieldReader<bool, bool>);
impl GAIN_CTRL_TX_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GAIN_CTRL_TX_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL_TX_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl_tx_hw` writer - "]
pub struct GAIN_CTRL_TX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL_TX_HW_W<'a> {
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
#[doc = "Field `pu_ctrl_hw` reader - "]
pub struct PU_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl PU_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_ctrl_hw` writer - "]
pub struct PU_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_CTRL_HW_W<'a> {
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
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn vco_idac_ctrl_hw(&self) -> VCO_IDAC_CTRL_HW_R {
        VCO_IDAC_CTRL_HW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn inc_fcal_en_ctrl_hw(&self) -> INC_FCAL_EN_CTRL_HW_R {
        INC_FCAL_EN_CTRL_HW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_fcw_ctrl_hw(&self) -> LO_FCW_CTRL_HW_R {
        LO_FCW_CTRL_HW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rbb_bw_ctrl_hw(&self) -> RBB_BW_CTRL_HW_R {
        RBB_BW_CTRL_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn kcal_ratio_ctrl_hw(&self) -> KCAL_RATIO_CTRL_HW_R {
        KCAL_RATIO_CTRL_HW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rosdac_ctrl_rccal(&self) -> ROSDAC_CTRL_RCCAL_R {
        ROSDAC_CTRL_RCCAL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rosdac_ctrl_hw(&self) -> ROSDAC_CTRL_HW_R {
        ROSDAC_CTRL_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gain_ctrl_rx_hw(&self) -> GAIN_CTRL_RX_HW_R {
        GAIN_CTRL_RX_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gain_ctrl_tx_hw(&self) -> GAIN_CTRL_TX_HW_R {
        GAIN_CTRL_TX_HW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ctrl_hw(&self) -> PU_CTRL_HW_R {
        PU_CTRL_HW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn vco_idac_ctrl_hw(&mut self) -> VCO_IDAC_CTRL_HW_W {
        VCO_IDAC_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn inc_fcal_en_ctrl_hw(&mut self) -> INC_FCAL_EN_CTRL_HW_W {
        INC_FCAL_EN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_fcw_ctrl_hw(&mut self) -> LO_FCW_CTRL_HW_W {
        LO_FCW_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rbb_bw_ctrl_hw(&mut self) -> RBB_BW_CTRL_HW_W {
        RBB_BW_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn kcal_ratio_ctrl_hw(&mut self) -> KCAL_RATIO_CTRL_HW_W {
        KCAL_RATIO_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rosdac_ctrl_rccal(&mut self) -> ROSDAC_CTRL_RCCAL_W {
        ROSDAC_CTRL_RCCAL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rosdac_ctrl_hw(&mut self) -> ROSDAC_CTRL_HW_W {
        ROSDAC_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gain_ctrl_rx_hw(&mut self) -> GAIN_CTRL_RX_HW_W {
        GAIN_CTRL_RX_HW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gain_ctrl_tx_hw(&mut self) -> GAIN_CTRL_TX_HW_W {
        GAIN_CTRL_TX_HW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ctrl_hw(&mut self) -> PU_CTRL_HW_W {
        PU_CTRL_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control logic switch\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ctrl_source](index.html) module"]
pub struct RF_CTRL_SOURCE_SPEC;
impl crate::RegisterSpec for RF_CTRL_SOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ctrl_source::R](R) reader structure"]
impl crate::Readable for RF_CTRL_SOURCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ctrl_source::W](W) writer structure"]
impl crate::Writable for RF_CTRL_SOURCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_ctrl_source to value 0"]
impl crate::Resettable for RF_CTRL_SOURCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
