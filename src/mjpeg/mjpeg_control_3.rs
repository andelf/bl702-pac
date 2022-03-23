#[doc = "Register `mjpeg_control_3` reader"]
pub struct R(crate::R<MJPEG_CONTROL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_CONTROL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_CONTROL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_CONTROL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_control_3` writer"]
pub struct W(crate::W<MJPEG_CONTROL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_CONTROL_3_SPEC>;
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
impl From<crate::W<MJPEG_CONTROL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_CONTROL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_swap_int` reader - "]
pub struct STS_SWAP_INT_R(crate::FieldReader<bool, bool>);
impl STS_SWAP_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STS_SWAP_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_SWAP_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_swap_int` writer - "]
pub struct STS_SWAP_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_SWAP_INT_W<'a> {
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
#[doc = "Field `reg_int_swap_en` reader - "]
pub struct REG_INT_SWAP_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_SWAP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_SWAP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_SWAP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_swap_en` writer - "]
pub struct REG_INT_SWAP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_SWAP_EN_W<'a> {
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
#[doc = "Field `frame_valid_cnt` reader - "]
pub struct FRAME_VALID_CNT_R(crate::FieldReader<u8, u8>);
impl FRAME_VALID_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRAME_VALID_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_VALID_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `frame_valid_cnt` writer - "]
pub struct FRAME_VALID_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_VALID_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `sts_idle_int` reader - "]
pub struct STS_IDLE_INT_R(crate::FieldReader<bool, bool>);
impl STS_IDLE_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STS_IDLE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_IDLE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_idle_int` writer - "]
pub struct STS_IDLE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_IDLE_INT_W<'a> {
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
#[doc = "Field `reg_int_idle_en` reader - "]
pub struct REG_INT_IDLE_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_IDLE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_IDLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_IDLE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_idle_en` writer - "]
pub struct REG_INT_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_IDLE_EN_W<'a> {
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
#[doc = "Field `reg_frame_cnt_trgr_int` reader - "]
pub struct REG_FRAME_CNT_TRGR_INT_R(crate::FieldReader<u8, u8>);
impl REG_FRAME_CNT_TRGR_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REG_FRAME_CNT_TRGR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_FRAME_CNT_TRGR_INT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_frame_cnt_trgr_int` writer - "]
pub struct REG_FRAME_CNT_TRGR_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_FRAME_CNT_TRGR_INT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `ahb_idle` reader - "]
pub struct AHB_IDLE_R(crate::FieldReader<bool, bool>);
impl AHB_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHB_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ahb_idle` writer - "]
pub struct AHB_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_IDLE_W<'a> {
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
#[doc = "Field `mjpeg_manf` reader - "]
pub struct MJPEG_MANF_R(crate::FieldReader<bool, bool>);
impl MJPEG_MANF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MJPEG_MANF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MJPEG_MANF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mjpeg_manf` writer - "]
pub struct MJPEG_MANF_W<'a> {
    w: &'a mut W,
}
impl<'a> MJPEG_MANF_W<'a> {
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
#[doc = "Field `mjpeg_mans` reader - "]
pub struct MJPEG_MANS_R(crate::FieldReader<bool, bool>);
impl MJPEG_MANS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MJPEG_MANS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MJPEG_MANS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mjpeg_mans` writer - "]
pub struct MJPEG_MANS_W<'a> {
    w: &'a mut W,
}
impl<'a> MJPEG_MANS_W<'a> {
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
#[doc = "Field `mjpeg_flsh` reader - "]
pub struct MJPEG_FLSH_R(crate::FieldReader<bool, bool>);
impl MJPEG_FLSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MJPEG_FLSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MJPEG_FLSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mjpeg_flsh` writer - "]
pub struct MJPEG_FLSH_W<'a> {
    w: &'a mut W,
}
impl<'a> MJPEG_FLSH_W<'a> {
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
#[doc = "Field `mjpeg_wait` reader - "]
pub struct MJPEG_WAIT_R(crate::FieldReader<bool, bool>);
impl MJPEG_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MJPEG_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MJPEG_WAIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mjpeg_wait` writer - "]
pub struct MJPEG_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> MJPEG_WAIT_W<'a> {
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
#[doc = "Field `mjpeg_func` reader - "]
pub struct MJPEG_FUNC_R(crate::FieldReader<bool, bool>);
impl MJPEG_FUNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MJPEG_FUNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MJPEG_FUNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mjpeg_func` writer - "]
pub struct MJPEG_FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> MJPEG_FUNC_W<'a> {
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
#[doc = "Field `mjpeg_idle` reader - "]
pub struct MJPEG_IDLE_R(crate::FieldReader<bool, bool>);
impl MJPEG_IDLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MJPEG_IDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MJPEG_IDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mjpeg_idle` writer - "]
pub struct MJPEG_IDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MJPEG_IDLE_W<'a> {
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
#[doc = "Field `sts_frame_int` reader - "]
pub struct STS_FRAME_INT_R(crate::FieldReader<bool, bool>);
impl STS_FRAME_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STS_FRAME_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_FRAME_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_frame_int` writer - "]
pub struct STS_FRAME_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_FRAME_INT_W<'a> {
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
#[doc = "Field `sts_mem_int` reader - "]
pub struct STS_MEM_INT_R(crate::FieldReader<bool, bool>);
impl STS_MEM_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STS_MEM_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_MEM_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_mem_int` writer - "]
pub struct STS_MEM_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_MEM_INT_W<'a> {
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
#[doc = "Field `sts_cam_int` reader - "]
pub struct STS_CAM_INT_R(crate::FieldReader<bool, bool>);
impl STS_CAM_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STS_CAM_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_CAM_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_cam_int` writer - "]
pub struct STS_CAM_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_CAM_INT_W<'a> {
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
#[doc = "Field `sts_normal_int` reader - "]
pub struct STS_NORMAL_INT_R(crate::FieldReader<bool, bool>);
impl STS_NORMAL_INT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STS_NORMAL_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_NORMAL_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_normal_int` writer - "]
pub struct STS_NORMAL_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_NORMAL_INT_W<'a> {
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
#[doc = "Field `reg_int_frame_en` reader - "]
pub struct REG_INT_FRAME_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_FRAME_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_FRAME_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_FRAME_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_frame_en` writer - "]
pub struct REG_INT_FRAME_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_FRAME_EN_W<'a> {
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
#[doc = "Field `reg_int_mem_en` reader - "]
pub struct REG_INT_MEM_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_MEM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_MEM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_MEM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_mem_en` writer - "]
pub struct REG_INT_MEM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_MEM_EN_W<'a> {
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
#[doc = "Field `reg_int_cam_en` reader - "]
pub struct REG_INT_CAM_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_CAM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_CAM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_CAM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_cam_en` writer - "]
pub struct REG_INT_CAM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_CAM_EN_W<'a> {
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
#[doc = "Field `reg_int_normal_en` reader - "]
pub struct REG_INT_NORMAL_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_NORMAL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_NORMAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_NORMAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_normal_en` writer - "]
pub struct REG_INT_NORMAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_NORMAL_EN_W<'a> {
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
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sts_swap_int(&self) -> STS_SWAP_INT_R {
        STS_SWAP_INT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_int_swap_en(&self) -> REG_INT_SWAP_EN_R {
        REG_INT_SWAP_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn frame_valid_cnt(&self) -> FRAME_VALID_CNT_R {
        FRAME_VALID_CNT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sts_idle_int(&self) -> STS_IDLE_INT_R {
        STS_IDLE_INT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_int_idle_en(&self) -> REG_INT_IDLE_EN_R {
        REG_INT_IDLE_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&self) -> REG_FRAME_CNT_TRGR_INT_R {
        REG_FRAME_CNT_TRGR_INT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ahb_idle(&self) -> AHB_IDLE_R {
        AHB_IDLE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn mjpeg_manf(&self) -> MJPEG_MANF_R {
        MJPEG_MANF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mjpeg_mans(&self) -> MJPEG_MANS_R {
        MJPEG_MANS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mjpeg_flsh(&self) -> MJPEG_FLSH_R {
        MJPEG_FLSH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mjpeg_wait(&self) -> MJPEG_WAIT_R {
        MJPEG_WAIT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn mjpeg_func(&self) -> MJPEG_FUNC_R {
        MJPEG_FUNC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mjpeg_idle(&self) -> MJPEG_IDLE_R {
        MJPEG_IDLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sts_frame_int(&self) -> STS_FRAME_INT_R {
        STS_FRAME_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sts_mem_int(&self) -> STS_MEM_INT_R {
        STS_MEM_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sts_cam_int(&self) -> STS_CAM_INT_R {
        STS_CAM_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sts_normal_int(&self) -> STS_NORMAL_INT_R {
        STS_NORMAL_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_int_frame_en(&self) -> REG_INT_FRAME_EN_R {
        REG_INT_FRAME_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_int_mem_en(&self) -> REG_INT_MEM_EN_R {
        REG_INT_MEM_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_int_cam_en(&self) -> REG_INT_CAM_EN_R {
        REG_INT_CAM_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_int_normal_en(&self) -> REG_INT_NORMAL_EN_R {
        REG_INT_NORMAL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sts_swap_int(&mut self) -> STS_SWAP_INT_W {
        STS_SWAP_INT_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_int_swap_en(&mut self) -> REG_INT_SWAP_EN_W {
        REG_INT_SWAP_EN_W { w: self }
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn frame_valid_cnt(&mut self) -> FRAME_VALID_CNT_W {
        FRAME_VALID_CNT_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sts_idle_int(&mut self) -> STS_IDLE_INT_W {
        STS_IDLE_INT_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_int_idle_en(&mut self) -> REG_INT_IDLE_EN_W {
        REG_INT_IDLE_EN_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&mut self) -> REG_FRAME_CNT_TRGR_INT_W {
        REG_FRAME_CNT_TRGR_INT_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ahb_idle(&mut self) -> AHB_IDLE_W {
        AHB_IDLE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn mjpeg_manf(&mut self) -> MJPEG_MANF_W {
        MJPEG_MANF_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mjpeg_mans(&mut self) -> MJPEG_MANS_W {
        MJPEG_MANS_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mjpeg_flsh(&mut self) -> MJPEG_FLSH_W {
        MJPEG_FLSH_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mjpeg_wait(&mut self) -> MJPEG_WAIT_W {
        MJPEG_WAIT_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn mjpeg_func(&mut self) -> MJPEG_FUNC_W {
        MJPEG_FUNC_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mjpeg_idle(&mut self) -> MJPEG_IDLE_W {
        MJPEG_IDLE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sts_frame_int(&mut self) -> STS_FRAME_INT_W {
        STS_FRAME_INT_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sts_mem_int(&mut self) -> STS_MEM_INT_W {
        STS_MEM_INT_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sts_cam_int(&mut self) -> STS_CAM_INT_W {
        STS_CAM_INT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sts_normal_int(&mut self) -> STS_NORMAL_INT_W {
        STS_NORMAL_INT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_int_frame_en(&mut self) -> REG_INT_FRAME_EN_W {
        REG_INT_FRAME_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_int_mem_en(&mut self) -> REG_INT_MEM_EN_W {
        REG_INT_MEM_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_int_cam_en(&mut self) -> REG_INT_CAM_EN_W {
        REG_INT_CAM_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_int_normal_en(&mut self) -> REG_INT_NORMAL_EN_W {
        REG_INT_NORMAL_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_control_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_control_3](index.html) module"]
pub struct MJPEG_CONTROL_3_SPEC;
impl crate::RegisterSpec for MJPEG_CONTROL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_control_3::R](R) reader structure"]
impl crate::Readable for MJPEG_CONTROL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_control_3::W](W) writer structure"]
impl crate::Writable for MJPEG_CONTROL_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_control_3 to value 0"]
impl crate::Resettable for MJPEG_CONTROL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
