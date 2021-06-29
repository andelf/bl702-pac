#[doc = "Register `sf_if_iahb_12` reader"]
pub struct R(crate::R<SF_IF_IAHB_12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_IF_IAHB_12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_IF_IAHB_12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_IF_IAHB_12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_if_iahb_12` writer"]
pub struct W(crate::W<SF_IF_IAHB_12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_IF_IAHB_12_SPEC>;
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
impl From<crate::W<SF_IF_IAHB_12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_IF_IAHB_12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf2_if_read_dly_src` reader - "]
pub struct SF2_IF_READ_DLY_SRC_R(crate::FieldReader<bool, bool>);
impl SF2_IF_READ_DLY_SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF2_IF_READ_DLY_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF2_IF_READ_DLY_SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf2_if_read_dly_src` writer - "]
pub struct SF2_IF_READ_DLY_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_IF_READ_DLY_SRC_W<'a> {
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
#[doc = "Field `sf2_if_read_dly_en` reader - "]
pub struct SF2_IF_READ_DLY_EN_R(crate::FieldReader<bool, bool>);
impl SF2_IF_READ_DLY_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF2_IF_READ_DLY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF2_IF_READ_DLY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf2_if_read_dly_en` writer - "]
pub struct SF2_IF_READ_DLY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_IF_READ_DLY_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `sf2_if_read_dly_n` reader - "]
pub struct SF2_IF_READ_DLY_N_R(crate::FieldReader<u8, u8>);
impl SF2_IF_READ_DLY_N_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF2_IF_READ_DLY_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF2_IF_READ_DLY_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf2_if_read_dly_n` writer - "]
pub struct SF2_IF_READ_DLY_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_IF_READ_DLY_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `sf3_clk_out_inv_sel` reader - "]
pub struct SF3_CLK_OUT_INV_SEL_R(crate::FieldReader<bool, bool>);
impl SF3_CLK_OUT_INV_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF3_CLK_OUT_INV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF3_CLK_OUT_INV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf3_clk_out_inv_sel` writer - "]
pub struct SF3_CLK_OUT_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF3_CLK_OUT_INV_SEL_W<'a> {
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
#[doc = "Field `sf2_clk_out_inv_sel` reader - "]
pub struct SF2_CLK_OUT_INV_SEL_R(crate::FieldReader<bool, bool>);
impl SF2_CLK_OUT_INV_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF2_CLK_OUT_INV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF2_CLK_OUT_INV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf2_clk_out_inv_sel` writer - "]
pub struct SF2_CLK_OUT_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_CLK_OUT_INV_SEL_W<'a> {
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
#[doc = "Field `sf2_clk_sf_rx_inv_src` reader - "]
pub struct SF2_CLK_SF_RX_INV_SRC_R(crate::FieldReader<bool, bool>);
impl SF2_CLK_SF_RX_INV_SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF2_CLK_SF_RX_INV_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF2_CLK_SF_RX_INV_SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf2_clk_sf_rx_inv_src` writer - "]
pub struct SF2_CLK_SF_RX_INV_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_CLK_SF_RX_INV_SRC_W<'a> {
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
#[doc = "Field `sf2_clk_sf_rx_inv_sel` reader - "]
pub struct SF2_CLK_SF_RX_INV_SEL_R(crate::FieldReader<bool, bool>);
impl SF2_CLK_SF_RX_INV_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF2_CLK_SF_RX_INV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF2_CLK_SF_RX_INV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf2_clk_sf_rx_inv_sel` writer - "]
pub struct SF2_CLK_SF_RX_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF2_CLK_SF_RX_INV_SEL_W<'a> {
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
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sf2_if_read_dly_src(&self) -> SF2_IF_READ_DLY_SRC_R {
        SF2_IF_READ_DLY_SRC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf2_if_read_dly_en(&self) -> SF2_IF_READ_DLY_EN_R {
        SF2_IF_READ_DLY_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf2_if_read_dly_n(&self) -> SF2_IF_READ_DLY_N_R {
        SF2_IF_READ_DLY_N_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf3_clk_out_inv_sel(&self) -> SF3_CLK_OUT_INV_SEL_R {
        SF3_CLK_OUT_INV_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf2_clk_out_inv_sel(&self) -> SF2_CLK_OUT_INV_SEL_R {
        SF2_CLK_OUT_INV_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf2_clk_sf_rx_inv_src(&self) -> SF2_CLK_SF_RX_INV_SRC_R {
        SF2_CLK_SF_RX_INV_SRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf2_clk_sf_rx_inv_sel(&self) -> SF2_CLK_SF_RX_INV_SEL_R {
        SF2_CLK_SF_RX_INV_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sf2_if_read_dly_src(&mut self) -> SF2_IF_READ_DLY_SRC_W {
        SF2_IF_READ_DLY_SRC_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf2_if_read_dly_en(&mut self) -> SF2_IF_READ_DLY_EN_W {
        SF2_IF_READ_DLY_EN_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf2_if_read_dly_n(&mut self) -> SF2_IF_READ_DLY_N_W {
        SF2_IF_READ_DLY_N_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf3_clk_out_inv_sel(&mut self) -> SF3_CLK_OUT_INV_SEL_W {
        SF3_CLK_OUT_INV_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf2_clk_out_inv_sel(&mut self) -> SF2_CLK_OUT_INV_SEL_W {
        SF2_CLK_OUT_INV_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf2_clk_sf_rx_inv_src(&mut self) -> SF2_CLK_SF_RX_INV_SRC_W {
        SF2_CLK_SF_RX_INV_SRC_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf2_clk_sf_rx_inv_sel(&mut self) -> SF2_CLK_SF_RX_INV_SEL_W {
        SF2_CLK_SF_RX_INV_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_if_iahb_12.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_12](index.html) module"]
pub struct SF_IF_IAHB_12_SPEC;
impl crate::RegisterSpec for SF_IF_IAHB_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_if_iahb_12::R](R) reader structure"]
impl crate::Readable for SF_IF_IAHB_12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_12::W](W) writer structure"]
impl crate::Writable for SF_IF_IAHB_12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_if_iahb_12 to value 0"]
impl crate::Resettable for SF_IF_IAHB_12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
