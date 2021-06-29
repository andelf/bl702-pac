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
#[doc = "Field `clkpll_dc_tp_out_en` reader - "]
pub struct CLKPLL_DC_TP_OUT_EN_R(crate::FieldReader<bool, bool>);
impl CLKPLL_DC_TP_OUT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_DC_TP_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_DC_TP_OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_dc_tp_out_en` writer - "]
pub struct CLKPLL_DC_TP_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_DC_TP_OUT_EN_W<'a> {
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
#[doc = "Field `ten_clkpll` reader - "]
pub struct TEN_CLKPLL_R(crate::FieldReader<bool, bool>);
impl TEN_CLKPLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_CLKPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_CLKPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_clkpll` writer - "]
pub struct TEN_CLKPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_CLKPLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ten_clkpll_sfreg` reader - "]
pub struct TEN_CLKPLL_SFREG_R(crate::FieldReader<bool, bool>);
impl TEN_CLKPLL_SFREG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_CLKPLL_SFREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_CLKPLL_SFREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_clkpll_sfreg` writer - "]
pub struct TEN_CLKPLL_SFREG_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_CLKPLL_SFREG_W<'a> {
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
#[doc = "Field `dten_clkpll_fin` reader - "]
pub struct DTEN_CLKPLL_FIN_R(crate::FieldReader<bool, bool>);
impl DTEN_CLKPLL_FIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLKPLL_FIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLKPLL_FIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clkpll_fin` writer - "]
pub struct DTEN_CLKPLL_FIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_FIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `dten_clkpll_fref` reader - "]
pub struct DTEN_CLKPLL_FREF_R(crate::FieldReader<bool, bool>);
impl DTEN_CLKPLL_FREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLKPLL_FREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLKPLL_FREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clkpll_fref` writer - "]
pub struct DTEN_CLKPLL_FREF_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_FREF_W<'a> {
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
#[doc = "Field `dten_clkpll_fsdm` reader - "]
pub struct DTEN_CLKPLL_FSDM_R(crate::FieldReader<bool, bool>);
impl DTEN_CLKPLL_FSDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLKPLL_FSDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLKPLL_FSDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clkpll_fsdm` writer - "]
pub struct DTEN_CLKPLL_FSDM_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_FSDM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `dten_clk32M` reader - "]
pub struct DTEN_CLK32M_R(crate::FieldReader<bool, bool>);
impl DTEN_CLK32M_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLK32M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLK32M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clk32M` writer - "]
pub struct DTEN_CLK32M_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLK32M_W<'a> {
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
#[doc = "Field `dten_clk96M` reader - "]
pub struct DTEN_CLK96M_R(crate::FieldReader<bool, bool>);
impl DTEN_CLK96M_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLK96M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLK96M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clk96M` writer - "]
pub struct DTEN_CLK96M_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLK96M_W<'a> {
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
#[doc = "Field `dten_clkpll_postdiv_clk` reader - "]
pub struct DTEN_CLKPLL_POSTDIV_CLK_R(crate::FieldReader<bool, bool>);
impl DTEN_CLKPLL_POSTDIV_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_CLKPLL_POSTDIV_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_CLKPLL_POSTDIV_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_clkpll_postdiv_clk` writer - "]
pub struct DTEN_CLKPLL_POSTDIV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_CLKPLL_POSTDIV_CLK_W<'a> {
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_dc_tp_out_en(&self) -> CLKPLL_DC_TP_OUT_EN_R {
        CLKPLL_DC_TP_OUT_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ten_clkpll(&self) -> TEN_CLKPLL_R {
        TEN_CLKPLL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_clkpll_sfreg(&self) -> TEN_CLKPLL_SFREG_R {
        TEN_CLKPLL_SFREG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_clkpll_fin(&self) -> DTEN_CLKPLL_FIN_R {
        DTEN_CLKPLL_FIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_clkpll_fref(&self) -> DTEN_CLKPLL_FREF_R {
        DTEN_CLKPLL_FREF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dten_clkpll_fsdm(&self) -> DTEN_CLKPLL_FSDM_R {
        DTEN_CLKPLL_FSDM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dten_clk32m(&self) -> DTEN_CLK32M_R {
        DTEN_CLK32M_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dten_clk96m(&self) -> DTEN_CLK96M_R {
        DTEN_CLK96M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dten_clkpll_postdiv_clk(&self) -> DTEN_CLKPLL_POSTDIV_CLK_R {
        DTEN_CLKPLL_POSTDIV_CLK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_dc_tp_out_en(&mut self) -> CLKPLL_DC_TP_OUT_EN_W {
        CLKPLL_DC_TP_OUT_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ten_clkpll(&mut self) -> TEN_CLKPLL_W {
        TEN_CLKPLL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_clkpll_sfreg(&mut self) -> TEN_CLKPLL_SFREG_W {
        TEN_CLKPLL_SFREG_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_clkpll_fin(&mut self) -> DTEN_CLKPLL_FIN_W {
        DTEN_CLKPLL_FIN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dten_clkpll_fref(&mut self) -> DTEN_CLKPLL_FREF_W {
        DTEN_CLKPLL_FREF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dten_clkpll_fsdm(&mut self) -> DTEN_CLKPLL_FSDM_W {
        DTEN_CLKPLL_FSDM_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dten_clk32m(&mut self) -> DTEN_CLK32M_W {
        DTEN_CLK32M_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dten_clk96m(&mut self) -> DTEN_CLK96M_W {
        DTEN_CLK96M_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dten_clkpll_postdiv_clk(&mut self) -> DTEN_CLKPLL_POSTDIV_CLK_W {
        DTEN_CLKPLL_POSTDIV_CLK_W { w: self }
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
}
#[doc = "`reset()` method sets clkpll_test_enable to value 0"]
impl crate::Resettable for CLKPLL_TEST_ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
