#[doc = "Register `adpll_lf_tx` reader"]
pub struct R(crate::R<ADPLL_LF_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_LF_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_LF_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_LF_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_lf_tx` writer"]
pub struct W(crate::W<ADPLL_LF_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_LF_TX_SPEC>;
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
impl From<crate::W<ADPLL_LF_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_LF_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_lf_alpha_base_tx` reader - "]
pub struct ADPLL_LF_ALPHA_BASE_TX_R(crate::FieldReader<bool, bool>);
impl ADPLL_LF_ALPHA_BASE_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_LF_ALPHA_BASE_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LF_ALPHA_BASE_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lf_alpha_base_tx` writer - "]
pub struct ADPLL_LF_ALPHA_BASE_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LF_ALPHA_BASE_TX_W<'a> {
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
#[doc = "Field `adpll_lf_alpha_exp_tx` reader - "]
pub struct ADPLL_LF_ALPHA_EXP_TX_R(crate::FieldReader<u8, u8>);
impl ADPLL_LF_ALPHA_EXP_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_LF_ALPHA_EXP_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LF_ALPHA_EXP_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lf_alpha_exp_tx` writer - "]
pub struct ADPLL_LF_ALPHA_EXP_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LF_ALPHA_EXP_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `adpll_lf_alpha_fast_tx` reader - "]
pub struct ADPLL_LF_ALPHA_FAST_TX_R(crate::FieldReader<u8, u8>);
impl ADPLL_LF_ALPHA_FAST_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_LF_ALPHA_FAST_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LF_ALPHA_FAST_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lf_alpha_fast_tx` writer - "]
pub struct ADPLL_LF_ALPHA_FAST_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LF_ALPHA_FAST_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `adpll_lf_beta_base_tx` reader - "]
pub struct ADPLL_LF_BETA_BASE_TX_R(crate::FieldReader<u8, u8>);
impl ADPLL_LF_BETA_BASE_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_LF_BETA_BASE_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LF_BETA_BASE_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lf_beta_base_tx` writer - "]
pub struct ADPLL_LF_BETA_BASE_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LF_BETA_BASE_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `adpll_lf_beta_exp_tx` reader - "]
pub struct ADPLL_LF_BETA_EXP_TX_R(crate::FieldReader<u8, u8>);
impl ADPLL_LF_BETA_EXP_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_LF_BETA_EXP_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LF_BETA_EXP_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lf_beta_exp_tx` writer - "]
pub struct ADPLL_LF_BETA_EXP_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LF_BETA_EXP_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | ((value as u32 & 0x07) << 14);
        self.w
    }
}
#[doc = "Field `adpll_lf_beta_fast_tx` reader - "]
pub struct ADPLL_LF_BETA_FAST_TX_R(crate::FieldReader<bool, bool>);
impl ADPLL_LF_BETA_FAST_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_LF_BETA_FAST_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LF_BETA_FAST_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lf_beta_fast_tx` writer - "]
pub struct ADPLL_LF_BETA_FAST_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LF_BETA_FAST_TX_W<'a> {
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
#[doc = "Field `adpll_lf_f_p3_tx` reader - "]
pub struct ADPLL_LF_F_P3_TX_R(crate::FieldReader<u8, u8>);
impl ADPLL_LF_F_P3_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_LF_F_P3_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_LF_F_P3_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_lf_f_p3_tx` writer - "]
pub struct ADPLL_LF_F_P3_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_LF_F_P3_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_lf_alpha_base_tx(&self) -> ADPLL_LF_ALPHA_BASE_TX_R {
        ADPLL_LF_ALPHA_BASE_TX_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adpll_lf_alpha_exp_tx(&self) -> ADPLL_LF_ALPHA_EXP_TX_R {
        ADPLL_LF_ALPHA_EXP_TX_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_lf_alpha_fast_tx(&self) -> ADPLL_LF_ALPHA_FAST_TX_R {
        ADPLL_LF_ALPHA_FAST_TX_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn adpll_lf_beta_base_tx(&self) -> ADPLL_LF_BETA_BASE_TX_R {
        ADPLL_LF_BETA_BASE_TX_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn adpll_lf_beta_exp_tx(&self) -> ADPLL_LF_BETA_EXP_TX_R {
        ADPLL_LF_BETA_EXP_TX_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_lf_beta_fast_tx(&self) -> ADPLL_LF_BETA_FAST_TX_R {
        ADPLL_LF_BETA_FAST_TX_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adpll_lf_f_p3_tx(&self) -> ADPLL_LF_F_P3_TX_R {
        ADPLL_LF_F_P3_TX_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_lf_alpha_base_tx(&mut self) -> ADPLL_LF_ALPHA_BASE_TX_W {
        ADPLL_LF_ALPHA_BASE_TX_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adpll_lf_alpha_exp_tx(&mut self) -> ADPLL_LF_ALPHA_EXP_TX_W {
        ADPLL_LF_ALPHA_EXP_TX_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_lf_alpha_fast_tx(&mut self) -> ADPLL_LF_ALPHA_FAST_TX_W {
        ADPLL_LF_ALPHA_FAST_TX_W { w: self }
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn adpll_lf_beta_base_tx(&mut self) -> ADPLL_LF_BETA_BASE_TX_W {
        ADPLL_LF_BETA_BASE_TX_W { w: self }
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn adpll_lf_beta_exp_tx(&mut self) -> ADPLL_LF_BETA_EXP_TX_W {
        ADPLL_LF_BETA_EXP_TX_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_lf_beta_fast_tx(&mut self) -> ADPLL_LF_BETA_FAST_TX_W {
        ADPLL_LF_BETA_FAST_TX_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adpll_lf_f_p3_tx(&mut self) -> ADPLL_LF_F_P3_TX_W {
        ADPLL_LF_F_P3_TX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_lf_tx.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_lf_tx](index.html) module"]
pub struct ADPLL_LF_TX_SPEC;
impl crate::RegisterSpec for ADPLL_LF_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_lf_tx::R](R) reader structure"]
impl crate::Readable for ADPLL_LF_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_lf_tx::W](W) writer structure"]
impl crate::Writable for ADPLL_LF_TX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_lf_tx to value 0"]
impl crate::Resettable for ADPLL_LF_TX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
