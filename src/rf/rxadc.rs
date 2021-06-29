#[doc = "Register `rxadc` reader"]
pub struct R(crate::R<RXADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxadc` writer"]
pub struct W(crate::W<RXADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXADC_SPEC>;
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
impl From<crate::W<RXADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxadc_dly_ctrl` reader - "]
pub struct RXADC_DLY_CTRL_R(crate::FieldReader<u8, u8>);
impl RXADC_DLY_CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXADC_DLY_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXADC_DLY_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxadc_dly_ctrl` writer - "]
pub struct RXADC_DLY_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXADC_DLY_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `rxadc_glitch_remove` reader - "]
pub struct RXADC_GLITCH_REMOVE_R(crate::FieldReader<bool, bool>);
impl RXADC_GLITCH_REMOVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXADC_GLITCH_REMOVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXADC_GLITCH_REMOVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxadc_glitch_remove` writer - "]
pub struct RXADC_GLITCH_REMOVE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXADC_GLITCH_REMOVE_W<'a> {
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
#[doc = "Field `rxadc_clk_div_sel` reader - "]
pub struct RXADC_CLK_DIV_SEL_R(crate::FieldReader<bool, bool>);
impl RXADC_CLK_DIV_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXADC_CLK_DIV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXADC_CLK_DIV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxadc_clk_div_sel` writer - "]
pub struct RXADC_CLK_DIV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXADC_CLK_DIV_SEL_W<'a> {
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
#[doc = "Field `rxadc_clk_inv` reader - "]
pub struct RXADC_CLK_INV_R(crate::FieldReader<bool, bool>);
impl RXADC_CLK_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXADC_CLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXADC_CLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxadc_clk_inv` writer - "]
pub struct RXADC_CLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXADC_CLK_INV_W<'a> {
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
#[doc = "Field `rxadc_clk_sync_inv` reader - "]
pub struct RXADC_CLK_SYNC_INV_R(crate::FieldReader<bool, bool>);
impl RXADC_CLK_SYNC_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXADC_CLK_SYNC_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXADC_CLK_SYNC_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxadc_clk_sync_inv` writer - "]
pub struct RXADC_CLK_SYNC_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXADC_CLK_SYNC_INV_W<'a> {
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
#[doc = "Field `rxadc_vref_sel` reader - "]
pub struct RXADC_VREF_SEL_R(crate::FieldReader<u8, u8>);
impl RXADC_VREF_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXADC_VREF_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXADC_VREF_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxadc_vref_sel` writer - "]
pub struct RXADC_VREF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXADC_VREF_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `rxadc_oscal_en` reader - "]
pub struct RXADC_OSCAL_EN_R(crate::FieldReader<bool, bool>);
impl RXADC_OSCAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXADC_OSCAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXADC_OSCAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rxadc_oscal_en` writer - "]
pub struct RXADC_OSCAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXADC_OSCAL_EN_W<'a> {
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
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rxadc_dly_ctrl(&self) -> RXADC_DLY_CTRL_R {
        RXADC_DLY_CTRL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rxadc_glitch_remove(&self) -> RXADC_GLITCH_REMOVE_R {
        RXADC_GLITCH_REMOVE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rxadc_clk_div_sel(&self) -> RXADC_CLK_DIV_SEL_R {
        RXADC_CLK_DIV_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxadc_clk_inv(&self) -> RXADC_CLK_INV_R {
        RXADC_CLK_INV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxadc_clk_sync_inv(&self) -> RXADC_CLK_SYNC_INV_R {
        RXADC_CLK_SYNC_INV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn rxadc_vref_sel(&self) -> RXADC_VREF_SEL_R {
        RXADC_VREF_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxadc_oscal_en(&self) -> RXADC_OSCAL_EN_R {
        RXADC_OSCAL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rxadc_dly_ctrl(&mut self) -> RXADC_DLY_CTRL_W {
        RXADC_DLY_CTRL_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rxadc_glitch_remove(&mut self) -> RXADC_GLITCH_REMOVE_W {
        RXADC_GLITCH_REMOVE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rxadc_clk_div_sel(&mut self) -> RXADC_CLK_DIV_SEL_W {
        RXADC_CLK_DIV_SEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxadc_clk_inv(&mut self) -> RXADC_CLK_INV_W {
        RXADC_CLK_INV_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxadc_clk_sync_inv(&mut self) -> RXADC_CLK_SYNC_INV_W {
        RXADC_CLK_SYNC_INV_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn rxadc_vref_sel(&mut self) -> RXADC_VREF_SEL_W {
        RXADC_VREF_SEL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxadc_oscal_en(&mut self) -> RXADC_OSCAL_EN_W {
        RXADC_OSCAL_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rxadc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxadc](index.html) module"]
pub struct RXADC_SPEC;
impl crate::RegisterSpec for RXADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxadc::R](R) reader structure"]
impl crate::Readable for RXADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxadc::W](W) writer structure"]
impl crate::Writable for RXADC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rxadc to value 0"]
impl crate::Resettable for RXADC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
