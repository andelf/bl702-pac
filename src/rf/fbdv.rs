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
#[doc = "Field `fbdv_tpm_clk_sel` reader - "]
pub type FBDV_TPM_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fbdv_tpm_clk_sel` writer - "]
pub type FBDV_TPM_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBDV_SPEC, u8, u8, 3, O>;
#[doc = "Field `fbdv_adpll_clk_sel` reader - "]
pub type FBDV_ADPLL_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `fbdv_adpll_clk_sel` writer - "]
pub type FBDV_ADPLL_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
#[doc = "Field `fbdv_dco_dither_clk_sel` reader - "]
pub type FBDV_DCO_DITHER_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `fbdv_dco_dither_clk_sel` writer - "]
pub type FBDV_DCO_DITHER_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
#[doc = "Field `fbdv_fb_clk_sel` reader - "]
pub type FBDV_FB_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `fbdv_fb_clk_sel` writer - "]
pub type FBDV_FB_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
#[doc = "Field `fbdv_sample_clk_sel` reader - "]
pub type FBDV_SAMPLE_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fbdv_sample_clk_sel` writer - "]
pub type FBDV_SAMPLE_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FBDV_SPEC, u8, u8, 2, O>;
#[doc = "Field `fbdv_stg_sel` reader - "]
pub type FBDV_STG_SEL_R = crate::BitReader<bool>;
#[doc = "Field `fbdv_stg_sel` writer - "]
pub type FBDV_STG_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
#[doc = "Field `rst_mmdiv` reader - "]
pub type RST_MMDIV_R = crate::BitReader<bool>;
#[doc = "Field `rst_mmdiv` writer - "]
pub type RST_MMDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
#[doc = "Field `lotpm_fmash_clk_polarity` reader - "]
pub type LOTPM_FMASH_CLK_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_fmash_clk_polarity` writer - "]
pub type LOTPM_FMASH_CLK_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
#[doc = "Field `dco_dither_clk_polarity` reader - "]
pub type DCO_DITHER_CLK_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `dco_dither_clk_polarity` writer - "]
pub type DCO_DITHER_CLK_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBDV_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fbdv_tpm_clk_sel(&self) -> FBDV_TPM_CLK_SEL_R {
        FBDV_TPM_CLK_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fbdv_adpll_clk_sel(&self) -> FBDV_ADPLL_CLK_SEL_R {
        FBDV_ADPLL_CLK_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fbdv_dco_dither_clk_sel(&self) -> FBDV_DCO_DITHER_CLK_SEL_R {
        FBDV_DCO_DITHER_CLK_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fbdv_fb_clk_sel(&self) -> FBDV_FB_CLK_SEL_R {
        FBDV_FB_CLK_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fbdv_sample_clk_sel(&self) -> FBDV_SAMPLE_CLK_SEL_R {
        FBDV_SAMPLE_CLK_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fbdv_stg_sel(&self) -> FBDV_STG_SEL_R {
        FBDV_STG_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rst_mmdiv(&self) -> RST_MMDIV_R {
        RST_MMDIV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lotpm_fmash_clk_polarity(&self) -> LOTPM_FMASH_CLK_POLARITY_R {
        LOTPM_FMASH_CLK_POLARITY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dco_dither_clk_polarity(&self) -> DCO_DITHER_CLK_POLARITY_R {
        DCO_DITHER_CLK_POLARITY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn fbdv_tpm_clk_sel(&mut self) -> FBDV_TPM_CLK_SEL_W<0> {
        FBDV_TPM_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fbdv_adpll_clk_sel(&mut self) -> FBDV_ADPLL_CLK_SEL_W<4> {
        FBDV_ADPLL_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fbdv_dco_dither_clk_sel(&mut self) -> FBDV_DCO_DITHER_CLK_SEL_W<8> {
        FBDV_DCO_DITHER_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn fbdv_fb_clk_sel(&mut self) -> FBDV_FB_CLK_SEL_W<12> {
        FBDV_FB_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn fbdv_sample_clk_sel(&mut self) -> FBDV_SAMPLE_CLK_SEL_W<16> {
        FBDV_SAMPLE_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn fbdv_stg_sel(&mut self) -> FBDV_STG_SEL_W<20> {
        FBDV_STG_SEL_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rst_mmdiv(&mut self) -> RST_MMDIV_W<24> {
        RST_MMDIV_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lotpm_fmash_clk_polarity(&mut self) -> LOTPM_FMASH_CLK_POLARITY_W<28> {
        LOTPM_FMASH_CLK_POLARITY_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dco_dither_clk_polarity(&mut self) -> DCO_DITHER_CLK_POLARITY_W<29> {
        DCO_DITHER_CLK_POLARITY_W::new(self)
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
