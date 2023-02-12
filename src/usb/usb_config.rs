#[doc = "Register `usb_config` reader"]
pub struct R(crate::R<USB_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_config` writer"]
pub struct W(crate::W<USB_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_CONFIG_SPEC>;
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
impl From<crate::W<USB_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_usb_en` reader - "]
pub type CR_USB_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_usb_en` writer - "]
pub type CR_USB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_usb_rom_dct_en` reader - "]
pub type CR_USB_ROM_DCT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_usb_rom_dct_en` writer - "]
pub type CR_USB_ROM_DCT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_usb_ep0_sw_ctrl` reader - "]
pub type CR_USB_EP0_SW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `cr_usb_ep0_sw_ctrl` writer - "]
pub type CR_USB_EP0_SW_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_usb_ep0_sw_addr` reader - "]
pub type CR_USB_EP0_SW_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_usb_ep0_sw_addr` writer - "]
pub type CR_USB_EP0_SW_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_CONFIG_SPEC, u8, u8, 7, O>;
#[doc = "Field `cr_usb_ep0_sw_size` reader - "]
pub type CR_USB_EP0_SW_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_usb_ep0_sw_size` writer - "]
pub type CR_USB_EP0_SW_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_usb_ep0_sw_stall` reader - "]
pub type CR_USB_EP0_SW_STALL_R = crate::BitReader<bool>;
#[doc = "Field `cr_usb_ep0_sw_stall` writer - "]
pub type CR_USB_EP0_SW_STALL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_usb_ep0_sw_nack_in` reader - "]
pub type CR_USB_EP0_SW_NACK_IN_R = crate::BitReader<bool>;
#[doc = "Field `cr_usb_ep0_sw_nack_in` writer - "]
pub type CR_USB_EP0_SW_NACK_IN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_usb_ep0_sw_nack_out` reader - "]
pub type CR_USB_EP0_SW_NACK_OUT_R = crate::BitReader<bool>;
#[doc = "Field `cr_usb_ep0_sw_nack_out` writer - "]
pub type CR_USB_EP0_SW_NACK_OUT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_usb_ep0_sw_rdy` reader - "]
pub type CR_USB_EP0_SW_RDY_R = crate::BitReader<bool>;
#[doc = "Field `cr_usb_ep0_sw_rdy` writer - "]
pub type CR_USB_EP0_SW_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_CONFIG_SPEC, bool, O>;
#[doc = "Field `sts_usb_ep0_sw_rdy` reader - "]
pub type STS_USB_EP0_SW_RDY_R = crate::BitReader<bool>;
#[doc = "Field `sts_usb_ep0_sw_rdy` writer - "]
pub type STS_USB_EP0_SW_RDY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_usb_en(&self) -> CR_USB_EN_R {
        CR_USB_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_usb_rom_dct_en(&self) -> CR_USB_ROM_DCT_EN_R {
        CR_USB_ROM_DCT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_ctrl(&self) -> CR_USB_EP0_SW_CTRL_R {
        CR_USB_EP0_SW_CTRL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_addr(&self) -> CR_USB_EP0_SW_ADDR_R {
        CR_USB_EP0_SW_ADDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_size(&self) -> CR_USB_EP0_SW_SIZE_R {
        CR_USB_EP0_SW_SIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_stall(&self) -> CR_USB_EP0_SW_STALL_R {
        CR_USB_EP0_SW_STALL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_nack_in(&self) -> CR_USB_EP0_SW_NACK_IN_R {
        CR_USB_EP0_SW_NACK_IN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_nack_out(&self) -> CR_USB_EP0_SW_NACK_OUT_R {
        CR_USB_EP0_SW_NACK_OUT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_rdy(&self) -> CR_USB_EP0_SW_RDY_R {
        CR_USB_EP0_SW_RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sts_usb_ep0_sw_rdy(&self) -> STS_USB_EP0_SW_RDY_R {
        STS_USB_EP0_SW_RDY_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_usb_en(&mut self) -> CR_USB_EN_W<0> {
        CR_USB_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn cr_usb_rom_dct_en(&mut self) -> CR_USB_ROM_DCT_EN_W<4> {
        CR_USB_ROM_DCT_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_usb_ep0_sw_ctrl(&mut self) -> CR_USB_EP0_SW_CTRL_W<8> {
        CR_USB_EP0_SW_CTRL_W::new(self)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_usb_ep0_sw_addr(&mut self) -> CR_USB_EP0_SW_ADDR_W<9> {
        CR_USB_EP0_SW_ADDR_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_usb_ep0_sw_size(&mut self) -> CR_USB_EP0_SW_SIZE_W<16> {
        CR_USB_EP0_SW_SIZE_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn cr_usb_ep0_sw_stall(&mut self) -> CR_USB_EP0_SW_STALL_W<24> {
        CR_USB_EP0_SW_STALL_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn cr_usb_ep0_sw_nack_in(&mut self) -> CR_USB_EP0_SW_NACK_IN_W<25> {
        CR_USB_EP0_SW_NACK_IN_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cr_usb_ep0_sw_nack_out(&mut self) -> CR_USB_EP0_SW_NACK_OUT_W<26> {
        CR_USB_EP0_SW_NACK_OUT_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_usb_ep0_sw_rdy(&mut self) -> CR_USB_EP0_SW_RDY_W<27> {
        CR_USB_EP0_SW_RDY_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn sts_usb_ep0_sw_rdy(&mut self) -> STS_USB_EP0_SW_RDY_W<28> {
        STS_USB_EP0_SW_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_config](index.html) module"]
pub struct USB_CONFIG_SPEC;
impl crate::RegisterSpec for USB_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_config::R](R) reader structure"]
impl crate::Readable for USB_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_config::W](W) writer structure"]
impl crate::Writable for USB_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usb_config to value 0"]
impl crate::Resettable for USB_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
