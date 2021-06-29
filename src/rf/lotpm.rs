#[doc = "Register `lotpm` reader"]
pub struct R(crate::R<LOTPM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOTPM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOTPM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOTPM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lotpm` writer"]
pub struct W(crate::W<LOTPM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOTPM_SPEC>;
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
impl From<crate::W<LOTPM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOTPM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lotpm_hfp_mash1_sel` reader - "]
pub struct LOTPM_HFP_MASH1_SEL_R(crate::FieldReader<bool, bool>);
impl LOTPM_HFP_MASH1_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOTPM_HFP_MASH1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_HFP_MASH1_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_hfp_mash1_sel` writer - "]
pub struct LOTPM_HFP_MASH1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_HFP_MASH1_SEL_W<'a> {
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
#[doc = "Field `lotpm_hfp_polarity` reader - "]
pub struct LOTPM_HFP_POLARITY_R(crate::FieldReader<bool, bool>);
impl LOTPM_HFP_POLARITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOTPM_HFP_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_HFP_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_hfp_polarity` writer - "]
pub struct LOTPM_HFP_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_HFP_POLARITY_W<'a> {
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
#[doc = "Field `lotpm_hfp_delay_fref` reader - "]
pub struct LOTPM_HFP_DELAY_FREF_R(crate::FieldReader<u8, u8>);
impl LOTPM_HFP_DELAY_FREF_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOTPM_HFP_DELAY_FREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_HFP_DELAY_FREF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_hfp_delay_fref` writer - "]
pub struct LOTPM_HFP_DELAY_FREF_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_HFP_DELAY_FREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `lotpm_hfp_delay_fmash` reader - "]
pub struct LOTPM_HFP_DELAY_FMASH_R(crate::FieldReader<u8, u8>);
impl LOTPM_HFP_DELAY_FMASH_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOTPM_HFP_DELAY_FMASH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_HFP_DELAY_FMASH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_hfp_delay_fmash` writer - "]
pub struct LOTPM_HFP_DELAY_FMASH_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_HFP_DELAY_FMASH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `lotpm_lfp_delay_sel` reader - "]
pub struct LOTPM_LFP_DELAY_SEL_R(crate::FieldReader<u8, u8>);
impl LOTPM_LFP_DELAY_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOTPM_LFP_DELAY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_LFP_DELAY_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_lfp_delay_sel` writer - "]
pub struct LOTPM_LFP_DELAY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_LFP_DELAY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lotpm_hfp_mash1_sel(&self) -> LOTPM_HFP_MASH1_SEL_R {
        LOTPM_HFP_MASH1_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lotpm_hfp_polarity(&self) -> LOTPM_HFP_POLARITY_R {
        LOTPM_HFP_POLARITY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lotpm_hfp_delay_fref(&self) -> LOTPM_HFP_DELAY_FREF_R {
        LOTPM_HFP_DELAY_FREF_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lotpm_hfp_delay_fmash(&self) -> LOTPM_HFP_DELAY_FMASH_R {
        LOTPM_HFP_DELAY_FMASH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lotpm_lfp_delay_sel(&self) -> LOTPM_LFP_DELAY_SEL_R {
        LOTPM_LFP_DELAY_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lotpm_hfp_mash1_sel(&mut self) -> LOTPM_HFP_MASH1_SEL_W {
        LOTPM_HFP_MASH1_SEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lotpm_hfp_polarity(&mut self) -> LOTPM_HFP_POLARITY_W {
        LOTPM_HFP_POLARITY_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lotpm_hfp_delay_fref(&mut self) -> LOTPM_HFP_DELAY_FREF_W {
        LOTPM_HFP_DELAY_FREF_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn lotpm_hfp_delay_fmash(&mut self) -> LOTPM_HFP_DELAY_FMASH_W {
        LOTPM_HFP_DELAY_FMASH_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lotpm_lfp_delay_sel(&mut self) -> LOTPM_LFP_DELAY_SEL_W {
        LOTPM_LFP_DELAY_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lotpm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lotpm](index.html) module"]
pub struct LOTPM_SPEC;
impl crate::RegisterSpec for LOTPM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lotpm::R](R) reader structure"]
impl crate::Readable for LOTPM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lotpm::W](W) writer structure"]
impl crate::Writable for LOTPM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lotpm to value 0"]
impl crate::Resettable for LOTPM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
