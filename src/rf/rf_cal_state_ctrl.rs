#[doc = "Register `rf_cal_state_ctrl` reader"]
pub struct R(crate::R<RF_CAL_STATE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_CAL_STATE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_CAL_STATE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_CAL_STATE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_cal_state_ctrl` writer"]
pub struct W(crate::W<RF_CAL_STATE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_CAL_STATE_CTRL_SPEC>;
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
impl From<crate::W<RF_CAL_STATE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_CAL_STATE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `inc_roscal_state_en` reader - "]
pub struct INC_ROSCAL_STATE_EN_R(crate::FieldReader<bool, bool>);
impl INC_ROSCAL_STATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_ROSCAL_STATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_ROSCAL_STATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_roscal_state_en` writer - "]
pub struct INC_ROSCAL_STATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_ROSCAL_STATE_EN_W<'a> {
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
#[doc = "Field `inc_acal_state_en` reader - "]
pub struct INC_ACAL_STATE_EN_R(crate::FieldReader<bool, bool>);
impl INC_ACAL_STATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_ACAL_STATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_ACAL_STATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_acal_state_en` writer - "]
pub struct INC_ACAL_STATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_ACAL_STATE_EN_W<'a> {
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
#[doc = "Field `inc_fcal_state_en` reader - "]
pub struct INC_FCAL_STATE_EN_R(crate::FieldReader<bool, bool>);
impl INC_FCAL_STATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_FCAL_STATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_FCAL_STATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_fcal_state_en` writer - "]
pub struct INC_FCAL_STATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_FCAL_STATE_EN_W<'a> {
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
#[doc = "Field `rccal_state_en` reader - "]
pub struct RCCAL_STATE_EN_R(crate::FieldReader<bool, bool>);
impl RCCAL_STATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCCAL_STATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCAL_STATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rccal_state_en` writer - "]
pub struct RCCAL_STATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCAL_STATE_EN_W<'a> {
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
#[doc = "Field `roscal_state_en` reader - "]
pub struct ROSCAL_STATE_EN_R(crate::FieldReader<bool, bool>);
impl ROSCAL_STATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROSCAL_STATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSCAL_STATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `roscal_state_en` writer - "]
pub struct ROSCAL_STATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSCAL_STATE_EN_W<'a> {
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
#[doc = "Field `kcal_state_en` reader - "]
pub struct KCAL_STATE_EN_R(crate::FieldReader<bool, bool>);
impl KCAL_STATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        KCAL_STATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_STATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_state_en` writer - "]
pub struct KCAL_STATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_STATE_EN_W<'a> {
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
#[doc = "Field `acal_state_en` reader - "]
pub struct ACAL_STATE_EN_R(crate::FieldReader<bool, bool>);
impl ACAL_STATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACAL_STATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACAL_STATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acal_state_en` writer - "]
pub struct ACAL_STATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_STATE_EN_W<'a> {
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
#[doc = "Field `fcal_state_en` reader - "]
pub struct FCAL_STATE_EN_R(crate::FieldReader<bool, bool>);
impl FCAL_STATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCAL_STATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_STATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_state_en` writer - "]
pub struct FCAL_STATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_STATE_EN_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inc_roscal_state_en(&self) -> INC_ROSCAL_STATE_EN_R {
        INC_ROSCAL_STATE_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inc_acal_state_en(&self) -> INC_ACAL_STATE_EN_R {
        INC_ACAL_STATE_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inc_fcal_state_en(&self) -> INC_FCAL_STATE_EN_R {
        INC_FCAL_STATE_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rccal_state_en(&self) -> RCCAL_STATE_EN_R {
        RCCAL_STATE_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn roscal_state_en(&self) -> ROSCAL_STATE_EN_R {
        ROSCAL_STATE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn kcal_state_en(&self) -> KCAL_STATE_EN_R {
        KCAL_STATE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acal_state_en(&self) -> ACAL_STATE_EN_R {
        ACAL_STATE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fcal_state_en(&self) -> FCAL_STATE_EN_R {
        FCAL_STATE_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inc_roscal_state_en(&mut self) -> INC_ROSCAL_STATE_EN_W {
        INC_ROSCAL_STATE_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inc_acal_state_en(&mut self) -> INC_ACAL_STATE_EN_W {
        INC_ACAL_STATE_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inc_fcal_state_en(&mut self) -> INC_FCAL_STATE_EN_W {
        INC_FCAL_STATE_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rccal_state_en(&mut self) -> RCCAL_STATE_EN_W {
        RCCAL_STATE_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn roscal_state_en(&mut self) -> ROSCAL_STATE_EN_W {
        ROSCAL_STATE_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn kcal_state_en(&mut self) -> KCAL_STATE_EN_W {
        KCAL_STATE_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acal_state_en(&mut self) -> ACAL_STATE_EN_W {
        ACAL_STATE_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fcal_state_en(&mut self) -> FCAL_STATE_EN_W {
        FCAL_STATE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf calibration state enable in full cal list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_cal_state_ctrl](index.html) module"]
pub struct RF_CAL_STATE_CTRL_SPEC;
impl crate::RegisterSpec for RF_CAL_STATE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_cal_state_ctrl::R](R) reader structure"]
impl crate::Readable for RF_CAL_STATE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_cal_state_ctrl::W](W) writer structure"]
impl crate::Writable for RF_CAL_STATE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_cal_state_ctrl to value 0"]
impl crate::Resettable for RF_CAL_STATE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
