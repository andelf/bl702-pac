#[doc = "Register `testbuf` reader"]
pub struct R(crate::R<TESTBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TESTBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TESTBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TESTBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `testbuf` writer"]
pub struct W(crate::W<TESTBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TESTBUF_SPEC>;
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
impl From<crate::W<TESTBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TESTBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_testbuf` reader - "]
pub struct PU_TESTBUF_R(crate::FieldReader<bool, bool>);
impl PU_TESTBUF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_TESTBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_TESTBUF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_testbuf` writer - "]
pub struct PU_TESTBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TESTBUF_W<'a> {
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
#[doc = "Field `testbuf_vcm` reader - "]
pub struct TESTBUF_VCM_R(crate::FieldReader<u8, u8>);
impl TESTBUF_VCM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TESTBUF_VCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTBUF_VCM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `testbuf_vcm` writer - "]
pub struct TESTBUF_VCM_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTBUF_VCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `testbuf_bm` reader - "]
pub struct TESTBUF_BM_R(crate::FieldReader<u8, u8>);
impl TESTBUF_BM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TESTBUF_BM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTBUF_BM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `testbuf_bm` writer - "]
pub struct TESTBUF_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTBUF_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `testbuf_boost` reader - "]
pub struct TESTBUF_BOOST_R(crate::FieldReader<bool, bool>);
impl TESTBUF_BOOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TESTBUF_BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTBUF_BOOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `testbuf_boost` writer - "]
pub struct TESTBUF_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTBUF_BOOST_W<'a> {
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
#[doc = "Field `testbuf_op_cc` reader - "]
pub struct TESTBUF_OP_CC_R(crate::FieldReader<u8, u8>);
impl TESTBUF_OP_CC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TESTBUF_OP_CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTBUF_OP_CC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `testbuf_op_cc` writer - "]
pub struct TESTBUF_OP_CC_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTBUF_OP_CC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `testbuf_rfb` reader - "]
pub struct TESTBUF_RFB_R(crate::FieldReader<bool, bool>);
impl TESTBUF_RFB_R {
    pub(crate) fn new(bits: bool) -> Self {
        TESTBUF_RFB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTBUF_RFB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `testbuf_rfb` writer - "]
pub struct TESTBUF_RFB_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTBUF_RFB_W<'a> {
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
#[doc = "Field `testbuf_rin` reader - "]
pub struct TESTBUF_RIN_R(crate::FieldReader<bool, bool>);
impl TESTBUF_RIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TESTBUF_RIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESTBUF_RIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `testbuf_rin` writer - "]
pub struct TESTBUF_RIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTBUF_RIN_W<'a> {
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
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_testbuf(&self) -> PU_TESTBUF_R {
        PU_TESTBUF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn testbuf_vcm(&self) -> TESTBUF_VCM_R {
        TESTBUF_VCM_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn testbuf_bm(&self) -> TESTBUF_BM_R {
        TESTBUF_BM_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn testbuf_boost(&self) -> TESTBUF_BOOST_R {
        TESTBUF_BOOST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn testbuf_op_cc(&self) -> TESTBUF_OP_CC_R {
        TESTBUF_OP_CC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn testbuf_rfb(&self) -> TESTBUF_RFB_R {
        TESTBUF_RFB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn testbuf_rin(&self) -> TESTBUF_RIN_R {
        TESTBUF_RIN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_testbuf(&mut self) -> PU_TESTBUF_W {
        PU_TESTBUF_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn testbuf_vcm(&mut self) -> TESTBUF_VCM_W {
        TESTBUF_VCM_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn testbuf_bm(&mut self) -> TESTBUF_BM_W {
        TESTBUF_BM_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn testbuf_boost(&mut self) -> TESTBUF_BOOST_W {
        TESTBUF_BOOST_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn testbuf_op_cc(&mut self) -> TESTBUF_OP_CC_W {
        TESTBUF_OP_CC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn testbuf_rfb(&mut self) -> TESTBUF_RFB_W {
        TESTBUF_RFB_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn testbuf_rin(&mut self) -> TESTBUF_RIN_W {
        TESTBUF_RIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "testbuf.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [testbuf](index.html) module"]
pub struct TESTBUF_SPEC;
impl crate::RegisterSpec for TESTBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [testbuf::R](R) reader structure"]
impl crate::Readable for TESTBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [testbuf::W](W) writer structure"]
impl crate::Writable for TESTBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets testbuf to value 0"]
impl crate::Resettable for TESTBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
