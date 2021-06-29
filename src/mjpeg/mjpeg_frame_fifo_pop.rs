#[doc = "Register `mjpeg_frame_fifo_pop` reader"]
pub struct R(crate::R<MJPEG_FRAME_FIFO_POP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_FRAME_FIFO_POP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_FRAME_FIFO_POP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_FRAME_FIFO_POP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_frame_fifo_pop` writer"]
pub struct W(crate::W<MJPEG_FRAME_FIFO_POP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_FRAME_FIFO_POP_SPEC>;
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
impl From<crate::W<MJPEG_FRAME_FIFO_POP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_FRAME_FIFO_POP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_int_swap_clr` reader - "]
pub struct REG_INT_SWAP_CLR_R(crate::FieldReader<bool, bool>);
impl REG_INT_SWAP_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_SWAP_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_SWAP_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_swap_clr` writer - "]
pub struct REG_INT_SWAP_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_SWAP_CLR_W<'a> {
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
#[doc = "Field `reg_int_idle_clr` reader - "]
pub struct REG_INT_IDLE_CLR_R(crate::FieldReader<bool, bool>);
impl REG_INT_IDLE_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_IDLE_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_IDLE_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_idle_clr` writer - "]
pub struct REG_INT_IDLE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_IDLE_CLR_W<'a> {
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
#[doc = "Field `reg_int_frame_clr` reader - "]
pub struct REG_INT_FRAME_CLR_R(crate::FieldReader<bool, bool>);
impl REG_INT_FRAME_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_FRAME_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_FRAME_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_frame_clr` writer - "]
pub struct REG_INT_FRAME_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_FRAME_CLR_W<'a> {
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
#[doc = "Field `reg_int_mem_clr` reader - "]
pub struct REG_INT_MEM_CLR_R(crate::FieldReader<bool, bool>);
impl REG_INT_MEM_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_MEM_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_MEM_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_mem_clr` writer - "]
pub struct REG_INT_MEM_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_MEM_CLR_W<'a> {
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
#[doc = "Field `reg_int_cam_clr` reader - "]
pub struct REG_INT_CAM_CLR_R(crate::FieldReader<bool, bool>);
impl REG_INT_CAM_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_CAM_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_CAM_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_cam_clr` writer - "]
pub struct REG_INT_CAM_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_CAM_CLR_W<'a> {
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
#[doc = "Field `reg_int_normal_clr` reader - "]
pub struct REG_INT_NORMAL_CLR_R(crate::FieldReader<bool, bool>);
impl REG_INT_NORMAL_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_INT_NORMAL_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_INT_NORMAL_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_int_normal_clr` writer - "]
pub struct REG_INT_NORMAL_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_INT_NORMAL_CLR_W<'a> {
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
#[doc = "Field `reg_w_swap_clr` reader - "]
pub struct REG_W_SWAP_CLR_R(crate::FieldReader<bool, bool>);
impl REG_W_SWAP_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_W_SWAP_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_W_SWAP_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_w_swap_clr` writer - "]
pub struct REG_W_SWAP_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_W_SWAP_CLR_W<'a> {
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
#[doc = "Field `rfifo_pop` reader - "]
pub struct RFIFO_POP_R(crate::FieldReader<bool, bool>);
impl RFIFO_POP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFIFO_POP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIFO_POP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfifo_pop` writer - "]
pub struct RFIFO_POP_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIFO_POP_W<'a> {
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
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_int_swap_clr(&self) -> REG_INT_SWAP_CLR_R {
        REG_INT_SWAP_CLR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_int_idle_clr(&self) -> REG_INT_IDLE_CLR_R {
        REG_INT_IDLE_CLR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_int_frame_clr(&self) -> REG_INT_FRAME_CLR_R {
        REG_INT_FRAME_CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_int_mem_clr(&self) -> REG_INT_MEM_CLR_R {
        REG_INT_MEM_CLR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_int_cam_clr(&self) -> REG_INT_CAM_CLR_R {
        REG_INT_CAM_CLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_int_normal_clr(&self) -> REG_INT_NORMAL_CLR_R {
        REG_INT_NORMAL_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_w_swap_clr(&self) -> REG_W_SWAP_CLR_R {
        REG_W_SWAP_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfifo_pop(&self) -> RFIFO_POP_R {
        RFIFO_POP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_int_swap_clr(&mut self) -> REG_INT_SWAP_CLR_W {
        REG_INT_SWAP_CLR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_int_idle_clr(&mut self) -> REG_INT_IDLE_CLR_W {
        REG_INT_IDLE_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_int_frame_clr(&mut self) -> REG_INT_FRAME_CLR_W {
        REG_INT_FRAME_CLR_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_int_mem_clr(&mut self) -> REG_INT_MEM_CLR_W {
        REG_INT_MEM_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_int_cam_clr(&mut self) -> REG_INT_CAM_CLR_W {
        REG_INT_CAM_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_int_normal_clr(&mut self) -> REG_INT_NORMAL_CLR_W {
        REG_INT_NORMAL_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_w_swap_clr(&mut self) -> REG_W_SWAP_CLR_W {
        REG_W_SWAP_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfifo_pop(&mut self) -> RFIFO_POP_W {
        RFIFO_POP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_frame_fifo_pop.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_frame_fifo_pop](index.html) module"]
pub struct MJPEG_FRAME_FIFO_POP_SPEC;
impl crate::RegisterSpec for MJPEG_FRAME_FIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_frame_fifo_pop::R](R) reader structure"]
impl crate::Readable for MJPEG_FRAME_FIFO_POP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_frame_fifo_pop::W](W) writer structure"]
impl crate::Writable for MJPEG_FRAME_FIFO_POP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_frame_fifo_pop to value 0"]
impl crate::Resettable for MJPEG_FRAME_FIFO_POP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
