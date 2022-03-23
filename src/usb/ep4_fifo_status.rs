#[doc = "Register `ep4_fifo_status` reader"]
pub struct R(crate::R<EP4_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP4_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP4_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP4_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep4_fifo_status` writer"]
pub struct W(crate::W<EP4_FIFO_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP4_FIFO_STATUS_SPEC>;
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
impl From<crate::W<EP4_FIFO_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP4_FIFO_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ep4_rx_fifo_full` reader - "]
pub struct EP4_RX_FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl EP4_RX_FIFO_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP4_RX_FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_RX_FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep4_rx_fifo_full` writer - "]
pub struct EP4_RX_FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_RX_FIFO_FULL_W<'a> {
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
#[doc = "Field `ep4_rx_fifo_empty` reader - "]
pub struct EP4_RX_FIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl EP4_RX_FIFO_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP4_RX_FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_RX_FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep4_rx_fifo_empty` writer - "]
pub struct EP4_RX_FIFO_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_RX_FIFO_EMPTY_W<'a> {
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
#[doc = "Field `ep4_rx_fifo_cnt` reader - "]
pub struct EP4_RX_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl EP4_RX_FIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EP4_RX_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_RX_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep4_rx_fifo_cnt` writer - "]
pub struct EP4_RX_FIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_RX_FIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `ep4_tx_fifo_full` reader - "]
pub struct EP4_TX_FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl EP4_TX_FIFO_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP4_TX_FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_TX_FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep4_tx_fifo_full` writer - "]
pub struct EP4_TX_FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_TX_FIFO_FULL_W<'a> {
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
#[doc = "Field `ep4_tx_fifo_empty` reader - "]
pub struct EP4_TX_FIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl EP4_TX_FIFO_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP4_TX_FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_TX_FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep4_tx_fifo_empty` writer - "]
pub struct EP4_TX_FIFO_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_TX_FIFO_EMPTY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `ep4_tx_fifo_cnt` reader - "]
pub struct EP4_TX_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl EP4_TX_FIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EP4_TX_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_TX_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep4_tx_fifo_cnt` writer - "]
pub struct EP4_TX_FIFO_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_TX_FIFO_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ep4_rx_fifo_full(&self) -> EP4_RX_FIFO_FULL_R {
        EP4_RX_FIFO_FULL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ep4_rx_fifo_empty(&self) -> EP4_RX_FIFO_EMPTY_R {
        EP4_RX_FIFO_EMPTY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn ep4_rx_fifo_cnt(&self) -> EP4_RX_FIFO_CNT_R {
        EP4_RX_FIFO_CNT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep4_tx_fifo_full(&self) -> EP4_TX_FIFO_FULL_R {
        EP4_TX_FIFO_FULL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep4_tx_fifo_empty(&self) -> EP4_TX_FIFO_EMPTY_R {
        EP4_TX_FIFO_EMPTY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn ep4_tx_fifo_cnt(&self) -> EP4_TX_FIFO_CNT_R {
        EP4_TX_FIFO_CNT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ep4_rx_fifo_full(&mut self) -> EP4_RX_FIFO_FULL_W {
        EP4_RX_FIFO_FULL_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ep4_rx_fifo_empty(&mut self) -> EP4_RX_FIFO_EMPTY_W {
        EP4_RX_FIFO_EMPTY_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn ep4_rx_fifo_cnt(&mut self) -> EP4_RX_FIFO_CNT_W {
        EP4_RX_FIFO_CNT_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep4_tx_fifo_full(&mut self) -> EP4_TX_FIFO_FULL_W {
        EP4_TX_FIFO_FULL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep4_tx_fifo_empty(&mut self) -> EP4_TX_FIFO_EMPTY_W {
        EP4_TX_FIFO_EMPTY_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn ep4_tx_fifo_cnt(&mut self) -> EP4_TX_FIFO_CNT_W {
        EP4_TX_FIFO_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep4_fifo_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep4_fifo_status](index.html) module"]
pub struct EP4_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for EP4_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep4_fifo_status::R](R) reader structure"]
impl crate::Readable for EP4_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep4_fifo_status::W](W) writer structure"]
impl crate::Writable for EP4_FIFO_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ep4_fifo_status to value 0"]
impl crate::Resettable for EP4_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
