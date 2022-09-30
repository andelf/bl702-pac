#[doc = "Register `i2s_fifo_config_0` reader"]
pub struct R(crate::R<I2S_FIFO_CONFIG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_FIFO_CONFIG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_FIFO_CONFIG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_FIFO_CONFIG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_fifo_config_0` writer"]
pub struct W(crate::W<I2S_FIFO_CONFIG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_FIFO_CONFIG_0_SPEC>;
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
impl From<crate::W<I2S_FIFO_CONFIG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_FIFO_CONFIG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `i2s_dma_tx_en` reader - "]
pub type I2S_DMA_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `i2s_dma_tx_en` writer - "]
pub type I2S_DMA_TX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `i2s_dma_rx_en` reader - "]
pub type I2S_DMA_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `i2s_dma_rx_en` writer - "]
pub type I2S_DMA_RX_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `tx_fifo_clr` reader - "]
pub type TX_FIFO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_clr` writer - "]
pub type TX_FIFO_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `rx_fifo_clr` reader - "]
pub type RX_FIFO_CLR_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_clr` writer - "]
pub type RX_FIFO_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `tx_fifo_overflow` reader - "]
pub type TX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_overflow` writer - "]
pub type TX_FIFO_OVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `tx_fifo_underflow` reader - "]
pub type TX_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_underflow` writer - "]
pub type TX_FIFO_UNDERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `rx_fifo_overflow` reader - "]
pub type RX_FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_overflow` writer - "]
pub type RX_FIFO_OVERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `rx_fifo_underflow` reader - "]
pub type RX_FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_underflow` writer - "]
pub type RX_FIFO_UNDERFLOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `cr_fifo_lr_merge` reader - "]
pub type CR_FIFO_LR_MERGE_R = crate::BitReader<bool>;
#[doc = "Field `cr_fifo_lr_merge` writer - "]
pub type CR_FIFO_LR_MERGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `cr_fifo_lr_exchg` reader - "]
pub type CR_FIFO_LR_EXCHG_R = crate::BitReader<bool>;
#[doc = "Field `cr_fifo_lr_exchg` writer - "]
pub type CR_FIFO_LR_EXCHG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
#[doc = "Field `cr_fifo_24b_lj` reader - "]
pub type CR_FIFO_24B_LJ_R = crate::BitReader<bool>;
#[doc = "Field `cr_fifo_24b_lj` writer - "]
pub type CR_FIFO_24B_LJ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_FIFO_CONFIG_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_dma_tx_en(&self) -> I2S_DMA_TX_EN_R {
        I2S_DMA_TX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_dma_rx_en(&self) -> I2S_DMA_RX_EN_R {
        I2S_DMA_RX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&self) -> TX_FIFO_CLR_R {
        TX_FIFO_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&self) -> RX_FIFO_CLR_R {
        RX_FIFO_CLR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TX_FIFO_OVERFLOW_R {
        TX_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TX_FIFO_UNDERFLOW_R {
        TX_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RX_FIFO_UNDERFLOW_R {
        RX_FIFO_UNDERFLOW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_fifo_lr_merge(&self) -> CR_FIFO_LR_MERGE_R {
        CR_FIFO_LR_MERGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_fifo_lr_exchg(&self) -> CR_FIFO_LR_EXCHG_R {
        CR_FIFO_LR_EXCHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_fifo_24b_lj(&self) -> CR_FIFO_24B_LJ_R {
        CR_FIFO_24B_LJ_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_dma_tx_en(&mut self) -> I2S_DMA_TX_EN_W<0> {
        I2S_DMA_TX_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_dma_rx_en(&mut self) -> I2S_DMA_RX_EN_W<1> {
        I2S_DMA_RX_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TX_FIFO_CLR_W<2> {
        TX_FIFO_CLR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RX_FIFO_CLR_W<3> {
        RX_FIFO_CLR_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&mut self) -> TX_FIFO_OVERFLOW_W<4> {
        TX_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&mut self) -> TX_FIFO_UNDERFLOW_W<5> {
        TX_FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&mut self) -> RX_FIFO_OVERFLOW_W<6> {
        RX_FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&mut self) -> RX_FIFO_UNDERFLOW_W<7> {
        RX_FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_fifo_lr_merge(&mut self) -> CR_FIFO_LR_MERGE_W<8> {
        CR_FIFO_LR_MERGE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_fifo_lr_exchg(&mut self) -> CR_FIFO_LR_EXCHG_W<9> {
        CR_FIFO_LR_EXCHG_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_fifo_24b_lj(&mut self) -> CR_FIFO_24B_LJ_W<10> {
        CR_FIFO_24B_LJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2s_fifo_config_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_fifo_config_0](index.html) module"]
pub struct I2S_FIFO_CONFIG_0_SPEC;
impl crate::RegisterSpec for I2S_FIFO_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_fifo_config_0::R](R) reader structure"]
impl crate::Readable for I2S_FIFO_CONFIG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_fifo_config_0::W](W) writer structure"]
impl crate::Writable for I2S_FIFO_CONFIG_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2s_fifo_config_0 to value 0"]
impl crate::Resettable for I2S_FIFO_CONFIG_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
