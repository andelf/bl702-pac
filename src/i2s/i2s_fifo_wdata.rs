#[doc = "Register `i2s_fifo_wdata` reader"]
pub struct R(crate::R<I2S_FIFO_WDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_FIFO_WDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_FIFO_WDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_FIFO_WDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_fifo_wdata` writer"]
pub struct W(crate::W<I2S_FIFO_WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_FIFO_WDATA_SPEC>;
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
impl From<crate::W<I2S_FIFO_WDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_FIFO_WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `i2s_fifo_wdata` reader - "]
pub type I2S_FIFO_WDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `i2s_fifo_wdata` writer - "]
pub type I2S_FIFO_WDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_FIFO_WDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_fifo_wdata(&self) -> I2S_FIFO_WDATA_R {
        I2S_FIFO_WDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_fifo_wdata(&mut self) -> I2S_FIFO_WDATA_W<0> {
        I2S_FIFO_WDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2s_fifo_wdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_fifo_wdata](index.html) module"]
pub struct I2S_FIFO_WDATA_SPEC;
impl crate::RegisterSpec for I2S_FIFO_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_fifo_wdata::R](R) reader structure"]
impl crate::Readable for I2S_FIFO_WDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_fifo_wdata::W](W) writer structure"]
impl crate::Writable for I2S_FIFO_WDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2s_fifo_wdata to value 0"]
impl crate::Resettable for I2S_FIFO_WDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
