#[doc = "Register `mjpeg_control_2` reader"]
pub struct R(crate::R<MJPEG_CONTROL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_CONTROL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_CONTROL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_CONTROL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_control_2` writer"]
pub struct W(crate::W<MJPEG_CONTROL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_CONTROL_2_SPEC>;
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
impl From<crate::W<MJPEG_CONTROL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_CONTROL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mjpeg_wait_cycle` reader - "]
pub struct REG_MJPEG_WAIT_CYCLE_R(crate::FieldReader<u16, u16>);
impl REG_MJPEG_WAIT_CYCLE_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG_MJPEG_WAIT_CYCLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MJPEG_WAIT_CYCLE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mjpeg_wait_cycle` writer - "]
pub struct REG_MJPEG_WAIT_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MJPEG_WAIT_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `reg_uv_dvp2ahb_fsel` reader - "]
pub struct REG_UV_DVP2AHB_FSEL_R(crate::FieldReader<bool, bool>);
impl REG_UV_DVP2AHB_FSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_UV_DVP2AHB_FSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_UV_DVP2AHB_FSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_uv_dvp2ahb_fsel` writer - "]
pub struct REG_UV_DVP2AHB_FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_UV_DVP2AHB_FSEL_W<'a> {
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
#[doc = "Field `reg_uv_dvp2ahb_lsel` reader - "]
pub struct REG_UV_DVP2AHB_LSEL_R(crate::FieldReader<bool, bool>);
impl REG_UV_DVP2AHB_LSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_UV_DVP2AHB_LSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_UV_DVP2AHB_LSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_uv_dvp2ahb_lsel` writer - "]
pub struct REG_UV_DVP2AHB_LSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_UV_DVP2AHB_LSEL_W<'a> {
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
#[doc = "Field `reg_yy_dvp2ahb_fsel` reader - "]
pub struct REG_YY_DVP2AHB_FSEL_R(crate::FieldReader<bool, bool>);
impl REG_YY_DVP2AHB_FSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_YY_DVP2AHB_FSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_YY_DVP2AHB_FSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_yy_dvp2ahb_fsel` writer - "]
pub struct REG_YY_DVP2AHB_FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_YY_DVP2AHB_FSEL_W<'a> {
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
#[doc = "Field `reg_yy_dvp2ahb_lsel` reader - "]
pub struct REG_YY_DVP2AHB_LSEL_R(crate::FieldReader<bool, bool>);
impl REG_YY_DVP2AHB_LSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_YY_DVP2AHB_LSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_YY_DVP2AHB_LSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_yy_dvp2ahb_lsel` writer - "]
pub struct REG_YY_DVP2AHB_LSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_YY_DVP2AHB_LSEL_W<'a> {
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
#[doc = "Field `reg_mjpeg_sw_run` reader - "]
pub struct REG_MJPEG_SW_RUN_R(crate::FieldReader<bool, bool>);
impl REG_MJPEG_SW_RUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_MJPEG_SW_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MJPEG_SW_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mjpeg_sw_run` writer - "]
pub struct REG_MJPEG_SW_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MJPEG_SW_RUN_W<'a> {
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
#[doc = "Field `reg_mjpeg_sw_mode` reader - "]
pub struct REG_MJPEG_SW_MODE_R(crate::FieldReader<bool, bool>);
impl REG_MJPEG_SW_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_MJPEG_SW_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MJPEG_SW_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mjpeg_sw_mode` writer - "]
pub struct REG_MJPEG_SW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MJPEG_SW_MODE_W<'a> {
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
#[doc = "Field `reg_sw_frame` reader - "]
pub struct REG_SW_FRAME_R(crate::FieldReader<u8, u8>);
impl REG_SW_FRAME_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_SW_FRAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SW_FRAME_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_sw_frame` writer - "]
pub struct REG_SW_FRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SW_FRAME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_mjpeg_wait_cycle(&self) -> REG_MJPEG_WAIT_CYCLE_R {
        REG_MJPEG_WAIT_CYCLE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_uv_dvp2ahb_fsel(&self) -> REG_UV_DVP2AHB_FSEL_R {
        REG_UV_DVP2AHB_FSEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_uv_dvp2ahb_lsel(&self) -> REG_UV_DVP2AHB_LSEL_R {
        REG_UV_DVP2AHB_LSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_yy_dvp2ahb_fsel(&self) -> REG_YY_DVP2AHB_FSEL_R {
        REG_YY_DVP2AHB_FSEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_yy_dvp2ahb_lsel(&self) -> REG_YY_DVP2AHB_LSEL_R {
        REG_YY_DVP2AHB_LSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mjpeg_sw_run(&self) -> REG_MJPEG_SW_RUN_R {
        REG_MJPEG_SW_RUN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mjpeg_sw_mode(&self) -> REG_MJPEG_SW_MODE_R {
        REG_MJPEG_SW_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_sw_frame(&self) -> REG_SW_FRAME_R {
        REG_SW_FRAME_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_mjpeg_wait_cycle(&mut self) -> REG_MJPEG_WAIT_CYCLE_W {
        REG_MJPEG_WAIT_CYCLE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_uv_dvp2ahb_fsel(&mut self) -> REG_UV_DVP2AHB_FSEL_W {
        REG_UV_DVP2AHB_FSEL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_uv_dvp2ahb_lsel(&mut self) -> REG_UV_DVP2AHB_LSEL_W {
        REG_UV_DVP2AHB_LSEL_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_yy_dvp2ahb_fsel(&mut self) -> REG_YY_DVP2AHB_FSEL_W {
        REG_YY_DVP2AHB_FSEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_yy_dvp2ahb_lsel(&mut self) -> REG_YY_DVP2AHB_LSEL_W {
        REG_YY_DVP2AHB_LSEL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mjpeg_sw_run(&mut self) -> REG_MJPEG_SW_RUN_W {
        REG_MJPEG_SW_RUN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mjpeg_sw_mode(&mut self) -> REG_MJPEG_SW_MODE_W {
        REG_MJPEG_SW_MODE_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_sw_frame(&mut self) -> REG_SW_FRAME_W {
        REG_SW_FRAME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_control_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_control_2](index.html) module"]
pub struct MJPEG_CONTROL_2_SPEC;
impl crate::RegisterSpec for MJPEG_CONTROL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_control_2::R](R) reader structure"]
impl crate::Readable for MJPEG_CONTROL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_control_2::W](W) writer structure"]
impl crate::Writable for MJPEG_CONTROL_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_control_2 to value 0"]
impl crate::Resettable for MJPEG_CONTROL_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
