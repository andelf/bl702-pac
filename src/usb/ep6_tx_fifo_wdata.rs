#[doc = "Register `ep6_tx_fifo_wdata` reader"]
pub struct R(crate::R<EP6_TX_FIFO_WDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP6_TX_FIFO_WDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP6_TX_FIFO_WDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP6_TX_FIFO_WDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep6_tx_fifo_wdata` writer"]
pub struct W(crate::W<EP6_TX_FIFO_WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP6_TX_FIFO_WDATA_SPEC>;
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
impl From<crate::W<EP6_TX_FIFO_WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP6_TX_FIFO_WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ep6_tx_fifo_wdata` reader - "]
pub type EP6_TX_FIFO_WDATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ep6_tx_fifo_wdata` writer - "]
pub type EP6_TX_FIFO_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP6_TX_FIFO_WDATA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ep6_tx_fifo_wdata(&self) -> EP6_TX_FIFO_WDATA_R {
        EP6_TX_FIFO_WDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_tx_fifo_wdata(&mut self) -> EP6_TX_FIFO_WDATA_W<0> {
        EP6_TX_FIFO_WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep6_tx_fifo_wdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6_tx_fifo_wdata](index.html) module"]
pub struct EP6_TX_FIFO_WDATA_SPEC;
impl crate::RegisterSpec for EP6_TX_FIFO_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep6_tx_fifo_wdata::R](R) reader structure"]
impl crate::Readable for EP6_TX_FIFO_WDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep6_tx_fifo_wdata::W](W) writer structure"]
impl crate::Writable for EP6_TX_FIFO_WDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ep6_tx_fifo_wdata to value 0"]
impl crate::Resettable for EP6_TX_FIFO_WDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
