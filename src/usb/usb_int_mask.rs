#[doc = "Register `usb_int_mask` reader"]
pub struct R(crate::R<USB_INT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_INT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_INT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_INT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_int_mask` writer"]
pub struct W(crate::W<USB_INT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_INT_MASK_SPEC>;
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
impl From<crate::W<USB_INT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_INT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_usb_err_mask` reader - "]
pub struct CR_USB_ERR_MASK_R(crate::FieldReader<bool, bool>);
impl CR_USB_ERR_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_USB_ERR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_ERR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_err_mask` writer - "]
pub struct CR_USB_ERR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_ERR_MASK_W<'a> {
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
#[doc = "Field `cr_sof_3ms_mask` reader - "]
pub struct CR_SOF_3MS_MASK_R(crate::FieldReader<bool, bool>);
impl CR_SOF_3MS_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SOF_3MS_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SOF_3MS_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_sof_3ms_mask` writer - "]
pub struct CR_SOF_3MS_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SOF_3MS_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `cr_lpm_pkt_mask` reader - "]
pub struct CR_LPM_PKT_MASK_R(crate::FieldReader<bool, bool>);
impl CR_LPM_PKT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_LPM_PKT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_LPM_PKT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_lpm_pkt_mask` writer - "]
pub struct CR_LPM_PKT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_LPM_PKT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `cr_lpm_wkup_mask` reader - "]
pub struct CR_LPM_WKUP_MASK_R(crate::FieldReader<bool, bool>);
impl CR_LPM_WKUP_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_LPM_WKUP_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_LPM_WKUP_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_lpm_wkup_mask` writer - "]
pub struct CR_LPM_WKUP_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_LPM_WKUP_MASK_W<'a> {
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
#[doc = "Field `rsvd_27_24` reader - "]
pub struct RSVD_27_24_R(crate::FieldReader<u8, u8>);
impl RSVD_27_24_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSVD_27_24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_27_24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_27_24` writer - "]
pub struct RSVD_27_24_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_27_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `cr_ep7_done_mask` reader - "]
pub struct CR_EP7_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP7_DONE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP7_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP7_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep7_done_mask` writer - "]
pub struct CR_EP7_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP7_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `cr_ep7_cmd_mask` reader - "]
pub struct CR_EP7_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP7_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP7_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP7_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep7_cmd_mask` writer - "]
pub struct CR_EP7_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP7_CMD_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `cr_ep6_done_mask` reader - "]
pub struct CR_EP6_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP6_DONE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP6_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP6_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep6_done_mask` writer - "]
pub struct CR_EP6_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP6_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `cr_ep6_cmd_mask` reader - "]
pub struct CR_EP6_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP6_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP6_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP6_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep6_cmd_mask` writer - "]
pub struct CR_EP6_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP6_CMD_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `cr_ep5_done_mask` reader - "]
pub struct CR_EP5_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP5_DONE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP5_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP5_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep5_done_mask` writer - "]
pub struct CR_EP5_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP5_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `cr_ep5_cmd_mask` reader - "]
pub struct CR_EP5_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP5_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP5_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP5_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep5_cmd_mask` writer - "]
pub struct CR_EP5_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP5_CMD_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `cr_ep4_done_mask` reader - "]
pub struct CR_EP4_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP4_DONE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP4_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP4_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep4_done_mask` writer - "]
pub struct CR_EP4_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP4_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `cr_ep4_cmd_mask` reader - "]
pub struct CR_EP4_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP4_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP4_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP4_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep4_cmd_mask` writer - "]
pub struct CR_EP4_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP4_CMD_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `cr_ep3_done_mask` reader - "]
pub struct CR_EP3_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP3_DONE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP3_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP3_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep3_done_mask` writer - "]
pub struct CR_EP3_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP3_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `cr_ep3_cmd_mask` reader - "]
pub struct CR_EP3_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP3_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP3_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP3_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep3_cmd_mask` writer - "]
pub struct CR_EP3_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP3_CMD_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `cr_ep2_done_mask` reader - "]
pub struct CR_EP2_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP2_DONE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP2_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP2_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep2_done_mask` writer - "]
pub struct CR_EP2_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP2_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `cr_ep2_cmd_mask` reader - "]
pub struct CR_EP2_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP2_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP2_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP2_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep2_cmd_mask` writer - "]
pub struct CR_EP2_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP2_CMD_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `cr_ep1_done_mask` reader - "]
pub struct CR_EP1_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP1_DONE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP1_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP1_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep1_done_mask` writer - "]
pub struct CR_EP1_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP1_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `cr_ep1_cmd_mask` reader - "]
pub struct CR_EP1_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP1_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP1_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP1_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep1_cmd_mask` writer - "]
pub struct CR_EP1_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP1_CMD_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `cr_ep0_out_done_mask` reader - "]
pub struct CR_EP0_OUT_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP0_OUT_DONE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP0_OUT_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP0_OUT_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep0_out_done_mask` writer - "]
pub struct CR_EP0_OUT_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP0_OUT_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `cr_ep0_out_cmd_mask` reader - "]
pub struct CR_EP0_OUT_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP0_OUT_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP0_OUT_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP0_OUT_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep0_out_cmd_mask` writer - "]
pub struct CR_EP0_OUT_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP0_OUT_CMD_MASK_W<'a> {
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
#[doc = "Field `cr_ep0_in_done_mask` reader - "]
pub struct CR_EP0_IN_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP0_IN_DONE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP0_IN_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP0_IN_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep0_in_done_mask` writer - "]
pub struct CR_EP0_IN_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP0_IN_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `cr_ep0_in_cmd_mask` reader - "]
pub struct CR_EP0_IN_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP0_IN_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP0_IN_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP0_IN_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep0_in_cmd_mask` writer - "]
pub struct CR_EP0_IN_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP0_IN_CMD_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `cr_ep0_setup_done_mask` reader - "]
pub struct CR_EP0_SETUP_DONE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP0_SETUP_DONE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP0_SETUP_DONE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP0_SETUP_DONE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep0_setup_done_mask` writer - "]
pub struct CR_EP0_SETUP_DONE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP0_SETUP_DONE_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `cr_ep0_setup_cmd_mask` reader - "]
pub struct CR_EP0_SETUP_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_EP0_SETUP_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_EP0_SETUP_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_EP0_SETUP_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ep0_setup_cmd_mask` writer - "]
pub struct CR_EP0_SETUP_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_EP0_SETUP_CMD_MASK_W<'a> {
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
#[doc = "Field `cr_get_dct_cmd_mask` reader - "]
pub struct CR_GET_DCT_CMD_MASK_R(crate::FieldReader<bool, bool>);
impl CR_GET_DCT_CMD_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_GET_DCT_CMD_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_GET_DCT_CMD_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_get_dct_cmd_mask` writer - "]
pub struct CR_GET_DCT_CMD_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_GET_DCT_CMD_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `cr_vbus_tgl_mask` reader - "]
pub struct CR_VBUS_TGL_MASK_R(crate::FieldReader<bool, bool>);
impl CR_VBUS_TGL_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_VBUS_TGL_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_VBUS_TGL_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_vbus_tgl_mask` writer - "]
pub struct CR_VBUS_TGL_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_VBUS_TGL_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `cr_usb_reset_mask` reader - "]
pub struct CR_USB_RESET_MASK_R(crate::FieldReader<bool, bool>);
impl CR_USB_RESET_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_USB_RESET_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_USB_RESET_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_usb_reset_mask` writer - "]
pub struct CR_USB_RESET_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_USB_RESET_MASK_W<'a> {
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
#[doc = "Field `cr_sof_mask` reader - "]
pub struct CR_SOF_MASK_R(crate::FieldReader<bool, bool>);
impl CR_SOF_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SOF_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SOF_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_sof_mask` writer - "]
pub struct CR_SOF_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SOF_MASK_W<'a> {
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
    pub fn cr_usb_err_mask(&self) -> CR_USB_ERR_MASK_R {
        CR_USB_ERR_MASK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_sof_3ms_mask(&self) -> CR_SOF_3MS_MASK_R {
        CR_SOF_3MS_MASK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_lpm_pkt_mask(&self) -> CR_LPM_PKT_MASK_R {
        CR_LPM_PKT_MASK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_lpm_wkup_mask(&self) -> CR_LPM_WKUP_MASK_R {
        CR_LPM_WKUP_MASK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rsvd_27_24(&self) -> RSVD_27_24_R {
        RSVD_27_24_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_ep7_done_mask(&self) -> CR_EP7_DONE_MASK_R {
        CR_EP7_DONE_MASK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_ep7_cmd_mask(&self) -> CR_EP7_CMD_MASK_R {
        CR_EP7_CMD_MASK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_ep6_done_mask(&self) -> CR_EP6_DONE_MASK_R {
        CR_EP6_DONE_MASK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_ep6_cmd_mask(&self) -> CR_EP6_CMD_MASK_R {
        CR_EP6_CMD_MASK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_ep5_done_mask(&self) -> CR_EP5_DONE_MASK_R {
        CR_EP5_DONE_MASK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_ep5_cmd_mask(&self) -> CR_EP5_CMD_MASK_R {
        CR_EP5_CMD_MASK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_ep4_done_mask(&self) -> CR_EP4_DONE_MASK_R {
        CR_EP4_DONE_MASK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_ep4_cmd_mask(&self) -> CR_EP4_CMD_MASK_R {
        CR_EP4_CMD_MASK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_ep3_done_mask(&self) -> CR_EP3_DONE_MASK_R {
        CR_EP3_DONE_MASK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_ep3_cmd_mask(&self) -> CR_EP3_CMD_MASK_R {
        CR_EP3_CMD_MASK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_ep2_done_mask(&self) -> CR_EP2_DONE_MASK_R {
        CR_EP2_DONE_MASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_ep2_cmd_mask(&self) -> CR_EP2_CMD_MASK_R {
        CR_EP2_CMD_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_ep1_done_mask(&self) -> CR_EP1_DONE_MASK_R {
        CR_EP1_DONE_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_ep1_cmd_mask(&self) -> CR_EP1_CMD_MASK_R {
        CR_EP1_CMD_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_ep0_out_done_mask(&self) -> CR_EP0_OUT_DONE_MASK_R {
        CR_EP0_OUT_DONE_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_ep0_out_cmd_mask(&self) -> CR_EP0_OUT_CMD_MASK_R {
        CR_EP0_OUT_CMD_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_ep0_in_done_mask(&self) -> CR_EP0_IN_DONE_MASK_R {
        CR_EP0_IN_DONE_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_ep0_in_cmd_mask(&self) -> CR_EP0_IN_CMD_MASK_R {
        CR_EP0_IN_CMD_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_ep0_setup_done_mask(&self) -> CR_EP0_SETUP_DONE_MASK_R {
        CR_EP0_SETUP_DONE_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_ep0_setup_cmd_mask(&self) -> CR_EP0_SETUP_CMD_MASK_R {
        CR_EP0_SETUP_CMD_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_get_dct_cmd_mask(&self) -> CR_GET_DCT_CMD_MASK_R {
        CR_GET_DCT_CMD_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_vbus_tgl_mask(&self) -> CR_VBUS_TGL_MASK_R {
        CR_VBUS_TGL_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_usb_reset_mask(&self) -> CR_USB_RESET_MASK_R {
        CR_USB_RESET_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_sof_mask(&self) -> CR_SOF_MASK_R {
        CR_SOF_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_usb_err_mask(&mut self) -> CR_USB_ERR_MASK_W {
        CR_USB_ERR_MASK_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_sof_3ms_mask(&mut self) -> CR_SOF_3MS_MASK_W {
        CR_SOF_3MS_MASK_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_lpm_pkt_mask(&mut self) -> CR_LPM_PKT_MASK_W {
        CR_LPM_PKT_MASK_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_lpm_wkup_mask(&mut self) -> CR_LPM_WKUP_MASK_W {
        CR_LPM_WKUP_MASK_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rsvd_27_24(&mut self) -> RSVD_27_24_W {
        RSVD_27_24_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_ep7_done_mask(&mut self) -> CR_EP7_DONE_MASK_W {
        CR_EP7_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_ep7_cmd_mask(&mut self) -> CR_EP7_CMD_MASK_W {
        CR_EP7_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_ep6_done_mask(&mut self) -> CR_EP6_DONE_MASK_W {
        CR_EP6_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_ep6_cmd_mask(&mut self) -> CR_EP6_CMD_MASK_W {
        CR_EP6_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_ep5_done_mask(&mut self) -> CR_EP5_DONE_MASK_W {
        CR_EP5_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_ep5_cmd_mask(&mut self) -> CR_EP5_CMD_MASK_W {
        CR_EP5_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_ep4_done_mask(&mut self) -> CR_EP4_DONE_MASK_W {
        CR_EP4_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_ep4_cmd_mask(&mut self) -> CR_EP4_CMD_MASK_W {
        CR_EP4_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_ep3_done_mask(&mut self) -> CR_EP3_DONE_MASK_W {
        CR_EP3_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_ep3_cmd_mask(&mut self) -> CR_EP3_CMD_MASK_W {
        CR_EP3_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_ep2_done_mask(&mut self) -> CR_EP2_DONE_MASK_W {
        CR_EP2_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_ep2_cmd_mask(&mut self) -> CR_EP2_CMD_MASK_W {
        CR_EP2_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_ep1_done_mask(&mut self) -> CR_EP1_DONE_MASK_W {
        CR_EP1_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_ep1_cmd_mask(&mut self) -> CR_EP1_CMD_MASK_W {
        CR_EP1_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_ep0_out_done_mask(&mut self) -> CR_EP0_OUT_DONE_MASK_W {
        CR_EP0_OUT_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_ep0_out_cmd_mask(&mut self) -> CR_EP0_OUT_CMD_MASK_W {
        CR_EP0_OUT_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_ep0_in_done_mask(&mut self) -> CR_EP0_IN_DONE_MASK_W {
        CR_EP0_IN_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_ep0_in_cmd_mask(&mut self) -> CR_EP0_IN_CMD_MASK_W {
        CR_EP0_IN_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_ep0_setup_done_mask(&mut self) -> CR_EP0_SETUP_DONE_MASK_W {
        CR_EP0_SETUP_DONE_MASK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_ep0_setup_cmd_mask(&mut self) -> CR_EP0_SETUP_CMD_MASK_W {
        CR_EP0_SETUP_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_get_dct_cmd_mask(&mut self) -> CR_GET_DCT_CMD_MASK_W {
        CR_GET_DCT_CMD_MASK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_vbus_tgl_mask(&mut self) -> CR_VBUS_TGL_MASK_W {
        CR_VBUS_TGL_MASK_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_usb_reset_mask(&mut self) -> CR_USB_RESET_MASK_W {
        CR_USB_RESET_MASK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_sof_mask(&mut self) -> CR_SOF_MASK_W {
        CR_SOF_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_int_mask](index.html) module"]
pub struct USB_INT_MASK_SPEC;
impl crate::RegisterSpec for USB_INT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_int_mask::R](R) reader structure"]
impl crate::Readable for USB_INT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_int_mask::W](W) writer structure"]
impl crate::Writable for USB_INT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_int_mask to value 0"]
impl crate::Resettable for USB_INT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
