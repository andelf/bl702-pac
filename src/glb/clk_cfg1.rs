#[doc = "Register `clk_cfg1` reader"]
pub struct R(crate::R<CLK_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg1` writer"]
pub struct W(crate::W<CLK_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG1_SPEC>;
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
impl From<crate::W<CLK_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_cam_ref_clk_div` reader - "]
pub struct REG_CAM_REF_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl REG_CAM_REF_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REG_CAM_REF_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CAM_REF_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_cam_ref_clk_div` writer - "]
pub struct REG_CAM_REF_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CAM_REF_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `reg_cam_ref_clk_src_sel` reader - "]
pub struct REG_CAM_REF_CLK_SRC_SEL_R(crate::FieldReader<bool, bool>);
impl REG_CAM_REF_CLK_SRC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_CAM_REF_CLK_SRC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CAM_REF_CLK_SRC_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_cam_ref_clk_src_sel` writer - "]
pub struct REG_CAM_REF_CLK_SRC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CAM_REF_CLK_SRC_SEL_W<'a> {
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
#[doc = "Field `reg_cam_ref_clk_en` reader - "]
pub struct REG_CAM_REF_CLK_EN_R(crate::FieldReader<bool, bool>);
impl REG_CAM_REF_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_CAM_REF_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CAM_REF_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_cam_ref_clk_en` writer - "]
pub struct REG_CAM_REF_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CAM_REF_CLK_EN_W<'a> {
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
#[doc = "Field `m154_zbEn` reader - "]
pub struct M154_ZBEN_R(crate::FieldReader<bool, bool>);
impl M154_ZBEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M154_ZBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M154_ZBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `m154_zbEn` writer - "]
pub struct M154_ZBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> M154_ZBEN_W<'a> {
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
#[doc = "Field `ble_en` reader - "]
pub struct BLE_EN_R(crate::FieldReader<bool, bool>);
impl BLE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ble_en` writer - "]
pub struct BLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_EN_W<'a> {
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
#[doc = "Field `ble_clk_sel` reader - "]
pub struct BLE_CLK_SEL_R(crate::FieldReader<u8, u8>);
impl BLE_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BLE_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_CLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ble_clk_sel` writer - "]
pub struct BLE_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_CLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `reg_i2s_0_ref_clk_oe` reader - "]
pub struct REG_I2S_0_REF_CLK_OE_R(crate::FieldReader<bool, bool>);
impl REG_I2S_0_REF_CLK_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_I2S_0_REF_CLK_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_I2S_0_REF_CLK_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_i2s_0_ref_clk_oe` writer - "]
pub struct REG_I2S_0_REF_CLK_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_I2S_0_REF_CLK_OE_W<'a> {
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
#[doc = "Field `reg_i2s0_clk_en` reader - "]
pub struct REG_I2S0_CLK_EN_R(crate::FieldReader<bool, bool>);
impl REG_I2S0_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_I2S0_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_I2S0_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_i2s0_clk_en` writer - "]
pub struct REG_I2S0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_I2S0_CLK_EN_W<'a> {
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
#[doc = "Field `reg_i2s_clk_sel` reader - "]
pub struct REG_I2S_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl REG_I2S_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_I2S_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_I2S_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_i2s_clk_sel` writer - "]
pub struct REG_I2S_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_I2S_CLK_SEL_W<'a> {
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
#[doc = "Field `dll_48m_div_en` reader - "]
pub struct DLL_48M_DIV_EN_R(crate::FieldReader<bool, bool>);
impl DLL_48M_DIV_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_48M_DIV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_48M_DIV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_48m_div_en` writer - "]
pub struct DLL_48M_DIV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_48M_DIV_EN_W<'a> {
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
#[doc = "Field `usb_clk_en` reader - "]
pub struct USB_CLK_EN_R(crate::FieldReader<bool, bool>);
impl USB_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_clk_en` writer - "]
pub struct USB_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_CLK_EN_W<'a> {
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
#[doc = "Field `qdec_clk_sel` reader - "]
pub struct QDEC_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl QDEC_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QDEC_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `qdec_clk_sel` writer - "]
pub struct QDEC_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_CLK_SEL_W<'a> {
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
#[doc = "Field `qdec_clk_div` reader - "]
pub struct QDEC_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl QDEC_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        QDEC_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `qdec_clk_div` writer - "]
pub struct QDEC_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_div(&self) -> REG_CAM_REF_CLK_DIV_R {
        REG_CAM_REF_CLK_DIV_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_src_sel(&self) -> REG_CAM_REF_CLK_SRC_SEL_R {
        REG_CAM_REF_CLK_SRC_SEL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_en(&self) -> REG_CAM_REF_CLK_EN_R {
        REG_CAM_REF_CLK_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn m154_zb_en(&self) -> M154_ZBEN_R {
        M154_ZBEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ble_en(&self) -> BLE_EN_R {
        BLE_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ble_clk_sel(&self) -> BLE_CLK_SEL_R {
        BLE_CLK_SEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_i2s_0_ref_clk_oe(&self) -> REG_I2S_0_REF_CLK_OE_R {
        REG_I2S_0_REF_CLK_OE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_i2s0_clk_en(&self) -> REG_I2S0_CLK_EN_R {
        REG_I2S0_CLK_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_i2s_clk_sel(&self) -> REG_I2S_CLK_SEL_R {
        REG_I2S_CLK_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dll_48m_div_en(&self) -> DLL_48M_DIV_EN_R {
        DLL_48M_DIV_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn usb_clk_en(&self) -> USB_CLK_EN_R {
        USB_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn qdec_clk_sel(&self) -> QDEC_CLK_SEL_R {
        QDEC_CLK_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn qdec_clk_div(&self) -> QDEC_CLK_DIV_R {
        QDEC_CLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_div(&mut self) -> REG_CAM_REF_CLK_DIV_W {
        REG_CAM_REF_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_src_sel(&mut self) -> REG_CAM_REF_CLK_SRC_SEL_W {
        REG_CAM_REF_CLK_SRC_SEL_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_en(&mut self) -> REG_CAM_REF_CLK_EN_W {
        REG_CAM_REF_CLK_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn m154_zb_en(&mut self) -> M154_ZBEN_W {
        M154_ZBEN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ble_en(&mut self) -> BLE_EN_W {
        BLE_EN_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ble_clk_sel(&mut self) -> BLE_CLK_SEL_W {
        BLE_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_i2s_0_ref_clk_oe(&mut self) -> REG_I2S_0_REF_CLK_OE_W {
        REG_I2S_0_REF_CLK_OE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_i2s0_clk_en(&mut self) -> REG_I2S0_CLK_EN_W {
        REG_I2S0_CLK_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_i2s_clk_sel(&mut self) -> REG_I2S_CLK_SEL_W {
        REG_I2S_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dll_48m_div_en(&mut self) -> DLL_48M_DIV_EN_W {
        DLL_48M_DIV_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn usb_clk_en(&mut self) -> USB_CLK_EN_W {
        USB_CLK_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn qdec_clk_sel(&mut self) -> QDEC_CLK_SEL_W {
        QDEC_CLK_SEL_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn qdec_clk_div(&mut self) -> QDEC_CLK_DIV_W {
        QDEC_CLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clk_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg1](index.html) module"]
pub struct CLK_CFG1_SPEC;
impl crate::RegisterSpec for CLK_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg1::R](R) reader structure"]
impl crate::Readable for CLK_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg1::W](W) writer structure"]
impl crate::Writable for CLK_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clk_cfg1 to value 0"]
impl crate::Resettable for CLK_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
