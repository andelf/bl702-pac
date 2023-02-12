#[doc = "Register `dvp_frame_fifo_pop` reader"]
pub struct R(crate::R<DVP_FRAME_FIFO_POP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_FRAME_FIFO_POP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_FRAME_FIFO_POP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_FRAME_FIFO_POP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dvp_frame_fifo_pop` writer"]
pub struct W(crate::W<DVP_FRAME_FIFO_POP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP_FRAME_FIFO_POP_SPEC>;
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
impl From<crate::W<DVP_FRAME_FIFO_POP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP_FRAME_FIFO_POP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rfifo_pop_0` reader - "]
pub type RFIFO_POP_0_R = crate::BitReader<bool>;
#[doc = "Field `rfifo_pop_0` writer - "]
pub type RFIFO_POP_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `rfifo_pop_1` reader - "]
pub type RFIFO_POP_1_R = crate::BitReader<bool>;
#[doc = "Field `rfifo_pop_1` writer - "]
pub type RFIFO_POP_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_normal_clr_0` reader - "]
pub type REG_INT_NORMAL_CLR_0_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_normal_clr_0` writer - "]
pub type REG_INT_NORMAL_CLR_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_mem_clr_0` reader - "]
pub type REG_INT_MEM_CLR_0_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_mem_clr_0` writer - "]
pub type REG_INT_MEM_CLR_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_frame_clr_0` reader - "]
pub type REG_INT_FRAME_CLR_0_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_frame_clr_0` writer - "]
pub type REG_INT_FRAME_CLR_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_fifo_clr_0` reader - "]
pub type REG_INT_FIFO_CLR_0_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_fifo_clr_0` writer - "]
pub type REG_INT_FIFO_CLR_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_hcnt_clr_0` reader - "]
pub type REG_INT_HCNT_CLR_0_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_hcnt_clr_0` writer - "]
pub type REG_INT_HCNT_CLR_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_vcnt_clr_0` reader - "]
pub type REG_INT_VCNT_CLR_0_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_vcnt_clr_0` writer - "]
pub type REG_INT_VCNT_CLR_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_normal_clr_1` reader - "]
pub type REG_INT_NORMAL_CLR_1_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_normal_clr_1` writer - "]
pub type REG_INT_NORMAL_CLR_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_mem_clr_1` reader - "]
pub type REG_INT_MEM_CLR_1_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_mem_clr_1` writer - "]
pub type REG_INT_MEM_CLR_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_frame_clr_1` reader - "]
pub type REG_INT_FRAME_CLR_1_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_frame_clr_1` writer - "]
pub type REG_INT_FRAME_CLR_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
#[doc = "Field `reg_int_fifo_clr_1` reader - "]
pub type REG_INT_FIFO_CLR_1_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_fifo_clr_1` writer - "]
pub type REG_INT_FIFO_CLR_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_FRAME_FIFO_POP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfifo_pop_0(&self) -> RFIFO_POP_0_R {
        RFIFO_POP_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfifo_pop_1(&self) -> RFIFO_POP_1_R {
        RFIFO_POP_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_int_normal_clr_0(&self) -> REG_INT_NORMAL_CLR_0_R {
        REG_INT_NORMAL_CLR_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_int_mem_clr_0(&self) -> REG_INT_MEM_CLR_0_R {
        REG_INT_MEM_CLR_0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_int_frame_clr_0(&self) -> REG_INT_FRAME_CLR_0_R {
        REG_INT_FRAME_CLR_0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_int_fifo_clr_0(&self) -> REG_INT_FIFO_CLR_0_R {
        REG_INT_FIFO_CLR_0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_int_hcnt_clr_0(&self) -> REG_INT_HCNT_CLR_0_R {
        REG_INT_HCNT_CLR_0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_int_vcnt_clr_0(&self) -> REG_INT_VCNT_CLR_0_R {
        REG_INT_VCNT_CLR_0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_int_normal_clr_1(&self) -> REG_INT_NORMAL_CLR_1_R {
        REG_INT_NORMAL_CLR_1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_int_mem_clr_1(&self) -> REG_INT_MEM_CLR_1_R {
        REG_INT_MEM_CLR_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_int_frame_clr_1(&self) -> REG_INT_FRAME_CLR_1_R {
        REG_INT_FRAME_CLR_1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_int_fifo_clr_1(&self) -> REG_INT_FIFO_CLR_1_R {
        REG_INT_FIFO_CLR_1_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rfifo_pop_0(&mut self) -> RFIFO_POP_0_W<0> {
        RFIFO_POP_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn rfifo_pop_1(&mut self) -> RFIFO_POP_1_W<1> {
        RFIFO_POP_1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_normal_clr_0(&mut self) -> REG_INT_NORMAL_CLR_0_W<4> {
        REG_INT_NORMAL_CLR_0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_mem_clr_0(&mut self) -> REG_INT_MEM_CLR_0_W<5> {
        REG_INT_MEM_CLR_0_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_frame_clr_0(&mut self) -> REG_INT_FRAME_CLR_0_W<6> {
        REG_INT_FRAME_CLR_0_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_fifo_clr_0(&mut self) -> REG_INT_FIFO_CLR_0_W<7> {
        REG_INT_FIFO_CLR_0_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_hcnt_clr_0(&mut self) -> REG_INT_HCNT_CLR_0_W<8> {
        REG_INT_HCNT_CLR_0_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_vcnt_clr_0(&mut self) -> REG_INT_VCNT_CLR_0_W<9> {
        REG_INT_VCNT_CLR_0_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_normal_clr_1(&mut self) -> REG_INT_NORMAL_CLR_1_W<16> {
        REG_INT_NORMAL_CLR_1_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_mem_clr_1(&mut self) -> REG_INT_MEM_CLR_1_W<17> {
        REG_INT_MEM_CLR_1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_frame_clr_1(&mut self) -> REG_INT_FRAME_CLR_1_W<18> {
        REG_INT_FRAME_CLR_1_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_fifo_clr_1(&mut self) -> REG_INT_FIFO_CLR_1_W<19> {
        REG_INT_FIFO_CLR_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dvp_frame_fifo_pop.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_frame_fifo_pop](index.html) module"]
pub struct DVP_FRAME_FIFO_POP_SPEC;
impl crate::RegisterSpec for DVP_FRAME_FIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp_frame_fifo_pop::R](R) reader structure"]
impl crate::Readable for DVP_FRAME_FIFO_POP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp_frame_fifo_pop::W](W) writer structure"]
impl crate::Writable for DVP_FRAME_FIFO_POP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dvp_frame_fifo_pop to value 0"]
impl crate::Resettable for DVP_FRAME_FIFO_POP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
