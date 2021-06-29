#[doc = "Register `mjpeg_swap_mode` reader"]
pub struct R(crate::R<MJPEG_SWAP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_SWAP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_SWAP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_SWAP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_swap_mode` writer"]
pub struct W(crate::W<MJPEG_SWAP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_SWAP_MODE_SPEC>;
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
impl From<crate::W<MJPEG_SWAP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_SWAP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_swap_fend` reader - "]
pub struct STS_SWAP_FEND_R(crate::FieldReader<bool, bool>);
impl STS_SWAP_FEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_SWAP_FEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_SWAP_FEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_swap_fend` writer - "]
pub struct STS_SWAP_FEND_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_SWAP_FEND_W<'a> {
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
#[doc = "Field `sts_swap_fstart` reader - "]
pub struct STS_SWAP_FSTART_R(crate::FieldReader<bool, bool>);
impl STS_SWAP_FSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_SWAP_FSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_SWAP_FSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_swap_fstart` writer - "]
pub struct STS_SWAP_FSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_SWAP_FSTART_W<'a> {
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
#[doc = "Field `sts_read_swap_idx` reader - "]
pub struct STS_READ_SWAP_IDX_R(crate::FieldReader<bool, bool>);
impl STS_READ_SWAP_IDX_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_READ_SWAP_IDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_READ_SWAP_IDX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_read_swap_idx` writer - "]
pub struct STS_READ_SWAP_IDX_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_READ_SWAP_IDX_W<'a> {
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
#[doc = "Field `sts_swap1_full` reader - "]
pub struct STS_SWAP1_FULL_R(crate::FieldReader<bool, bool>);
impl STS_SWAP1_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_SWAP1_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_SWAP1_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_swap1_full` writer - "]
pub struct STS_SWAP1_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_SWAP1_FULL_W<'a> {
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
#[doc = "Field `sts_swap0_full` reader - "]
pub struct STS_SWAP0_FULL_R(crate::FieldReader<bool, bool>);
impl STS_SWAP0_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_SWAP0_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_SWAP0_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_swap0_full` writer - "]
pub struct STS_SWAP0_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_SWAP0_FULL_W<'a> {
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
#[doc = "Field `reg_w_swap_mode` reader - "]
pub struct REG_W_SWAP_MODE_R(crate::FieldReader<bool, bool>);
impl REG_W_SWAP_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_W_SWAP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_W_SWAP_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_w_swap_mode` writer - "]
pub struct REG_W_SWAP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_W_SWAP_MODE_W<'a> {
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sts_swap_fend(&self) -> STS_SWAP_FEND_R {
        STS_SWAP_FEND_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sts_swap_fstart(&self) -> STS_SWAP_FSTART_R {
        STS_SWAP_FSTART_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sts_read_swap_idx(&self) -> STS_READ_SWAP_IDX_R {
        STS_READ_SWAP_IDX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sts_swap1_full(&self) -> STS_SWAP1_FULL_R {
        STS_SWAP1_FULL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sts_swap0_full(&self) -> STS_SWAP0_FULL_R {
        STS_SWAP0_FULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_w_swap_mode(&self) -> REG_W_SWAP_MODE_R {
        REG_W_SWAP_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sts_swap_fend(&mut self) -> STS_SWAP_FEND_W {
        STS_SWAP_FEND_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sts_swap_fstart(&mut self) -> STS_SWAP_FSTART_W {
        STS_SWAP_FSTART_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sts_read_swap_idx(&mut self) -> STS_READ_SWAP_IDX_W {
        STS_READ_SWAP_IDX_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sts_swap1_full(&mut self) -> STS_SWAP1_FULL_W {
        STS_SWAP1_FULL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sts_swap0_full(&mut self) -> STS_SWAP0_FULL_W {
        STS_SWAP0_FULL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_w_swap_mode(&mut self) -> REG_W_SWAP_MODE_W {
        REG_W_SWAP_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_swap_mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_swap_mode](index.html) module"]
pub struct MJPEG_SWAP_MODE_SPEC;
impl crate::RegisterSpec for MJPEG_SWAP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_swap_mode::R](R) reader structure"]
impl crate::Readable for MJPEG_SWAP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_swap_mode::W](W) writer structure"]
impl crate::Writable for MJPEG_SWAP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_swap_mode to value 0"]
impl crate::Resettable for MJPEG_SWAP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
