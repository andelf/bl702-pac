#[doc = "Register `usb_lpm_config` reader"]
pub struct R(crate::R<USB_LPM_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_LPM_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_LPM_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_LPM_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_lpm_config` writer"]
pub struct W(crate::W<USB_LPM_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_LPM_CONFIG_SPEC>;
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
impl From<crate::W<USB_LPM_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_LPM_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_lpm` reader - "]
pub struct STS_LPM_R(crate::FieldReader<bool, bool>);
impl STS_LPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_LPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_LPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_lpm` writer - "]
pub struct STS_LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_LPM_W<'a> {
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
#[doc = "Field `sts_lpm_attr` reader - "]
pub struct STS_LPM_ATTR_R(crate::FieldReader<u16, u16>);
impl STS_LPM_ATTR_R {
    pub(crate) fn new(bits: u16) -> Self {
        STS_LPM_ATTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_LPM_ATTR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_lpm_attr` writer - "]
pub struct STS_LPM_ATTR_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_LPM_ATTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 20)) | ((value as u32 & 0x07ff) << 20);
        self.w
    }
}
#[doc = "Field `cr_lpm_resp` reader - "]
pub struct CR_LPM_RESP_R(crate::FieldReader<u8, u8>);
impl CR_LPM_RESP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_LPM_RESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_LPM_RESP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_lpm_resp` writer - "]
pub struct CR_LPM_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_LPM_RESP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `cr_lpm_resp_upd` reader - "]
pub struct CR_LPM_RESP_UPD_R(crate::FieldReader<bool, bool>);
impl CR_LPM_RESP_UPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_LPM_RESP_UPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_LPM_RESP_UPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_lpm_resp_upd` writer - "]
pub struct CR_LPM_RESP_UPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_LPM_RESP_UPD_W<'a> {
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
#[doc = "Field `cr_lpm_en` reader - "]
pub struct CR_LPM_EN_R(crate::FieldReader<bool, bool>);
impl CR_LPM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_LPM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_LPM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_lpm_en` writer - "]
pub struct CR_LPM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_LPM_EN_W<'a> {
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
    pub fn sts_lpm(&self) -> STS_LPM_R {
        STS_LPM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 20:30"]
    #[inline(always)]
    pub fn sts_lpm_attr(&self) -> STS_LPM_ATTR_R {
        STS_LPM_ATTR_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_lpm_resp(&self) -> CR_LPM_RESP_R {
        CR_LPM_RESP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_lpm_resp_upd(&self) -> CR_LPM_RESP_UPD_R {
        CR_LPM_RESP_UPD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_lpm_en(&self) -> CR_LPM_EN_R {
        CR_LPM_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sts_lpm(&mut self) -> STS_LPM_W {
        STS_LPM_W { w: self }
    }
    #[doc = "Bits 20:30"]
    #[inline(always)]
    pub fn sts_lpm_attr(&mut self) -> STS_LPM_ATTR_W {
        STS_LPM_ATTR_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_lpm_resp(&mut self) -> CR_LPM_RESP_W {
        CR_LPM_RESP_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_lpm_resp_upd(&mut self) -> CR_LPM_RESP_UPD_W {
        CR_LPM_RESP_UPD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_lpm_en(&mut self) -> CR_LPM_EN_W {
        CR_LPM_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_lpm_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_lpm_config](index.html) module"]
pub struct USB_LPM_CONFIG_SPEC;
impl crate::RegisterSpec for USB_LPM_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_lpm_config::R](R) reader structure"]
impl crate::Readable for USB_LPM_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_lpm_config::W](W) writer structure"]
impl crate::Writable for USB_LPM_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_lpm_config to value 0"]
impl crate::Resettable for USB_LPM_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
