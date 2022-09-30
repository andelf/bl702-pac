#[doc = "Register `clk_cfg3` reader"]
pub struct R(crate::R<CLK_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg3` writer"]
pub struct W(crate::W<CLK_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG3_SPEC>;
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
impl From<crate::W<CLK_CFG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_clk_div` reader - "]
pub type SPI_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spi_clk_div` writer - "]
pub type SPI_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG3_SPEC, u8, u8, 5, O>;
#[doc = "Field `cfg_sel_eth_ref_clk_o` reader - "]
pub type CFG_SEL_ETH_REF_CLK_O_R = crate::BitReader<bool>;
#[doc = "Field `cfg_sel_eth_ref_clk_o` writer - "]
pub type CFG_SEL_ETH_REF_CLK_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CFG3_SPEC, bool, O>;
#[doc = "Field `cfg_inv_eth_ref_clk_o` reader - "]
pub type CFG_INV_ETH_REF_CLK_O_R = crate::BitReader<bool>;
#[doc = "Field `cfg_inv_eth_ref_clk_o` writer - "]
pub type CFG_INV_ETH_REF_CLK_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CFG3_SPEC, bool, O>;
#[doc = "Field `cfg_inv_eth_tx_clk` reader - "]
pub type CFG_INV_ETH_TX_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cfg_inv_eth_tx_clk` writer - "]
pub type CFG_INV_ETH_TX_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG3_SPEC, bool, O>;
#[doc = "Field `spi_clk_en` reader - "]
pub type SPI_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `spi_clk_en` writer - "]
pub type SPI_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG3_SPEC, bool, O>;
#[doc = "Field `cfg_inv_rf_test_clk_o` reader - "]
pub type CFG_INV_RF_TEST_CLK_O_R = crate::BitReader<bool>;
#[doc = "Field `cfg_inv_rf_test_clk_o` writer - "]
pub type CFG_INV_RF_TEST_CLK_O_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CFG3_SPEC, bool, O>;
#[doc = "Field `cfg_inv_eth_rx_clk` reader - "]
pub type CFG_INV_ETH_RX_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cfg_inv_eth_rx_clk` writer - "]
pub type CFG_INV_ETH_RX_CLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG3_SPEC, bool, O>;
#[doc = "Field `i2c_clk_div` reader - "]
pub type I2C_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `i2c_clk_div` writer - "]
pub type I2C_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG3_SPEC, u8, u8, 8, O>;
#[doc = "Field `i2c_clk_en` reader - "]
pub type I2C_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `i2c_clk_en` writer - "]
pub type I2C_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG3_SPEC, bool, O>;
#[doc = "Field `chip_clk_out_0_sel` reader - "]
pub type CHIP_CLK_OUT_0_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `chip_clk_out_0_sel` writer - "]
pub type CHIP_CLK_OUT_0_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_CFG3_SPEC, u8, u8, 2, O>;
#[doc = "Field `chip_clk_out_1_sel` reader - "]
pub type CHIP_CLK_OUT_1_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `chip_clk_out_1_sel` writer - "]
pub type CHIP_CLK_OUT_1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_CFG3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&self) -> SPI_CLK_DIV_R {
        SPI_CLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cfg_sel_eth_ref_clk_o(&self) -> CFG_SEL_ETH_REF_CLK_O_R {
        CFG_SEL_ETH_REF_CLK_O_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cfg_inv_eth_ref_clk_o(&self) -> CFG_INV_ETH_REF_CLK_O_R {
        CFG_INV_ETH_REF_CLK_O_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_inv_eth_tx_clk(&self) -> CFG_INV_ETH_TX_CLK_R {
        CFG_INV_ETH_TX_CLK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SPI_CLK_EN_R {
        SPI_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_inv_rf_test_clk_o(&self) -> CFG_INV_RF_TEST_CLK_O_R {
        CFG_INV_RF_TEST_CLK_O_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_inv_eth_rx_clk(&self) -> CFG_INV_ETH_RX_CLK_R {
        CFG_INV_ETH_RX_CLK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&self) -> I2C_CLK_DIV_R {
        I2C_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn chip_clk_out_0_sel(&self) -> CHIP_CLK_OUT_0_SEL_R {
        CHIP_CLK_OUT_0_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn chip_clk_out_1_sel(&self) -> CHIP_CLK_OUT_1_SEL_R {
        CHIP_CLK_OUT_1_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&mut self) -> SPI_CLK_DIV_W<0> {
        SPI_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cfg_sel_eth_ref_clk_o(&mut self) -> CFG_SEL_ETH_REF_CLK_O_W<5> {
        CFG_SEL_ETH_REF_CLK_O_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cfg_inv_eth_ref_clk_o(&mut self) -> CFG_INV_ETH_REF_CLK_O_W<6> {
        CFG_INV_ETH_REF_CLK_O_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_inv_eth_tx_clk(&mut self) -> CFG_INV_ETH_TX_CLK_W<7> {
        CFG_INV_ETH_TX_CLK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&mut self) -> SPI_CLK_EN_W<8> {
        SPI_CLK_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_inv_rf_test_clk_o(&mut self) -> CFG_INV_RF_TEST_CLK_O_W<9> {
        CFG_INV_RF_TEST_CLK_O_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_inv_eth_rx_clk(&mut self) -> CFG_INV_ETH_RX_CLK_W<10> {
        CFG_INV_ETH_RX_CLK_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&mut self) -> I2C_CLK_DIV_W<16> {
        I2C_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W<24> {
        I2C_CLK_EN_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn chip_clk_out_0_sel(&mut self) -> CHIP_CLK_OUT_0_SEL_W<28> {
        CHIP_CLK_OUT_0_SEL_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn chip_clk_out_1_sel(&mut self) -> CHIP_CLK_OUT_1_SEL_W<30> {
        CHIP_CLK_OUT_1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clk_cfg3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg3](index.html) module"]
pub struct CLK_CFG3_SPEC;
impl crate::RegisterSpec for CLK_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg3::R](R) reader structure"]
impl crate::Readable for CLK_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg3::W](W) writer structure"]
impl crate::Writable for CLK_CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clk_cfg3 to value 0"]
impl crate::Resettable for CLK_CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
