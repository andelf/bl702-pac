#[doc = "Register `rf_test_mode` reader"]
pub struct R(crate::R<RF_TEST_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_TEST_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_TEST_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_TEST_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_test_mode` writer"]
pub struct W(crate::W<RF_TEST_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_TEST_MODE_SPEC>;
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
impl From<crate::W<RF_TEST_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_TEST_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_test_mode_en` reader - "]
pub struct RF_TEST_MODE_EN_R(crate::FieldReader<bool, bool>);
impl RF_TEST_MODE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_TEST_MODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TEST_MODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_test_mode_en` writer - "]
pub struct RF_TEST_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TEST_MODE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `dacout_4s_sram_en` reader - "]
pub struct DACOUT_4S_SRAM_EN_R(crate::FieldReader<bool, bool>);
impl DACOUT_4S_SRAM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DACOUT_4S_SRAM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACOUT_4S_SRAM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dacout_4s_sram_en` writer - "]
pub struct DACOUT_4S_SRAM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACOUT_4S_SRAM_EN_W<'a> {
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
#[doc = "Field `dacout_4s_en` reader - "]
pub struct DACOUT_4S_EN_R(crate::FieldReader<bool, bool>);
impl DACOUT_4S_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DACOUT_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACOUT_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dacout_4s_en` writer - "]
pub struct DACOUT_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACOUT_4S_EN_W<'a> {
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
#[doc = "Field `dacout_4s` reader - "]
pub struct DACOUT_4S_R(crate::FieldReader<u16, u16>);
impl DACOUT_4S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DACOUT_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACOUT_4S_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dacout_4s` writer - "]
pub struct DACOUT_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> DACOUT_4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `dacout_hw` reader - "]
pub struct DACOUT_HW_R(crate::FieldReader<u16, u16>);
impl DACOUT_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DACOUT_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DACOUT_HW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dacout_hw` writer - "]
pub struct DACOUT_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> DACOUT_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_test_mode_en(&self) -> RF_TEST_MODE_EN_R {
        RF_TEST_MODE_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dacout_4s_sram_en(&self) -> DACOUT_4S_SRAM_EN_R {
        DACOUT_4S_SRAM_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dacout_4s_en(&self) -> DACOUT_4S_EN_R {
        DACOUT_4S_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn dacout_4s(&self) -> DACOUT_4S_R {
        DACOUT_4S_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dacout_hw(&self) -> DACOUT_HW_R {
        DACOUT_HW_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_test_mode_en(&mut self) -> RF_TEST_MODE_EN_W {
        RF_TEST_MODE_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dacout_4s_sram_en(&mut self) -> DACOUT_4S_SRAM_EN_W {
        DACOUT_4S_SRAM_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dacout_4s_en(&mut self) -> DACOUT_4S_EN_W {
        DACOUT_4S_EN_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn dacout_4s(&mut self) -> DACOUT_4S_W {
        DACOUT_4S_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dacout_hw(&mut self) -> DACOUT_HW_W {
        DACOUT_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_test_mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_test_mode](index.html) module"]
pub struct RF_TEST_MODE_SPEC;
impl crate::RegisterSpec for RF_TEST_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_test_mode::R](R) reader structure"]
impl crate::Readable for RF_TEST_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_test_mode::W](W) writer structure"]
impl crate::Writable for RF_TEST_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_test_mode to value 0"]
impl crate::Resettable for RF_TEST_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
