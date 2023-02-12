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
#[doc = "Field `rfifo_pop` reader - "]
pub type RFIFO_POP_R = crate::BitReader<bool>;
#[doc = "Field `rfifo_pop` writer - "]
pub type RFIFO_POP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_w_swap_clr` reader - "]
pub type REG_W_SWAP_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_w_swap_clr` writer - "]
pub type REG_W_SWAP_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_normal_clr` reader - "]
pub type REG_INT_NORMAL_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_normal_clr` writer - "]
pub type REG_INT_NORMAL_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_cam_clr` reader - "]
pub type REG_INT_CAM_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_cam_clr` writer - "]
pub type REG_INT_CAM_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_mem_clr` reader - "]
pub type REG_INT_MEM_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_mem_clr` writer - "]
pub type REG_INT_MEM_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_frame_clr` reader - "]
pub type REG_INT_FRAME_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_frame_clr` writer - "]
pub type REG_INT_FRAME_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_idle_clr` reader - "]
pub type REG_INT_IDLE_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_idle_clr` writer - "]
pub type REG_INT_IDLE_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_swap_clr` reader - "]
pub type REG_INT_SWAP_CLR_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_swap_clr` writer - "]
pub type REG_INT_SWAP_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_FRAME_FIFO_POP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfifo_pop(&self) -> RFIFO_POP_R {
        RFIFO_POP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_w_swap_clr(&self) -> REG_W_SWAP_CLR_R {
        REG_W_SWAP_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_int_normal_clr(&self) -> REG_INT_NORMAL_CLR_R {
        REG_INT_NORMAL_CLR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_int_cam_clr(&self) -> REG_INT_CAM_CLR_R {
        REG_INT_CAM_CLR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_int_mem_clr(&self) -> REG_INT_MEM_CLR_R {
        REG_INT_MEM_CLR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_int_frame_clr(&self) -> REG_INT_FRAME_CLR_R {
        REG_INT_FRAME_CLR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_int_idle_clr(&self) -> REG_INT_IDLE_CLR_R {
        REG_INT_IDLE_CLR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_int_swap_clr(&self) -> REG_INT_SWAP_CLR_R {
        REG_INT_SWAP_CLR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rfifo_pop(&mut self) -> RFIFO_POP_W<0> {
        RFIFO_POP_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_w_swap_clr(&mut self) -> REG_W_SWAP_CLR_W<1> {
        REG_W_SWAP_CLR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_normal_clr(&mut self) -> REG_INT_NORMAL_CLR_W<8> {
        REG_INT_NORMAL_CLR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_cam_clr(&mut self) -> REG_INT_CAM_CLR_W<9> {
        REG_INT_CAM_CLR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_mem_clr(&mut self) -> REG_INT_MEM_CLR_W<10> {
        REG_INT_MEM_CLR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_frame_clr(&mut self) -> REG_INT_FRAME_CLR_W<11> {
        REG_INT_FRAME_CLR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_idle_clr(&mut self) -> REG_INT_IDLE_CLR_W<12> {
        REG_INT_IDLE_CLR_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_swap_clr(&mut self) -> REG_INT_SWAP_CLR_W<13> {
        REG_INT_SWAP_CLR_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mjpeg_frame_fifo_pop to value 0"]
impl crate::Resettable for MJPEG_FRAME_FIFO_POP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
