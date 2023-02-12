#[doc = "Register `spi_bus_busy` reader"]
pub struct R(crate::R<SPI_BUS_BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_BUS_BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_BUS_BUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_BUS_BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_bus_busy` writer"]
pub struct W(crate::W<SPI_BUS_BUSY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_BUS_BUSY_SPEC>;
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
impl From<crate::W<SPI_BUS_BUSY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_BUS_BUSY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_spi_bus_busy` reader - "]
pub type STS_SPI_BUS_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `sts_spi_bus_busy` writer - "]
pub type STS_SPI_BUS_BUSY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_BUS_BUSY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_spi_bus_busy(&self) -> STS_SPI_BUS_BUSY_R {
        STS_SPI_BUS_BUSY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sts_spi_bus_busy(&mut self) -> STS_SPI_BUS_BUSY_W<0> {
        STS_SPI_BUS_BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_bus_busy.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_bus_busy](index.html) module"]
pub struct SPI_BUS_BUSY_SPEC;
impl crate::RegisterSpec for SPI_BUS_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_bus_busy::R](R) reader structure"]
impl crate::Readable for SPI_BUS_BUSY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_bus_busy::W](W) writer structure"]
impl crate::Writable for SPI_BUS_BUSY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_bus_busy to value 0"]
impl crate::Resettable for SPI_BUS_BUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
