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
#[doc = "Field `gc_lna` reader - "]
pub type GC_LNA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_lna` writer - "]
pub type GC_LNA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN_BW_SPEC, u8, u8, 3, O>;
#[doc = "Field `gc_rbb1` reader - "]
pub type GC_RBB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_rbb1` writer - "]
pub type GC_RBB1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN_BW_SPEC, u8, u8, 2, O>;
#[doc = "Field `gc_rbb2` reader - "]
pub type GC_RBB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gc_rbb2` writer - "]
pub type GC_RBB2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRX_GAIN_BW_SPEC, u8, u8, 3, O>;
#[doc = "Field `rbb_bw` reader - "]
pub type RBB_BW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bw` writer - "]
pub type RBB_BW_W<'a, const O: u8> = crate::BitWriter<'a, u32, TRX_GAIN_BW_SPEC, bool, O>;
#[doc = "Field `pa_ref_dac` reader - "]
pub type PA_REF_DAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_ref_dac` writer - "]
pub type PA_REF_DAC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_BW_SPEC, u8, u8, 5, O>;
#[doc = "Field `pa_inbuf_unit` reader - "]
pub type PA_INBUF_UNIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_inbuf_unit` writer - "]
pub type PA_INBUF_UNIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRX_GAIN_BW_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn gc_lna(&self) -> GC_LNA_R {
        GC_LNA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn gc_rbb1(&self) -> GC_RBB1_R {
        GC_RBB1_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn gc_rbb2(&self) -> GC_RBB2_R {
        GC_RBB2_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rbb_bw(&self) -> RBB_BW_R {
        RBB_BW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn pa_ref_dac(&self) -> PA_REF_DAC_R {
        PA_REF_DAC_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_inbuf_unit(&self) -> PA_INBUF_UNIT_R {
        PA_INBUF_UNIT_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn gc_lna(&mut self) -> GC_LNA_W<0> {
        GC_LNA_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn gc_rbb1(&mut self) -> GC_RBB1_W<3> {
        GC_RBB1_W::new(self)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    #[must_use]
    pub fn gc_rbb2(&mut self) -> GC_RBB2_W<5> {
        GC_RBB2_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bw(&mut self) -> RBB_BW_W<8> {
        RBB_BW_W::new(self)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    #[must_use]
    pub fn pa_ref_dac(&mut self) -> PA_REF_DAC_W<12> {
        PA_REF_DAC_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn pa_inbuf_unit(&mut self) -> PA_INBUF_UNIT_W<20> {
        PA_INBUF_UNIT_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets trx_gain_bw to value 0"]
impl crate::Resettable for TRX_GAIN_BW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
