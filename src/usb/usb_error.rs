#[doc = "Register `usb_error` reader"]
pub struct R(crate::R<USB_ERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_ERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_ERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_ERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_error` writer"]
pub struct W(crate::W<USB_ERROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_ERROR_SPEC>;
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
impl From<crate::W<USB_ERROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_ERROR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `crc16_err` reader - "]
pub struct CRC16_ERR_R(crate::FieldReader<bool, bool>);
impl CRC16_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC16_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC16_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `crc16_err` writer - "]
pub struct CRC16_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16_ERR_W<'a> {
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
#[doc = "Field `crc5_err` reader - "]
pub struct CRC5_ERR_R(crate::FieldReader<bool, bool>);
impl CRC5_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC5_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC5_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `crc5_err` writer - "]
pub struct CRC5_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC5_ERR_W<'a> {
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
#[doc = "Field `pid_cks_err` reader - "]
pub struct PID_CKS_ERR_R(crate::FieldReader<bool, bool>);
impl PID_CKS_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID_CKS_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_CKS_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pid_cks_err` writer - "]
pub struct PID_CKS_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_CKS_ERR_W<'a> {
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
#[doc = "Field `pid_seq_err` reader - "]
pub struct PID_SEQ_ERR_R(crate::FieldReader<bool, bool>);
impl PID_SEQ_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PID_SEQ_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID_SEQ_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pid_seq_err` writer - "]
pub struct PID_SEQ_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_SEQ_ERR_W<'a> {
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
#[doc = "Field `ivld_ep_err` reader - "]
pub struct IVLD_EP_ERR_R(crate::FieldReader<bool, bool>);
impl IVLD_EP_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IVLD_EP_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IVLD_EP_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ivld_ep_err` writer - "]
pub struct IVLD_EP_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> IVLD_EP_ERR_W<'a> {
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
#[doc = "Field `xfer_to_err` reader - "]
pub struct XFER_TO_ERR_R(crate::FieldReader<bool, bool>);
impl XFER_TO_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XFER_TO_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFER_TO_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xfer_to_err` writer - "]
pub struct XFER_TO_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> XFER_TO_ERR_W<'a> {
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
#[doc = "Field `utmi_rx_err` reader - "]
pub struct UTMI_RX_ERR_R(crate::FieldReader<bool, bool>);
impl UTMI_RX_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UTMI_RX_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTMI_RX_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `utmi_rx_err` writer - "]
pub struct UTMI_RX_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> UTMI_RX_ERR_W<'a> {
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
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crc16_err(&self) -> CRC16_ERR_R {
        CRC16_ERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crc5_err(&self) -> CRC5_ERR_R {
        CRC5_ERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pid_cks_err(&self) -> PID_CKS_ERR_R {
        PID_CKS_ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pid_seq_err(&self) -> PID_SEQ_ERR_R {
        PID_SEQ_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ivld_ep_err(&self) -> IVLD_EP_ERR_R {
        IVLD_EP_ERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_to_err(&self) -> XFER_TO_ERR_R {
        XFER_TO_ERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utmi_rx_err(&self) -> UTMI_RX_ERR_R {
        UTMI_RX_ERR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crc16_err(&mut self) -> CRC16_ERR_W {
        CRC16_ERR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crc5_err(&mut self) -> CRC5_ERR_W {
        CRC5_ERR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pid_cks_err(&mut self) -> PID_CKS_ERR_W {
        PID_CKS_ERR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pid_seq_err(&mut self) -> PID_SEQ_ERR_W {
        PID_SEQ_ERR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ivld_ep_err(&mut self) -> IVLD_EP_ERR_W {
        IVLD_EP_ERR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_to_err(&mut self) -> XFER_TO_ERR_W {
        XFER_TO_ERR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utmi_rx_err(&mut self) -> UTMI_RX_ERR_W {
        UTMI_RX_ERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_error](index.html) module"]
pub struct USB_ERROR_SPEC;
impl crate::RegisterSpec for USB_ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_error::R](R) reader structure"]
impl crate::Readable for USB_ERROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_error::W](W) writer structure"]
impl crate::Writable for USB_ERROR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_error to value 0"]
impl crate::Resettable for USB_ERROR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
