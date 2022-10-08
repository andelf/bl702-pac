#[doc = "Register `gpadc_dma_rdata` reader"]
pub struct R(crate::R<GPADC_DMA_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_DMA_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_DMA_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_DMA_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_dma_rdata` writer"]
pub struct W(crate::W<GPADC_DMA_RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_DMA_RDATA_SPEC>;
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
impl From<crate::W<GPADC_DMA_RDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_DMA_RDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_dma_rdata` reader - "]
pub type GPADC_DMA_RDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `gpadc_dma_rdata` writer - "]
pub type GPADC_DMA_RDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_DMA_RDATA_SPEC, u32, u32, 26, O>;
#[doc = "Field `rsvd_31_26` reader - "]
pub type RSVD_31_26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rsvd_31_26` writer - "]
pub type RSVD_31_26_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPADC_DMA_RDATA_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&self) -> GPADC_DMA_RDATA_R {
        GPADC_DMA_RDATA_R::new((self.bits & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rsvd_31_26(&self) -> RSVD_31_26_R {
        RSVD_31_26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&mut self) -> GPADC_DMA_RDATA_W<0> {
        GPADC_DMA_RDATA_W::new(self)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rsvd_31_26(&mut self) -> RSVD_31_26_W<26> {
        RSVD_31_26_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_dma_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_dma_rdata](index.html) module"]
pub struct GPADC_DMA_RDATA_SPEC;
impl crate::RegisterSpec for GPADC_DMA_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_dma_rdata::R](R) reader structure"]
impl crate::Readable for GPADC_DMA_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_dma_rdata::W](W) writer structure"]
impl crate::Writable for GPADC_DMA_RDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_dma_rdata to value 0"]
impl crate::Resettable for GPADC_DMA_RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
