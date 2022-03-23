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
#[doc = "Field `chip_clk_out_1_sel` reader - "]
pub struct CHIP_CLK_OUT_1_SEL_R(crate::FieldReader<u8, u8>);
impl CHIP_CLK_OUT_1_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHIP_CLK_OUT_1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_CLK_OUT_1_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `chip_clk_out_1_sel` writer - "]
pub struct CHIP_CLK_OUT_1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_CLK_OUT_1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `chip_clk_out_0_sel` reader - "]
pub struct CHIP_CLK_OUT_0_SEL_R(crate::FieldReader<u8, u8>);
impl CHIP_CLK_OUT_0_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHIP_CLK_OUT_0_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_CLK_OUT_0_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `chip_clk_out_0_sel` writer - "]
pub struct CHIP_CLK_OUT_0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_CLK_OUT_0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `i2c_clk_en` reader - "]
pub struct I2C_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2C_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c_clk_en` writer - "]
pub struct I2C_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `i2c_clk_div` reader - "]
pub struct I2C_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl I2C_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2C_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c_clk_div` writer - "]
pub struct I2C_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `cfg_inv_eth_rx_clk` reader - "]
pub struct CFG_INV_ETH_RX_CLK_R(crate::FieldReader<bool, bool>);
impl CFG_INV_ETH_RX_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_INV_ETH_RX_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_INV_ETH_RX_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cfg_inv_eth_rx_clk` writer - "]
pub struct CFG_INV_ETH_RX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_INV_ETH_RX_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `cfg_inv_rf_test_clk_o` reader - "]
pub struct CFG_INV_RF_TEST_CLK_O_R(crate::FieldReader<bool, bool>);
impl CFG_INV_RF_TEST_CLK_O_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_INV_RF_TEST_CLK_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_INV_RF_TEST_CLK_O_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cfg_inv_rf_test_clk_o` writer - "]
pub struct CFG_INV_RF_TEST_CLK_O_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_INV_RF_TEST_CLK_O_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `spi_clk_en` reader - "]
pub struct SPI_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spi_clk_en` writer - "]
pub struct SPI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `cfg_inv_eth_tx_clk` reader - "]
pub struct CFG_INV_ETH_TX_CLK_R(crate::FieldReader<bool, bool>);
impl CFG_INV_ETH_TX_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_INV_ETH_TX_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_INV_ETH_TX_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cfg_inv_eth_tx_clk` writer - "]
pub struct CFG_INV_ETH_TX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_INV_ETH_TX_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `cfg_inv_eth_ref_clk_o` reader - "]
pub struct CFG_INV_ETH_REF_CLK_O_R(crate::FieldReader<bool, bool>);
impl CFG_INV_ETH_REF_CLK_O_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_INV_ETH_REF_CLK_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_INV_ETH_REF_CLK_O_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cfg_inv_eth_ref_clk_o` writer - "]
pub struct CFG_INV_ETH_REF_CLK_O_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_INV_ETH_REF_CLK_O_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `cfg_sel_eth_ref_clk_o` reader - "]
pub struct CFG_SEL_ETH_REF_CLK_O_R(crate::FieldReader<bool, bool>);
impl CFG_SEL_ETH_REF_CLK_O_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_SEL_ETH_REF_CLK_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_SEL_ETH_REF_CLK_O_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cfg_sel_eth_ref_clk_o` writer - "]
pub struct CFG_SEL_ETH_REF_CLK_O_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_SEL_ETH_REF_CLK_O_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `spi_clk_div` reader - "]
pub struct SPI_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl SPI_CLK_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spi_clk_div` writer - "]
pub struct SPI_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn chip_clk_out_1_sel(&self) -> CHIP_CLK_OUT_1_SEL_R {
        CHIP_CLK_OUT_1_SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn chip_clk_out_0_sel(&self) -> CHIP_CLK_OUT_0_SEL_R {
        CHIP_CLK_OUT_0_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&self) -> I2C_CLK_DIV_R {
        I2C_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_inv_eth_rx_clk(&self) -> CFG_INV_ETH_RX_CLK_R {
        CFG_INV_ETH_RX_CLK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_inv_rf_test_clk_o(&self) -> CFG_INV_RF_TEST_CLK_O_R {
        CFG_INV_RF_TEST_CLK_O_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SPI_CLK_EN_R {
        SPI_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_inv_eth_tx_clk(&self) -> CFG_INV_ETH_TX_CLK_R {
        CFG_INV_ETH_TX_CLK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cfg_inv_eth_ref_clk_o(&self) -> CFG_INV_ETH_REF_CLK_O_R {
        CFG_INV_ETH_REF_CLK_O_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cfg_sel_eth_ref_clk_o(&self) -> CFG_SEL_ETH_REF_CLK_O_R {
        CFG_SEL_ETH_REF_CLK_O_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&self) -> SPI_CLK_DIV_R {
        SPI_CLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn chip_clk_out_1_sel(&mut self) -> CHIP_CLK_OUT_1_SEL_W {
        CHIP_CLK_OUT_1_SEL_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn chip_clk_out_0_sel(&mut self) -> CHIP_CLK_OUT_0_SEL_W {
        CHIP_CLK_OUT_0_SEL_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W {
        I2C_CLK_EN_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&mut self) -> I2C_CLK_DIV_W {
        I2C_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_inv_eth_rx_clk(&mut self) -> CFG_INV_ETH_RX_CLK_W {
        CFG_INV_ETH_RX_CLK_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_inv_rf_test_clk_o(&mut self) -> CFG_INV_RF_TEST_CLK_O_W {
        CFG_INV_RF_TEST_CLK_O_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&mut self) -> SPI_CLK_EN_W {
        SPI_CLK_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cfg_inv_eth_tx_clk(&mut self) -> CFG_INV_ETH_TX_CLK_W {
        CFG_INV_ETH_TX_CLK_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cfg_inv_eth_ref_clk_o(&mut self) -> CFG_INV_ETH_REF_CLK_O_W {
        CFG_INV_ETH_REF_CLK_O_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cfg_sel_eth_ref_clk_o(&mut self) -> CFG_SEL_ETH_REF_CLK_O_W {
        CFG_SEL_ETH_REF_CLK_O_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&mut self) -> SPI_CLK_DIV_W {
        SPI_CLK_DIV_W { w: self }
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
