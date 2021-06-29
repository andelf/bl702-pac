#[doc = "Register `dvp2axi_configue` reader"]
pub struct R(crate::R<DVP2AXI_CONFIGUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP2AXI_CONFIGUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP2AXI_CONFIGUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP2AXI_CONFIGUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dvp2axi_configue` writer"]
pub struct W(crate::W<DVP2AXI_CONFIGUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP2AXI_CONFIGUE_SPEC>;
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
impl From<crate::W<DVP2AXI_CONFIGUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP2AXI_CONFIGUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_dvp_wait_cycle` reader - "]
pub struct REG_DVP_WAIT_CYCLE_R(crate::FieldReader<u8, u8>);
impl REG_DVP_WAIT_CYCLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_DVP_WAIT_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_DVP_WAIT_CYCLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_dvp_wait_cycle` writer - "]
pub struct REG_DVP_WAIT_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_DVP_WAIT_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `reg_dvp_pix_clk_cg` reader - "]
pub struct REG_DVP_PIX_CLK_CG_R(crate::FieldReader<bool, bool>);
impl REG_DVP_PIX_CLK_CG_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_DVP_PIX_CLK_CG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_DVP_PIX_CLK_CG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_dvp_pix_clk_cg` writer - "]
pub struct REG_DVP_PIX_CLK_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_DVP_PIX_CLK_CG_W<'a> {
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
#[doc = "Field `reg_interlv_mode` reader - "]
pub struct REG_INTERLV_MODE_R(crate::FieldReader<bool, bool>);
impl REG_INTERLV_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_INTERLV_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INTERLV_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_interlv_mode` writer - "]
pub struct REG_INTERLV_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INTERLV_MODE_W<'a> {
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
#[doc = "Field `reg_subsample_even` reader - "]
pub struct REG_SUBSAMPLE_EVEN_R(crate::FieldReader<bool, bool>);
impl REG_SUBSAMPLE_EVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_SUBSAMPLE_EVEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SUBSAMPLE_EVEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_subsample_even` writer - "]
pub struct REG_SUBSAMPLE_EVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SUBSAMPLE_EVEN_W<'a> {
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
#[doc = "Field `reg_subsample_en` reader - "]
pub struct REG_SUBSAMPLE_EN_R(crate::FieldReader<bool, bool>);
impl REG_SUBSAMPLE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_SUBSAMPLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SUBSAMPLE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_subsample_en` writer - "]
pub struct REG_SUBSAMPLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SUBSAMPLE_EN_W<'a> {
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
#[doc = "Field `reg_drop_even` reader - "]
pub struct REG_DROP_EVEN_R(crate::FieldReader<bool, bool>);
impl REG_DROP_EVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_DROP_EVEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_DROP_EVEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_drop_even` writer - "]
pub struct REG_DROP_EVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_DROP_EVEN_W<'a> {
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
#[doc = "Field `reg_drop_en` reader - "]
pub struct REG_DROP_EN_R(crate::FieldReader<bool, bool>);
impl REG_DROP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_DROP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_DROP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_drop_en` writer - "]
pub struct REG_DROP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_DROP_EN_W<'a> {
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
#[doc = "Field `reg_hw_mode_fwrap` reader - "]
pub struct REG_HW_MODE_FWRAP_R(crate::FieldReader<bool, bool>);
impl REG_HW_MODE_FWRAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_HW_MODE_FWRAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_HW_MODE_FWRAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_hw_mode_fwrap` writer - "]
pub struct REG_HW_MODE_FWRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_HW_MODE_FWRAP_W<'a> {
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
#[doc = "Field `reg_dvp_mode` reader - "]
pub struct REG_DVP_MODE_R(crate::FieldReader<u8, u8>);
impl REG_DVP_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_DVP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_DVP_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_dvp_mode` writer - "]
pub struct REG_DVP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_DVP_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `reg_hburst` reader - "]
pub struct REG_HBURST_R(crate::FieldReader<u8, u8>);
impl REG_HBURST_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_HBURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_HBURST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_hburst` writer - "]
pub struct REG_HBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_HBURST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `reg_line_vld_pol` reader - "]
pub struct REG_LINE_VLD_POL_R(crate::FieldReader<bool, bool>);
impl REG_LINE_VLD_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_LINE_VLD_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_LINE_VLD_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_line_vld_pol` writer - "]
pub struct REG_LINE_VLD_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_LINE_VLD_POL_W<'a> {
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
#[doc = "Field `reg_fram_vld_pol` reader - "]
pub struct REG_FRAM_VLD_POL_R(crate::FieldReader<bool, bool>);
impl REG_FRAM_VLD_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_FRAM_VLD_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_FRAM_VLD_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_fram_vld_pol` writer - "]
pub struct REG_FRAM_VLD_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_FRAM_VLD_POL_W<'a> {
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
#[doc = "Field `reg_sw_mode` reader - "]
pub struct REG_SW_MODE_R(crate::FieldReader<bool, bool>);
impl REG_SW_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_SW_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SW_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_sw_mode` writer - "]
pub struct REG_SW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SW_MODE_W<'a> {
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
#[doc = "Field `reg_dvp_enable` reader - "]
pub struct REG_DVP_ENABLE_R(crate::FieldReader<bool, bool>);
impl REG_DVP_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_DVP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_DVP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_dvp_enable` writer - "]
pub struct REG_DVP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_DVP_ENABLE_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_dvp_wait_cycle(&self) -> REG_DVP_WAIT_CYCLE_R {
        REG_DVP_WAIT_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_dvp_pix_clk_cg(&self) -> REG_DVP_PIX_CLK_CG_R {
        REG_DVP_PIX_CLK_CG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_interlv_mode(&self) -> REG_INTERLV_MODE_R {
        REG_INTERLV_MODE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_subsample_even(&self) -> REG_SUBSAMPLE_EVEN_R {
        REG_SUBSAMPLE_EVEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_subsample_en(&self) -> REG_SUBSAMPLE_EN_R {
        REG_SUBSAMPLE_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_drop_even(&self) -> REG_DROP_EVEN_R {
        REG_DROP_EVEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_drop_en(&self) -> REG_DROP_EN_R {
        REG_DROP_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_hw_mode_fwrap(&self) -> REG_HW_MODE_FWRAP_R {
        REG_HW_MODE_FWRAP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn reg_dvp_mode(&self) -> REG_DVP_MODE_R {
        REG_DVP_MODE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_hburst(&self) -> REG_HBURST_R {
        REG_HBURST_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_line_vld_pol(&self) -> REG_LINE_VLD_POL_R {
        REG_LINE_VLD_POL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_fram_vld_pol(&self) -> REG_FRAM_VLD_POL_R {
        REG_FRAM_VLD_POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_sw_mode(&self) -> REG_SW_MODE_R {
        REG_SW_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dvp_enable(&self) -> REG_DVP_ENABLE_R {
        REG_DVP_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_dvp_wait_cycle(&mut self) -> REG_DVP_WAIT_CYCLE_W {
        REG_DVP_WAIT_CYCLE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_dvp_pix_clk_cg(&mut self) -> REG_DVP_PIX_CLK_CG_W {
        REG_DVP_PIX_CLK_CG_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_interlv_mode(&mut self) -> REG_INTERLV_MODE_W {
        REG_INTERLV_MODE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_subsample_even(&mut self) -> REG_SUBSAMPLE_EVEN_W {
        REG_SUBSAMPLE_EVEN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_subsample_en(&mut self) -> REG_SUBSAMPLE_EN_W {
        REG_SUBSAMPLE_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_drop_even(&mut self) -> REG_DROP_EVEN_W {
        REG_DROP_EVEN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_drop_en(&mut self) -> REG_DROP_EN_W {
        REG_DROP_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_hw_mode_fwrap(&mut self) -> REG_HW_MODE_FWRAP_W {
        REG_HW_MODE_FWRAP_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn reg_dvp_mode(&mut self) -> REG_DVP_MODE_W {
        REG_DVP_MODE_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_hburst(&mut self) -> REG_HBURST_W {
        REG_HBURST_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_line_vld_pol(&mut self) -> REG_LINE_VLD_POL_W {
        REG_LINE_VLD_POL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_fram_vld_pol(&mut self) -> REG_FRAM_VLD_POL_W {
        REG_FRAM_VLD_POL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_sw_mode(&mut self) -> REG_SW_MODE_W {
        REG_SW_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dvp_enable(&mut self) -> REG_DVP_ENABLE_W {
        REG_DVP_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dvp2axi_configue.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp2axi_configue](index.html) module"]
pub struct DVP2AXI_CONFIGUE_SPEC;
impl crate::RegisterSpec for DVP2AXI_CONFIGUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp2axi_configue::R](R) reader structure"]
impl crate::Readable for DVP2AXI_CONFIGUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp2axi_configue::W](W) writer structure"]
impl crate::Writable for DVP2AXI_CONFIGUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dvp2axi_configue to value 0"]
impl crate::Resettable for DVP2AXI_CONFIGUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
