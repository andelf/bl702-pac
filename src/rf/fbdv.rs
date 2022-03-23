#[doc = "Register `fbdv` reader"]
pub struct R(crate::R<FBDV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBDV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBDV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBDV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fbdv` writer"]
pub struct W(crate::W<FBDV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBDV_SPEC>;
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
impl From<crate::W<FBDV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBDV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dco_dither_clk_polarity` reader - "]
pub struct DCO_DITHER_CLK_POLARITY_R(crate::FieldReader<bool, bool>);
impl DCO_DITHER_CLK_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCO_DITHER_CLK_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCO_DITHER_CLK_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dco_dither_clk_polarity` writer - "]
pub struct DCO_DITHER_CLK_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> DCO_DITHER_CLK_POLARITY_W<'a> {
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
#[doc = "Field `lotpm_fmash_clk_polarity` reader - "]
pub struct LOTPM_FMASH_CLK_POLARITY_R(crate::FieldReader<bool, bool>);
impl LOTPM_FMASH_CLK_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOTPM_FMASH_CLK_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOTPM_FMASH_CLK_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lotpm_fmash_clk_polarity` writer - "]
pub struct LOTPM_FMASH_CLK_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> LOTPM_FMASH_CLK_POLARITY_W<'a> {
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
#[doc = "Field `rst_mmdiv` reader - "]
pub struct RST_MMDIV_R(crate::FieldReader<bool, bool>);
impl RST_MMDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST_MMDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_MMDIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rst_mmdiv` writer - "]
pub struct RST_MMDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_MMDIV_W<'a> {
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
#[doc = "Field `fbdv_stg_sel` reader - "]
pub struct FBDV_STG_SEL_R(crate::FieldReader<bool, bool>);
impl FBDV_STG_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FBDV_STG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBDV_STG_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fbdv_stg_sel` writer - "]
pub struct FBDV_STG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBDV_STG_SEL_W<'a> {
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
#[doc = "Field `fbdv_sample_clk_sel` reader - "]
pub struct FBDV_SAMPLE_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl FBDV_SAMPLE_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FBDV_SAMPLE_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBDV_SAMPLE_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fbdv_sample_clk_sel` writer - "]
pub struct FBDV_SAMPLE_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBDV_SAMPLE_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `fbdv_fb_clk_sel` reader - "]
pub struct FBDV_FB_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl FBDV_FB_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FBDV_FB_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBDV_FB_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fbdv_fb_clk_sel` writer - "]
pub struct FBDV_FB_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBDV_FB_CLK_SEL_W<'a> {
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
#[doc = "Field `fbdv_dco_dither_clk_sel` reader - "]
pub struct FBDV_DCO_DITHER_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl FBDV_DCO_DITHER_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FBDV_DCO_DITHER_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBDV_DCO_DITHER_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fbdv_dco_dither_clk_sel` writer - "]
pub struct FBDV_DCO_DITHER_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBDV_DCO_DITHER_CLK_SEL_W<'a> {
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
#[doc = "Field `fbdv_adpll_clk_sel` reader - "]
pub struct FBDV_ADPLL_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl FBDV_ADPLL_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FBDV_ADPLL_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBDV_ADPLL_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fbdv_adpll_clk_sel` writer - "]
pub struct FBDV_ADPLL_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBDV_ADPLL_CLK_SEL_W<'a> {
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
#[doc = "Field `fbdv_tpm_clk_sel` reader - "]
pub struct FBDV_TPM_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl FBDV_TPM_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FBDV_TPM_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBDV_TPM_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fbdv_tpm_clk_sel` writer - "]
pub struct FBDV_TPM_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FBDV_TPM_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dco_dither_clk_polarity(&self) -> DCO_DITHER_CLK_POLARITY_R {
        DCO_DITHER_CLK_POLARITY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lotpm_fmash_clk_polarity(&self) -> LOTPM_FMASH_CLK_POLARITY_R {
        LOTPM_FMASH_CLK_POLARITY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rst_mmdiv(&self) -> RST_MMDIV_R {
        RST_MMDIV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fbdv_stg_sel(&self) -> FBDV_STG_SEL_R {
        FBDV_STG_SEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fbdv_sample_clk_sel(&self) -> FBDV_SAMPLE_CLK_SEL_R {
        FBDV_SAMPLE_CLK_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fbdv_fb_clk_sel(&self) -> FBDV_FB_CLK_SEL_R {
        FBDV_FB_CLK_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fbdv_dco_dither_clk_sel(&self) -> FBDV_DCO_DITHER_CLK_SEL_R {
        FBDV_DCO_DITHER_CLK_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fbdv_adpll_clk_sel(&self) -> FBDV_ADPLL_CLK_SEL_R {
        FBDV_ADPLL_CLK_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fbdv_tpm_clk_sel(&self) -> FBDV_TPM_CLK_SEL_R {
        FBDV_TPM_CLK_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dco_dither_clk_polarity(&mut self) -> DCO_DITHER_CLK_POLARITY_W {
        DCO_DITHER_CLK_POLARITY_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lotpm_fmash_clk_polarity(&mut self) -> LOTPM_FMASH_CLK_POLARITY_W {
        LOTPM_FMASH_CLK_POLARITY_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rst_mmdiv(&mut self) -> RST_MMDIV_W {
        RST_MMDIV_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fbdv_stg_sel(&mut self) -> FBDV_STG_SEL_W {
        FBDV_STG_SEL_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fbdv_sample_clk_sel(&mut self) -> FBDV_SAMPLE_CLK_SEL_W {
        FBDV_SAMPLE_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fbdv_fb_clk_sel(&mut self) -> FBDV_FB_CLK_SEL_W {
        FBDV_FB_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fbdv_dco_dither_clk_sel(&mut self) -> FBDV_DCO_DITHER_CLK_SEL_W {
        FBDV_DCO_DITHER_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fbdv_adpll_clk_sel(&mut self) -> FBDV_ADPLL_CLK_SEL_W {
        FBDV_ADPLL_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fbdv_tpm_clk_sel(&mut self) -> FBDV_TPM_CLK_SEL_W {
        FBDV_TPM_CLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "fbdv.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbdv](index.html) module"]
pub struct FBDV_SPEC;
impl crate::RegisterSpec for FBDV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbdv::R](R) reader structure"]
impl crate::Readable for FBDV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbdv::W](W) writer structure"]
impl crate::Writable for FBDV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets fbdv to value 0"]
impl crate::Resettable for FBDV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
