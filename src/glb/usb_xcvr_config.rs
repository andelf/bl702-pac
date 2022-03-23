#[doc = "Register `usb_xcvr_config` reader"]
pub struct R(crate::R<USB_XCVR_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_XCVR_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_XCVR_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_XCVR_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_xcvr_config` writer"]
pub struct W(crate::W<USB_XCVR_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_XCVR_CONFIG_SPEC>;
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
impl From<crate::W<USB_XCVR_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_XCVR_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `usb_slewrate_p_rise` reader - "]
pub struct USB_SLEWRATE_P_RISE_R(crate::FieldReader<u8, u8>);
impl USB_SLEWRATE_P_RISE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_SLEWRATE_P_RISE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_SLEWRATE_P_RISE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_slewrate_p_rise` writer - "]
pub struct USB_SLEWRATE_P_RISE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_SLEWRATE_P_RISE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `usb_slewrate_p_fall` reader - "]
pub struct USB_SLEWRATE_P_FALL_R(crate::FieldReader<u8, u8>);
impl USB_SLEWRATE_P_FALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_SLEWRATE_P_FALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_SLEWRATE_P_FALL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_slewrate_p_fall` writer - "]
pub struct USB_SLEWRATE_P_FALL_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_SLEWRATE_P_FALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `usb_slewrate_m_rise` reader - "]
pub struct USB_SLEWRATE_M_RISE_R(crate::FieldReader<u8, u8>);
impl USB_SLEWRATE_M_RISE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_SLEWRATE_M_RISE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_SLEWRATE_M_RISE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_slewrate_m_rise` writer - "]
pub struct USB_SLEWRATE_M_RISE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_SLEWRATE_M_RISE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `usb_slewrate_m_fall` reader - "]
pub struct USB_SLEWRATE_M_FALL_R(crate::FieldReader<u8, u8>);
impl USB_SLEWRATE_M_FALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_SLEWRATE_M_FALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_SLEWRATE_M_FALL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_slewrate_m_fall` writer - "]
pub struct USB_SLEWRATE_M_FALL_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_SLEWRATE_M_FALL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `usb_res_pullup_tune` reader - "]
pub struct USB_RES_PULLUP_TUNE_R(crate::FieldReader<u8, u8>);
impl USB_RES_PULLUP_TUNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_RES_PULLUP_TUNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_RES_PULLUP_TUNE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_res_pullup_tune` writer - "]
pub struct USB_RES_PULLUP_TUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_RES_PULLUP_TUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `reg_usb_use_ctrl` reader - "]
pub struct REG_USB_USE_CTRL_R(crate::FieldReader<bool, bool>);
impl REG_USB_USE_CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_USB_USE_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_USB_USE_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_usb_use_ctrl` writer - "]
pub struct REG_USB_USE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_USB_USE_CTRL_W<'a> {
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
#[doc = "Field `usb_str_drv` reader - "]
pub struct USB_STR_DRV_R(crate::FieldReader<u8, u8>);
impl USB_STR_DRV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_STR_DRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_STR_DRV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_str_drv` writer - "]
pub struct USB_STR_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_STR_DRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `reg_usb_use_xcvr` reader - "]
pub struct REG_USB_USE_XCVR_R(crate::FieldReader<bool, bool>);
impl REG_USB_USE_XCVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_USB_USE_XCVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_USB_USE_XCVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_usb_use_xcvr` writer - "]
pub struct REG_USB_USE_XCVR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_USB_USE_XCVR_W<'a> {
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
#[doc = "Field `usb_bd_vth` reader - "]
pub struct USB_BD_VTH_R(crate::FieldReader<u8, u8>);
impl USB_BD_VTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_BD_VTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_BD_VTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_bd_vth` writer - "]
pub struct USB_BD_VTH_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_BD_VTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `usb_v_hys_p` reader - "]
pub struct USB_V_HYS_P_R(crate::FieldReader<u8, u8>);
impl USB_V_HYS_P_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_V_HYS_P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_V_HYS_P_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_v_hys_p` writer - "]
pub struct USB_V_HYS_P_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_V_HYS_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `usb_v_hys_m` reader - "]
pub struct USB_V_HYS_M_R(crate::FieldReader<u8, u8>);
impl USB_V_HYS_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        USB_V_HYS_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_V_HYS_M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_v_hys_m` writer - "]
pub struct USB_V_HYS_M_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_V_HYS_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn usb_slewrate_p_rise(&self) -> USB_SLEWRATE_P_RISE_R {
        USB_SLEWRATE_P_RISE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn usb_slewrate_p_fall(&self) -> USB_SLEWRATE_P_FALL_R {
        USB_SLEWRATE_P_FALL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn usb_slewrate_m_rise(&self) -> USB_SLEWRATE_M_RISE_R {
        USB_SLEWRATE_M_RISE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn usb_slewrate_m_fall(&self) -> USB_SLEWRATE_M_FALL_R {
        USB_SLEWRATE_M_FALL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn usb_res_pullup_tune(&self) -> USB_RES_PULLUP_TUNE_R {
        USB_RES_PULLUP_TUNE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_usb_use_ctrl(&self) -> REG_USB_USE_CTRL_R {
        REG_USB_USE_CTRL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn usb_str_drv(&self) -> USB_STR_DRV_R {
        USB_STR_DRV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_usb_use_xcvr(&self) -> REG_USB_USE_XCVR_R {
        REG_USB_USE_XCVR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn usb_bd_vth(&self) -> USB_BD_VTH_R {
        USB_BD_VTH_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn usb_v_hys_p(&self) -> USB_V_HYS_P_R {
        USB_V_HYS_P_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn usb_v_hys_m(&self) -> USB_V_HYS_M_R {
        USB_V_HYS_M_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn usb_slewrate_p_rise(&mut self) -> USB_SLEWRATE_P_RISE_W {
        USB_SLEWRATE_P_RISE_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn usb_slewrate_p_fall(&mut self) -> USB_SLEWRATE_P_FALL_W {
        USB_SLEWRATE_P_FALL_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn usb_slewrate_m_rise(&mut self) -> USB_SLEWRATE_M_RISE_W {
        USB_SLEWRATE_M_RISE_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn usb_slewrate_m_fall(&mut self) -> USB_SLEWRATE_M_FALL_W {
        USB_SLEWRATE_M_FALL_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn usb_res_pullup_tune(&mut self) -> USB_RES_PULLUP_TUNE_W {
        USB_RES_PULLUP_TUNE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_usb_use_ctrl(&mut self) -> REG_USB_USE_CTRL_W {
        REG_USB_USE_CTRL_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn usb_str_drv(&mut self) -> USB_STR_DRV_W {
        USB_STR_DRV_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_usb_use_xcvr(&mut self) -> REG_USB_USE_XCVR_W {
        REG_USB_USE_XCVR_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn usb_bd_vth(&mut self) -> USB_BD_VTH_W {
        USB_BD_VTH_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn usb_v_hys_p(&mut self) -> USB_V_HYS_P_W {
        USB_V_HYS_P_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn usb_v_hys_m(&mut self) -> USB_V_HYS_M_W {
        USB_V_HYS_M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_xcvr_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_xcvr_config](index.html) module"]
pub struct USB_XCVR_CONFIG_SPEC;
impl crate::RegisterSpec for USB_XCVR_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_xcvr_config::R](R) reader structure"]
impl crate::Readable for USB_XCVR_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_xcvr_config::W](W) writer structure"]
impl crate::Writable for USB_XCVR_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_xcvr_config to value 0"]
impl crate::Resettable for USB_XCVR_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
