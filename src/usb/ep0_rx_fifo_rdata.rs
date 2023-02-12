#[doc = "Register `ep0_rx_fifo_rdata` reader"]
pub struct R(crate::R<EP0_RX_FIFO_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP0_RX_FIFO_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP0_RX_FIFO_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP0_RX_FIFO_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep0_rx_fifo_rdata` writer"]
pub struct W(crate::W<EP0_RX_FIFO_RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP0_RX_FIFO_RDATA_SPEC>;
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
impl From<crate::W<EP0_RX_FIFO_RDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP0_RX_FIFO_RDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ep0_rx_fifo_rdata` reader - "]
pub type EP0_RX_FIFO_RDATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ep0_rx_fifo_rdata` writer - "]
pub type EP0_RX_FIFO_RDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP0_RX_FIFO_RDATA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ep0_rx_fifo_rdata(&self) -> EP0_RX_FIFO_RDATA_R {
        EP0_RX_FIFO_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_rx_fifo_rdata(&mut self) -> EP0_RX_FIFO_RDATA_W<0> {
        EP0_RX_FIFO_RDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep0_rx_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep0_rx_fifo_rdata](index.html) module"]
pub struct EP0_RX_FIFO_RDATA_SPEC;
impl crate::RegisterSpec for EP0_RX_FIFO_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep0_rx_fifo_rdata::R](R) reader structure"]
impl crate::Readable for EP0_RX_FIFO_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep0_rx_fifo_rdata::W](W) writer structure"]
impl crate::Writable for EP0_RX_FIFO_RDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ep0_rx_fifo_rdata to value 0"]
impl crate::Resettable for EP0_RX_FIFO_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
