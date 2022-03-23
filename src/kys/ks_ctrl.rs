#[doc = "Register `ks_ctrl` reader"]
pub struct R(crate::R<KS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ks_ctrl` writer"]
pub struct W(crate::W<KS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KS_CTRL_SPEC>;
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
impl From<crate::W<KS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `col_num` reader - "]
pub struct COL_NUM_R(crate::FieldReader<u8, u8>);
impl COL_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COL_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COL_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `col_num` writer - "]
pub struct COL_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> COL_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `row_num` reader - "]
pub struct ROW_NUM_R(crate::FieldReader<u8, u8>);
impl ROW_NUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROW_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROW_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `row_num` writer - "]
pub struct ROW_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROW_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `rc_ext` reader - "]
pub struct RC_EXT_R(crate::FieldReader<u8, u8>);
impl RC_EXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RC_EXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC_EXT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rc_ext` writer - "]
pub struct RC_EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> RC_EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `deg_cnt` reader - "]
pub struct DEG_CNT_R(crate::FieldReader<u8, u8>);
impl DEG_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEG_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEG_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `deg_cnt` writer - "]
pub struct DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `deg_en` reader - "]
pub struct DEG_EN_R(crate::FieldReader<bool, bool>);
impl DEG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DEG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `deg_en` writer - "]
pub struct DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEG_EN_W<'a> {
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
#[doc = "Field `ghost_en` reader - "]
pub struct GHOST_EN_R(crate::FieldReader<bool, bool>);
impl GHOST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GHOST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GHOST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ghost_en` writer - "]
pub struct GHOST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GHOST_EN_W<'a> {
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
#[doc = "Field `ks_en` reader - "]
pub struct KS_EN_R(crate::FieldReader<bool, bool>);
impl KS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ks_en` writer - "]
pub struct KS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> KS_EN_W<'a> {
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
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn col_num(&self) -> COL_NUM_R {
        COL_NUM_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn row_num(&self) -> ROW_NUM_R {
        ROW_NUM_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rc_ext(&self) -> RC_EXT_R {
        RC_EXT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn deg_cnt(&self) -> DEG_CNT_R {
        DEG_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn deg_en(&self) -> DEG_EN_R {
        DEG_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ghost_en(&self) -> GHOST_EN_R {
        GHOST_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ks_en(&self) -> KS_EN_R {
        KS_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn col_num(&mut self) -> COL_NUM_W {
        COL_NUM_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn row_num(&mut self) -> ROW_NUM_W {
        ROW_NUM_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rc_ext(&mut self) -> RC_EXT_W {
        RC_EXT_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn deg_cnt(&mut self) -> DEG_CNT_W {
        DEG_CNT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn deg_en(&mut self) -> DEG_EN_W {
        DEG_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ghost_en(&mut self) -> GHOST_EN_W {
        GHOST_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ks_en(&mut self) -> KS_EN_W {
        KS_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ks_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ks_ctrl](index.html) module"]
pub struct KS_CTRL_SPEC;
impl crate::RegisterSpec for KS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ks_ctrl::R](R) reader structure"]
impl crate::Readable for KS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ks_ctrl::W](W) writer structure"]
impl crate::Writable for KS_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ks_ctrl to value 0"]
impl crate::Resettable for KS_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
