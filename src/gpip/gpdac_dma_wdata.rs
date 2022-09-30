#[doc = "Register `gpdac_dma_wdata` reader"]
pub struct R(crate::R<GPDAC_DMA_WDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_DMA_WDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_DMA_WDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_DMA_WDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_dma_wdata` writer"]
pub struct W(crate::W<GPDAC_DMA_WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_DMA_WDATA_SPEC>;
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
impl From<crate::W<GPDAC_DMA_WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDAC_DMA_WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpdac_dma_wdata` reader - "]
pub type GPDAC_DMA_WDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `gpdac_dma_wdata` writer - "]
pub type GPDAC_DMA_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPDAC_DMA_WDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpdac_dma_wdata(&self) -> GPDAC_DMA_WDATA_R {
        GPDAC_DMA_WDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gpdac_dma_wdata(&mut self) -> GPDAC_DMA_WDATA_W<0> {
        GPDAC_DMA_WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpdac_dma_wdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_dma_wdata](index.html) module"]
pub struct GPDAC_DMA_WDATA_SPEC;
impl crate::RegisterSpec for GPDAC_DMA_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_dma_wdata::R](R) reader structure"]
impl crate::Readable for GPDAC_DMA_WDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_dma_wdata::W](W) writer structure"]
impl crate::Writable for GPDAC_DMA_WDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpdac_dma_wdata to value 0"]
impl crate::Resettable for GPDAC_DMA_WDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
