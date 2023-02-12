#[doc = "Register `trx_gain_bw_hw` reader"]
pub struct R(crate::R<TRX_GAIN_BW_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRX_GAIN_BW_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRX_GAIN_BW_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRX_GAIN_BW_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `trx_gain_bw_hw` writer"]
pub struct W(crate::W<TRX_GAIN_BW_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRX_GAIN_BW_HW_SPEC>;
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
impl From<crate::W<TRX_GAIN_BW_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRX_GAIN_BW_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gc_lna_hw` reader - "]
pub type GC_LNA_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_lna_hw` writer - "]
pub type GC_LNA_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_BW_HW_SPEC, u8, u8, 3, O>;
#[doc = "Field `gc_rbb1_hw` reader - "]
pub type GC_RBB1_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_rbb1_hw` writer - "]
pub type GC_RBB1_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_BW_HW_SPEC, u8, u8, 2, O>;
#[doc = "Field `gc_rbb2_hw` reader - "]
pub type GC_RBB2_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_rbb2_hw` writer - "]
pub type GC_RBB2_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_BW_HW_SPEC, u8, u8, 3, O>;
#[doc = "Field `rbb_bw_hw` reader - "]
pub type RBB_BW_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bw_hw` writer - "]
pub type RBB_BW_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRX_GAIN_BW_HW_SPEC, bool, O>;
#[doc = "Field `pa_ref_dac_hw` reader - "]
pub type PA_REF_DAC_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_ref_dac_hw` writer - "]
pub type PA_REF_DAC_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_BW_HW_SPEC, u8, u8, 5, O>;
#[doc = "Field `pa_inbuf_unit_hw` reader - "]
pub type PA_INBUF_UNIT_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_inbuf_unit_hw` writer - "]
pub type PA_INBUF_UNIT_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_BW_HW_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna_hw(&self) -> GC_LNA_HW_R {
        GC_LNA_HW_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn gc_rbb1_hw(&self) -> GC_RBB1_HW_R {
        GC_RBB1_HW_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn gc_rbb2_hw(&self) -> GC_RBB2_HW_R {
        GC_RBB2_HW_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_bw_hw(&self) -> RBB_BW_HW_R {
        RBB_BW_HW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn pa_ref_dac_hw(&self) -> PA_REF_DAC_HW_R {
        PA_REF_DAC_HW_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_inbuf_unit_hw(&self) -> PA_INBUF_UNIT_HW_R {
        PA_INBUF_UNIT_HW_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn gc_lna_hw(&mut self) -> GC_LNA_HW_W<0> {
        GC_LNA_HW_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn gc_rbb1_hw(&mut self) -> GC_RBB1_HW_W<3> {
        GC_RBB1_HW_W::new(self)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    #[must_use]
    pub fn gc_rbb2_hw(&mut self) -> GC_RBB2_HW_W<5> {
        GC_RBB2_HW_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bw_hw(&mut self) -> RBB_BW_HW_W<8> {
        RBB_BW_HW_W::new(self)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    #[must_use]
    pub fn pa_ref_dac_hw(&mut self) -> PA_REF_DAC_HW_W<12> {
        PA_REF_DAC_HW_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn pa_inbuf_unit_hw(&mut self) -> PA_INBUF_UNIT_HW_W<20> {
        PA_INBUF_UNIT_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware read back of TX/RX gain\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trx_gain_bw_hw](index.html) module"]
pub struct TRX_GAIN_BW_HW_SPEC;
impl crate::RegisterSpec for TRX_GAIN_BW_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trx_gain_bw_hw::R](R) reader structure"]
impl crate::Readable for TRX_GAIN_BW_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trx_gain_bw_hw::W](W) writer structure"]
impl crate::Writable for TRX_GAIN_BW_HW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets trx_gain_bw_hw to value 0"]
impl crate::Resettable for TRX_GAIN_BW_HW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
