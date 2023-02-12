#[doc = "Register `uart_status` reader"]
pub struct R(crate::R<UART_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_status` writer"]
pub struct W(crate::W<UART_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_STATUS_SPEC>;
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
impl From<crate::W<UART_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_utx_bus_busy` reader - "]
pub type STS_UTX_BUS_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `sts_utx_bus_busy` writer - "]
pub type STS_UTX_BUS_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_STATUS_SPEC, bool, O>;
#[doc = "Field `sts_urx_bus_busy` reader - "]
pub type STS_URX_BUS_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `sts_urx_bus_busy` writer - "]
pub type STS_URX_BUS_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_utx_bus_busy(&self) -> STS_UTX_BUS_BUSY_R {
        STS_UTX_BUS_BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_urx_bus_busy(&self) -> STS_URX_BUS_BUSY_R {
        STS_URX_BUS_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sts_utx_bus_busy(&mut self) -> STS_UTX_BUS_BUSY_W<0> {
        STS_UTX_BUS_BUSY_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sts_urx_bus_busy(&mut self) -> STS_URX_BUS_BUSY_W<1> {
        STS_URX_BUS_BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_status](index.html) module"]
pub struct UART_STATUS_SPEC;
impl crate::RegisterSpec for UART_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_status::R](R) reader structure"]
impl crate::Readable for UART_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_status::W](W) writer structure"]
impl crate::Writable for UART_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_status to value 0"]
impl crate::Resettable for UART_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
