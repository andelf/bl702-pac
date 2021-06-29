#[doc = "Register `i2s_int_sts` reader"]
pub struct R(crate::R<I2S_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_INT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_int_sts` writer"]
pub struct W(crate::W<I2S_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_INT_STS_SPEC>;
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
impl From<crate::W<I2S_INT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_i2s_fer_en` reader - "]
pub struct CR_I2S_FER_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2S_FER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_FER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_FER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_fer_en` writer - "]
pub struct CR_I2S_FER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_FER_EN_W<'a> {
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
#[doc = "Field `cr_i2s_rxf_en` reader - "]
pub struct CR_I2S_RXF_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2S_RXF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_RXF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_RXF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_rxf_en` writer - "]
pub struct CR_I2S_RXF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_RXF_EN_W<'a> {
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
#[doc = "Field `cr_i2s_txf_en` reader - "]
pub struct CR_I2S_TXF_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2S_TXF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_TXF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_TXF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_txf_en` writer - "]
pub struct CR_I2S_TXF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_TXF_EN_W<'a> {
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
#[doc = "Field `cr_i2s_fer_mask` reader - "]
pub struct CR_I2S_FER_MASK_R(crate::FieldReader<bool, bool>);
impl CR_I2S_FER_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_FER_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_FER_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_fer_mask` writer - "]
pub struct CR_I2S_FER_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_FER_MASK_W<'a> {
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
#[doc = "Field `cr_i2s_rxf_mask` reader - "]
pub struct CR_I2S_RXF_MASK_R(crate::FieldReader<bool, bool>);
impl CR_I2S_RXF_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_RXF_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_RXF_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_rxf_mask` writer - "]
pub struct CR_I2S_RXF_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_RXF_MASK_W<'a> {
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
#[doc = "Field `cr_i2s_txf_mask` reader - "]
pub struct CR_I2S_TXF_MASK_R(crate::FieldReader<bool, bool>);
impl CR_I2S_TXF_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_TXF_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_TXF_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_txf_mask` writer - "]
pub struct CR_I2S_TXF_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_TXF_MASK_W<'a> {
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
#[doc = "Field `i2s_fer_int` reader - "]
pub struct I2S_FER_INT_R(crate::FieldReader<bool, bool>);
impl I2S_FER_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2S_FER_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_FER_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2s_fer_int` writer - "]
pub struct I2S_FER_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_FER_INT_W<'a> {
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
#[doc = "Field `i2s_rxf_int` reader - "]
pub struct I2S_RXF_INT_R(crate::FieldReader<bool, bool>);
impl I2S_RXF_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2S_RXF_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_RXF_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2s_rxf_int` writer - "]
pub struct I2S_RXF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RXF_INT_W<'a> {
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
#[doc = "Field `i2s_txf_int` reader - "]
pub struct I2S_TXF_INT_R(crate::FieldReader<bool, bool>);
impl I2S_TXF_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2S_TXF_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2S_TXF_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2s_txf_int` writer - "]
pub struct I2S_TXF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_TXF_INT_W<'a> {
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
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_i2s_fer_en(&self) -> CR_I2S_FER_EN_R {
        CR_I2S_FER_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_i2s_rxf_en(&self) -> CR_I2S_RXF_EN_R {
        CR_I2S_RXF_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_i2s_txf_en(&self) -> CR_I2S_TXF_EN_R {
        CR_I2S_TXF_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_i2s_fer_mask(&self) -> CR_I2S_FER_MASK_R {
        CR_I2S_FER_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_i2s_rxf_mask(&self) -> CR_I2S_RXF_MASK_R {
        CR_I2S_RXF_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_i2s_txf_mask(&self) -> CR_I2S_TXF_MASK_R {
        CR_I2S_TXF_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_fer_int(&self) -> I2S_FER_INT_R {
        I2S_FER_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_rxf_int(&self) -> I2S_RXF_INT_R {
        I2S_RXF_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_txf_int(&self) -> I2S_TXF_INT_R {
        I2S_TXF_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_i2s_fer_en(&mut self) -> CR_I2S_FER_EN_W {
        CR_I2S_FER_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_i2s_rxf_en(&mut self) -> CR_I2S_RXF_EN_W {
        CR_I2S_RXF_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_i2s_txf_en(&mut self) -> CR_I2S_TXF_EN_W {
        CR_I2S_TXF_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_i2s_fer_mask(&mut self) -> CR_I2S_FER_MASK_W {
        CR_I2S_FER_MASK_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_i2s_rxf_mask(&mut self) -> CR_I2S_RXF_MASK_W {
        CR_I2S_RXF_MASK_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_i2s_txf_mask(&mut self) -> CR_I2S_TXF_MASK_W {
        CR_I2S_TXF_MASK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_fer_int(&mut self) -> I2S_FER_INT_W {
        I2S_FER_INT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_rxf_int(&mut self) -> I2S_RXF_INT_W {
        I2S_RXF_INT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_txf_int(&mut self) -> I2S_TXF_INT_W {
        I2S_TXF_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2s_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_int_sts](index.html) module"]
pub struct I2S_INT_STS_SPEC;
impl crate::RegisterSpec for I2S_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_int_sts::R](R) reader structure"]
impl crate::Readable for I2S_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_int_sts::W](W) writer structure"]
impl crate::Writable for I2S_INT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2s_int_sts to value 0"]
impl crate::Resettable for I2S_INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
