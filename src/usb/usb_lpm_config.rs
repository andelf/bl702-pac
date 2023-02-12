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
#[doc = "Field `cr_lpm_en` reader - "]
pub type CR_LPM_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_lpm_en` writer - "]
pub type CR_LPM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_LPM_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_lpm_resp_upd` reader - "]
pub type CR_LPM_RESP_UPD_R = crate::BitReader<bool>;
#[doc = "Field `cr_lpm_resp_upd` writer - "]
pub type CR_LPM_RESP_UPD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_LPM_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_lpm_resp` reader - "]
pub type CR_LPM_RESP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_lpm_resp` writer - "]
pub type CR_LPM_RESP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_LPM_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `sts_lpm_attr` reader - "]
pub type STS_LPM_ATTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sts_lpm_attr` writer - "]
pub type STS_LPM_ATTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_LPM_CONFIG_SPEC, u16, u16, 11, O>;
#[doc = "Field `sts_lpm` reader - "]
pub type STS_LPM_R = crate::BitReader<bool>;
#[doc = "Field `sts_lpm` writer - "]
pub type STS_LPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_LPM_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_lpm_en(&self) -> CR_LPM_EN_R {
        CR_LPM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_lpm_resp_upd(&self) -> CR_LPM_RESP_UPD_R {
        CR_LPM_RESP_UPD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_lpm_resp(&self) -> CR_LPM_RESP_R {
        CR_LPM_RESP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 20:30"]
    #[inline(always)]
    pub fn sts_lpm_attr(&self) -> STS_LPM_ATTR_R {
        STS_LPM_ATTR_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sts_lpm(&self) -> STS_LPM_R {
        STS_LPM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_lpm_en(&mut self) -> CR_LPM_EN_W<0> {
        CR_LPM_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_lpm_resp_upd(&mut self) -> CR_LPM_RESP_UPD_W<1> {
        CR_LPM_RESP_UPD_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_lpm_resp(&mut self) -> CR_LPM_RESP_W<2> {
        CR_LPM_RESP_W::new(self)
    }
    #[doc = "Bits 20:30"]
    #[inline(always)]
    #[must_use]
    pub fn sts_lpm_attr(&mut self) -> STS_LPM_ATTR_W<20> {
        STS_LPM_ATTR_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn sts_lpm(&mut self) -> STS_LPM_W<31> {
        STS_LPM_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usb_lpm_config to value 0"]
impl crate::Resettable for USB_LPM_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
