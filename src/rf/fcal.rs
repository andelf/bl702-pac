#[doc = "Register `fcal` reader"]
pub struct R(crate::R<FCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fcal` writer"]
pub struct W(crate::W<FCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCAL_SPEC>;
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
impl From<crate::W<FCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fcal_mom_ini_ext` reader - "]
pub struct FCAL_MOM_INI_EXT_R(crate::FieldReader<u8, u8>);
impl FCAL_MOM_INI_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FCAL_MOM_INI_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_MOM_INI_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_mom_ini_ext` writer - "]
pub struct FCAL_MOM_INI_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_MOM_INI_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `fcal_mode` reader - "]
pub struct FCAL_MODE_R(crate::FieldReader<u8, u8>);
impl FCAL_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FCAL_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_mode` writer - "]
pub struct FCAL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `fcal_clk_period` reader - "]
pub struct FCAL_CLK_PERIOD_R(crate::FieldReader<u8, u8>);
impl FCAL_CLK_PERIOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FCAL_CLK_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_CLK_PERIOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_clk_period` writer - "]
pub struct FCAL_CLK_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_CLK_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `fcal_mom_toggle_cnt` reader - "]
pub struct FCAL_MOM_TOGGLE_CNT_R(crate::FieldReader<bool, bool>);
impl FCAL_MOM_TOGGLE_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCAL_MOM_TOGGLE_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_MOM_TOGGLE_CNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_mom_toggle_cnt` writer - "]
pub struct FCAL_MOM_TOGGLE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_MOM_TOGGLE_CNT_W<'a> {
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
#[doc = "Field `fcal_coarse_pha_threshold` reader - "]
pub struct FCAL_COARSE_PHA_THRESHOLD_R(crate::FieldReader<u8, u8>);
impl FCAL_COARSE_PHA_THRESHOLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FCAL_COARSE_PHA_THRESHOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_COARSE_PHA_THRESHOLD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_coarse_pha_threshold` writer - "]
pub struct FCAL_COARSE_PHA_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_COARSE_PHA_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `fcal_div_ratio_adj_en` reader - "]
pub struct FCAL_DIV_RATIO_ADJ_EN_R(crate::FieldReader<bool, bool>);
impl FCAL_DIV_RATIO_ADJ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FCAL_DIV_RATIO_ADJ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCAL_DIV_RATIO_ADJ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fcal_div_ratio_adj_en` writer - "]
pub struct FCAL_DIV_RATIO_ADJ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FCAL_DIV_RATIO_ADJ_EN_W<'a> {
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
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn fcal_mom_ini_ext(&self) -> FCAL_MOM_INI_EXT_R {
        FCAL_MOM_INI_EXT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn fcal_mode(&self) -> FCAL_MODE_R {
        FCAL_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_clk_period(&self) -> FCAL_CLK_PERIOD_R {
        FCAL_CLK_PERIOD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_mom_toggle_cnt(&self) -> FCAL_MOM_TOGGLE_CNT_R {
        FCAL_MOM_TOGGLE_CNT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fcal_coarse_pha_threshold(&self) -> FCAL_COARSE_PHA_THRESHOLD_R {
        FCAL_COARSE_PHA_THRESHOLD_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fcal_div_ratio_adj_en(&self) -> FCAL_DIV_RATIO_ADJ_EN_R {
        FCAL_DIV_RATIO_ADJ_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn fcal_mom_ini_ext(&mut self) -> FCAL_MOM_INI_EXT_W {
        FCAL_MOM_INI_EXT_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn fcal_mode(&mut self) -> FCAL_MODE_W {
        FCAL_MODE_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_clk_period(&mut self) -> FCAL_CLK_PERIOD_W {
        FCAL_CLK_PERIOD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_mom_toggle_cnt(&mut self) -> FCAL_MOM_TOGGLE_CNT_W {
        FCAL_MOM_TOGGLE_CNT_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fcal_coarse_pha_threshold(&mut self) -> FCAL_COARSE_PHA_THRESHOLD_W {
        FCAL_COARSE_PHA_THRESHOLD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fcal_div_ratio_adj_en(&mut self) -> FCAL_DIV_RATIO_ADJ_EN_W {
        FCAL_DIV_RATIO_ADJ_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "fcal.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcal](index.html) module"]
pub struct FCAL_SPEC;
impl crate::RegisterSpec for FCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcal::R](R) reader structure"]
impl crate::Readable for FCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcal::W](W) writer structure"]
impl crate::Writable for FCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets fcal to value 0"]
impl crate::Resettable for FCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
