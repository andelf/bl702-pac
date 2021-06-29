#[doc = "Register `usb_xcvr` reader"]
pub struct R(crate::R<USB_XCVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_XCVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_XCVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_XCVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_xcvr` writer"]
pub struct W(crate::W<USB_XCVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_XCVR_SPEC>;
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
impl From<crate::W<USB_XCVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_XCVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `usb_rcv` reader - "]
pub struct USB_RCV_R(crate::FieldReader<bool, bool>);
impl USB_RCV_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_RCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_RCV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_rcv` writer - "]
pub struct USB_RCV_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_RCV_W<'a> {
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
#[doc = "Field `usb_vip` reader - "]
pub struct USB_VIP_R(crate::FieldReader<bool, bool>);
impl USB_VIP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_VIP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_VIP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_vip` writer - "]
pub struct USB_VIP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_VIP_W<'a> {
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
#[doc = "Field `usb_vim` reader - "]
pub struct USB_VIM_R(crate::FieldReader<bool, bool>);
impl USB_VIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_VIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_VIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_vim` writer - "]
pub struct USB_VIM_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_VIM_W<'a> {
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
#[doc = "Field `usb_bd` reader - "]
pub struct USB_BD_R(crate::FieldReader<bool, bool>);
impl USB_BD_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_BD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_BD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_bd` writer - "]
pub struct USB_BD_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_BD_W<'a> {
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
#[doc = "Field `pu_usb` reader - "]
pub struct PU_USB_R(crate::FieldReader<bool, bool>);
impl PU_USB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_USB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_USB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_usb` writer - "]
pub struct PU_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_USB_W<'a> {
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
#[doc = "Field `usb_sus` reader - "]
pub struct USB_SUS_R(crate::FieldReader<bool, bool>);
impl USB_SUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_SUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_SUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_sus` writer - "]
pub struct USB_SUS_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_SUS_W<'a> {
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
#[doc = "Field `usb_spd` reader - "]
pub struct USB_SPD_R(crate::FieldReader<bool, bool>);
impl USB_SPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_SPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_SPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_spd` writer - "]
pub struct USB_SPD_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_SPD_W<'a> {
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
#[doc = "Field `usb_enum` reader - "]
pub struct USB_ENUM_R(crate::FieldReader<bool, bool>);
impl USB_ENUM_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_ENUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ENUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_enum` writer - "]
pub struct USB_ENUM_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ENUM_W<'a> {
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
#[doc = "Field `usb_data_convert` reader - "]
pub struct USB_DATA_CONVERT_R(crate::FieldReader<bool, bool>);
impl USB_DATA_CONVERT_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_DATA_CONVERT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_DATA_CONVERT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_data_convert` writer - "]
pub struct USB_DATA_CONVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_DATA_CONVERT_W<'a> {
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
#[doc = "Field `usb_oeb` reader - "]
pub struct USB_OEB_R(crate::FieldReader<bool, bool>);
impl USB_OEB_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_OEB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_OEB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_oeb` writer - "]
pub struct USB_OEB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_OEB_W<'a> {
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
#[doc = "Field `usb_oeb_reg` reader - "]
pub struct USB_OEB_REG_R(crate::FieldReader<bool, bool>);
impl USB_OEB_REG_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_OEB_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_OEB_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_oeb_reg` writer - "]
pub struct USB_OEB_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_OEB_REG_W<'a> {
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
#[doc = "Field `usb_oeb_sel` reader - "]
pub struct USB_OEB_SEL_R(crate::FieldReader<bool, bool>);
impl USB_OEB_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_OEB_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_OEB_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_oeb_sel` writer - "]
pub struct USB_OEB_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_OEB_SEL_W<'a> {
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
#[doc = "Field `usb_rout_pmos` reader - "]
pub struct USB_ROUT_PMOS_R(crate::FieldReader<u8, u8>);
impl USB_ROUT_PMOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        USB_ROUT_PMOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ROUT_PMOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_rout_pmos` writer - "]
pub struct USB_ROUT_PMOS_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ROUT_PMOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `usb_rout_nmos` reader - "]
pub struct USB_ROUT_NMOS_R(crate::FieldReader<u8, u8>);
impl USB_ROUT_NMOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        USB_ROUT_NMOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ROUT_NMOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_rout_nmos` writer - "]
pub struct USB_ROUT_NMOS_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ROUT_NMOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `pu_usb_ldo` reader - "]
pub struct PU_USB_LDO_R(crate::FieldReader<bool, bool>);
impl PU_USB_LDO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_USB_LDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_USB_LDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_usb_ldo` writer - "]
pub struct PU_USB_LDO_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_USB_LDO_W<'a> {
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
#[doc = "Field `usb_ldo_vfb` reader - "]
pub struct USB_LDO_VFB_R(crate::FieldReader<u8, u8>);
impl USB_LDO_VFB_R {
    pub(crate) fn new(bits: u8) -> Self {
        USB_LDO_VFB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_LDO_VFB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_ldo_vfb` writer - "]
pub struct USB_LDO_VFB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_LDO_VFB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn usb_rcv(&self) -> USB_RCV_R {
        USB_RCV_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usb_vip(&self) -> USB_VIP_R {
        USB_VIP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usb_vim(&self) -> USB_VIM_R {
        USB_VIM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usb_bd(&self) -> USB_BD_R {
        USB_BD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_usb(&self) -> PU_USB_R {
        PU_USB_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn usb_sus(&self) -> USB_SUS_R {
        USB_SUS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usb_spd(&self) -> USB_SPD_R {
        USB_SPD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn usb_enum(&self) -> USB_ENUM_R {
        USB_ENUM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn usb_data_convert(&self) -> USB_DATA_CONVERT_R {
        USB_DATA_CONVERT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usb_oeb(&self) -> USB_OEB_R {
        USB_OEB_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn usb_oeb_reg(&self) -> USB_OEB_REG_R {
        USB_OEB_REG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn usb_oeb_sel(&self) -> USB_OEB_SEL_R {
        USB_OEB_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn usb_rout_pmos(&self) -> USB_ROUT_PMOS_R {
        USB_ROUT_PMOS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn usb_rout_nmos(&self) -> USB_ROUT_NMOS_R {
        USB_ROUT_NMOS_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_usb_ldo(&self) -> PU_USB_LDO_R {
        PU_USB_LDO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn usb_ldo_vfb(&self) -> USB_LDO_VFB_R {
        USB_LDO_VFB_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn usb_rcv(&mut self) -> USB_RCV_W {
        USB_RCV_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usb_vip(&mut self) -> USB_VIP_W {
        USB_VIP_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usb_vim(&mut self) -> USB_VIM_W {
        USB_VIM_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usb_bd(&mut self) -> USB_BD_W {
        USB_BD_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_usb(&mut self) -> PU_USB_W {
        PU_USB_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn usb_sus(&mut self) -> USB_SUS_W {
        USB_SUS_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usb_spd(&mut self) -> USB_SPD_W {
        USB_SPD_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn usb_enum(&mut self) -> USB_ENUM_W {
        USB_ENUM_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn usb_data_convert(&mut self) -> USB_DATA_CONVERT_W {
        USB_DATA_CONVERT_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usb_oeb(&mut self) -> USB_OEB_W {
        USB_OEB_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn usb_oeb_reg(&mut self) -> USB_OEB_REG_W {
        USB_OEB_REG_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn usb_oeb_sel(&mut self) -> USB_OEB_SEL_W {
        USB_OEB_SEL_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn usb_rout_pmos(&mut self) -> USB_ROUT_PMOS_W {
        USB_ROUT_PMOS_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn usb_rout_nmos(&mut self) -> USB_ROUT_NMOS_W {
        USB_ROUT_NMOS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_usb_ldo(&mut self) -> PU_USB_LDO_W {
        PU_USB_LDO_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn usb_ldo_vfb(&mut self) -> USB_LDO_VFB_W {
        USB_LDO_VFB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_xcvr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_xcvr](index.html) module"]
pub struct USB_XCVR_SPEC;
impl crate::RegisterSpec for USB_XCVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_xcvr::R](R) reader structure"]
impl crate::Readable for USB_XCVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_xcvr::W](W) writer structure"]
impl crate::Writable for USB_XCVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_xcvr to value 0"]
impl crate::Resettable for USB_XCVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
