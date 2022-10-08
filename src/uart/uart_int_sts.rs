#[doc = "Register `uart_int_sts` reader"]
pub struct R(crate::R<UART_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_INT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_int_sts` writer"]
pub struct W(crate::W<UART_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_INT_STS_SPEC>;
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
impl From<crate::W<UART_INT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `utx_end_int` reader - "]
pub type UTX_END_INT_R = crate::BitReader<bool>;
#[doc = "Field `utx_end_int` writer - "]
pub type UTX_END_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_STS_SPEC, bool, O>;
#[doc = "Field `urx_end_int` reader - "]
pub type URX_END_INT_R = crate::BitReader<bool>;
#[doc = "Field `urx_end_int` writer - "]
pub type URX_END_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_STS_SPEC, bool, O>;
#[doc = "Field `utx_fifo_int` reader - "]
pub type UTX_FIFO_INT_R = crate::BitReader<bool>;
#[doc = "Field `utx_fifo_int` writer - "]
pub type UTX_FIFO_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_STS_SPEC, bool, O>;
#[doc = "Field `urx_fifo_int` reader - "]
pub type URX_FIFO_INT_R = crate::BitReader<bool>;
#[doc = "Field `urx_fifo_int` writer - "]
pub type URX_FIFO_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_STS_SPEC, bool, O>;
#[doc = "Field `urx_rto_int` reader - "]
pub type URX_RTO_INT_R = crate::BitReader<bool>;
#[doc = "Field `urx_rto_int` writer - "]
pub type URX_RTO_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_STS_SPEC, bool, O>;
#[doc = "Field `urx_pce_int` reader - "]
pub type URX_PCE_INT_R = crate::BitReader<bool>;
#[doc = "Field `urx_pce_int` writer - "]
pub type URX_PCE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_STS_SPEC, bool, O>;
#[doc = "Field `utx_fer_int` reader - "]
pub type UTX_FER_INT_R = crate::BitReader<bool>;
#[doc = "Field `utx_fer_int` writer - "]
pub type UTX_FER_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_STS_SPEC, bool, O>;
#[doc = "Field `urx_fer_int` reader - "]
pub type URX_FER_INT_R = crate::BitReader<bool>;
#[doc = "Field `urx_fer_int` writer - "]
pub type URX_FER_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_STS_SPEC, bool, O>;
#[doc = "Field `urx_lse_int` reader - "]
pub type URX_LSE_INT_R = crate::BitReader<bool>;
#[doc = "Field `urx_lse_int` writer - "]
pub type URX_LSE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utx_end_int(&self) -> UTX_END_INT_R {
        UTX_END_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn urx_end_int(&self) -> URX_END_INT_R {
        URX_END_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn utx_fifo_int(&self) -> UTX_FIFO_INT_R {
        UTX_FIFO_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn urx_fifo_int(&self) -> URX_FIFO_INT_R {
        URX_FIFO_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn urx_rto_int(&self) -> URX_RTO_INT_R {
        URX_RTO_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn urx_pce_int(&self) -> URX_PCE_INT_R {
        URX_PCE_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn utx_fer_int(&self) -> UTX_FER_INT_R {
        UTX_FER_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn urx_fer_int(&self) -> URX_FER_INT_R {
        URX_FER_INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn urx_lse_int(&self) -> URX_LSE_INT_R {
        URX_LSE_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utx_end_int(&mut self) -> UTX_END_INT_W<0> {
        UTX_END_INT_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn urx_end_int(&mut self) -> URX_END_INT_W<1> {
        URX_END_INT_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn utx_fifo_int(&mut self) -> UTX_FIFO_INT_W<2> {
        UTX_FIFO_INT_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn urx_fifo_int(&mut self) -> URX_FIFO_INT_W<3> {
        URX_FIFO_INT_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn urx_rto_int(&mut self) -> URX_RTO_INT_W<4> {
        URX_RTO_INT_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn urx_pce_int(&mut self) -> URX_PCE_INT_W<5> {
        URX_PCE_INT_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn utx_fer_int(&mut self) -> UTX_FER_INT_W<6> {
        UTX_FER_INT_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn urx_fer_int(&mut self) -> URX_FER_INT_W<7> {
        URX_FER_INT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn urx_lse_int(&mut self) -> URX_LSE_INT_W<8> {
        URX_LSE_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_sts](index.html) module"]
pub struct UART_INT_STS_SPEC;
impl crate::RegisterSpec for UART_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_int_sts::R](R) reader structure"]
impl crate::Readable for UART_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_int_sts::W](W) writer structure"]
impl crate::Writable for UART_INT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets uart_int_sts to value 0"]
impl crate::Resettable for UART_INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
