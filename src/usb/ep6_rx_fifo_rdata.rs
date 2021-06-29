#[doc = "Register `ep6_rx_fifo_rdata` reader"]
pub struct R(crate::R<EP6_RX_FIFO_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP6_RX_FIFO_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP6_RX_FIFO_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP6_RX_FIFO_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep6_rx_fifo_rdata` writer"]
pub struct W(crate::W<EP6_RX_FIFO_RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP6_RX_FIFO_RDATA_SPEC>;
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
impl From<crate::W<EP6_RX_FIFO_RDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP6_RX_FIFO_RDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ep6_rx_fifo_rdata` reader - "]
pub struct EP6_RX_FIFO_RDATA_R(crate::FieldReader<u8, u8>);
impl EP6_RX_FIFO_RDATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        EP6_RX_FIFO_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6_RX_FIFO_RDATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep6_rx_fifo_rdata` writer - "]
pub struct EP6_RX_FIFO_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_RX_FIFO_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ep6_rx_fifo_rdata(&self) -> EP6_RX_FIFO_RDATA_R {
        EP6_RX_FIFO_RDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ep6_rx_fifo_rdata(&mut self) -> EP6_RX_FIFO_RDATA_W {
        EP6_RX_FIFO_RDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep6_rx_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep6_rx_fifo_rdata](index.html) module"]
pub struct EP6_RX_FIFO_RDATA_SPEC;
impl crate::RegisterSpec for EP6_RX_FIFO_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep6_rx_fifo_rdata::R](R) reader structure"]
impl crate::Readable for EP6_RX_FIFO_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep6_rx_fifo_rdata::W](W) writer structure"]
impl crate::Writable for EP6_RX_FIFO_RDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ep6_rx_fifo_rdata to value 0"]
impl crate::Resettable for EP6_RX_FIFO_RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
