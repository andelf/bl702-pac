#[doc = "Register `rf_top` reader"]
pub struct R(crate::R<RF_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_top` writer"]
pub struct W(crate::W<RF_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_TOP_SPEC>;
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
impl From<crate::W<RF_TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_tx_en_src` reader - "]
pub struct RF_TX_EN_SRC_R(crate::FieldReader<bool, bool>);
impl RF_TX_EN_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_TX_EN_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TX_EN_SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tx_en_src` writer - "]
pub struct RF_TX_EN_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TX_EN_SRC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `rf_tx_en_4s` reader - "]
pub struct RF_TX_EN_4S_R(crate::FieldReader<bool, bool>);
impl RF_TX_EN_4S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_TX_EN_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TX_EN_4S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tx_en_4s` writer - "]
pub struct RF_TX_EN_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TX_EN_4S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `rf_rx_en_src` reader - "]
pub struct RF_RX_EN_SRC_R(crate::FieldReader<bool, bool>);
impl RF_RX_EN_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_RX_EN_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RX_EN_SRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_rx_en_src` writer - "]
pub struct RF_RX_EN_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RX_EN_SRC_W<'a> {
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
#[doc = "Field `rf_rx_en_4s` reader - "]
pub struct RF_RX_EN_4S_R(crate::FieldReader<bool, bool>);
impl RF_RX_EN_4S_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_RX_EN_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RX_EN_4S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_rx_en_4s` writer - "]
pub struct RF_RX_EN_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RX_EN_4S_W<'a> {
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
#[doc = "Field `rf_rx_mode_4s_en` reader - "]
pub struct RF_RX_MODE_4S_EN_R(crate::FieldReader<bool, bool>);
impl RF_RX_MODE_4S_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RF_RX_MODE_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RX_MODE_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_rx_mode_4s_en` writer - "]
pub struct RF_RX_MODE_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RX_MODE_4S_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `rf_rx_mode_4s` reader - "]
pub struct RF_RX_MODE_4S_R(crate::FieldReader<u8, u8>);
impl RF_RX_MODE_4S_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_RX_MODE_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RX_MODE_4S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_rx_mode_4s` writer - "]
pub struct RF_RX_MODE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RX_MODE_4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `rf_rx_mode_hw` reader - "]
pub struct RF_RX_MODE_HW_R(crate::FieldReader<u8, u8>);
impl RF_RX_MODE_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_RX_MODE_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RX_MODE_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_rx_mode_hw` writer - "]
pub struct RF_RX_MODE_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RX_MODE_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `rf_mac_lo_time_offset` reader - "]
pub struct RF_MAC_LO_TIME_OFFSET_R(crate::FieldReader<u8, u8>);
impl RF_MAC_LO_TIME_OFFSET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_MAC_LO_TIME_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_MAC_LO_TIME_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_mac_lo_time_offset` writer - "]
pub struct RF_MAC_LO_TIME_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_MAC_LO_TIME_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `rfckg_afifo_tx_inv` reader - "]
pub struct RFCKG_AFIFO_TX_INV_R(crate::FieldReader<bool, bool>);
impl RFCKG_AFIFO_TX_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFCKG_AFIFO_TX_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCKG_AFIFO_TX_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfckg_afifo_tx_inv` writer - "]
pub struct RFCKG_AFIFO_TX_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_AFIFO_TX_INV_W<'a> {
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
#[doc = "Field `rfckg_afifo_rxadc_inv` reader - "]
pub struct RFCKG_AFIFO_RXADC_INV_R(crate::FieldReader<bool, bool>);
impl RFCKG_AFIFO_RXADC_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFCKG_AFIFO_RXADC_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCKG_AFIFO_RXADC_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfckg_afifo_rxadc_inv` writer - "]
pub struct RFCKG_AFIFO_RXADC_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_AFIFO_RXADC_INV_W<'a> {
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
#[doc = "Field `rfckg_afifo_adpll_inv` reader - "]
pub struct RFCKG_AFIFO_ADPLL_INV_R(crate::FieldReader<bool, bool>);
impl RFCKG_AFIFO_ADPLL_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFCKG_AFIFO_ADPLL_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCKG_AFIFO_ADPLL_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfckg_afifo_adpll_inv` writer - "]
pub struct RFCKG_AFIFO_ADPLL_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_AFIFO_ADPLL_INV_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_tx_en_src(&self) -> RF_TX_EN_SRC_R {
        RF_TX_EN_SRC_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_tx_en_4s(&self) -> RF_TX_EN_4S_R {
        RF_TX_EN_4S_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rf_rx_en_src(&self) -> RF_RX_EN_SRC_R {
        RF_RX_EN_SRC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rf_rx_en_4s(&self) -> RF_RX_EN_4S_R {
        RF_RX_EN_4S_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rf_rx_mode_4s_en(&self) -> RF_RX_MODE_4S_EN_R {
        RF_RX_MODE_4S_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rf_rx_mode_4s(&self) -> RF_RX_MODE_4S_R {
        RF_RX_MODE_4S_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn rf_rx_mode_hw(&self) -> RF_RX_MODE_HW_R {
        RF_RX_MODE_HW_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rf_mac_lo_time_offset(&self) -> RF_MAC_LO_TIME_OFFSET_R {
        RF_MAC_LO_TIME_OFFSET_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_afifo_tx_inv(&self) -> RFCKG_AFIFO_TX_INV_R {
        RFCKG_AFIFO_TX_INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_afifo_rxadc_inv(&self) -> RFCKG_AFIFO_RXADC_INV_R {
        RFCKG_AFIFO_RXADC_INV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_afifo_adpll_inv(&self) -> RFCKG_AFIFO_ADPLL_INV_R {
        RFCKG_AFIFO_ADPLL_INV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_tx_en_src(&mut self) -> RF_TX_EN_SRC_W {
        RF_TX_EN_SRC_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_tx_en_4s(&mut self) -> RF_TX_EN_4S_W {
        RF_TX_EN_4S_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rf_rx_en_src(&mut self) -> RF_RX_EN_SRC_W {
        RF_RX_EN_SRC_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rf_rx_en_4s(&mut self) -> RF_RX_EN_4S_W {
        RF_RX_EN_4S_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rf_rx_mode_4s_en(&mut self) -> RF_RX_MODE_4S_EN_W {
        RF_RX_MODE_4S_EN_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rf_rx_mode_4s(&mut self) -> RF_RX_MODE_4S_W {
        RF_RX_MODE_4S_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn rf_rx_mode_hw(&mut self) -> RF_RX_MODE_HW_W {
        RF_RX_MODE_HW_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rf_mac_lo_time_offset(&mut self) -> RF_MAC_LO_TIME_OFFSET_W {
        RF_MAC_LO_TIME_OFFSET_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_afifo_tx_inv(&mut self) -> RFCKG_AFIFO_TX_INV_W {
        RFCKG_AFIFO_TX_INV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_afifo_rxadc_inv(&mut self) -> RFCKG_AFIFO_RXADC_INV_W {
        RFCKG_AFIFO_RXADC_INV_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_afifo_adpll_inv(&mut self) -> RFCKG_AFIFO_ADPLL_INV_W {
        RFCKG_AFIFO_ADPLL_INV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_top.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_top](index.html) module"]
pub struct RF_TOP_SPEC;
impl crate::RegisterSpec for RF_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_top::R](R) reader structure"]
impl crate::Readable for RF_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_top::W](W) writer structure"]
impl crate::Writable for RF_TOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_top to value 0"]
impl crate::Resettable for RF_TOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
