#[doc = "Register `trx_gain_bw` reader"]
pub struct R(crate::R<TRX_GAIN_BW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRX_GAIN_BW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRX_GAIN_BW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRX_GAIN_BW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `trx_gain_bw` writer"]
pub struct W(crate::W<TRX_GAIN_BW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRX_GAIN_BW_SPEC>;
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
impl From<crate::W<TRX_GAIN_BW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRX_GAIN_BW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_inbuf_unit` reader - "]
pub struct PA_INBUF_UNIT_R(crate::FieldReader<u8, u8>);
impl PA_INBUF_UNIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_INBUF_UNIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_INBUF_UNIT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_inbuf_unit` writer - "]
pub struct PA_INBUF_UNIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_INBUF_UNIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `pa_ref_dac` reader - "]
pub struct PA_REF_DAC_R(crate::FieldReader<u8, u8>);
impl PA_REF_DAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_REF_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_REF_DAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_ref_dac` writer - "]
pub struct PA_REF_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_REF_DAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | ((value as u32 & 0x1f) << 12);
        self.w
    }
}
#[doc = "Field `rbb_bw` reader - "]
pub struct RBB_BW_R(crate::FieldReader<bool, bool>);
impl RBB_BW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RBB_BW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bw` writer - "]
pub struct RBB_BW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BW_W<'a> {
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
#[doc = "Field `gc_rbb2` reader - "]
pub struct GC_RBB2_R(crate::FieldReader<u8, u8>);
impl GC_RBB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GC_RBB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_RBB2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gc_rbb2` writer - "]
pub struct GC_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `gc_rbb1` reader - "]
pub struct GC_RBB1_R(crate::FieldReader<u8, u8>);
impl GC_RBB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GC_RBB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_RBB1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gc_rbb1` writer - "]
pub struct GC_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `gc_lna` reader - "]
pub struct GC_LNA_R(crate::FieldReader<u8, u8>);
impl GC_LNA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GC_LNA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GC_LNA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gc_lna` writer - "]
pub struct GC_LNA_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_LNA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_inbuf_unit(&self) -> PA_INBUF_UNIT_R {
        PA_INBUF_UNIT_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn pa_ref_dac(&self) -> PA_REF_DAC_R {
        PA_REF_DAC_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_bw(&self) -> RBB_BW_R {
        RBB_BW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn gc_rbb2(&self) -> GC_RBB2_R {
        GC_RBB2_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn gc_rbb1(&self) -> GC_RBB1_R {
        GC_RBB1_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna(&self) -> GC_LNA_R {
        GC_LNA_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_inbuf_unit(&mut self) -> PA_INBUF_UNIT_W {
        PA_INBUF_UNIT_W { w: self }
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn pa_ref_dac(&mut self) -> PA_REF_DAC_W {
        PA_REF_DAC_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_bw(&mut self) -> RBB_BW_W {
        RBB_BW_W { w: self }
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn gc_rbb2(&mut self) -> GC_RBB2_W {
        GC_RBB2_W { w: self }
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn gc_rbb1(&mut self) -> GC_RBB1_W {
        GC_RBB1_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna(&mut self) -> GC_LNA_W {
        GC_LNA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register control of TX/RX gain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trx_gain_bw](index.html) module"]
pub struct TRX_GAIN_BW_SPEC;
impl crate::RegisterSpec for TRX_GAIN_BW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trx_gain_bw::R](R) reader structure"]
impl crate::Readable for TRX_GAIN_BW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trx_gain_bw::W](W) writer structure"]
impl crate::Writable for TRX_GAIN_BW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets trx_gain_bw to value 0"]
impl crate::Resettable for TRX_GAIN_BW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
