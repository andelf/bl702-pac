#[doc = "Register `rbb` reader"]
pub struct R(crate::R<RBB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb` writer"]
pub struct W(crate::W<RBB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_SPEC>;
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
impl From<crate::W<RBB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rbb_bm_op` reader - "]
pub struct RBB_BM_OP_R(crate::FieldReader<u8, u8>);
impl RBB_BM_OP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_BM_OP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BM_OP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bm_op` writer - "]
pub struct RBB_BM_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BM_OP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `rbb_vcm` reader - "]
pub struct RBB_VCM_R(crate::FieldReader<u8, u8>);
impl RBB_VCM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_VCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_VCM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_vcm` writer - "]
pub struct RBB_VCM_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_VCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `rbb_deq` reader - "]
pub struct RBB_DEQ_R(crate::FieldReader<u8, u8>);
impl RBB_DEQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_DEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_DEQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_deq` writer - "]
pub struct RBB_DEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_DEQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `rbb_lpf_en` reader - "]
pub struct RBB_LPF_EN_R(crate::FieldReader<bool, bool>);
impl RBB_LPF_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_LPF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_LPF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_lpf_en` writer - "]
pub struct RBB_LPF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_LPF_EN_W<'a> {
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
#[doc = "Field `rosdac_range` reader - "]
pub struct ROSDAC_RANGE_R(crate::FieldReader<u8, u8>);
impl ROSDAC_RANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROSDAC_RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_RANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_range` writer - "]
pub struct ROSDAC_RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_RANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `rbb_pkdet_vth` reader - "]
pub struct RBB_PKDET_VTH_R(crate::FieldReader<u8, u8>);
impl RBB_PKDET_VTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_PKDET_VTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_VTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_vth` writer - "]
pub struct RBB_PKDET_VTH_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_VTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `rbb_pkdet_en` reader - "]
pub struct RBB_PKDET_EN_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_en` writer - "]
pub struct RBB_PKDET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_EN_W<'a> {
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
#[doc = "Field `rbb_pkdet_en_hw` reader - "]
pub struct RBB_PKDET_EN_HW_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_EN_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_en_hw` writer - "]
pub struct RBB_PKDET_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_EN_HW_W<'a> {
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
#[doc = "Field `rbb_pkdet_en_ctrl_hw` reader - "]
pub struct RBB_PKDET_EN_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_EN_CTRL_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_EN_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_EN_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_en_ctrl_hw` writer - "]
pub struct RBB_PKDET_EN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_EN_CTRL_HW_W<'a> {
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
#[doc = "Field `rbb_pkdet_out_rstn` reader - "]
pub struct RBB_PKDET_OUT_RSTN_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_OUT_RSTN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_OUT_RSTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_OUT_RSTN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_out_rstn` writer - "]
pub struct RBB_PKDET_OUT_RSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_OUT_RSTN_W<'a> {
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
#[doc = "Field `rbb_pkdet_out_rstn_hw` reader - "]
pub struct RBB_PKDET_OUT_RSTN_HW_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_OUT_RSTN_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_OUT_RSTN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_OUT_RSTN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_out_rstn_hw` writer - "]
pub struct RBB_PKDET_OUT_RSTN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_OUT_RSTN_HW_W<'a> {
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
#[doc = "Field `rbb_pkdet_out_rstn_ctrl_hw` reader - "]
pub struct RBB_PKDET_OUT_RSTN_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_OUT_RSTN_CTRL_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_OUT_RSTN_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_OUT_RSTN_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_out_rstn_ctrl_hw` writer - "]
pub struct RBB_PKDET_OUT_RSTN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_OUT_RSTN_CTRL_HW_W<'a> {
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
#[doc = "Field `pkdet_out_raw` reader - "]
pub struct PKDET_OUT_RAW_R(crate::FieldReader<bool, bool>);
impl PKDET_OUT_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PKDET_OUT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKDET_OUT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pkdet_out_raw` writer - "]
pub struct PKDET_OUT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_RAW_W<'a> {
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
#[doc = "Field `pkdet_out_latch` reader - "]
pub struct PKDET_OUT_LATCH_R(crate::FieldReader<bool, bool>);
impl PKDET_OUT_LATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PKDET_OUT_LATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKDET_OUT_LATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pkdet_out_latch` writer - "]
pub struct PKDET_OUT_LATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PKDET_OUT_LATCH_W<'a> {
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
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn rbb_bm_op(&self) -> RBB_BM_OP_R {
        RBB_BM_OP_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rbb_vcm(&self) -> RBB_VCM_R {
        RBB_VCM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn rbb_deq(&self) -> RBB_DEQ_R {
        RBB_DEQ_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rbb_lpf_en(&self) -> RBB_LPF_EN_R {
        RBB_LPF_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rosdac_range(&self) -> ROSDAC_RANGE_R {
        ROSDAC_RANGE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rbb_pkdet_vth(&self) -> RBB_PKDET_VTH_R {
        RBB_PKDET_VTH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rbb_pkdet_en(&self) -> RBB_PKDET_EN_R {
        RBB_PKDET_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rbb_pkdet_en_hw(&self) -> RBB_PKDET_EN_HW_R {
        RBB_PKDET_EN_HW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_pkdet_en_ctrl_hw(&self) -> RBB_PKDET_EN_CTRL_HW_R {
        RBB_PKDET_EN_CTRL_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn(&self) -> RBB_PKDET_OUT_RSTN_R {
        RBB_PKDET_OUT_RSTN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_hw(&self) -> RBB_PKDET_OUT_RSTN_HW_R {
        RBB_PKDET_OUT_RSTN_HW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&self) -> RBB_PKDET_OUT_RSTN_CTRL_HW_R {
        RBB_PKDET_OUT_RSTN_CTRL_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pkdet_out_raw(&self) -> PKDET_OUT_RAW_R {
        PKDET_OUT_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pkdet_out_latch(&self) -> PKDET_OUT_LATCH_R {
        PKDET_OUT_LATCH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn rbb_bm_op(&mut self) -> RBB_BM_OP_W {
        RBB_BM_OP_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rbb_vcm(&mut self) -> RBB_VCM_W {
        RBB_VCM_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn rbb_deq(&mut self) -> RBB_DEQ_W {
        RBB_DEQ_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rbb_lpf_en(&mut self) -> RBB_LPF_EN_W {
        RBB_LPF_EN_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rosdac_range(&mut self) -> ROSDAC_RANGE_W {
        ROSDAC_RANGE_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rbb_pkdet_vth(&mut self) -> RBB_PKDET_VTH_W {
        RBB_PKDET_VTH_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rbb_pkdet_en(&mut self) -> RBB_PKDET_EN_W {
        RBB_PKDET_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rbb_pkdet_en_hw(&mut self) -> RBB_PKDET_EN_HW_W {
        RBB_PKDET_EN_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_pkdet_en_ctrl_hw(&mut self) -> RBB_PKDET_EN_CTRL_HW_W {
        RBB_PKDET_EN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn(&mut self) -> RBB_PKDET_OUT_RSTN_W {
        RBB_PKDET_OUT_RSTN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_hw(&mut self) -> RBB_PKDET_OUT_RSTN_HW_W {
        RBB_PKDET_OUT_RSTN_HW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&mut self) -> RBB_PKDET_OUT_RSTN_CTRL_HW_W {
        RBB_PKDET_OUT_RSTN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pkdet_out_raw(&mut self) -> PKDET_OUT_RAW_W {
        PKDET_OUT_RAW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pkdet_out_latch(&mut self) -> PKDET_OUT_LATCH_W {
        PKDET_OUT_LATCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb](index.html) module"]
pub struct RBB_SPEC;
impl crate::RegisterSpec for RBB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb::R](R) reader structure"]
impl crate::Readable for RBB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb::W](W) writer structure"]
impl crate::Writable for RBB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rbb to value 0"]
impl crate::Resettable for RBB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
