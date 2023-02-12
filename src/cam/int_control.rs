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
#[doc = "Field `reg_int_normal_0_en` reader - "]
pub type REG_INT_NORMAL_0_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_normal_0_en` writer - "]
pub type REG_INT_NORMAL_0_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_int_normal_1_en` reader - "]
pub type REG_INT_NORMAL_1_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_normal_1_en` writer - "]
pub type REG_INT_NORMAL_1_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_int_mem_en` reader - "]
pub type REG_INT_MEM_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_mem_en` writer - "]
pub type REG_INT_MEM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_int_frame_en` reader - "]
pub type REG_INT_FRAME_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_frame_en` writer - "]
pub type REG_INT_FRAME_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_int_fifo_en` reader - "]
pub type REG_INT_FIFO_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_fifo_en` writer - "]
pub type REG_INT_FIFO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_int_hcnt_en` reader - "]
pub type REG_INT_HCNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_hcnt_en` writer - "]
pub type REG_INT_HCNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_int_vcnt_en` reader - "]
pub type REG_INT_VCNT_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_vcnt_en` writer - "]
pub type REG_INT_VCNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_frame_cnt_trgr_int` reader - "]
pub type REG_FRAME_CNT_TRGR_INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_frame_cnt_trgr_int` writer - "]
pub type REG_FRAME_CNT_TRGR_INT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INT_CONTROL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_int_normal_0_en(&self) -> REG_INT_NORMAL_0_EN_R {
        REG_INT_NORMAL_0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_int_normal_1_en(&self) -> REG_INT_NORMAL_1_EN_R {
        REG_INT_NORMAL_1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_int_mem_en(&self) -> REG_INT_MEM_EN_R {
        REG_INT_MEM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_int_frame_en(&self) -> REG_INT_FRAME_EN_R {
        REG_INT_FRAME_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_int_fifo_en(&self) -> REG_INT_FIFO_EN_R {
        REG_INT_FIFO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_int_hcnt_en(&self) -> REG_INT_HCNT_EN_R {
        REG_INT_HCNT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_int_vcnt_en(&self) -> REG_INT_VCNT_EN_R {
        REG_INT_VCNT_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&self) -> REG_FRAME_CNT_TRGR_INT_R {
        REG_FRAME_CNT_TRGR_INT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_normal_0_en(&mut self) -> REG_INT_NORMAL_0_EN_W<0> {
        REG_INT_NORMAL_0_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_normal_1_en(&mut self) -> REG_INT_NORMAL_1_EN_W<1> {
        REG_INT_NORMAL_1_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_mem_en(&mut self) -> REG_INT_MEM_EN_W<2> {
        REG_INT_MEM_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_frame_en(&mut self) -> REG_INT_FRAME_EN_W<3> {
        REG_INT_FRAME_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_fifo_en(&mut self) -> REG_INT_FIFO_EN_W<4> {
        REG_INT_FIFO_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_hcnt_en(&mut self) -> REG_INT_HCNT_EN_W<5> {
        REG_INT_HCNT_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_int_vcnt_en(&mut self) -> REG_INT_VCNT_EN_W<6> {
        REG_INT_VCNT_EN_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_frame_cnt_trgr_int(&mut self) -> REG_FRAME_CNT_TRGR_INT_W<28> {
        REG_FRAME_CNT_TRGR_INT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets int_control to value 0"]
impl crate::Resettable for INT_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
