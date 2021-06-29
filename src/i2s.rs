#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - i2s_config."]
    pub i2s_config: crate::Reg<i2s_config::I2S_CONFIG_SPEC>,
    #[doc = "0x04 - i2s_int_sts."]
    pub i2s_int_sts: crate::Reg<i2s_int_sts::I2S_INT_STS_SPEC>,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - i2s_bclk_config."]
    pub i2s_bclk_config: crate::Reg<i2s_bclk_config::I2S_BCLK_CONFIG_SPEC>,
    _reserved3: [u8; 0x6c],
    #[doc = "0x80 - i2s_fifo_config_0."]
    pub i2s_fifo_config_0: crate::Reg<i2s_fifo_config_0::I2S_FIFO_CONFIG_0_SPEC>,
    #[doc = "0x84 - i2s_fifo_config_1."]
    pub i2s_fifo_config_1: crate::Reg<i2s_fifo_config_1::I2S_FIFO_CONFIG_1_SPEC>,
    #[doc = "0x88 - i2s_fifo_wdata."]
    pub i2s_fifo_wdata: crate::Reg<i2s_fifo_wdata::I2S_FIFO_WDATA_SPEC>,
    #[doc = "0x8c - i2s_fifo_rdata."]
    pub i2s_fifo_rdata: crate::Reg<i2s_fifo_rdata::I2S_FIFO_RDATA_SPEC>,
    _reserved7: [u8; 0x6c],
    #[doc = "0xfc - i2s_io_config."]
    pub i2s_io_config: crate::Reg<i2s_io_config::I2S_IO_CONFIG_SPEC>,
}
#[doc = "i2s_config register accessor: an alias for `Reg<I2S_CONFIG_SPEC>`"]
pub type I2S_CONFIG = crate::Reg<i2s_config::I2S_CONFIG_SPEC>;
#[doc = "i2s_config."]
pub mod i2s_config;
#[doc = "i2s_int_sts register accessor: an alias for `Reg<I2S_INT_STS_SPEC>`"]
pub type I2S_INT_STS = crate::Reg<i2s_int_sts::I2S_INT_STS_SPEC>;
#[doc = "i2s_int_sts."]
pub mod i2s_int_sts;
#[doc = "i2s_bclk_config register accessor: an alias for `Reg<I2S_BCLK_CONFIG_SPEC>`"]
pub type I2S_BCLK_CONFIG = crate::Reg<i2s_bclk_config::I2S_BCLK_CONFIG_SPEC>;
#[doc = "i2s_bclk_config."]
pub mod i2s_bclk_config;
#[doc = "i2s_fifo_config_0 register accessor: an alias for `Reg<I2S_FIFO_CONFIG_0_SPEC>`"]
pub type I2S_FIFO_CONFIG_0 = crate::Reg<i2s_fifo_config_0::I2S_FIFO_CONFIG_0_SPEC>;
#[doc = "i2s_fifo_config_0."]
pub mod i2s_fifo_config_0;
#[doc = "i2s_fifo_config_1 register accessor: an alias for `Reg<I2S_FIFO_CONFIG_1_SPEC>`"]
pub type I2S_FIFO_CONFIG_1 = crate::Reg<i2s_fifo_config_1::I2S_FIFO_CONFIG_1_SPEC>;
#[doc = "i2s_fifo_config_1."]
pub mod i2s_fifo_config_1;
#[doc = "i2s_fifo_wdata register accessor: an alias for `Reg<I2S_FIFO_WDATA_SPEC>`"]
pub type I2S_FIFO_WDATA = crate::Reg<i2s_fifo_wdata::I2S_FIFO_WDATA_SPEC>;
#[doc = "i2s_fifo_wdata."]
pub mod i2s_fifo_wdata;
#[doc = "i2s_fifo_rdata register accessor: an alias for `Reg<I2S_FIFO_RDATA_SPEC>`"]
pub type I2S_FIFO_RDATA = crate::Reg<i2s_fifo_rdata::I2S_FIFO_RDATA_SPEC>;
#[doc = "i2s_fifo_rdata."]
pub mod i2s_fifo_rdata;
#[doc = "i2s_io_config register accessor: an alias for `Reg<I2S_IO_CONFIG_SPEC>`"]
pub type I2S_IO_CONFIG = crate::Reg<i2s_io_config::I2S_IO_CONFIG_SPEC>;
#[doc = "i2s_io_config."]
pub mod i2s_io_config;
