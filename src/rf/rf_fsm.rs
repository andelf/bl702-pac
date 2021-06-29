#[doc = "Register `rf_fsm` reader"]
pub struct R(crate::R<RF_FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm` writer"]
pub struct W(crate::W<RF_FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_SPEC>;
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
impl From<crate::W<RF_FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_fsm_afifo_dly_time` reader - "]
pub struct RF_FSM_AFIFO_DLY_TIME_R(crate::FieldReader<u8, u8>);
impl RF_FSM_AFIFO_DLY_TIME_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_FSM_AFIFO_DLY_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_AFIFO_DLY_TIME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_afifo_dly_time` writer - "]
pub struct RF_FSM_AFIFO_DLY_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_AFIFO_DLY_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
#[doc = "Field `rf_fsm_tx_afifo_4s_en` reader - "]
pub struct RF_FSM_TX_AFIFO_4S_EN_R(crate::FieldReader<bool, bool>);
impl RF_FSM_TX_AFIFO_4S_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_TX_AFIFO_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_TX_AFIFO_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_tx_afifo_4s_en` writer - "]
pub struct RF_FSM_TX_AFIFO_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_TX_AFIFO_4S_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `rf_fsm_tx_afifo_4s` reader - "]
pub struct RF_FSM_TX_AFIFO_4S_R(crate::FieldReader<bool, bool>);
impl RF_FSM_TX_AFIFO_4S_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_TX_AFIFO_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_TX_AFIFO_4S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_tx_afifo_4s` writer - "]
pub struct RF_FSM_TX_AFIFO_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_TX_AFIFO_4S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `rf_fsm_rx_afifo_4s_en` reader - "]
pub struct RF_FSM_RX_AFIFO_4S_EN_R(crate::FieldReader<bool, bool>);
impl RF_FSM_RX_AFIFO_4S_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_RX_AFIFO_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_RX_AFIFO_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_rx_afifo_4s_en` writer - "]
pub struct RF_FSM_RX_AFIFO_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_RX_AFIFO_4S_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `rf_fsm_rx_afifo_4s` reader - "]
pub struct RF_FSM_RX_AFIFO_4S_R(crate::FieldReader<bool, bool>);
impl RF_FSM_RX_AFIFO_4S_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_RX_AFIFO_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_RX_AFIFO_4S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_rx_afifo_4s` writer - "]
pub struct RF_FSM_RX_AFIFO_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_RX_AFIFO_4S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `rf_fsm_en` reader - "]
pub struct RF_FSM_EN_R(crate::FieldReader<bool, bool>);
impl RF_FSM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_en` writer - "]
pub struct RF_FSM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `rf_fsm_st_4s_en` reader - "]
pub struct RF_FSM_ST_4S_EN_R(crate::FieldReader<bool, bool>);
impl RF_FSM_ST_4S_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_ST_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_ST_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_st_4s_en` writer - "]
pub struct RF_FSM_ST_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_4S_EN_W<'a> {
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
#[doc = "Field `rf_fsm_st_4s` reader - "]
pub struct RF_FSM_ST_4S_R(crate::FieldReader<u8, u8>);
impl RF_FSM_ST_4S_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_FSM_ST_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_ST_4S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_st_4s` writer - "]
pub struct RF_FSM_ST_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `rf_fsm_state` reader - "]
pub struct RF_FSM_STATE_R(crate::FieldReader<u8, u8>);
impl RF_FSM_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_FSM_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_state` writer - "]
pub struct RF_FSM_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `rf_fsm_lo_time` reader - "]
pub struct RF_FSM_LO_TIME_R(crate::FieldReader<u16, u16>);
impl RF_FSM_LO_TIME_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_FSM_LO_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_LO_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_lo_time` writer - "]
pub struct RF_FSM_LO_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rf_fsm_afifo_dly_time(&self) -> RF_FSM_AFIFO_DLY_TIME_R {
        RF_FSM_AFIFO_DLY_TIME_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rf_fsm_tx_afifo_4s_en(&self) -> RF_FSM_TX_AFIFO_4S_EN_R {
        RF_FSM_TX_AFIFO_4S_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rf_fsm_tx_afifo_4s(&self) -> RF_FSM_TX_AFIFO_4S_R {
        RF_FSM_TX_AFIFO_4S_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rf_fsm_rx_afifo_4s_en(&self) -> RF_FSM_RX_AFIFO_4S_EN_R {
        RF_FSM_RX_AFIFO_4S_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rf_fsm_rx_afifo_4s(&self) -> RF_FSM_RX_AFIFO_4S_R {
        RF_FSM_RX_AFIFO_4S_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rf_fsm_en(&self) -> RF_FSM_EN_R {
        RF_FSM_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rf_fsm_st_4s_en(&self) -> RF_FSM_ST_4S_EN_R {
        RF_FSM_ST_4S_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_fsm_st_4s(&self) -> RF_FSM_ST_4S_R {
        RF_FSM_ST_4S_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_fsm_state(&self) -> RF_FSM_STATE_R {
        RF_FSM_STATE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn rf_fsm_lo_time(&self) -> RF_FSM_LO_TIME_R {
        RF_FSM_LO_TIME_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rf_fsm_afifo_dly_time(&mut self) -> RF_FSM_AFIFO_DLY_TIME_W {
        RF_FSM_AFIFO_DLY_TIME_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rf_fsm_tx_afifo_4s_en(&mut self) -> RF_FSM_TX_AFIFO_4S_EN_W {
        RF_FSM_TX_AFIFO_4S_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rf_fsm_tx_afifo_4s(&mut self) -> RF_FSM_TX_AFIFO_4S_W {
        RF_FSM_TX_AFIFO_4S_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rf_fsm_rx_afifo_4s_en(&mut self) -> RF_FSM_RX_AFIFO_4S_EN_W {
        RF_FSM_RX_AFIFO_4S_EN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rf_fsm_rx_afifo_4s(&mut self) -> RF_FSM_RX_AFIFO_4S_W {
        RF_FSM_RX_AFIFO_4S_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rf_fsm_en(&mut self) -> RF_FSM_EN_W {
        RF_FSM_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rf_fsm_st_4s_en(&mut self) -> RF_FSM_ST_4S_EN_W {
        RF_FSM_ST_4S_EN_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_fsm_st_4s(&mut self) -> RF_FSM_ST_4S_W {
        RF_FSM_ST_4S_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_fsm_state(&mut self) -> RF_FSM_STATE_W {
        RF_FSM_STATE_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn rf_fsm_lo_time(&mut self) -> RF_FSM_LO_TIME_W {
        RF_FSM_LO_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_fsm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm](index.html) module"]
pub struct RF_FSM_SPEC;
impl crate::RegisterSpec for RF_FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm::R](R) reader structure"]
impl crate::Readable for RF_FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm::W](W) writer structure"]
impl crate::Writable for RF_FSM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_fsm to value 0"]
impl crate::Resettable for RF_FSM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
