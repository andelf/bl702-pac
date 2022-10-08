#[doc = "Register `usb_setup_data_1` reader"]
pub struct R(crate::R<USB_SETUP_DATA_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_SETUP_DATA_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_SETUP_DATA_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_SETUP_DATA_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_setup_data_1` writer"]
pub struct W(crate::W<USB_SETUP_DATA_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_SETUP_DATA_1_SPEC>;
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
impl From<crate::W<USB_SETUP_DATA_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_SETUP_DATA_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_setup_data_b4` reader - "]
pub type STS_SETUP_DATA_B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_setup_data_b4` writer - "]
pub type STS_SETUP_DATA_B4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_SETUP_DATA_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `sts_setup_data_b5` reader - "]
pub type STS_SETUP_DATA_B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_setup_data_b5` writer - "]
pub type STS_SETUP_DATA_B5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_SETUP_DATA_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `sts_setup_data_b6` reader - "]
pub type STS_SETUP_DATA_B6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_setup_data_b6` writer - "]
pub type STS_SETUP_DATA_B6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_SETUP_DATA_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `sts_setup_data_b7` reader - "]
pub type STS_SETUP_DATA_B7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sts_setup_data_b7` writer - "]
pub type STS_SETUP_DATA_B7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_SETUP_DATA_1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sts_setup_data_b4(&self) -> STS_SETUP_DATA_B4_R {
        STS_SETUP_DATA_B4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sts_setup_data_b5(&self) -> STS_SETUP_DATA_B5_R {
        STS_SETUP_DATA_B5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sts_setup_data_b6(&self) -> STS_SETUP_DATA_B6_R {
        STS_SETUP_DATA_B6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sts_setup_data_b7(&self) -> STS_SETUP_DATA_B7_R {
        STS_SETUP_DATA_B7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sts_setup_data_b4(&mut self) -> STS_SETUP_DATA_B4_W<0> {
        STS_SETUP_DATA_B4_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sts_setup_data_b5(&mut self) -> STS_SETUP_DATA_B5_W<8> {
        STS_SETUP_DATA_B5_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sts_setup_data_b6(&mut self) -> STS_SETUP_DATA_B6_W<16> {
        STS_SETUP_DATA_B6_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sts_setup_data_b7(&mut self) -> STS_SETUP_DATA_B7_W<24> {
        STS_SETUP_DATA_B7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_setup_data_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_setup_data_1](index.html) module"]
pub struct USB_SETUP_DATA_1_SPEC;
impl crate::RegisterSpec for USB_SETUP_DATA_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_setup_data_1::R](R) reader structure"]
impl crate::Readable for USB_SETUP_DATA_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_setup_data_1::W](W) writer structure"]
impl crate::Writable for USB_SETUP_DATA_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_setup_data_1 to value 0"]
impl crate::Resettable for USB_SETUP_DATA_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
