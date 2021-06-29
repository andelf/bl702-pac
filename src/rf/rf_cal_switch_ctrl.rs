#[doc = "Register `rf_cal_switch_ctrl` reader"]
pub struct R(crate::R<RF_CAL_SWITCH_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_CAL_SWITCH_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_CAL_SWITCH_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_CAL_SWITCH_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_cal_switch_ctrl` writer"]
pub struct W(crate::W<RF_CAL_SWITCH_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_CAL_SWITCH_CTRL_SPEC>;
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
impl From<crate::W<RF_CAL_SWITCH_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_CAL_SWITCH_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `inc_fcal_en_hw` reader - "]
pub struct INC_FCAL_EN_HW_R(crate::FieldReader<bool, bool>);
impl INC_FCAL_EN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_FCAL_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_FCAL_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_fcal_en_hw` writer - "]
pub struct INC_FCAL_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_FCAL_EN_HW_W<'a> {
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
#[doc = "Field `inc_fcal_en` reader - "]
pub struct INC_FCAL_EN_R(crate::FieldReader<bool, bool>);
impl INC_FCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_FCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_FCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_fcal_en` writer - "]
pub struct INC_FCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_FCAL_EN_W<'a> {
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
#[doc = "Field `inc_acal_en` reader - "]
pub struct INC_ACAL_EN_R(crate::FieldReader<bool, bool>);
impl INC_ACAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_ACAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_ACAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_acal_en` writer - "]
pub struct INC_ACAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_ACAL_EN_W<'a> {
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
#[doc = "Field `rccal_en` reader - "]
pub struct RCCAL_EN_R(crate::FieldReader<bool, bool>);
impl RCCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rccal_en` writer - "]
pub struct RCCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCAL_EN_W<'a> {
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
#[doc = "Field `kcal_en` reader - "]
pub struct KCAL_EN_R(crate::FieldReader<bool, bool>);
impl KCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        KCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_en` writer - "]
pub struct KCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_EN_W<'a> {
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
#[doc = "Field `acal_en` reader - "]
pub struct ACAL_EN_R(crate::FieldReader<bool, bool>);
impl ACAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acal_en` writer - "]
pub struct ACAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_EN_W<'a> {
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
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn inc_fcal_en_hw(&self) -> INC_FCAL_EN_HW_R {
        INC_FCAL_EN_HW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_fcal_en(&self) -> INC_FCAL_EN_R {
        INC_FCAL_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn inc_acal_en(&self) -> INC_ACAL_EN_R {
        INC_ACAL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rccal_en(&self) -> RCCAL_EN_R {
        RCCAL_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn kcal_en(&self) -> KCAL_EN_R {
        KCAL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acal_en(&self) -> ACAL_EN_R {
        ACAL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn inc_fcal_en_hw(&mut self) -> INC_FCAL_EN_HW_W {
        INC_FCAL_EN_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_fcal_en(&mut self) -> INC_FCAL_EN_W {
        INC_FCAL_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn inc_acal_en(&mut self) -> INC_ACAL_EN_W {
        INC_ACAL_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rccal_en(&mut self) -> RCCAL_EN_W {
        RCCAL_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn kcal_en(&mut self) -> KCAL_EN_W {
        KCAL_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acal_en(&mut self) -> ACAL_EN_W {
        ACAL_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_cal_switch_ctrl](index.html) module"]
pub struct RF_CAL_SWITCH_CTRL_SPEC;
impl crate::RegisterSpec for RF_CAL_SWITCH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_cal_switch_ctrl::R](R) reader structure"]
impl crate::Readable for RF_CAL_SWITCH_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_cal_switch_ctrl::W](W) writer structure"]
impl crate::Writable for RF_CAL_SWITCH_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_cal_switch_ctrl to value 0"]
impl crate::Resettable for RF_CAL_SWITCH_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
