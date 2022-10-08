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
#[doc = "Field `sts_frame_no` reader - "]
pub type STS_FRAME_NO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sts_frame_no` writer - "]
pub type STS_FRAME_NO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_FRAME_NO_SPEC, u16, u16, 11, O>;
#[doc = "Field `sts_pid` reader - "]
pub type STS_PID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_pid` writer - "]
pub type STS_PID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USB_FRAME_NO_SPEC, u8, u8, 4, O>;
#[doc = "Field `sts_ep_no` reader - "]
pub type STS_EP_NO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_ep_no` writer - "]
pub type STS_EP_NO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_FRAME_NO_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sts_frame_no(&self) -> STS_FRAME_NO_R {
        STS_FRAME_NO_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sts_pid(&self) -> STS_PID_R {
        STS_PID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sts_ep_no(&self) -> STS_EP_NO_R {
        STS_EP_NO_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn sts_frame_no(&mut self) -> STS_FRAME_NO_W<0> {
        STS_FRAME_NO_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sts_pid(&mut self) -> STS_PID_W<12> {
        STS_PID_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn sts_ep_no(&mut self) -> STS_EP_NO_W<16> {
        STS_EP_NO_W::new(self)
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
