#[doc = "Register `usb_frame_no` reader"]
pub struct R(crate::R<USB_FRAME_NO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_FRAME_NO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_FRAME_NO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_FRAME_NO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_frame_no` writer"]
pub struct W(crate::W<USB_FRAME_NO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_FRAME_NO_SPEC>;
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
impl From<crate::W<USB_FRAME_NO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_FRAME_NO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_ep_no` reader - "]
pub struct STS_EP_NO_R(crate::FieldReader<u8, u8>);
impl STS_EP_NO_R {
    pub(crate) fn new(bits: u8) -> Self {
        STS_EP_NO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_EP_NO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_ep_no` writer - "]
pub struct STS_EP_NO_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_EP_NO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `sts_pid` reader - "]
pub struct STS_PID_R(crate::FieldReader<u8, u8>);
impl STS_PID_R {
    pub(crate) fn new(bits: u8) -> Self {
        STS_PID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_PID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_pid` writer - "]
pub struct STS_PID_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `sts_frame_no` reader - "]
pub struct STS_FRAME_NO_R(crate::FieldReader<u16, u16>);
impl STS_FRAME_NO_R {
    pub(crate) fn new(bits: u16) -> Self {
        STS_FRAME_NO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_FRAME_NO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_frame_no` writer - "]
pub struct STS_FRAME_NO_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_FRAME_NO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sts_ep_no(&self) -> STS_EP_NO_R {
        STS_EP_NO_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sts_pid(&self) -> STS_PID_R {
        STS_PID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sts_frame_no(&self) -> STS_FRAME_NO_R {
        STS_FRAME_NO_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sts_ep_no(&mut self) -> STS_EP_NO_W {
        STS_EP_NO_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sts_pid(&mut self) -> STS_PID_W {
        STS_PID_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sts_frame_no(&mut self) -> STS_FRAME_NO_W {
        STS_FRAME_NO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_frame_no.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_frame_no](index.html) module"]
pub struct USB_FRAME_NO_SPEC;
impl crate::RegisterSpec for USB_FRAME_NO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_frame_no::R](R) reader structure"]
impl crate::Readable for USB_FRAME_NO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_frame_no::W](W) writer structure"]
impl crate::Writable for USB_FRAME_NO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_frame_no to value 0"]
impl crate::Resettable for USB_FRAME_NO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
