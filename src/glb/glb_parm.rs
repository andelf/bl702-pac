#[doc = "Register `glb_parm` reader"]
pub struct R(crate::R<GLB_PARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLB_PARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GLB_PARM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GLB_PARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `glb_parm` writer"]
pub struct W(crate::W<GLB_PARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLB_PARM_SPEC>;
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
impl From<crate::W<GLB_PARM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GLB_PARM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin_sel_emac_cam` reader - "]
pub struct PIN_SEL_EMAC_CAM_R(crate::FieldReader<bool, bool>);
impl PIN_SEL_EMAC_CAM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN_SEL_EMAC_CAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_SEL_EMAC_CAM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin_sel_emac_cam` writer - "]
pub struct PIN_SEL_EMAC_CAM_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_SEL_EMAC_CAM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `reg_ext_rst_smt` reader - "]
pub struct REG_EXT_RST_SMT_R(crate::FieldReader<bool, bool>);
impl REG_EXT_RST_SMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_EXT_RST_SMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_EXT_RST_SMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_ext_rst_smt` writer - "]
pub struct REG_EXT_RST_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_EXT_RST_SMT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `reg_kys_drv_val` reader - "]
pub struct REG_KYS_DRV_VAL_R(crate::FieldReader<bool, bool>);
impl REG_KYS_DRV_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_KYS_DRV_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_KYS_DRV_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_kys_drv_val` writer - "]
pub struct REG_KYS_DRV_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_KYS_DRV_VAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `uart_swap_set` reader - "]
pub struct UART_SWAP_SET_R(crate::FieldReader<u8, u8>);
impl UART_SWAP_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_SWAP_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SWAP_SET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_swap_set` writer - "]
pub struct UART_SWAP_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SWAP_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `p6_jtag_use_io_0_2_7` reader - "]
pub struct P6_JTAG_USE_IO_0_2_7_R(crate::FieldReader<bool, bool>);
impl P6_JTAG_USE_IO_0_2_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P6_JTAG_USE_IO_0_2_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6_JTAG_USE_IO_0_2_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p6_jtag_use_io_0_2_7` writer - "]
pub struct P6_JTAG_USE_IO_0_2_7_W<'a> {
    w: &'a mut W,
}
impl<'a> P6_JTAG_USE_IO_0_2_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `p5_dac_test_with_jtag` reader - "]
pub struct P5_DAC_TEST_WITH_JTAG_R(crate::FieldReader<bool, bool>);
impl P5_DAC_TEST_WITH_JTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P5_DAC_TEST_WITH_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5_DAC_TEST_WITH_JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p5_dac_test_with_jtag` writer - "]
pub struct P5_DAC_TEST_WITH_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> P5_DAC_TEST_WITH_JTAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `p4_adc_test_with_jtag` reader - "]
pub struct P4_ADC_TEST_WITH_JTAG_R(crate::FieldReader<bool, bool>);
impl P4_ADC_TEST_WITH_JTAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P4_ADC_TEST_WITH_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4_ADC_TEST_WITH_JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p4_adc_test_with_jtag` writer - "]
pub struct P4_ADC_TEST_WITH_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_ADC_TEST_WITH_JTAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `p3_cci_use_io_0_2_7` reader - "]
pub struct P3_CCI_USE_IO_0_2_7_R(crate::FieldReader<bool, bool>);
impl P3_CCI_USE_IO_0_2_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P3_CCI_USE_IO_0_2_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3_CCI_USE_IO_0_2_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p3_cci_use_io_0_2_7` writer - "]
pub struct P3_CCI_USE_IO_0_2_7_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_CCI_USE_IO_0_2_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `p2_dac_test_with_cci` reader - "]
pub struct P2_DAC_TEST_WITH_CCI_R(crate::FieldReader<bool, bool>);
impl P2_DAC_TEST_WITH_CCI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P2_DAC_TEST_WITH_CCI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_DAC_TEST_WITH_CCI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p2_dac_test_with_cci` writer - "]
pub struct P2_DAC_TEST_WITH_CCI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_DAC_TEST_WITH_CCI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `p1_adc_test_with_cci` reader - "]
pub struct P1_ADC_TEST_WITH_CCI_R(crate::FieldReader<bool, bool>);
impl P1_ADC_TEST_WITH_CCI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        P1_ADC_TEST_WITH_CCI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1_ADC_TEST_WITH_CCI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p1_adc_test_with_cci` writer - "]
pub struct P1_ADC_TEST_WITH_CCI_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_ADC_TEST_WITH_CCI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `reg_cci_use_jtag_pin` reader - "]
pub struct REG_CCI_USE_JTAG_PIN_R(crate::FieldReader<bool, bool>);
impl REG_CCI_USE_JTAG_PIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_CCI_USE_JTAG_PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CCI_USE_JTAG_PIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_cci_use_jtag_pin` writer - "]
pub struct REG_CCI_USE_JTAG_PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CCI_USE_JTAG_PIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `reg_spi_0_swap` reader - "]
pub struct REG_SPI_0_SWAP_R(crate::FieldReader<bool, bool>);
impl REG_SPI_0_SWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_SPI_0_SWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SPI_0_SWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_spi_0_swap` writer - "]
pub struct REG_SPI_0_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SPI_0_SWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `reg_spi_0_master_mode` reader - "]
pub struct REG_SPI_0_MASTER_MODE_R(crate::FieldReader<bool, bool>);
impl REG_SPI_0_MASTER_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_SPI_0_MASTER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SPI_0_MASTER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_spi_0_master_mode` writer - "]
pub struct REG_SPI_0_MASTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SPI_0_MASTER_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `cfg_flash_scenario` reader - "]
pub struct CFG_FLASH_SCENARIO_R(crate::FieldReader<u8, u8>);
impl CFG_FLASH_SCENARIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFG_FLASH_SCENARIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_FLASH_SCENARIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cfg_flash_scenario` writer - "]
pub struct CFG_FLASH_SCENARIO_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_FLASH_SCENARIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `cfg_sflash2_swap_cs_io2` reader - "]
pub struct CFG_SFLASH2_SWAP_CS_IO2_R(crate::FieldReader<bool, bool>);
impl CFG_SFLASH2_SWAP_CS_IO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_SFLASH2_SWAP_CS_IO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_SFLASH2_SWAP_CS_IO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cfg_sflash2_swap_cs_io2` writer - "]
pub struct CFG_SFLASH2_SWAP_CS_IO2_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_SFLASH2_SWAP_CS_IO2_W<'a> {
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
#[doc = "Field `cfg_sflash2_swap_io0_io3` reader - "]
pub struct CFG_SFLASH2_SWAP_IO0_IO3_R(crate::FieldReader<bool, bool>);
impl CFG_SFLASH2_SWAP_IO0_IO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CFG_SFLASH2_SWAP_IO0_IO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_SFLASH2_SWAP_IO0_IO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cfg_sflash2_swap_io0_io3` writer - "]
pub struct CFG_SFLASH2_SWAP_IO0_IO3_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_SFLASH2_SWAP_IO0_IO3_W<'a> {
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
#[doc = "Field `jtag_swap_set` reader - "]
pub struct JTAG_SWAP_SET_R(crate::FieldReader<u8, u8>);
impl JTAG_SWAP_SET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        JTAG_SWAP_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_SWAP_SET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `jtag_swap_set` writer - "]
pub struct JTAG_SWAP_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_SWAP_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pin_sel_emac_cam(&self) -> PIN_SEL_EMAC_CAM_R {
        PIN_SEL_EMAC_CAM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reg_ext_rst_smt(&self) -> REG_EXT_RST_SMT_R {
        REG_EXT_RST_SMT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_kys_drv_val(&self) -> REG_KYS_DRV_VAL_R {
        REG_KYS_DRV_VAL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn uart_swap_set(&self) -> UART_SWAP_SET_R {
        UART_SWAP_SET_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p6_jtag_use_io_0_2_7(&self) -> P6_JTAG_USE_IO_0_2_7_R {
        P6_JTAG_USE_IO_0_2_7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&self) -> P5_DAC_TEST_WITH_JTAG_R {
        P5_DAC_TEST_WITH_JTAG_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&self) -> P4_ADC_TEST_WITH_JTAG_R {
        P4_ADC_TEST_WITH_JTAG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_0_2_7(&self) -> P3_CCI_USE_IO_0_2_7_R {
        P3_CCI_USE_IO_0_2_7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&self) -> P2_DAC_TEST_WITH_CCI_R {
        P2_DAC_TEST_WITH_CCI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&self) -> P1_ADC_TEST_WITH_CCI_R {
        P1_ADC_TEST_WITH_CCI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_cci_use_jtag_pin(&self) -> REG_CCI_USE_JTAG_PIN_R {
        REG_CCI_USE_JTAG_PIN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&self) -> REG_SPI_0_SWAP_R {
        REG_SPI_0_SWAP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&self) -> REG_SPI_0_MASTER_MODE_R {
        REG_SPI_0_MASTER_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn cfg_flash_scenario(&self) -> CFG_FLASH_SCENARIO_R {
        CFG_FLASH_SCENARIO_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_sflash2_swap_cs_io2(&self) -> CFG_SFLASH2_SWAP_CS_IO2_R {
        CFG_SFLASH2_SWAP_CS_IO2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cfg_sflash2_swap_io0_io3(&self) -> CFG_SFLASH2_SWAP_IO0_IO3_R {
        CFG_SFLASH2_SWAP_IO0_IO3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn jtag_swap_set(&self) -> JTAG_SWAP_SET_R {
        JTAG_SWAP_SET_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pin_sel_emac_cam(&mut self) -> PIN_SEL_EMAC_CAM_W {
        PIN_SEL_EMAC_CAM_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reg_ext_rst_smt(&mut self) -> REG_EXT_RST_SMT_W {
        REG_EXT_RST_SMT_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_kys_drv_val(&mut self) -> REG_KYS_DRV_VAL_W {
        REG_KYS_DRV_VAL_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn uart_swap_set(&mut self) -> UART_SWAP_SET_W {
        UART_SWAP_SET_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p6_jtag_use_io_0_2_7(&mut self) -> P6_JTAG_USE_IO_0_2_7_W {
        P6_JTAG_USE_IO_0_2_7_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&mut self) -> P5_DAC_TEST_WITH_JTAG_W {
        P5_DAC_TEST_WITH_JTAG_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&mut self) -> P4_ADC_TEST_WITH_JTAG_W {
        P4_ADC_TEST_WITH_JTAG_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_0_2_7(&mut self) -> P3_CCI_USE_IO_0_2_7_W {
        P3_CCI_USE_IO_0_2_7_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&mut self) -> P2_DAC_TEST_WITH_CCI_W {
        P2_DAC_TEST_WITH_CCI_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&mut self) -> P1_ADC_TEST_WITH_CCI_W {
        P1_ADC_TEST_WITH_CCI_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_cci_use_jtag_pin(&mut self) -> REG_CCI_USE_JTAG_PIN_W {
        REG_CCI_USE_JTAG_PIN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&mut self) -> REG_SPI_0_SWAP_W {
        REG_SPI_0_SWAP_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&mut self) -> REG_SPI_0_MASTER_MODE_W {
        REG_SPI_0_MASTER_MODE_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn cfg_flash_scenario(&mut self) -> CFG_FLASH_SCENARIO_W {
        CFG_FLASH_SCENARIO_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_sflash2_swap_cs_io2(&mut self) -> CFG_SFLASH2_SWAP_CS_IO2_W {
        CFG_SFLASH2_SWAP_CS_IO2_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cfg_sflash2_swap_io0_io3(&mut self) -> CFG_SFLASH2_SWAP_IO0_IO3_W {
        CFG_SFLASH2_SWAP_IO0_IO3_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn jtag_swap_set(&mut self) -> JTAG_SWAP_SET_W {
        JTAG_SWAP_SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "glb_parm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glb_parm](index.html) module"]
pub struct GLB_PARM_SPEC;
impl crate::RegisterSpec for GLB_PARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [glb_parm::R](R) reader structure"]
impl crate::Readable for GLB_PARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glb_parm::W](W) writer structure"]
impl crate::Writable for GLB_PARM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets glb_parm to value 0"]
impl crate::Resettable for GLB_PARM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
