#[doc = "Register `usb_setup_data_0` reader"]
pub struct R(crate::R<USB_SETUP_DATA_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_SETUP_DATA_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_SETUP_DATA_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_SETUP_DATA_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_setup_data_0` writer"]
pub struct W(crate::W<USB_SETUP_DATA_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_SETUP_DATA_0_SPEC>;
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
impl From<crate::W<USB_SETUP_DATA_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_SETUP_DATA_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_setup_data_b3` reader - "]
pub struct STS_SETUP_DATA_B3_R(crate::FieldReader<u8, u8>);
impl STS_SETUP_DATA_B3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STS_SETUP_DATA_B3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_SETUP_DATA_B3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_setup_data_b3` writer - "]
pub struct STS_SETUP_DATA_B3_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_SETUP_DATA_B3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `sts_setup_data_b2` reader - "]
pub struct STS_SETUP_DATA_B2_R(crate::FieldReader<u8, u8>);
impl STS_SETUP_DATA_B2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STS_SETUP_DATA_B2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_SETUP_DATA_B2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_setup_data_b2` writer - "]
pub struct STS_SETUP_DATA_B2_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_SETUP_DATA_B2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `sts_setup_data_b1` reader - "]
pub struct STS_SETUP_DATA_B1_R(crate::FieldReader<u8, u8>);
impl STS_SETUP_DATA_B1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STS_SETUP_DATA_B1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_SETUP_DATA_B1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_setup_data_b1` writer - "]
pub struct STS_SETUP_DATA_B1_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_SETUP_DATA_B1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `sts_setup_data_b0` reader - "]
pub struct STS_SETUP_DATA_B0_R(crate::FieldReader<u8, u8>);
impl STS_SETUP_DATA_B0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STS_SETUP_DATA_B0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_SETUP_DATA_B0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_setup_data_b0` writer - "]
pub struct STS_SETUP_DATA_B0_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_SETUP_DATA_B0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sts_setup_data_b3(&self) -> STS_SETUP_DATA_B3_R {
        STS_SETUP_DATA_B3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sts_setup_data_b2(&self) -> STS_SETUP_DATA_B2_R {
        STS_SETUP_DATA_B2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sts_setup_data_b1(&self) -> STS_SETUP_DATA_B1_R {
        STS_SETUP_DATA_B1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sts_setup_data_b0(&self) -> STS_SETUP_DATA_B0_R {
        STS_SETUP_DATA_B0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sts_setup_data_b3(&mut self) -> STS_SETUP_DATA_B3_W {
        STS_SETUP_DATA_B3_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sts_setup_data_b2(&mut self) -> STS_SETUP_DATA_B2_W {
        STS_SETUP_DATA_B2_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sts_setup_data_b1(&mut self) -> STS_SETUP_DATA_B1_W {
        STS_SETUP_DATA_B1_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sts_setup_data_b0(&mut self) -> STS_SETUP_DATA_B0_W {
        STS_SETUP_DATA_B0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_setup_data_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_setup_data_0](index.html) module"]
pub struct USB_SETUP_DATA_0_SPEC;
impl crate::RegisterSpec for USB_SETUP_DATA_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_setup_data_0::R](R) reader structure"]
impl crate::Readable for USB_SETUP_DATA_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_setup_data_0::W](W) writer structure"]
impl crate::Writable for USB_SETUP_DATA_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_setup_data_0 to value 0"]
impl crate::Resettable for USB_SETUP_DATA_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
