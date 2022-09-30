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
#[doc = "Field `adpll_slope_gen_r_sel` reader - "]
pub type ADPLL_SLOPE_GEN_R_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_slope_gen_r_sel` writer - "]
pub type ADPLL_SLOPE_GEN_R_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SLOPE_GEN_SPEC, u8, u8, 3, O>;
#[doc = "Field `adpll_slope_gen_dc_corr` reader - "]
pub type ADPLL_SLOPE_GEN_DC_CORR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_slope_gen_dc_corr` writer - "]
pub type ADPLL_SLOPE_GEN_DC_CORR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SLOPE_GEN_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_slope_gen_pulse_width_enhance` reader - "]
pub type ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_R = crate::BitReader<bool>;
#[doc = "Field `adpll_slope_gen_pulse_width_enhance` writer - "]
pub type ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_SLOPE_GEN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn adpll_slope_gen_r_sel(&self) -> ADPLL_SLOPE_GEN_R_SEL_R {
        ADPLL_SLOPE_GEN_R_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adpll_slope_gen_dc_corr(&self) -> ADPLL_SLOPE_GEN_DC_CORR_R {
        ADPLL_SLOPE_GEN_DC_CORR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_slope_gen_pulse_width_enhance(&self) -> ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_R {
        ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn adpll_slope_gen_r_sel(&mut self) -> ADPLL_SLOPE_GEN_R_SEL_W<0> {
        ADPLL_SLOPE_GEN_R_SEL_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adpll_slope_gen_dc_corr(&mut self) -> ADPLL_SLOPE_GEN_DC_CORR_W<4> {
        ADPLL_SLOPE_GEN_DC_CORR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_slope_gen_pulse_width_enhance(
        &mut self,
    ) -> ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_W<6> {
        ADPLL_SLOPE_GEN_PULSE_WIDTH_ENHANCE_W::new(self)
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
