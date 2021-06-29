#[doc = "Register `non_reg_readback` reader"]
pub struct R(crate::R<NON_REG_READBACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NON_REG_READBACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NON_REG_READBACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NON_REG_READBACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `non_reg_readback` writer"]
pub struct W(crate::W<NON_REG_READBACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NON_REG_READBACK_SPEC>;
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
impl From<crate::W<NON_REG_READBACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NON_REG_READBACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ppu_lna_hw` reader - "]
pub struct PPU_LNA_HW_R(crate::FieldReader<bool, bool>);
impl PPU_LNA_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_LNA_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_LNA_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_lna_hw` writer - "]
pub struct PPU_LNA_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_LNA_HW_W<'a> {
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
#[doc = "Field `ppu_rbb_hw` reader - "]
pub struct PPU_RBB_HW_R(crate::FieldReader<bool, bool>);
impl PPU_RBB_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_RBB_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_RBB_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_rbb_hw` writer - "]
pub struct PPU_RBB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_RBB_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ppu_lodist_body_bias_hw` reader - "]
pub struct PPU_LODIST_BODY_BIAS_HW_R(crate::FieldReader<bool, bool>);
impl PPU_LODIST_BODY_BIAS_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_LODIST_BODY_BIAS_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_LODIST_BODY_BIAS_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_lodist_body_bias_hw` writer - "]
pub struct PPU_LODIST_BODY_BIAS_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_LODIST_BODY_BIAS_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ppu_vco_ldo_hw` reader - "]
pub struct PPU_VCO_LDO_HW_R(crate::FieldReader<bool, bool>);
impl PPU_VCO_LDO_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_VCO_LDO_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_VCO_LDO_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_vco_ldo_hw` writer - "]
pub struct PPU_VCO_LDO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_VCO_LDO_HW_W<'a> {
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
#[doc = "Field `ppu_vco_hw` reader - "]
pub struct PPU_VCO_HW_R(crate::FieldReader<bool, bool>);
impl PPU_VCO_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_VCO_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_VCO_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_vco_hw` writer - "]
pub struct PPU_VCO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_VCO_HW_W<'a> {
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
#[doc = "Field `pud_vco_hw` reader - "]
pub struct PUD_VCO_HW_R(crate::FieldReader<bool, bool>);
impl PUD_VCO_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PUD_VCO_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUD_VCO_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pud_vco_hw` writer - "]
pub struct PUD_VCO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PUD_VCO_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `ppu_fbdv_hw` reader - "]
pub struct PPU_FBDV_HW_R(crate::FieldReader<bool, bool>);
impl PPU_FBDV_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_FBDV_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_FBDV_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_fbdv_hw` writer - "]
pub struct PPU_FBDV_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_FBDV_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ppu_adpll_sfreg_hw` reader - "]
pub struct PPU_ADPLL_SFREG_HW_R(crate::FieldReader<bool, bool>);
impl PPU_ADPLL_SFREG_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_ADPLL_SFREG_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_ADPLL_SFREG_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_adpll_sfreg_hw` writer - "]
pub struct PPU_ADPLL_SFREG_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_ADPLL_SFREG_HW_W<'a> {
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
#[doc = "Field `ppu_rxbuf_hw` reader - "]
pub struct PPU_RXBUF_HW_R(crate::FieldReader<bool, bool>);
impl PPU_RXBUF_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_RXBUF_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_RXBUF_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_rxbuf_hw` writer - "]
pub struct PPU_RXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_RXBUF_HW_W<'a> {
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
#[doc = "Field `ppu_txbuf_hw` reader - "]
pub struct PPU_TXBUF_HW_R(crate::FieldReader<bool, bool>);
impl PPU_TXBUF_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_TXBUF_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_TXBUF_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_txbuf_hw` writer - "]
pub struct PPU_TXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_TXBUF_HW_W<'a> {
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
#[doc = "Field `ppu_testbuf_hw` reader - "]
pub struct PPU_TESTBUF_HW_R(crate::FieldReader<bool, bool>);
impl PPU_TESTBUF_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPU_TESTBUF_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_TESTBUF_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_testbuf_hw` writer - "]
pub struct PPU_TESTBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_TESTBUF_HW_W<'a> {
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
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ppu_lna_hw(&self) -> PPU_LNA_HW_R {
        PPU_LNA_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ppu_rbb_hw(&self) -> PPU_RBB_HW_R {
        PPU_RBB_HW_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ppu_lodist_body_bias_hw(&self) -> PPU_LODIST_BODY_BIAS_HW_R {
        PPU_LODIST_BODY_BIAS_HW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ppu_vco_ldo_hw(&self) -> PPU_VCO_LDO_HW_R {
        PPU_VCO_LDO_HW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ppu_vco_hw(&self) -> PPU_VCO_HW_R {
        PPU_VCO_HW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pud_vco_hw(&self) -> PUD_VCO_HW_R {
        PUD_VCO_HW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ppu_fbdv_hw(&self) -> PPU_FBDV_HW_R {
        PPU_FBDV_HW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ppu_adpll_sfreg_hw(&self) -> PPU_ADPLL_SFREG_HW_R {
        PPU_ADPLL_SFREG_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ppu_rxbuf_hw(&self) -> PPU_RXBUF_HW_R {
        PPU_RXBUF_HW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ppu_txbuf_hw(&self) -> PPU_TXBUF_HW_R {
        PPU_TXBUF_HW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ppu_testbuf_hw(&self) -> PPU_TESTBUF_HW_R {
        PPU_TESTBUF_HW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ppu_lna_hw(&mut self) -> PPU_LNA_HW_W {
        PPU_LNA_HW_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ppu_rbb_hw(&mut self) -> PPU_RBB_HW_W {
        PPU_RBB_HW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ppu_lodist_body_bias_hw(&mut self) -> PPU_LODIST_BODY_BIAS_HW_W {
        PPU_LODIST_BODY_BIAS_HW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ppu_vco_ldo_hw(&mut self) -> PPU_VCO_LDO_HW_W {
        PPU_VCO_LDO_HW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ppu_vco_hw(&mut self) -> PPU_VCO_HW_W {
        PPU_VCO_HW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pud_vco_hw(&mut self) -> PUD_VCO_HW_W {
        PUD_VCO_HW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ppu_fbdv_hw(&mut self) -> PPU_FBDV_HW_W {
        PPU_FBDV_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ppu_adpll_sfreg_hw(&mut self) -> PPU_ADPLL_SFREG_HW_W {
        PPU_ADPLL_SFREG_HW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ppu_rxbuf_hw(&mut self) -> PPU_RXBUF_HW_W {
        PPU_RXBUF_HW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ppu_txbuf_hw(&mut self) -> PPU_TXBUF_HW_W {
        PPU_TXBUF_HW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ppu_testbuf_hw(&mut self) -> PPU_TESTBUF_HW_W {
        PPU_TESTBUF_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "non_reg_readback.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [non_reg_readback](index.html) module"]
pub struct NON_REG_READBACK_SPEC;
impl crate::RegisterSpec for NON_REG_READBACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [non_reg_readback::R](R) reader structure"]
impl crate::Readable for NON_REG_READBACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [non_reg_readback::W](W) writer structure"]
impl crate::Writable for NON_REG_READBACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets non_reg_readback to value 0"]
impl crate::Resettable for NON_REG_READBACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
