#[doc = "Register `usb_resume_config` reader"]
pub struct R(crate::R<USB_RESUME_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_RESUME_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_RESUME_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_RESUME_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_resume_config` writer"]
pub struct W(crate::W<USB_RESUME_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_RESUME_CONFIG_SPEC>;
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
impl From<crate::W<USB_RESUME_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_RESUME_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_res_width` reader - "]
pub type CR_RES_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_res_width` writer - "]
pub type CR_RES_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_RESUME_CONFIG_SPEC, u16, u16, 11, O>;
#[doc = "Field `cr_res_trig` reader - "]
pub type CR_RES_TRIG_R = crate::BitReader<bool>;
#[doc = "Field `cr_res_trig` writer - "]
pub type CR_RES_TRIG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_RESUME_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_res_force` reader - "]
pub type CR_RES_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `cr_res_force` writer - "]
pub type CR_RES_FORCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_RESUME_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn cr_res_width(&self) -> CR_RES_WIDTH_R {
        CR_RES_WIDTH_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_res_trig(&self) -> CR_RES_TRIG_R {
        CR_RES_TRIG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_res_force(&self) -> CR_RES_FORCE_R {
        CR_RES_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn cr_res_width(&mut self) -> CR_RES_WIDTH_W<0> {
        CR_RES_WIDTH_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_res_trig(&mut self) -> CR_RES_TRIG_W<12> {
        CR_RES_TRIG_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_res_force(&mut self) -> CR_RES_FORCE_W<31> {
        CR_RES_FORCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_resume_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_resume_config](index.html) module"]
pub struct USB_RESUME_CONFIG_SPEC;
impl crate::RegisterSpec for USB_RESUME_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_resume_config::R](R) reader structure"]
impl crate::Readable for USB_RESUME_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_resume_config::W](W) writer structure"]
impl crate::Writable for USB_RESUME_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_resume_config to value 0"]
impl crate::Resettable for USB_RESUME_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
