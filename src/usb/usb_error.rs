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
#[doc = "Field `utmi_rx_err` reader - "]
pub type UTMI_RX_ERR_R = crate::BitReader<bool>;
#[doc = "Field `utmi_rx_err` writer - "]
pub type UTMI_RX_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_ERROR_SPEC, bool, O>;
#[doc = "Field `xfer_to_err` reader - "]
pub type XFER_TO_ERR_R = crate::BitReader<bool>;
#[doc = "Field `xfer_to_err` writer - "]
pub type XFER_TO_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_ERROR_SPEC, bool, O>;
#[doc = "Field `ivld_ep_err` reader - "]
pub type IVLD_EP_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ivld_ep_err` writer - "]
pub type IVLD_EP_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_ERROR_SPEC, bool, O>;
#[doc = "Field `pid_seq_err` reader - "]
pub type PID_SEQ_ERR_R = crate::BitReader<bool>;
#[doc = "Field `pid_seq_err` writer - "]
pub type PID_SEQ_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_ERROR_SPEC, bool, O>;
#[doc = "Field `pid_cks_err` reader - "]
pub type PID_CKS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `pid_cks_err` writer - "]
pub type PID_CKS_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_ERROR_SPEC, bool, O>;
#[doc = "Field `crc5_err` reader - "]
pub type CRC5_ERR_R = crate::BitReader<bool>;
#[doc = "Field `crc5_err` writer - "]
pub type CRC5_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_ERROR_SPEC, bool, O>;
#[doc = "Field `crc16_err` reader - "]
pub type CRC16_ERR_R = crate::BitReader<bool>;
#[doc = "Field `crc16_err` writer - "]
pub type CRC16_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_ERROR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utmi_rx_err(&self) -> UTMI_RX_ERR_R {
        UTMI_RX_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_to_err(&self) -> XFER_TO_ERR_R {
        XFER_TO_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ivld_ep_err(&self) -> IVLD_EP_ERR_R {
        IVLD_EP_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pid_seq_err(&self) -> PID_SEQ_ERR_R {
        PID_SEQ_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pid_cks_err(&self) -> PID_CKS_ERR_R {
        PID_CKS_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crc5_err(&self) -> CRC5_ERR_R {
        CRC5_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crc16_err(&self) -> CRC16_ERR_R {
        CRC16_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utmi_rx_err(&mut self) -> UTMI_RX_ERR_W<0> {
        UTMI_RX_ERR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xfer_to_err(&mut self) -> XFER_TO_ERR_W<1> {
        XFER_TO_ERR_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ivld_ep_err(&mut self) -> IVLD_EP_ERR_W<2> {
        IVLD_EP_ERR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pid_seq_err(&mut self) -> PID_SEQ_ERR_W<3> {
        PID_SEQ_ERR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pid_cks_err(&mut self) -> PID_CKS_ERR_W<4> {
        PID_CKS_ERR_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crc5_err(&mut self) -> CRC5_ERR_W<5> {
        CRC5_ERR_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn crc16_err(&mut self) -> CRC16_ERR_W<6> {
        CRC16_ERR_W::new(self)
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
