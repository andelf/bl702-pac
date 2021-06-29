#[doc = "Register `ep2_tx_fifo_wdata` reader"]
pub struct R(crate::R<EP2_TX_FIFO_WDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP2_TX_FIFO_WDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP2_TX_FIFO_WDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP2_TX_FIFO_WDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep2_tx_fifo_wdata` writer"]
pub struct W(crate::W<EP2_TX_FIFO_WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP2_TX_FIFO_WDATA_SPEC>;
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
impl From<crate::W<EP2_TX_FIFO_WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP2_TX_FIFO_WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ep2_tx_fifo_wdata` reader - "]
pub struct EP2_TX_FIFO_WDATA_R(crate::FieldReader<u8, u8>);
impl EP2_TX_FIFO_WDATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        EP2_TX_FIFO_WDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_TX_FIFO_WDATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep2_tx_fifo_wdata` writer - "]
pub struct EP2_TX_FIFO_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_TX_FIFO_WDATA_W<'a> {
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
    pub fn ep2_tx_fifo_wdata(&self) -> EP2_TX_FIFO_WDATA_R {
        EP2_TX_FIFO_WDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ep2_tx_fifo_wdata(&mut self) -> EP2_TX_FIFO_WDATA_W {
        EP2_TX_FIFO_WDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep2_tx_fifo_wdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2_tx_fifo_wdata](index.html) module"]
pub struct EP2_TX_FIFO_WDATA_SPEC;
impl crate::RegisterSpec for EP2_TX_FIFO_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep2_tx_fifo_wdata::R](R) reader structure"]
impl crate::Readable for EP2_TX_FIFO_WDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep2_tx_fifo_wdata::W](W) writer structure"]
impl crate::Writable for EP2_TX_FIFO_WDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ep2_tx_fifo_wdata to value 0"]
impl crate::Resettable for EP2_TX_FIFO_WDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
