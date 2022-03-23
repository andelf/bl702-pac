#[doc = "Register `dsp_readback` reader"]
pub struct R(crate::R<DSP_READBACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_READBACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_READBACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_READBACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dsp_readback` writer"]
pub struct W(crate::W<DSP_READBACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSP_READBACK_SPEC>;
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
impl From<crate::W<DSP_READBACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSP_READBACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rbb_bw_ind_ctrl_hw` reader - "]
pub struct RBB_BW_IND_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl RBB_BW_IND_CTRL_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_BW_IND_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BW_IND_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bw_ind_ctrl_hw` writer - "]
pub struct RBB_BW_IND_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BW_IND_CTRL_HW_W<'a> {
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
#[doc = "Field `rbb_bw_ind` reader - "]
pub struct RBB_BW_IND_R(crate::FieldReader<bool, bool>);
impl RBB_BW_IND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_BW_IND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BW_IND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bw_ind` writer - "]
pub struct RBB_BW_IND_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BW_IND_W<'a> {
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
#[doc = "Field `rbb_ind_ctrl_hw` reader - "]
pub struct RBB_IND_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl RBB_IND_CTRL_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_IND_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_IND_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_ind_ctrl_hw` writer - "]
pub struct RBB_IND_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_IND_CTRL_HW_W<'a> {
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
#[doc = "Field `rbb_ind` reader - "]
pub struct RBB_IND_R(crate::FieldReader<u8, u8>);
impl RBB_IND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_IND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_IND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_ind` writer - "]
pub struct RBB_IND_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_IND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `ch_ind_ctrl_hw` reader - "]
pub struct CH_IND_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl CH_IND_CTRL_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH_IND_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_IND_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ch_ind_ctrl_hw` writer - "]
pub struct CH_IND_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_IND_CTRL_HW_W<'a> {
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
#[doc = "Field `ch_ind` reader - "]
pub struct CH_IND_R(crate::FieldReader<u8, u8>);
impl CH_IND_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH_IND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_IND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ch_ind` writer - "]
pub struct CH_IND_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_IND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `rbb_bw_ind_hw` reader - "]
pub struct RBB_BW_IND_HW_R(crate::FieldReader<bool, bool>);
impl RBB_BW_IND_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_BW_IND_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BW_IND_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bw_ind_hw` writer - "]
pub struct RBB_BW_IND_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BW_IND_HW_W<'a> {
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
#[doc = "Field `rbb_ind_hw` reader - "]
pub struct RBB_IND_HW_R(crate::FieldReader<u8, u8>);
impl RBB_IND_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_IND_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_IND_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_ind_hw` writer - "]
pub struct RBB_IND_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_IND_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 9)) | ((value as u32 & 0x1f) << 9);
        self.w
    }
}
#[doc = "Field `ch_ind_hw` reader - "]
pub struct CH_IND_HW_R(crate::FieldReader<u8, u8>);
impl CH_IND_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH_IND_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_IND_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ch_ind_hw` writer - "]
pub struct CH_IND_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_IND_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 2)) | ((value as u32 & 0x7f) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rbb_bw_ind_ctrl_hw(&self) -> RBB_BW_IND_CTRL_HW_R {
        RBB_BW_IND_CTRL_HW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rbb_bw_ind(&self) -> RBB_BW_IND_R {
        RBB_BW_IND_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rbb_ind_ctrl_hw(&self) -> RBB_IND_CTRL_HW_R {
        RBB_IND_CTRL_HW_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rbb_ind(&self) -> RBB_IND_R {
        RBB_IND_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ch_ind_ctrl_hw(&self) -> CH_IND_CTRL_HW_R {
        CH_IND_CTRL_HW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn ch_ind(&self) -> CH_IND_R {
        CH_IND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rbb_bw_ind_hw(&self) -> RBB_BW_IND_HW_R {
        RBB_BW_IND_HW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn rbb_ind_hw(&self) -> RBB_IND_HW_R {
        RBB_IND_HW_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 2:8"]
    #[inline(always)]
    pub fn ch_ind_hw(&self) -> CH_IND_HW_R {
        CH_IND_HW_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rbb_bw_ind_ctrl_hw(&mut self) -> RBB_BW_IND_CTRL_HW_W {
        RBB_BW_IND_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rbb_bw_ind(&mut self) -> RBB_BW_IND_W {
        RBB_BW_IND_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rbb_ind_ctrl_hw(&mut self) -> RBB_IND_CTRL_HW_W {
        RBB_IND_CTRL_HW_W { w: self }
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rbb_ind(&mut self) -> RBB_IND_W {
        RBB_IND_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ch_ind_ctrl_hw(&mut self) -> CH_IND_CTRL_HW_W {
        CH_IND_CTRL_HW_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn ch_ind(&mut self) -> CH_IND_W {
        CH_IND_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rbb_bw_ind_hw(&mut self) -> RBB_BW_IND_HW_W {
        RBB_BW_IND_HW_W { w: self }
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn rbb_ind_hw(&mut self) -> RBB_IND_HW_W {
        RBB_IND_HW_W { w: self }
    }
    #[doc = "Bits 2:8"]
    #[inline(always)]
    pub fn ch_ind_hw(&mut self) -> CH_IND_HW_W {
        CH_IND_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dsp_readback.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_readback](index.html) module"]
pub struct DSP_READBACK_SPEC;
impl crate::RegisterSpec for DSP_READBACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp_readback::R](R) reader structure"]
impl crate::Readable for DSP_READBACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsp_readback::W](W) writer structure"]
impl crate::Writable for DSP_READBACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dsp_readback to value 0"]
impl crate::Resettable for DSP_READBACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
