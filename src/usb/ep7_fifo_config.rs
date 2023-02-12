#[doc = "Register `ep7_fifo_config` reader"]
pub struct R(crate::R<EP7_FIFO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP7_FIFO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP7_FIFO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP7_FIFO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep7_fifo_config` writer"]
pub struct W(crate::W<EP7_FIFO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP7_FIFO_CONFIG_SPEC>;
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
impl From<crate::W<EP7_FIFO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP7_FIFO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ep7_dma_tx_en` reader - "]
pub type EP7_DMA_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `ep7_dma_tx_en` writer - "]
pub type EP7_DMA_TX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP7_FIFO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ep7_dma_rx_en` reader - "]
pub type EP7_DMA_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `ep7_dma_rx_en` writer - "]
pub type EP7_DMA_RX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP7_FIFO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ep7_tx_fifo_clr` reader - "]
pub type EP7_TX_FIFO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `ep7_tx_fifo_clr` writer - "]
pub type EP7_TX_FIFO_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP7_FIFO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ep7_rx_fifo_clr` reader - "]
pub type EP7_RX_FIFO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `ep7_rx_fifo_clr` writer - "]
pub type EP7_RX_FIFO_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP7_FIFO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ep7_tx_fifo_overflow` reader - "]
pub type EP7_TX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `ep7_tx_fifo_overflow` writer - "]
pub type EP7_TX_FIFO_OVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP7_FIFO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ep7_tx_fifo_underflow` reader - "]
pub type EP7_TX_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `ep7_tx_fifo_underflow` writer - "]
pub type EP7_TX_FIFO_UNDERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP7_FIFO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ep7_rx_fifo_overflow` reader - "]
pub type EP7_RX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `ep7_rx_fifo_overflow` writer - "]
pub type EP7_RX_FIFO_OVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP7_FIFO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ep7_rx_fifo_underflow` reader - "]
pub type EP7_RX_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `ep7_rx_fifo_underflow` writer - "]
pub type EP7_RX_FIFO_UNDERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP7_FIFO_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ep7_dma_tx_en(&self) -> EP7_DMA_TX_EN_R {
        EP7_DMA_TX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ep7_dma_rx_en(&self) -> EP7_DMA_RX_EN_R {
        EP7_DMA_RX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ep7_tx_fifo_clr(&self) -> EP7_TX_FIFO_CLR_R {
        EP7_TX_FIFO_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ep7_rx_fifo_clr(&self) -> EP7_RX_FIFO_CLR_R {
        EP7_RX_FIFO_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ep7_tx_fifo_overflow(&self) -> EP7_TX_FIFO_OVERFLOW_R {
        EP7_TX_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ep7_tx_fifo_underflow(&self) -> EP7_TX_FIFO_UNDERFLOW_R {
        EP7_TX_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ep7_rx_fifo_overflow(&self) -> EP7_RX_FIFO_OVERFLOW_R {
        EP7_RX_FIFO_OVERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ep7_rx_fifo_underflow(&self) -> EP7_RX_FIFO_UNDERFLOW_R {
        EP7_RX_FIFO_UNDERFLOW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_dma_tx_en(&mut self) -> EP7_DMA_TX_EN_W<0> {
        EP7_DMA_TX_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_dma_rx_en(&mut self) -> EP7_DMA_RX_EN_W<1> {
        EP7_DMA_RX_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_tx_fifo_clr(&mut self) -> EP7_TX_FIFO_CLR_W<2> {
        EP7_TX_FIFO_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_rx_fifo_clr(&mut self) -> EP7_RX_FIFO_CLR_W<3> {
        EP7_RX_FIFO_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_tx_fifo_overflow(&mut self) -> EP7_TX_FIFO_OVERFLOW_W<4> {
        EP7_TX_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_tx_fifo_underflow(&mut self) -> EP7_TX_FIFO_UNDERFLOW_W<5> {
        EP7_TX_FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_rx_fifo_overflow(&mut self) -> EP7_RX_FIFO_OVERFLOW_W<6> {
        EP7_RX_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_rx_fifo_underflow(&mut self) -> EP7_RX_FIFO_UNDERFLOW_W<7> {
        EP7_RX_FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep7_fifo_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep7_fifo_config](index.html) module"]
pub struct EP7_FIFO_CONFIG_SPEC;
impl crate::RegisterSpec for EP7_FIFO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep7_fifo_config::R](R) reader structure"]
impl crate::Readable for EP7_FIFO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep7_fifo_config::W](W) writer structure"]
impl crate::Writable for EP7_FIFO_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ep7_fifo_config to value 0"]
impl crate::Resettable for EP7_FIFO_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
