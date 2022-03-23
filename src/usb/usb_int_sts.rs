#[doc = "Register `usb_int_sts` reader"]
pub struct R(crate::R<USB_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_INT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_int_sts` writer"]
pub struct W(crate::W<USB_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_INT_STS_SPEC>;
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
impl From<crate::W<USB_INT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `usb_err_int` reader - "]
pub struct USB_ERR_INT_R(crate::FieldReader<bool, bool>);
impl USB_ERR_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_ERR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ERR_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_err_int` writer - "]
pub struct USB_ERR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ERR_INT_W<'a> {
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
#[doc = "Field `sof_3ms_int` reader - "]
pub struct SOF_3MS_INT_R(crate::FieldReader<bool, bool>);
impl SOF_3MS_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOF_3MS_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_3MS_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sof_3ms_int` writer - "]
pub struct SOF_3MS_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_3MS_INT_W<'a> {
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
#[doc = "Field `lpm_pkt_int` reader - "]
pub struct LPM_PKT_INT_R(crate::FieldReader<bool, bool>);
impl LPM_PKT_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPM_PKT_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_PKT_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lpm_pkt_int` writer - "]
pub struct LPM_PKT_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_PKT_INT_W<'a> {
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
#[doc = "Field `lpm_wkup_int` reader - "]
pub struct LPM_WKUP_INT_R(crate::FieldReader<bool, bool>);
impl LPM_WKUP_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPM_WKUP_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_WKUP_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lpm_wkup_int` writer - "]
pub struct LPM_WKUP_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_WKUP_INT_W<'a> {
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
    #[inline(always)]
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
#[doc = "Field `ep7_done_int` reader - "]
pub struct EP7_DONE_INT_R(crate::FieldReader<bool, bool>);
impl EP7_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP7_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep7_done_int` writer - "]
pub struct EP7_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_DONE_INT_W<'a> {
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
#[doc = "Field `ep7_cmd_int` reader - "]
pub struct EP7_CMD_INT_R(crate::FieldReader<bool, bool>);
impl EP7_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP7_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep7_cmd_int` writer - "]
pub struct EP7_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_CMD_INT_W<'a> {
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
#[doc = "Field `ep6_done_int` reader - "]
pub struct EP6_DONE_INT_R(crate::FieldReader<bool, bool>);
impl EP6_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP6_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep6_done_int` writer - "]
pub struct EP6_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_DONE_INT_W<'a> {
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
#[doc = "Field `ep6_cmd_int` reader - "]
pub struct EP6_CMD_INT_R(crate::FieldReader<bool, bool>);
impl EP6_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP6_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep6_cmd_int` writer - "]
pub struct EP6_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_CMD_INT_W<'a> {
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
#[doc = "Field `ep5_done_int` reader - "]
pub struct EP5_DONE_INT_R(crate::FieldReader<bool, bool>);
impl EP5_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP5_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep5_done_int` writer - "]
pub struct EP5_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_DONE_INT_W<'a> {
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
#[doc = "Field `ep5_cmd_int` reader - "]
pub struct EP5_CMD_INT_R(crate::FieldReader<bool, bool>);
impl EP5_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP5_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep5_cmd_int` writer - "]
pub struct EP5_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_CMD_INT_W<'a> {
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
#[doc = "Field `ep4_done_int` reader - "]
pub struct EP4_DONE_INT_R(crate::FieldReader<bool, bool>);
impl EP4_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP4_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep4_done_int` writer - "]
pub struct EP4_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_DONE_INT_W<'a> {
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
#[doc = "Field `ep4_cmd_int` reader - "]
pub struct EP4_CMD_INT_R(crate::FieldReader<bool, bool>);
impl EP4_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP4_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep4_cmd_int` writer - "]
pub struct EP4_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_CMD_INT_W<'a> {
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
#[doc = "Field `ep3_done_int` reader - "]
pub struct EP3_DONE_INT_R(crate::FieldReader<bool, bool>);
impl EP3_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP3_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep3_done_int` writer - "]
pub struct EP3_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_DONE_INT_W<'a> {
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
#[doc = "Field `ep3_cmd_int` reader - "]
pub struct EP3_CMD_INT_R(crate::FieldReader<bool, bool>);
impl EP3_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP3_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep3_cmd_int` writer - "]
pub struct EP3_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_CMD_INT_W<'a> {
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
#[doc = "Field `ep2_done_int` reader - "]
pub struct EP2_DONE_INT_R(crate::FieldReader<bool, bool>);
impl EP2_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP2_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep2_done_int` writer - "]
pub struct EP2_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_DONE_INT_W<'a> {
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
#[doc = "Field `ep2_cmd_int` reader - "]
pub struct EP2_CMD_INT_R(crate::FieldReader<bool, bool>);
impl EP2_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP2_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep2_cmd_int` writer - "]
pub struct EP2_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_CMD_INT_W<'a> {
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
#[doc = "Field `ep1_done_int` reader - "]
pub struct EP1_DONE_INT_R(crate::FieldReader<bool, bool>);
impl EP1_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP1_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep1_done_int` writer - "]
pub struct EP1_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_DONE_INT_W<'a> {
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
#[doc = "Field `ep1_cmd_int` reader - "]
pub struct EP1_CMD_INT_R(crate::FieldReader<bool, bool>);
impl EP1_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP1_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep1_cmd_int` writer - "]
pub struct EP1_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_CMD_INT_W<'a> {
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
#[doc = "Field `ep0_out_done_int` reader - "]
pub struct EP0_OUT_DONE_INT_R(crate::FieldReader<bool, bool>);
impl EP0_OUT_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_OUT_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_OUT_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep0_out_done_int` writer - "]
pub struct EP0_OUT_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_OUT_DONE_INT_W<'a> {
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
#[doc = "Field `ep0_out_cmd_int` reader - "]
pub struct EP0_OUT_CMD_INT_R(crate::FieldReader<bool, bool>);
impl EP0_OUT_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_OUT_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_OUT_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep0_out_cmd_int` writer - "]
pub struct EP0_OUT_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_OUT_CMD_INT_W<'a> {
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
#[doc = "Field `ep0_in_done_int` reader - "]
pub struct EP0_IN_DONE_INT_R(crate::FieldReader<bool, bool>);
impl EP0_IN_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_IN_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_IN_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep0_in_done_int` writer - "]
pub struct EP0_IN_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_IN_DONE_INT_W<'a> {
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
#[doc = "Field `ep0_in_cmd_int` reader - "]
pub struct EP0_IN_CMD_INT_R(crate::FieldReader<bool, bool>);
impl EP0_IN_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_IN_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_IN_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep0_in_cmd_int` writer - "]
pub struct EP0_IN_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_IN_CMD_INT_W<'a> {
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
#[doc = "Field `ep0_setup_done_int` reader - "]
pub struct EP0_SETUP_DONE_INT_R(crate::FieldReader<bool, bool>);
impl EP0_SETUP_DONE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_SETUP_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_SETUP_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep0_setup_done_int` writer - "]
pub struct EP0_SETUP_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_SETUP_DONE_INT_W<'a> {
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
#[doc = "Field `ep0_setup_cmd_int` reader - "]
pub struct EP0_SETUP_CMD_INT_R(crate::FieldReader<bool, bool>);
impl EP0_SETUP_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP0_SETUP_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP0_SETUP_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep0_setup_cmd_int` writer - "]
pub struct EP0_SETUP_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0_SETUP_CMD_INT_W<'a> {
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
#[doc = "Field `get_dct_cmd_int` reader - "]
pub struct GET_DCT_CMD_INT_R(crate::FieldReader<bool, bool>);
impl GET_DCT_CMD_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GET_DCT_CMD_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GET_DCT_CMD_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `get_dct_cmd_int` writer - "]
pub struct GET_DCT_CMD_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> GET_DCT_CMD_INT_W<'a> {
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
#[doc = "Field `vbus_tgl_int` reader - "]
pub struct VBUS_TGL_INT_R(crate::FieldReader<bool, bool>);
impl VBUS_TGL_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_TGL_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_TGL_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vbus_tgl_int` writer - "]
pub struct VBUS_TGL_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_TGL_INT_W<'a> {
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
#[doc = "Field `usb_reset_int` reader - "]
pub struct USB_RESET_INT_R(crate::FieldReader<bool, bool>);
impl USB_RESET_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_RESET_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_RESET_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_reset_int` writer - "]
pub struct USB_RESET_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_RESET_INT_W<'a> {
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
#[doc = "Field `sof_int` reader - "]
pub struct SOF_INT_R(crate::FieldReader<bool, bool>);
impl SOF_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SOF_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOF_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sof_int` writer - "]
pub struct SOF_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_INT_W<'a> {
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
    pub fn usb_err_int(&self) -> USB_ERR_INT_R {
        USB_ERR_INT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sof_3ms_int(&self) -> SOF_3MS_INT_R {
        SOF_3MS_INT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lpm_pkt_int(&self) -> LPM_PKT_INT_R {
        LPM_PKT_INT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lpm_wkup_int(&self) -> LPM_WKUP_INT_R {
        LPM_WKUP_INT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rsvd_27_24(&self) -> RSVD_27_24_R {
        RSVD_27_24_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ep7_done_int(&self) -> EP7_DONE_INT_R {
        EP7_DONE_INT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ep7_cmd_int(&self) -> EP7_CMD_INT_R {
        EP7_CMD_INT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ep6_done_int(&self) -> EP6_DONE_INT_R {
        EP6_DONE_INT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ep6_cmd_int(&self) -> EP6_CMD_INT_R {
        EP6_CMD_INT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ep5_done_int(&self) -> EP5_DONE_INT_R {
        EP5_DONE_INT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ep5_cmd_int(&self) -> EP5_CMD_INT_R {
        EP5_CMD_INT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ep4_done_int(&self) -> EP4_DONE_INT_R {
        EP4_DONE_INT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ep4_cmd_int(&self) -> EP4_CMD_INT_R {
        EP4_CMD_INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep3_done_int(&self) -> EP3_DONE_INT_R {
        EP3_DONE_INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep3_cmd_int(&self) -> EP3_CMD_INT_R {
        EP3_CMD_INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ep2_done_int(&self) -> EP2_DONE_INT_R {
        EP2_DONE_INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ep2_cmd_int(&self) -> EP2_CMD_INT_R {
        EP2_CMD_INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ep1_done_int(&self) -> EP1_DONE_INT_R {
        EP1_DONE_INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ep1_cmd_int(&self) -> EP1_CMD_INT_R {
        EP1_CMD_INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ep0_out_done_int(&self) -> EP0_OUT_DONE_INT_R {
        EP0_OUT_DONE_INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ep0_out_cmd_int(&self) -> EP0_OUT_CMD_INT_R {
        EP0_OUT_CMD_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ep0_in_done_int(&self) -> EP0_IN_DONE_INT_R {
        EP0_IN_DONE_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ep0_in_cmd_int(&self) -> EP0_IN_CMD_INT_R {
        EP0_IN_CMD_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ep0_setup_done_int(&self) -> EP0_SETUP_DONE_INT_R {
        EP0_SETUP_DONE_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ep0_setup_cmd_int(&self) -> EP0_SETUP_CMD_INT_R {
        EP0_SETUP_CMD_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn get_dct_cmd_int(&self) -> GET_DCT_CMD_INT_R {
        GET_DCT_CMD_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vbus_tgl_int(&self) -> VBUS_TGL_INT_R {
        VBUS_TGL_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn usb_reset_int(&self) -> USB_RESET_INT_R {
        USB_RESET_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sof_int(&self) -> SOF_INT_R {
        SOF_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn usb_err_int(&mut self) -> USB_ERR_INT_W {
        USB_ERR_INT_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sof_3ms_int(&mut self) -> SOF_3MS_INT_W {
        SOF_3MS_INT_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lpm_pkt_int(&mut self) -> LPM_PKT_INT_W {
        LPM_PKT_INT_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lpm_wkup_int(&mut self) -> LPM_WKUP_INT_W {
        LPM_WKUP_INT_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rsvd_27_24(&mut self) -> RSVD_27_24_W {
        RSVD_27_24_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ep7_done_int(&mut self) -> EP7_DONE_INT_W {
        EP7_DONE_INT_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ep7_cmd_int(&mut self) -> EP7_CMD_INT_W {
        EP7_CMD_INT_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ep6_done_int(&mut self) -> EP6_DONE_INT_W {
        EP6_DONE_INT_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ep6_cmd_int(&mut self) -> EP6_CMD_INT_W {
        EP6_CMD_INT_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ep5_done_int(&mut self) -> EP5_DONE_INT_W {
        EP5_DONE_INT_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ep5_cmd_int(&mut self) -> EP5_CMD_INT_W {
        EP5_CMD_INT_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ep4_done_int(&mut self) -> EP4_DONE_INT_W {
        EP4_DONE_INT_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ep4_cmd_int(&mut self) -> EP4_CMD_INT_W {
        EP4_CMD_INT_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep3_done_int(&mut self) -> EP3_DONE_INT_W {
        EP3_DONE_INT_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep3_cmd_int(&mut self) -> EP3_CMD_INT_W {
        EP3_CMD_INT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ep2_done_int(&mut self) -> EP2_DONE_INT_W {
        EP2_DONE_INT_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ep2_cmd_int(&mut self) -> EP2_CMD_INT_W {
        EP2_CMD_INT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ep1_done_int(&mut self) -> EP1_DONE_INT_W {
        EP1_DONE_INT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ep1_cmd_int(&mut self) -> EP1_CMD_INT_W {
        EP1_CMD_INT_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ep0_out_done_int(&mut self) -> EP0_OUT_DONE_INT_W {
        EP0_OUT_DONE_INT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ep0_out_cmd_int(&mut self) -> EP0_OUT_CMD_INT_W {
        EP0_OUT_CMD_INT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ep0_in_done_int(&mut self) -> EP0_IN_DONE_INT_W {
        EP0_IN_DONE_INT_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ep0_in_cmd_int(&mut self) -> EP0_IN_CMD_INT_W {
        EP0_IN_CMD_INT_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ep0_setup_done_int(&mut self) -> EP0_SETUP_DONE_INT_W {
        EP0_SETUP_DONE_INT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ep0_setup_cmd_int(&mut self) -> EP0_SETUP_CMD_INT_W {
        EP0_SETUP_CMD_INT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn get_dct_cmd_int(&mut self) -> GET_DCT_CMD_INT_W {
        GET_DCT_CMD_INT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vbus_tgl_int(&mut self) -> VBUS_TGL_INT_W {
        VBUS_TGL_INT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn usb_reset_int(&mut self) -> USB_RESET_INT_W {
        USB_RESET_INT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sof_int(&mut self) -> SOF_INT_W {
        SOF_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_int_sts](index.html) module"]
pub struct USB_INT_STS_SPEC;
impl crate::RegisterSpec for USB_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_int_sts::R](R) reader structure"]
impl crate::Readable for USB_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_int_sts::W](W) writer structure"]
impl crate::Writable for USB_INT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_int_sts to value 0"]
impl crate::Resettable for USB_INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
