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
#[doc = "Field `sts_usb_ep0_sw_rdy` reader - "]
pub struct STS_USB_EP0_SW_RDY_R(crate::FieldReader<bool, bool>);
impl STS_USB_EP0_SW_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_USB_EP0_SW_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_USB_EP0_SW_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_usb_ep0_sw_rdy` writer - "]
pub struct STS_USB_EP0_SW_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_USB_EP0_SW_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `cr_usb_ep0_sw_rdy` reader - "]
pub struct CR_USB_EP0_SW_RDY_R(crate::FieldReader<bool, bool>);
impl CR_USB_EP0_SW_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_USB_EP0_SW_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_EP0_SW_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_ep0_sw_rdy` writer - "]
pub struct CR_USB_EP0_SW_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_EP0_SW_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `cr_usb_ep0_sw_nack_out` reader - "]
pub struct CR_USB_EP0_SW_NACK_OUT_R(crate::FieldReader<bool, bool>);
impl CR_USB_EP0_SW_NACK_OUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_USB_EP0_SW_NACK_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_EP0_SW_NACK_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_ep0_sw_nack_out` writer - "]
pub struct CR_USB_EP0_SW_NACK_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_EP0_SW_NACK_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `cr_usb_ep0_sw_nack_in` reader - "]
pub struct CR_USB_EP0_SW_NACK_IN_R(crate::FieldReader<bool, bool>);
impl CR_USB_EP0_SW_NACK_IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_USB_EP0_SW_NACK_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_EP0_SW_NACK_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_ep0_sw_nack_in` writer - "]
pub struct CR_USB_EP0_SW_NACK_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_EP0_SW_NACK_IN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `cr_usb_ep0_sw_stall` reader - "]
pub struct CR_USB_EP0_SW_STALL_R(crate::FieldReader<bool, bool>);
impl CR_USB_EP0_SW_STALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_USB_EP0_SW_STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_EP0_SW_STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_ep0_sw_stall` writer - "]
pub struct CR_USB_EP0_SW_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_EP0_SW_STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `cr_usb_ep0_sw_size` reader - "]
pub struct CR_USB_EP0_SW_SIZE_R(crate::FieldReader<u8, u8>);
impl CR_USB_EP0_SW_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_USB_EP0_SW_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_EP0_SW_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_ep0_sw_size` writer - "]
pub struct CR_USB_EP0_SW_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_EP0_SW_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `cr_usb_ep0_sw_addr` reader - "]
pub struct CR_USB_EP0_SW_ADDR_R(crate::FieldReader<u8, u8>);
impl CR_USB_EP0_SW_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_USB_EP0_SW_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_EP0_SW_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_ep0_sw_addr` writer - "]
pub struct CR_USB_EP0_SW_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_EP0_SW_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | ((value as u32 & 0x7f) << 9);
        self.w
    }
}
#[doc = "Field `cr_usb_ep0_sw_ctrl` reader - "]
pub struct CR_USB_EP0_SW_CTRL_R(crate::FieldReader<bool, bool>);
impl CR_USB_EP0_SW_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_USB_EP0_SW_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_EP0_SW_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_ep0_sw_ctrl` writer - "]
pub struct CR_USB_EP0_SW_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_EP0_SW_CTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `cr_usb_rom_dct_en` reader - "]
pub struct CR_USB_ROM_DCT_EN_R(crate::FieldReader<bool, bool>);
impl CR_USB_ROM_DCT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_USB_ROM_DCT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_ROM_DCT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_rom_dct_en` writer - "]
pub struct CR_USB_ROM_DCT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_ROM_DCT_EN_W<'a> {
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
#[doc = "Field `cr_usb_en` reader - "]
pub struct CR_USB_EN_R(crate::FieldReader<bool, bool>);
impl CR_USB_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_USB_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_en` writer - "]
pub struct CR_USB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_EN_W<'a> {
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
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sts_usb_ep0_sw_rdy(&self) -> STS_USB_EP0_SW_RDY_R {
        STS_USB_EP0_SW_RDY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_rdy(&self) -> CR_USB_EP0_SW_RDY_R {
        CR_USB_EP0_SW_RDY_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_nack_out(&self) -> CR_USB_EP0_SW_NACK_OUT_R {
        CR_USB_EP0_SW_NACK_OUT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_nack_in(&self) -> CR_USB_EP0_SW_NACK_IN_R {
        CR_USB_EP0_SW_NACK_IN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_stall(&self) -> CR_USB_EP0_SW_STALL_R {
        CR_USB_EP0_SW_STALL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_size(&self) -> CR_USB_EP0_SW_SIZE_R {
        CR_USB_EP0_SW_SIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_addr(&self) -> CR_USB_EP0_SW_ADDR_R {
        CR_USB_EP0_SW_ADDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_ctrl(&self) -> CR_USB_EP0_SW_CTRL_R {
        CR_USB_EP0_SW_CTRL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_usb_rom_dct_en(&self) -> CR_USB_ROM_DCT_EN_R {
        CR_USB_ROM_DCT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_usb_en(&self) -> CR_USB_EN_R {
        CR_USB_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sts_usb_ep0_sw_rdy(&mut self) -> STS_USB_EP0_SW_RDY_W {
        STS_USB_EP0_SW_RDY_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_rdy(&mut self) -> CR_USB_EP0_SW_RDY_W {
        CR_USB_EP0_SW_RDY_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_nack_out(&mut self) -> CR_USB_EP0_SW_NACK_OUT_W {
        CR_USB_EP0_SW_NACK_OUT_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_nack_in(&mut self) -> CR_USB_EP0_SW_NACK_IN_W {
        CR_USB_EP0_SW_NACK_IN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_stall(&mut self) -> CR_USB_EP0_SW_STALL_W {
        CR_USB_EP0_SW_STALL_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_size(&mut self) -> CR_USB_EP0_SW_SIZE_W {
        CR_USB_EP0_SW_SIZE_W { w: self }
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_addr(&mut self) -> CR_USB_EP0_SW_ADDR_W {
        CR_USB_EP0_SW_ADDR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_usb_ep0_sw_ctrl(&mut self) -> CR_USB_EP0_SW_CTRL_W {
        CR_USB_EP0_SW_CTRL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_usb_rom_dct_en(&mut self) -> CR_USB_ROM_DCT_EN_W {
        CR_USB_ROM_DCT_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_usb_en(&mut self) -> CR_USB_EN_W {
        CR_USB_EN_W { w: self }
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
}
#[doc = "`reset()` method sets usb_config to value 0"]
impl crate::Resettable for USB_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
