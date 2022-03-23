#[doc = "Register `rbb_rx` reader"]
pub struct R(crate::R<RBB_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb_rx` writer"]
pub struct W(crate::W<RBB_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_RX_SPEC>;
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
impl From<crate::W<RBB_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rbb_rx1` reader - "]
pub struct RBB_RX1_R(crate::FieldReader<u8, u8>);
impl RBB_RX1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_RX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_RX1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_rx1` writer - "]
pub struct RBB_RX1_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_RX1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `rbb_rx2` reader - "]
pub struct RBB_RX2_R(crate::FieldReader<u8, u8>);
impl RBB_RX2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_RX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_RX2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_rx2` writer - "]
pub struct RBB_RX2_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_RX2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `rbb_rx1_hw` reader - "]
pub struct RBB_RX1_HW_R(crate::FieldReader<u8, u8>);
impl RBB_RX1_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_RX1_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_RX1_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_rx1_hw` writer - "]
pub struct RBB_RX1_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_RX1_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `rbb_rx2_hw` reader - "]
pub struct RBB_RX2_HW_R(crate::FieldReader<u8, u8>);
impl RBB_RX2_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_RX2_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_RX2_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_rx2_hw` writer - "]
pub struct RBB_RX2_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_RX2_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `rbb_rx1_bw0` reader - "]
pub struct RBB_RX1_BW0_R(crate::FieldReader<u8, u8>);
impl RBB_RX1_BW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_RX1_BW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_RX1_BW0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_rx1_bw0` writer - "]
pub struct RBB_RX1_BW0_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_RX1_BW0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `rbb_rx2_bw0` reader - "]
pub struct RBB_RX2_BW0_R(crate::FieldReader<u8, u8>);
impl RBB_RX2_BW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_RX2_BW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_RX2_BW0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_rx2_bw0` writer - "]
pub struct RBB_RX2_BW0_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_RX2_BW0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `rbb_rx1_bw1` reader - "]
pub struct RBB_RX1_BW1_R(crate::FieldReader<u8, u8>);
impl RBB_RX1_BW1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_RX1_BW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_RX1_BW1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_rx1_bw1` writer - "]
pub struct RBB_RX1_BW1_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_RX1_BW1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `rbb_rx2_bw1` reader - "]
pub struct RBB_RX2_BW1_R(crate::FieldReader<u8, u8>);
impl RBB_RX2_BW1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RBB_RX2_BW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_RX2_BW1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_rx2_bw1` writer - "]
pub struct RBB_RX2_BW1_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_RX2_BW1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rbb_rx1(&self) -> RBB_RX1_R {
        RBB_RX1_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rbb_rx2(&self) -> RBB_RX2_R {
        RBB_RX2_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rbb_rx1_hw(&self) -> RBB_RX1_HW_R {
        RBB_RX1_HW_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rbb_rx2_hw(&self) -> RBB_RX2_HW_R {
        RBB_RX2_HW_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rbb_rx1_bw0(&self) -> RBB_RX1_BW0_R {
        RBB_RX1_BW0_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rbb_rx2_bw0(&self) -> RBB_RX2_BW0_R {
        RBB_RX2_BW0_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rbb_rx1_bw1(&self) -> RBB_RX1_BW1_R {
        RBB_RX1_BW1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rbb_rx2_bw1(&self) -> RBB_RX2_BW1_R {
        RBB_RX2_BW1_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rbb_rx1(&mut self) -> RBB_RX1_W {
        RBB_RX1_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rbb_rx2(&mut self) -> RBB_RX2_W {
        RBB_RX2_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rbb_rx1_hw(&mut self) -> RBB_RX1_HW_W {
        RBB_RX1_HW_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rbb_rx2_hw(&mut self) -> RBB_RX2_HW_W {
        RBB_RX2_HW_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rbb_rx1_bw0(&mut self) -> RBB_RX1_BW0_W {
        RBB_RX1_BW0_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rbb_rx2_bw0(&mut self) -> RBB_RX2_BW0_W {
        RBB_RX2_BW0_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rbb_rx1_bw1(&mut self) -> RBB_RX1_BW1_W {
        RBB_RX1_BW1_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rbb_rx2_bw1(&mut self) -> RBB_RX2_BW1_W {
        RBB_RX2_BW1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb_rx.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_rx](index.html) module"]
pub struct RBB_RX_SPEC;
impl crate::RegisterSpec for RBB_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb_rx::R](R) reader structure"]
impl crate::Readable for RBB_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb_rx::W](W) writer structure"]
impl crate::Writable for RBB_RX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rbb_rx to value 0"]
impl crate::Resettable for RBB_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
