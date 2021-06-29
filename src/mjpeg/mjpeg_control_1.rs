#[doc = "Register `mjpeg_control_1` reader"]
pub struct R(crate::R<MJPEG_CONTROL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_CONTROL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_CONTROL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_CONTROL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_control_1` writer"]
pub struct W(crate::W<MJPEG_CONTROL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_CONTROL_1_SPEC>;
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
impl From<crate::W<MJPEG_CONTROL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_CONTROL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_v0_order` reader - "]
pub struct REG_V0_ORDER_R(crate::FieldReader<u8, u8>);
impl REG_V0_ORDER_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_V0_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_V0_ORDER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_v0_order` writer - "]
pub struct REG_V0_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_V0_ORDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `reg_y1_order` reader - "]
pub struct REG_Y1_ORDER_R(crate::FieldReader<u8, u8>);
impl REG_Y1_ORDER_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_Y1_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_Y1_ORDER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_y1_order` writer - "]
pub struct REG_Y1_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_Y1_ORDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `reg_u0_order` reader - "]
pub struct REG_U0_ORDER_R(crate::FieldReader<u8, u8>);
impl REG_U0_ORDER_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_U0_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_U0_ORDER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_u0_order` writer - "]
pub struct REG_U0_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_U0_ORDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `reg_y0_order` reader - "]
pub struct REG_Y0_ORDER_R(crate::FieldReader<u8, u8>);
impl REG_Y0_ORDER_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_Y0_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_Y0_ORDER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_y0_order` writer - "]
pub struct REG_Y0_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_Y0_ORDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `reg_q_mode` reader - "]
pub struct REG_Q_MODE_R(crate::FieldReader<u8, u8>);
impl REG_Q_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_Q_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_Q_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_q_mode` writer - "]
pub struct REG_Q_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_Q_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `reg_yuv_mode` reader - "]
pub struct REG_YUV_MODE_R(crate::FieldReader<u8, u8>);
impl REG_YUV_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_YUV_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_YUV_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_yuv_mode` writer - "]
pub struct REG_YUV_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_YUV_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `reg_h_bust` reader - "]
pub struct REG_H_BUST_R(crate::FieldReader<u8, u8>);
impl REG_H_BUST_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_H_BUST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_H_BUST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_h_bust` writer - "]
pub struct REG_H_BUST_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_H_BUST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `reg_reflect_dmy` reader - "]
pub struct REG_REFLECT_DMY_R(crate::FieldReader<bool, bool>);
impl REG_REFLECT_DMY_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_REFLECT_DMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_REFLECT_DMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_reflect_dmy` writer - "]
pub struct REG_REFLECT_DMY_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_REFLECT_DMY_W<'a> {
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
#[doc = "Field `reg_last_hf_hblk_dmy` reader - "]
pub struct REG_LAST_HF_HBLK_DMY_R(crate::FieldReader<bool, bool>);
impl REG_LAST_HF_HBLK_DMY_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_LAST_HF_HBLK_DMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_LAST_HF_HBLK_DMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_last_hf_hblk_dmy` writer - "]
pub struct REG_LAST_HF_HBLK_DMY_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_LAST_HF_HBLK_DMY_W<'a> {
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
#[doc = "Field `reg_last_hf_wblk_dmy` reader - "]
pub struct REG_LAST_HF_WBLK_DMY_R(crate::FieldReader<bool, bool>);
impl REG_LAST_HF_WBLK_DMY_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_LAST_HF_WBLK_DMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_LAST_HF_WBLK_DMY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_last_hf_wblk_dmy` writer - "]
pub struct REG_LAST_HF_WBLK_DMY_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_LAST_HF_WBLK_DMY_W<'a> {
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
#[doc = "Field `reg_wr_over_stop` reader - "]
pub struct REG_WR_OVER_STOP_R(crate::FieldReader<bool, bool>);
impl REG_WR_OVER_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_WR_OVER_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_WR_OVER_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_wr_over_stop` writer - "]
pub struct REG_WR_OVER_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_WR_OVER_STOP_W<'a> {
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
#[doc = "Field `reg_order_u_even` reader - "]
pub struct REG_ORDER_U_EVEN_R(crate::FieldReader<bool, bool>);
impl REG_ORDER_U_EVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_ORDER_U_EVEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_ORDER_U_EVEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_order_u_even` writer - "]
pub struct REG_ORDER_U_EVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ORDER_U_EVEN_W<'a> {
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
#[doc = "Field `reg_mjpeg_bit_order` reader - "]
pub struct REG_MJPEG_BIT_ORDER_R(crate::FieldReader<bool, bool>);
impl REG_MJPEG_BIT_ORDER_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_MJPEG_BIT_ORDER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MJPEG_BIT_ORDER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mjpeg_bit_order` writer - "]
pub struct REG_MJPEG_BIT_ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MJPEG_BIT_ORDER_W<'a> {
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
#[doc = "Field `reg_mjpeg_enable` reader - "]
pub struct REG_MJPEG_ENABLE_R(crate::FieldReader<bool, bool>);
impl REG_MJPEG_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_MJPEG_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MJPEG_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mjpeg_enable` writer - "]
pub struct REG_MJPEG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MJPEG_ENABLE_W<'a> {
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
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_v0_order(&self) -> REG_V0_ORDER_R {
        REG_V0_ORDER_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_y1_order(&self) -> REG_Y1_ORDER_R {
        REG_Y1_ORDER_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reg_u0_order(&self) -> REG_U0_ORDER_R {
        REG_U0_ORDER_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn reg_y0_order(&self) -> REG_Y0_ORDER_R {
        REG_Y0_ORDER_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn reg_q_mode(&self) -> REG_Q_MODE_R {
        REG_Q_MODE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn reg_yuv_mode(&self) -> REG_YUV_MODE_R {
        REG_YUV_MODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reg_h_bust(&self) -> REG_H_BUST_R {
        REG_H_BUST_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_reflect_dmy(&self) -> REG_REFLECT_DMY_R {
        REG_REFLECT_DMY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_last_hf_hblk_dmy(&self) -> REG_LAST_HF_HBLK_DMY_R {
        REG_LAST_HF_HBLK_DMY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_last_hf_wblk_dmy(&self) -> REG_LAST_HF_WBLK_DMY_R {
        REG_LAST_HF_WBLK_DMY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_wr_over_stop(&self) -> REG_WR_OVER_STOP_R {
        REG_WR_OVER_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_order_u_even(&self) -> REG_ORDER_U_EVEN_R {
        REG_ORDER_U_EVEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mjpeg_bit_order(&self) -> REG_MJPEG_BIT_ORDER_R {
        REG_MJPEG_BIT_ORDER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mjpeg_enable(&self) -> REG_MJPEG_ENABLE_R {
        REG_MJPEG_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_v0_order(&mut self) -> REG_V0_ORDER_W {
        REG_V0_ORDER_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_y1_order(&mut self) -> REG_Y1_ORDER_W {
        REG_Y1_ORDER_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reg_u0_order(&mut self) -> REG_U0_ORDER_W {
        REG_U0_ORDER_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn reg_y0_order(&mut self) -> REG_Y0_ORDER_W {
        REG_Y0_ORDER_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn reg_q_mode(&mut self) -> REG_Q_MODE_W {
        REG_Q_MODE_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn reg_yuv_mode(&mut self) -> REG_YUV_MODE_W {
        REG_YUV_MODE_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reg_h_bust(&mut self) -> REG_H_BUST_W {
        REG_H_BUST_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_reflect_dmy(&mut self) -> REG_REFLECT_DMY_W {
        REG_REFLECT_DMY_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_last_hf_hblk_dmy(&mut self) -> REG_LAST_HF_HBLK_DMY_W {
        REG_LAST_HF_HBLK_DMY_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_last_hf_wblk_dmy(&mut self) -> REG_LAST_HF_WBLK_DMY_W {
        REG_LAST_HF_WBLK_DMY_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_wr_over_stop(&mut self) -> REG_WR_OVER_STOP_W {
        REG_WR_OVER_STOP_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_order_u_even(&mut self) -> REG_ORDER_U_EVEN_W {
        REG_ORDER_U_EVEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mjpeg_bit_order(&mut self) -> REG_MJPEG_BIT_ORDER_W {
        REG_MJPEG_BIT_ORDER_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mjpeg_enable(&mut self) -> REG_MJPEG_ENABLE_W {
        REG_MJPEG_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_control_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_control_1](index.html) module"]
pub struct MJPEG_CONTROL_1_SPEC;
impl crate::RegisterSpec for MJPEG_CONTROL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_control_1::R](R) reader structure"]
impl crate::Readable for MJPEG_CONTROL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_control_1::W](W) writer structure"]
impl crate::Writable for MJPEG_CONTROL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_control_1 to value 0"]
impl crate::Resettable for MJPEG_CONTROL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
