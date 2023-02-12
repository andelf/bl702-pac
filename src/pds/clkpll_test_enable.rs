#[doc = "Register `clkpll_test_enable` reader"]
pub struct R(crate::R<CLKPLL_TEST_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_TEST_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPLL_TEST_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPLL_TEST_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_test_enable` writer"]
pub struct W(crate::W<CLKPLL_TEST_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_TEST_ENABLE_SPEC>;
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
impl From<crate::W<CLKPLL_TEST_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPLL_TEST_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dten_clkpll_postdiv_clk` reader - "]
pub type DTEN_CLKPLL_POSTDIV_CLK_R = crate::BitReader<bool>;
#[doc = "Field `dten_clkpll_postdiv_clk` writer - "]
pub type DTEN_CLKPLL_POSTDIV_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TEST_ENABLE_SPEC, bool, O>;
#[doc = "Field `dten_clk96M` reader - "]
pub type DTEN_CLK96M_R = crate::BitReader<bool>;
#[doc = "Field `dten_clk96M` writer - "]
pub type DTEN_CLK96M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TEST_ENABLE_SPEC, bool, O>;
#[doc = "Field `dten_clk32M` reader - "]
pub type DTEN_CLK32M_R = crate::BitReader<bool>;
#[doc = "Field `dten_clk32M` writer - "]
pub type DTEN_CLK32M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TEST_ENABLE_SPEC, bool, O>;
#[doc = "Field `dten_clkpll_fsdm` reader - "]
pub type DTEN_CLKPLL_FSDM_R = crate::BitReader<bool>;
#[doc = "Field `dten_clkpll_fsdm` writer - "]
pub type DTEN_CLKPLL_FSDM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TEST_ENABLE_SPEC, bool, O>;
#[doc = "Field `dten_clkpll_fref` reader - "]
pub type DTEN_CLKPLL_FREF_R = crate::BitReader<bool>;
#[doc = "Field `dten_clkpll_fref` writer - "]
pub type DTEN_CLKPLL_FREF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TEST_ENABLE_SPEC, bool, O>;
#[doc = "Field `dten_clkpll_fin` reader - "]
pub type DTEN_CLKPLL_FIN_R = crate::BitReader<bool>;
#[doc = "Field `dten_clkpll_fin` writer - "]
pub type DTEN_CLKPLL_FIN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TEST_ENABLE_SPEC, bool, O>;
#[doc = "Field `ten_clkpll_sfreg` reader - "]
pub type TEN_CLKPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `ten_clkpll_sfreg` writer - "]
pub type TEN_CLKPLL_SFREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TEST_ENABLE_SPEC, bool, O>;
#[doc = "Field `ten_clkpll` reader - "]
pub type TEN_CLKPLL_R = crate::BitReader<bool>;
#[doc = "Field `ten_clkpll` writer - "]
pub type TEN_CLKPLL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TEST_ENABLE_SPEC, bool, O>;
#[doc = "Field `clkpll_dc_tp_out_en` reader - "]
pub type CLKPLL_DC_TP_OUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `clkpll_dc_tp_out_en` writer - "]
pub type CLKPLL_DC_TP_OUT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLKPLL_TEST_ENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dten_clkpll_postdiv_clk(&self) -> DTEN_CLKPLL_POSTDIV_CLK_R {
        DTEN_CLKPLL_POSTDIV_CLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dten_clk96m(&self) -> DTEN_CLK96M_R {
        DTEN_CLK96M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dten_clk32m(&self) -> DTEN_CLK32M_R {
        DTEN_CLK32M_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dten_clkpll_fsdm(&self) -> DTEN_CLKPLL_FSDM_R {
        DTEN_CLKPLL_FSDM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_clkpll_fref(&self) -> DTEN_CLKPLL_FREF_R {
        DTEN_CLKPLL_FREF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_clkpll_fin(&self) -> DTEN_CLKPLL_FIN_R {
        DTEN_CLKPLL_FIN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_clkpll_sfreg(&self) -> TEN_CLKPLL_SFREG_R {
        TEN_CLKPLL_SFREG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ten_clkpll(&self) -> TEN_CLKPLL_R {
        TEN_CLKPLL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_dc_tp_out_en(&self) -> CLKPLL_DC_TP_OUT_EN_R {
        CLKPLL_DC_TP_OUT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clkpll_postdiv_clk(&mut self) -> DTEN_CLKPLL_POSTDIV_CLK_W<0> {
        DTEN_CLKPLL_POSTDIV_CLK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clk96m(&mut self) -> DTEN_CLK96M_W<1> {
        DTEN_CLK96M_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clk32m(&mut self) -> DTEN_CLK32M_W<2> {
        DTEN_CLK32M_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clkpll_fsdm(&mut self) -> DTEN_CLKPLL_FSDM_W<3> {
        DTEN_CLKPLL_FSDM_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clkpll_fref(&mut self) -> DTEN_CLKPLL_FREF_W<4> {
        DTEN_CLKPLL_FREF_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dten_clkpll_fin(&mut self) -> DTEN_CLKPLL_FIN_W<5> {
        DTEN_CLKPLL_FIN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ten_clkpll_sfreg(&mut self) -> TEN_CLKPLL_SFREG_W<6> {
        TEN_CLKPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ten_clkpll(&mut self) -> TEN_CLKPLL_W<7> {
        TEN_CLKPLL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clkpll_dc_tp_out_en(&mut self) -> CLKPLL_DC_TP_OUT_EN_W<8> {
        CLKPLL_DC_TP_OUT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_test_enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_test_enable](index.html) module"]
pub struct CLKPLL_TEST_ENABLE_SPEC;
impl crate::RegisterSpec for CLKPLL_TEST_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_test_enable::R](R) reader structure"]
impl crate::Readable for CLKPLL_TEST_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_test_enable::W](W) writer structure"]
impl crate::Writable for CLKPLL_TEST_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clkpll_test_enable to value 0"]
impl crate::Resettable for CLKPLL_TEST_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
