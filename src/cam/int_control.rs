#[doc = "Register `int_control` reader"]
pub struct R(crate::R<INT_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `int_control` writer"]
pub struct W(crate::W<INT_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CONTROL_SPEC>;
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
impl From<crate::W<INT_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CONTROL_SPEC>) -> Self {
        W(writer)
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
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `reg_int_vcnt_en` reader - "]
pub struct REG_INT_VCNT_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_VCNT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_VCNT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_VCNT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_vcnt_en` writer - "]
pub struct REG_INT_VCNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_VCNT_EN_W<'a> {
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
#[doc = "Field `reg_int_hcnt_en` reader - "]
pub struct REG_INT_HCNT_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_HCNT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_HCNT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_HCNT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_hcnt_en` writer - "]
pub struct REG_INT_HCNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_HCNT_EN_W<'a> {
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
#[doc = "Field `reg_int_fifo_en` reader - "]
pub struct REG_INT_FIFO_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_FIFO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_FIFO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_FIFO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_fifo_en` writer - "]
pub struct REG_INT_FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_FIFO_EN_W<'a> {
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
#[doc = "Field `reg_int_normal_1_en` reader - "]
pub struct REG_INT_NORMAL_1_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_NORMAL_1_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_NORMAL_1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_NORMAL_1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_normal_1_en` writer - "]
pub struct REG_INT_NORMAL_1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_NORMAL_1_EN_W<'a> {
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
#[doc = "Field `reg_int_normal_0_en` reader - "]
pub struct REG_INT_NORMAL_0_EN_R(crate::FieldReader<bool, bool>);
impl REG_INT_NORMAL_0_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_NORMAL_0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_NORMAL_0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_normal_0_en` writer - "]
pub struct REG_INT_NORMAL_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_NORMAL_0_EN_W<'a> {
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&self) -> REG_FRAME_CNT_TRGR_INT_R {
        REG_FRAME_CNT_TRGR_INT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_int_vcnt_en(&self) -> REG_INT_VCNT_EN_R {
        REG_INT_VCNT_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_int_hcnt_en(&self) -> REG_INT_HCNT_EN_R {
        REG_INT_HCNT_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_int_fifo_en(&self) -> REG_INT_FIFO_EN_R {
        REG_INT_FIFO_EN_R::new(((self.bits >> 4) & 0x01) != 0)
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
    pub fn reg_int_normal_1_en(&self) -> REG_INT_NORMAL_1_EN_R {
        REG_INT_NORMAL_1_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_int_normal_0_en(&self) -> REG_INT_NORMAL_0_EN_R {
        REG_INT_NORMAL_0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&mut self) -> REG_FRAME_CNT_TRGR_INT_W {
        REG_FRAME_CNT_TRGR_INT_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_int_vcnt_en(&mut self) -> REG_INT_VCNT_EN_W {
        REG_INT_VCNT_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_int_hcnt_en(&mut self) -> REG_INT_HCNT_EN_W {
        REG_INT_HCNT_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_int_fifo_en(&mut self) -> REG_INT_FIFO_EN_W {
        REG_INT_FIFO_EN_W { w: self }
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
    pub fn reg_int_normal_1_en(&mut self) -> REG_INT_NORMAL_1_EN_W {
        REG_INT_NORMAL_1_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_int_normal_0_en(&mut self) -> REG_INT_NORMAL_0_EN_W {
        REG_INT_NORMAL_0_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "int_control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_control](index.html) module"]
pub struct INT_CONTROL_SPEC;
impl crate::RegisterSpec for INT_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_control::R](R) reader structure"]
impl crate::Readable for INT_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_control::W](W) writer structure"]
impl crate::Writable for INT_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets int_control to value 0"]
impl crate::Resettable for INT_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
