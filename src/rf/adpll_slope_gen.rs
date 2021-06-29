#[doc = "Register `adpll_slope_gen` reader"]
pub struct R(crate::R<ADPLL_SLOPE_GEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_SLOPE_GEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_SLOPE_GEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_SLOPE_GEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_slope_gen` writer"]
pub struct W(crate::W<ADPLL_SLOPE_GEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_SLOPE_GEN_SPEC>;
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
impl From<crate::W<ADPLL_SLOPE_GEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_SLOPE_GEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_slope_gen_pulse_width_enhance` reader - "]
pub struct ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_R(crate::FieldReader<bool, bool>);
impl ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_slope_gen_pulse_width_enhance` writer - "]
pub struct ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_W<'a> {
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
#[doc = "Field `adpll_slope_gen_dc_corr` reader - "]
pub struct ADPLL_SLOPE_GEN_DC_CORR_R(crate::FieldReader<u8, u8>);
impl ADPLL_SLOPE_GEN_DC_CORR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_SLOPE_GEN_DC_CORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SLOPE_GEN_DC_CORR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_slope_gen_dc_corr` writer - "]
pub struct ADPLL_SLOPE_GEN_DC_CORR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SLOPE_GEN_DC_CORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `adpll_slope_gen_r_sel` reader - "]
pub struct ADPLL_SLOPE_GEN_R_SEL_R(crate::FieldReader<u8, u8>);
impl ADPLL_SLOPE_GEN_R_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_SLOPE_GEN_R_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SLOPE_GEN_R_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_slope_gen_r_sel` writer - "]
pub struct ADPLL_SLOPE_GEN_R_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SLOPE_GEN_R_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_slope_gen_pulse_width_enhance(&self) -> ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_R {
        ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adpll_slope_gen_dc_corr(&self) -> ADPLL_SLOPE_GEN_DC_CORR_R {
        ADPLL_SLOPE_GEN_DC_CORR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn adpll_slope_gen_r_sel(&self) -> ADPLL_SLOPE_GEN_R_SEL_R {
        ADPLL_SLOPE_GEN_R_SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_slope_gen_pulse_width_enhance(&mut self) -> ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_W {
        ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adpll_slope_gen_dc_corr(&mut self) -> ADPLL_SLOPE_GEN_DC_CORR_W {
        ADPLL_SLOPE_GEN_DC_CORR_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn adpll_slope_gen_r_sel(&mut self) -> ADPLL_SLOPE_GEN_R_SEL_W {
        ADPLL_SLOPE_GEN_R_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_slope_gen.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_slope_gen](index.html) module"]
pub struct ADPLL_SLOPE_GEN_SPEC;
impl crate::RegisterSpec for ADPLL_SLOPE_GEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_slope_gen::R](R) reader structure"]
impl crate::Readable for ADPLL_SLOPE_GEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_slope_gen::W](W) writer structure"]
impl crate::Writable for ADPLL_SLOPE_GEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_slope_gen to value 0"]
impl crate::Resettable for ADPLL_SLOPE_GEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
