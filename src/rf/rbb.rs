#[doc = "Register `rbb` reader"]
pub struct R(crate::R<RBB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb` writer"]
pub struct W(crate::W<RBB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_SPEC>;
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
impl From<crate::W<RBB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pkdet_out_latch` reader - "]
pub type PKDET_OUT_LATCH_R = crate::BitReader<bool>;
#[doc = "Field `pkdet_out_latch` writer - "]
pub type PKDET_OUT_LATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB_SPEC, bool, O>;
#[doc = "Field `pkdet_out_raw` reader - "]
pub type PKDET_OUT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `pkdet_out_raw` writer - "]
pub type PKDET_OUT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_out_rstn_ctrl_hw` reader - "]
pub type RBB_PKDET_OUT_RSTN_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_out_rstn_ctrl_hw` writer - "]
pub type RBB_PKDET_OUT_RSTN_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RBB_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_out_rstn_hw` reader - "]
pub type RBB_PKDET_OUT_RSTN_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_out_rstn_hw` writer - "]
pub type RBB_PKDET_OUT_RSTN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_out_rstn` reader - "]
pub type RBB_PKDET_OUT_RSTN_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_out_rstn` writer - "]
pub type RBB_PKDET_OUT_RSTN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_en_ctrl_hw` reader - "]
pub type RBB_PKDET_EN_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_en_ctrl_hw` writer - "]
pub type RBB_PKDET_EN_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_en_hw` reader - "]
pub type RBB_PKDET_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_en_hw` writer - "]
pub type RBB_PKDET_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_en` reader - "]
pub type RBB_PKDET_EN_R = crate::BitReader<bool>;
#[doc = "Field `rbb_pkdet_en` writer - "]
pub type RBB_PKDET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB_SPEC, bool, O>;
#[doc = "Field `rbb_pkdet_vth` reader - "]
pub type RBB_PKDET_VTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_pkdet_vth` writer - "]
pub type RBB_PKDET_VTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB_SPEC, u8, u8, 4, O>;
#[doc = "Field `rosdac_range` reader - "]
pub type ROSDAC_RANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_range` writer - "]
pub type ROSDAC_RANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB_SPEC, u8, u8, 2, O>;
#[doc = "Field `rbb_lpf_en` reader - "]
pub type RBB_LPF_EN_R = crate::BitReader<bool>;
#[doc = "Field `rbb_lpf_en` writer - "]
pub type RBB_LPF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RBB_SPEC, bool, O>;
#[doc = "Field `rbb_deq` reader - "]
pub type RBB_DEQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_deq` writer - "]
pub type RBB_DEQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB_SPEC, u8, u8, 2, O>;
#[doc = "Field `rbb_vcm` reader - "]
pub type RBB_VCM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_vcm` writer - "]
pub type RBB_VCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB_SPEC, u8, u8, 2, O>;
#[doc = "Field `rbb_bm_op` reader - "]
pub type RBB_BM_OP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_bm_op` writer - "]
pub type RBB_BM_OP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pkdet_out_latch(&self) -> PKDET_OUT_LATCH_R {
        PKDET_OUT_LATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pkdet_out_raw(&self) -> PKDET_OUT_RAW_R {
        PKDET_OUT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&self) -> RBB_PKDET_OUT_RSTN_CTRL_HW_R {
        RBB_PKDET_OUT_RSTN_CTRL_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_hw(&self) -> RBB_PKDET_OUT_RSTN_HW_R {
        RBB_PKDET_OUT_RSTN_HW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn(&self) -> RBB_PKDET_OUT_RSTN_R {
        RBB_PKDET_OUT_RSTN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_pkdet_en_ctrl_hw(&self) -> RBB_PKDET_EN_CTRL_HW_R {
        RBB_PKDET_EN_CTRL_HW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rbb_pkdet_en_hw(&self) -> RBB_PKDET_EN_HW_R {
        RBB_PKDET_EN_HW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rbb_pkdet_en(&self) -> RBB_PKDET_EN_R {
        RBB_PKDET_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn rbb_pkdet_vth(&self) -> RBB_PKDET_VTH_R {
        RBB_PKDET_VTH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rosdac_range(&self) -> ROSDAC_RANGE_R {
        ROSDAC_RANGE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rbb_lpf_en(&self) -> RBB_LPF_EN_R {
        RBB_LPF_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn rbb_deq(&self) -> RBB_DEQ_R {
        RBB_DEQ_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rbb_vcm(&self) -> RBB_VCM_R {
        RBB_VCM_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn rbb_bm_op(&self) -> RBB_BM_OP_R {
        RBB_BM_OP_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pkdet_out_latch(&mut self) -> PKDET_OUT_LATCH_W<0> {
        PKDET_OUT_LATCH_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pkdet_out_raw(&mut self) -> PKDET_OUT_RAW_W<1> {
        PKDET_OUT_RAW_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&mut self) -> RBB_PKDET_OUT_RSTN_CTRL_HW_W<4> {
        RBB_PKDET_OUT_RSTN_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_out_rstn_hw(&mut self) -> RBB_PKDET_OUT_RSTN_HW_W<5> {
        RBB_PKDET_OUT_RSTN_HW_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_out_rstn(&mut self) -> RBB_PKDET_OUT_RSTN_W<6> {
        RBB_PKDET_OUT_RSTN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_en_ctrl_hw(&mut self) -> RBB_PKDET_EN_CTRL_HW_W<8> {
        RBB_PKDET_EN_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_en_hw(&mut self) -> RBB_PKDET_EN_HW_W<9> {
        RBB_PKDET_EN_HW_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_en(&mut self) -> RBB_PKDET_EN_W<10> {
        RBB_PKDET_EN_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pkdet_vth(&mut self) -> RBB_PKDET_VTH_W<12> {
        RBB_PKDET_VTH_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_range(&mut self) -> ROSDAC_RANGE_W<16> {
        ROSDAC_RANGE_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_lpf_en(&mut self) -> RBB_LPF_EN_W<19> {
        RBB_LPF_EN_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_deq(&mut self) -> RBB_DEQ_W<20> {
        RBB_DEQ_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_vcm(&mut self) -> RBB_VCM_W<24> {
        RBB_VCM_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bm_op(&mut self) -> RBB_BM_OP_W<28> {
        RBB_BM_OP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb](index.html) module"]
pub struct RBB_SPEC;
impl crate::RegisterSpec for RBB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb::R](R) reader structure"]
impl crate::Readable for RBB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb::W](W) writer structure"]
impl crate::Writable for RBB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rbb to value 0"]
impl crate::Resettable for RBB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
