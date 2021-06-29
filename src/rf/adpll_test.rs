#[doc = "Register `adpll_test` reader"]
pub struct R(crate::R<ADPLL_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_test` writer"]
pub struct W(crate::W<ADPLL_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_TEST_SPEC>;
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
impl From<crate::W<ADPLL_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_test_start` reader - "]
pub struct ADPLL_TEST_START_R(crate::FieldReader<bool, bool>);
impl ADPLL_TEST_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_TEST_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_TEST_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_test_start` writer - "]
pub struct ADPLL_TEST_START_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_TEST_START_W<'a> {
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
#[doc = "Field `adpll_test_en` reader - "]
pub struct ADPLL_TEST_EN_R(crate::FieldReader<bool, bool>);
impl ADPLL_TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_test_en` writer - "]
pub struct ADPLL_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_TEST_EN_W<'a> {
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
#[doc = "Field `adpll_test_start_sel` reader - "]
pub struct ADPLL_TEST_START_SEL_R(crate::FieldReader<u8, u8>);
impl ADPLL_TEST_START_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_TEST_START_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_TEST_START_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_test_start_sel` writer - "]
pub struct ADPLL_TEST_START_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_TEST_START_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `adpll_test_data_sel` reader - "]
pub struct ADPLL_TEST_DATA_SEL_R(crate::FieldReader<u8, u8>);
impl ADPLL_TEST_DATA_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_TEST_DATA_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_TEST_DATA_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_test_data_sel` writer - "]
pub struct ADPLL_TEST_DATA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_TEST_DATA_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `adpll_test_out` reader - "]
pub struct ADPLL_TEST_OUT_R(crate::FieldReader<u16, u16>);
impl ADPLL_TEST_OUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADPLL_TEST_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_TEST_OUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_test_out` writer - "]
pub struct ADPLL_TEST_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_TEST_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adpll_test_start(&self) -> ADPLL_TEST_START_R {
        ADPLL_TEST_START_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_test_en(&self) -> ADPLL_TEST_EN_R {
        ADPLL_TEST_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_test_start_sel(&self) -> ADPLL_TEST_START_SEL_R {
        ADPLL_TEST_START_SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn adpll_test_data_sel(&self) -> ADPLL_TEST_DATA_SEL_R {
        ADPLL_TEST_DATA_SEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn adpll_test_out(&self) -> ADPLL_TEST_OUT_R {
        ADPLL_TEST_OUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adpll_test_start(&mut self) -> ADPLL_TEST_START_W {
        ADPLL_TEST_START_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_test_en(&mut self) -> ADPLL_TEST_EN_W {
        ADPLL_TEST_EN_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_test_start_sel(&mut self) -> ADPLL_TEST_START_SEL_W {
        ADPLL_TEST_START_SEL_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn adpll_test_data_sel(&mut self) -> ADPLL_TEST_DATA_SEL_W {
        ADPLL_TEST_DATA_SEL_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn adpll_test_out(&mut self) -> ADPLL_TEST_OUT_W {
        ADPLL_TEST_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_test.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_test](index.html) module"]
pub struct ADPLL_TEST_SPEC;
impl crate::RegisterSpec for ADPLL_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_test::R](R) reader structure"]
impl crate::Readable for ADPLL_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_test::W](W) writer structure"]
impl crate::Writable for ADPLL_TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_test to value 0"]
impl crate::Resettable for ADPLL_TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
