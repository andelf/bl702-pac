#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - usb_config."]
    pub usb_config: crate::Reg<usb_config::USB_CONFIG_SPEC>,
    #[doc = "0x04 - usb_lpm_config."]
    pub usb_lpm_config: crate::Reg<usb_lpm_config::USB_LPM_CONFIG_SPEC>,
    #[doc = "0x08 - usb_resume_config."]
    pub usb_resume_config: crate::Reg<usb_resume_config::USB_RESUME_CONFIG_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - usb_setup_data_0."]
    pub usb_setup_data_0: crate::Reg<usb_setup_data_0::USB_SETUP_DATA_0_SPEC>,
    #[doc = "0x14 - usb_setup_data_1."]
    pub usb_setup_data_1: crate::Reg<usb_setup_data_1::USB_SETUP_DATA_1_SPEC>,
    #[doc = "0x18 - usb_frame_no."]
    pub usb_frame_no: crate::Reg<usb_frame_no::USB_FRAME_NO_SPEC>,
    #[doc = "0x1c - usb_error."]
    pub usb_error: crate::Reg<usb_error::USB_ERROR_SPEC>,
    #[doc = "0x20 - USB interrupt enable"]
    pub usb_int_en: crate::Reg<usb_int_en::USB_INT_EN_SPEC>,
    #[doc = "0x24 - USB interrupt status"]
    pub usb_int_sts: crate::Reg<usb_int_sts::USB_INT_STS_SPEC>,
    #[doc = "0x28 - USB interrupt mask"]
    pub usb_int_mask: crate::Reg<usb_int_mask::USB_INT_MASK_SPEC>,
    #[doc = "0x2c - USB interrupt clear"]
    pub usb_int_clear: crate::Reg<usb_int_clear::USB_INT_CLEAR_SPEC>,
    _reserved11: [u8; 0x10],
    #[doc = "0x40 - ep1_config."]
    pub ep1_config: crate::Reg<ep1_config::EP1_CONFIG_SPEC>,
    #[doc = "0x44 - ep2_config."]
    pub ep2_config: crate::Reg<ep2_config::EP2_CONFIG_SPEC>,
    #[doc = "0x48 - ep3_config."]
    pub ep3_config: crate::Reg<ep3_config::EP3_CONFIG_SPEC>,
    #[doc = "0x4c - ep4_config."]
    pub ep4_config: crate::Reg<ep4_config::EP4_CONFIG_SPEC>,
    #[doc = "0x50 - ep5_config."]
    pub ep5_config: crate::Reg<ep5_config::EP5_CONFIG_SPEC>,
    #[doc = "0x54 - ep6_config."]
    pub ep6_config: crate::Reg<ep6_config::EP6_CONFIG_SPEC>,
    #[doc = "0x58 - ep7_config."]
    pub ep7_config: crate::Reg<ep7_config::EP7_CONFIG_SPEC>,
    _reserved18: [u8; 0xa4],
    #[doc = "0x100 - ep0_fifo_config."]
    pub ep0_fifo_config: crate::Reg<ep0_fifo_config::EP0_FIFO_CONFIG_SPEC>,
    #[doc = "0x104 - ep0_fifo_status."]
    pub ep0_fifo_status: crate::Reg<ep0_fifo_status::EP0_FIFO_STATUS_SPEC>,
    #[doc = "0x108 - ep0_tx_fifo_wdata."]
    pub ep0_tx_fifo_wdata: crate::Reg<ep0_tx_fifo_wdata::EP0_TX_FIFO_WDATA_SPEC>,
    #[doc = "0x10c - ep0_rx_fifo_rdata."]
    pub ep0_rx_fifo_rdata: crate::Reg<ep0_rx_fifo_rdata::EP0_RX_FIFO_RDATA_SPEC>,
    #[doc = "0x110 - ep1_fifo_config."]
    pub ep1_fifo_config: crate::Reg<ep1_fifo_config::EP1_FIFO_CONFIG_SPEC>,
    #[doc = "0x114 - ep1_fifo_status."]
    pub ep1_fifo_status: crate::Reg<ep1_fifo_status::EP1_FIFO_STATUS_SPEC>,
    #[doc = "0x118 - ep1_tx_fifo_wdata."]
    pub ep1_tx_fifo_wdata: crate::Reg<ep1_tx_fifo_wdata::EP1_TX_FIFO_WDATA_SPEC>,
    #[doc = "0x11c - ep1_rx_fifo_rdata."]
    pub ep1_rx_fifo_rdata: crate::Reg<ep1_rx_fifo_rdata::EP1_RX_FIFO_RDATA_SPEC>,
    #[doc = "0x120 - ep2_fifo_config."]
    pub ep2_fifo_config: crate::Reg<ep2_fifo_config::EP2_FIFO_CONFIG_SPEC>,
    #[doc = "0x124 - ep2_fifo_status."]
    pub ep2_fifo_status: crate::Reg<ep2_fifo_status::EP2_FIFO_STATUS_SPEC>,
    #[doc = "0x128 - ep2_tx_fifo_wdata."]
    pub ep2_tx_fifo_wdata: crate::Reg<ep2_tx_fifo_wdata::EP2_TX_FIFO_WDATA_SPEC>,
    #[doc = "0x12c - ep2_rx_fifo_rdata."]
    pub ep2_rx_fifo_rdata: crate::Reg<ep2_rx_fifo_rdata::EP2_RX_FIFO_RDATA_SPEC>,
    #[doc = "0x130 - ep3_fifo_config."]
    pub ep3_fifo_config: crate::Reg<ep3_fifo_config::EP3_FIFO_CONFIG_SPEC>,
    #[doc = "0x134 - ep3_fifo_status."]
    pub ep3_fifo_status: crate::Reg<ep3_fifo_status::EP3_FIFO_STATUS_SPEC>,
    #[doc = "0x138 - ep3_tx_fifo_wdata."]
    pub ep3_tx_fifo_wdata: crate::Reg<ep3_tx_fifo_wdata::EP3_TX_FIFO_WDATA_SPEC>,
    #[doc = "0x13c - ep3_rx_fifo_rdata."]
    pub ep3_rx_fifo_rdata: crate::Reg<ep3_rx_fifo_rdata::EP3_RX_FIFO_RDATA_SPEC>,
    #[doc = "0x140 - ep4_fifo_config."]
    pub ep4_fifo_config: crate::Reg<ep4_fifo_config::EP4_FIFO_CONFIG_SPEC>,
    #[doc = "0x144 - ep4_fifo_status."]
    pub ep4_fifo_status: crate::Reg<ep4_fifo_status::EP4_FIFO_STATUS_SPEC>,
    #[doc = "0x148 - ep4_tx_fifo_wdata."]
    pub ep4_tx_fifo_wdata: crate::Reg<ep4_tx_fifo_wdata::EP4_TX_FIFO_WDATA_SPEC>,
    #[doc = "0x14c - ep4_rx_fifo_rdata."]
    pub ep4_rx_fifo_rdata: crate::Reg<ep4_rx_fifo_rdata::EP4_RX_FIFO_RDATA_SPEC>,
    #[doc = "0x150 - ep5_fifo_config."]
    pub ep5_fifo_config: crate::Reg<ep5_fifo_config::EP5_FIFO_CONFIG_SPEC>,
    #[doc = "0x154 - ep5_fifo_status."]
    pub ep5_fifo_status: crate::Reg<ep5_fifo_status::EP5_FIFO_STATUS_SPEC>,
    #[doc = "0x158 - ep5_tx_fifo_wdata."]
    pub ep5_tx_fifo_wdata: crate::Reg<ep5_tx_fifo_wdata::EP5_TX_FIFO_WDATA_SPEC>,
    #[doc = "0x15c - ep5_rx_fifo_rdata."]
    pub ep5_rx_fifo_rdata: crate::Reg<ep5_rx_fifo_rdata::EP5_RX_FIFO_RDATA_SPEC>,
    #[doc = "0x160 - ep6_fifo_config."]
    pub ep6_fifo_config: crate::Reg<ep6_fifo_config::EP6_FIFO_CONFIG_SPEC>,
    #[doc = "0x164 - ep6_fifo_status."]
    pub ep6_fifo_status: crate::Reg<ep6_fifo_status::EP6_FIFO_STATUS_SPEC>,
    #[doc = "0x168 - ep6_tx_fifo_wdata."]
    pub ep6_tx_fifo_wdata: crate::Reg<ep6_tx_fifo_wdata::EP6_TX_FIFO_WDATA_SPEC>,
    #[doc = "0x16c - ep6_rx_fifo_rdata."]
    pub ep6_rx_fifo_rdata: crate::Reg<ep6_rx_fifo_rdata::EP6_RX_FIFO_RDATA_SPEC>,
    #[doc = "0x170 - ep7_fifo_config."]
    pub ep7_fifo_config: crate::Reg<ep7_fifo_config::EP7_FIFO_CONFIG_SPEC>,
    #[doc = "0x174 - ep7_fifo_status."]
    pub ep7_fifo_status: crate::Reg<ep7_fifo_status::EP7_FIFO_STATUS_SPEC>,
    #[doc = "0x178 - ep7_tx_fifo_wdata."]
    pub ep7_tx_fifo_wdata: crate::Reg<ep7_tx_fifo_wdata::EP7_TX_FIFO_WDATA_SPEC>,
    #[doc = "0x17c - ep7_rx_fifo_rdata."]
    pub ep7_rx_fifo_rdata: crate::Reg<ep7_rx_fifo_rdata::EP7_RX_FIFO_RDATA_SPEC>,
    _reserved50: [u8; 0x70],
    #[doc = "0x1f0 - rsvd_0."]
    pub rsvd_0: crate::Reg<rsvd_0::RSVD_0_SPEC>,
    #[doc = "0x1f4 - rsvd_1."]
    pub rsvd_1: crate::Reg<rsvd_1::RSVD_1_SPEC>,
    _reserved52: [u8; 0x04],
    #[doc = "0x1fc - xcvr_if_config."]
    pub xcvr_if_config: crate::Reg<xcvr_if_config::XCVR_IF_CONFIG_SPEC>,
}
#[doc = "usb_config register accessor: an alias for `Reg<USB_CONFIG_SPEC>`"]
pub type USB_CONFIG = crate::Reg<usb_config::USB_CONFIG_SPEC>;
#[doc = "usb_config."]
pub mod usb_config;
#[doc = "usb_lpm_config register accessor: an alias for `Reg<USB_LPM_CONFIG_SPEC>`"]
pub type USB_LPM_CONFIG = crate::Reg<usb_lpm_config::USB_LPM_CONFIG_SPEC>;
#[doc = "usb_lpm_config."]
pub mod usb_lpm_config;
#[doc = "usb_resume_config register accessor: an alias for `Reg<USB_RESUME_CONFIG_SPEC>`"]
pub type USB_RESUME_CONFIG = crate::Reg<usb_resume_config::USB_RESUME_CONFIG_SPEC>;
#[doc = "usb_resume_config."]
pub mod usb_resume_config;
#[doc = "usb_setup_data_0 register accessor: an alias for `Reg<USB_SETUP_DATA_0_SPEC>`"]
pub type USB_SETUP_DATA_0 = crate::Reg<usb_setup_data_0::USB_SETUP_DATA_0_SPEC>;
#[doc = "usb_setup_data_0."]
pub mod usb_setup_data_0;
#[doc = "usb_setup_data_1 register accessor: an alias for `Reg<USB_SETUP_DATA_1_SPEC>`"]
pub type USB_SETUP_DATA_1 = crate::Reg<usb_setup_data_1::USB_SETUP_DATA_1_SPEC>;
#[doc = "usb_setup_data_1."]
pub mod usb_setup_data_1;
#[doc = "usb_frame_no register accessor: an alias for `Reg<USB_FRAME_NO_SPEC>`"]
pub type USB_FRAME_NO = crate::Reg<usb_frame_no::USB_FRAME_NO_SPEC>;
#[doc = "usb_frame_no."]
pub mod usb_frame_no;
#[doc = "usb_error register accessor: an alias for `Reg<USB_ERROR_SPEC>`"]
pub type USB_ERROR = crate::Reg<usb_error::USB_ERROR_SPEC>;
#[doc = "usb_error."]
pub mod usb_error;
#[doc = "usb_int_en register accessor: an alias for `Reg<USB_INT_EN_SPEC>`"]
pub type USB_INT_EN = crate::Reg<usb_int_en::USB_INT_EN_SPEC>;
#[doc = "USB interrupt enable"]
pub mod usb_int_en;
#[doc = "usb_int_sts register accessor: an alias for `Reg<USB_INT_STS_SPEC>`"]
pub type USB_INT_STS = crate::Reg<usb_int_sts::USB_INT_STS_SPEC>;
#[doc = "USB interrupt status"]
pub mod usb_int_sts;
#[doc = "usb_int_mask register accessor: an alias for `Reg<USB_INT_MASK_SPEC>`"]
pub type USB_INT_MASK = crate::Reg<usb_int_mask::USB_INT_MASK_SPEC>;
#[doc = "USB interrupt mask"]
pub mod usb_int_mask;
#[doc = "usb_int_clear register accessor: an alias for `Reg<USB_INT_CLEAR_SPEC>`"]
pub type USB_INT_CLEAR = crate::Reg<usb_int_clear::USB_INT_CLEAR_SPEC>;
#[doc = "USB interrupt clear"]
pub mod usb_int_clear;
#[doc = "ep1_config register accessor: an alias for `Reg<EP1_CONFIG_SPEC>`"]
pub type EP1_CONFIG = crate::Reg<ep1_config::EP1_CONFIG_SPEC>;
#[doc = "ep1_config."]
pub mod ep1_config;
#[doc = "ep2_config register accessor: an alias for `Reg<EP2_CONFIG_SPEC>`"]
pub type EP2_CONFIG = crate::Reg<ep2_config::EP2_CONFIG_SPEC>;
#[doc = "ep2_config."]
pub mod ep2_config;
#[doc = "ep3_config register accessor: an alias for `Reg<EP3_CONFIG_SPEC>`"]
pub type EP3_CONFIG = crate::Reg<ep3_config::EP3_CONFIG_SPEC>;
#[doc = "ep3_config."]
pub mod ep3_config;
#[doc = "ep4_config register accessor: an alias for `Reg<EP4_CONFIG_SPEC>`"]
pub type EP4_CONFIG = crate::Reg<ep4_config::EP4_CONFIG_SPEC>;
#[doc = "ep4_config."]
pub mod ep4_config;
#[doc = "ep5_config register accessor: an alias for `Reg<EP5_CONFIG_SPEC>`"]
pub type EP5_CONFIG = crate::Reg<ep5_config::EP5_CONFIG_SPEC>;
#[doc = "ep5_config."]
pub mod ep5_config;
#[doc = "ep6_config register accessor: an alias for `Reg<EP6_CONFIG_SPEC>`"]
pub type EP6_CONFIG = crate::Reg<ep6_config::EP6_CONFIG_SPEC>;
#[doc = "ep6_config."]
pub mod ep6_config;
#[doc = "ep7_config register accessor: an alias for `Reg<EP7_CONFIG_SPEC>`"]
pub type EP7_CONFIG = crate::Reg<ep7_config::EP7_CONFIG_SPEC>;
#[doc = "ep7_config."]
pub mod ep7_config;
#[doc = "ep0_fifo_config register accessor: an alias for `Reg<EP0_FIFO_CONFIG_SPEC>`"]
pub type EP0_FIFO_CONFIG = crate::Reg<ep0_fifo_config::EP0_FIFO_CONFIG_SPEC>;
#[doc = "ep0_fifo_config."]
pub mod ep0_fifo_config;
#[doc = "ep0_fifo_status register accessor: an alias for `Reg<EP0_FIFO_STATUS_SPEC>`"]
pub type EP0_FIFO_STATUS = crate::Reg<ep0_fifo_status::EP0_FIFO_STATUS_SPEC>;
#[doc = "ep0_fifo_status."]
pub mod ep0_fifo_status;
#[doc = "ep0_tx_fifo_wdata register accessor: an alias for `Reg<EP0_TX_FIFO_WDATA_SPEC>`"]
pub type EP0_TX_FIFO_WDATA = crate::Reg<ep0_tx_fifo_wdata::EP0_TX_FIFO_WDATA_SPEC>;
#[doc = "ep0_tx_fifo_wdata."]
pub mod ep0_tx_fifo_wdata;
#[doc = "ep0_rx_fifo_rdata register accessor: an alias for `Reg<EP0_RX_FIFO_RDATA_SPEC>`"]
pub type EP0_RX_FIFO_RDATA = crate::Reg<ep0_rx_fifo_rdata::EP0_RX_FIFO_RDATA_SPEC>;
#[doc = "ep0_rx_fifo_rdata."]
pub mod ep0_rx_fifo_rdata;
#[doc = "ep1_fifo_config register accessor: an alias for `Reg<EP1_FIFO_CONFIG_SPEC>`"]
pub type EP1_FIFO_CONFIG = crate::Reg<ep1_fifo_config::EP1_FIFO_CONFIG_SPEC>;
#[doc = "ep1_fifo_config."]
pub mod ep1_fifo_config;
#[doc = "ep1_fifo_status register accessor: an alias for `Reg<EP1_FIFO_STATUS_SPEC>`"]
pub type EP1_FIFO_STATUS = crate::Reg<ep1_fifo_status::EP1_FIFO_STATUS_SPEC>;
#[doc = "ep1_fifo_status."]
pub mod ep1_fifo_status;
#[doc = "ep1_tx_fifo_wdata register accessor: an alias for `Reg<EP1_TX_FIFO_WDATA_SPEC>`"]
pub type EP1_TX_FIFO_WDATA = crate::Reg<ep1_tx_fifo_wdata::EP1_TX_FIFO_WDATA_SPEC>;
#[doc = "ep1_tx_fifo_wdata."]
pub mod ep1_tx_fifo_wdata;
#[doc = "ep1_rx_fifo_rdata register accessor: an alias for `Reg<EP1_RX_FIFO_RDATA_SPEC>`"]
pub type EP1_RX_FIFO_RDATA = crate::Reg<ep1_rx_fifo_rdata::EP1_RX_FIFO_RDATA_SPEC>;
#[doc = "ep1_rx_fifo_rdata."]
pub mod ep1_rx_fifo_rdata;
#[doc = "ep2_fifo_config register accessor: an alias for `Reg<EP2_FIFO_CONFIG_SPEC>`"]
pub type EP2_FIFO_CONFIG = crate::Reg<ep2_fifo_config::EP2_FIFO_CONFIG_SPEC>;
#[doc = "ep2_fifo_config."]
pub mod ep2_fifo_config;
#[doc = "ep2_fifo_status register accessor: an alias for `Reg<EP2_FIFO_STATUS_SPEC>`"]
pub type EP2_FIFO_STATUS = crate::Reg<ep2_fifo_status::EP2_FIFO_STATUS_SPEC>;
#[doc = "ep2_fifo_status."]
pub mod ep2_fifo_status;
#[doc = "ep2_tx_fifo_wdata register accessor: an alias for `Reg<EP2_TX_FIFO_WDATA_SPEC>`"]
pub type EP2_TX_FIFO_WDATA = crate::Reg<ep2_tx_fifo_wdata::EP2_TX_FIFO_WDATA_SPEC>;
#[doc = "ep2_tx_fifo_wdata."]
pub mod ep2_tx_fifo_wdata;
#[doc = "ep2_rx_fifo_rdata register accessor: an alias for `Reg<EP2_RX_FIFO_RDATA_SPEC>`"]
pub type EP2_RX_FIFO_RDATA = crate::Reg<ep2_rx_fifo_rdata::EP2_RX_FIFO_RDATA_SPEC>;
#[doc = "ep2_rx_fifo_rdata."]
pub mod ep2_rx_fifo_rdata;
#[doc = "ep3_fifo_config register accessor: an alias for `Reg<EP3_FIFO_CONFIG_SPEC>`"]
pub type EP3_FIFO_CONFIG = crate::Reg<ep3_fifo_config::EP3_FIFO_CONFIG_SPEC>;
#[doc = "ep3_fifo_config."]
pub mod ep3_fifo_config;
#[doc = "ep3_fifo_status register accessor: an alias for `Reg<EP3_FIFO_STATUS_SPEC>`"]
pub type EP3_FIFO_STATUS = crate::Reg<ep3_fifo_status::EP3_FIFO_STATUS_SPEC>;
#[doc = "ep3_fifo_status."]
pub mod ep3_fifo_status;
#[doc = "ep3_tx_fifo_wdata register accessor: an alias for `Reg<EP3_TX_FIFO_WDATA_SPEC>`"]
pub type EP3_TX_FIFO_WDATA = crate::Reg<ep3_tx_fifo_wdata::EP3_TX_FIFO_WDATA_SPEC>;
#[doc = "ep3_tx_fifo_wdata."]
pub mod ep3_tx_fifo_wdata;
#[doc = "ep3_rx_fifo_rdata register accessor: an alias for `Reg<EP3_RX_FIFO_RDATA_SPEC>`"]
pub type EP3_RX_FIFO_RDATA = crate::Reg<ep3_rx_fifo_rdata::EP3_RX_FIFO_RDATA_SPEC>;
#[doc = "ep3_rx_fifo_rdata."]
pub mod ep3_rx_fifo_rdata;
#[doc = "ep4_fifo_config register accessor: an alias for `Reg<EP4_FIFO_CONFIG_SPEC>`"]
pub type EP4_FIFO_CONFIG = crate::Reg<ep4_fifo_config::EP4_FIFO_CONFIG_SPEC>;
#[doc = "ep4_fifo_config."]
pub mod ep4_fifo_config;
#[doc = "ep4_fifo_status register accessor: an alias for `Reg<EP4_FIFO_STATUS_SPEC>`"]
pub type EP4_FIFO_STATUS = crate::Reg<ep4_fifo_status::EP4_FIFO_STATUS_SPEC>;
#[doc = "ep4_fifo_status."]
pub mod ep4_fifo_status;
#[doc = "ep4_tx_fifo_wdata register accessor: an alias for `Reg<EP4_TX_FIFO_WDATA_SPEC>`"]
pub type EP4_TX_FIFO_WDATA = crate::Reg<ep4_tx_fifo_wdata::EP4_TX_FIFO_WDATA_SPEC>;
#[doc = "ep4_tx_fifo_wdata."]
pub mod ep4_tx_fifo_wdata;
#[doc = "ep4_rx_fifo_rdata register accessor: an alias for `Reg<EP4_RX_FIFO_RDATA_SPEC>`"]
pub type EP4_RX_FIFO_RDATA = crate::Reg<ep4_rx_fifo_rdata::EP4_RX_FIFO_RDATA_SPEC>;
#[doc = "ep4_rx_fifo_rdata."]
pub mod ep4_rx_fifo_rdata;
#[doc = "ep5_fifo_config register accessor: an alias for `Reg<EP5_FIFO_CONFIG_SPEC>`"]
pub type EP5_FIFO_CONFIG = crate::Reg<ep5_fifo_config::EP5_FIFO_CONFIG_SPEC>;
#[doc = "ep5_fifo_config."]
pub mod ep5_fifo_config;
#[doc = "ep5_fifo_status register accessor: an alias for `Reg<EP5_FIFO_STATUS_SPEC>`"]
pub type EP5_FIFO_STATUS = crate::Reg<ep5_fifo_status::EP5_FIFO_STATUS_SPEC>;
#[doc = "ep5_fifo_status."]
pub mod ep5_fifo_status;
#[doc = "ep5_tx_fifo_wdata register accessor: an alias for `Reg<EP5_TX_FIFO_WDATA_SPEC>`"]
pub type EP5_TX_FIFO_WDATA = crate::Reg<ep5_tx_fifo_wdata::EP5_TX_FIFO_WDATA_SPEC>;
#[doc = "ep5_tx_fifo_wdata."]
pub mod ep5_tx_fifo_wdata;
#[doc = "ep5_rx_fifo_rdata register accessor: an alias for `Reg<EP5_RX_FIFO_RDATA_SPEC>`"]
pub type EP5_RX_FIFO_RDATA = crate::Reg<ep5_rx_fifo_rdata::EP5_RX_FIFO_RDATA_SPEC>;
#[doc = "ep5_rx_fifo_rdata."]
pub mod ep5_rx_fifo_rdata;
#[doc = "ep6_fifo_config register accessor: an alias for `Reg<EP6_FIFO_CONFIG_SPEC>`"]
pub type EP6_FIFO_CONFIG = crate::Reg<ep6_fifo_config::EP6_FIFO_CONFIG_SPEC>;
#[doc = "ep6_fifo_config."]
pub mod ep6_fifo_config;
#[doc = "ep6_fifo_status register accessor: an alias for `Reg<EP6_FIFO_STATUS_SPEC>`"]
pub type EP6_FIFO_STATUS = crate::Reg<ep6_fifo_status::EP6_FIFO_STATUS_SPEC>;
#[doc = "ep6_fifo_status."]
pub mod ep6_fifo_status;
#[doc = "ep6_tx_fifo_wdata register accessor: an alias for `Reg<EP6_TX_FIFO_WDATA_SPEC>`"]
pub type EP6_TX_FIFO_WDATA = crate::Reg<ep6_tx_fifo_wdata::EP6_TX_FIFO_WDATA_SPEC>;
#[doc = "ep6_tx_fifo_wdata."]
pub mod ep6_tx_fifo_wdata;
#[doc = "ep6_rx_fifo_rdata register accessor: an alias for `Reg<EP6_RX_FIFO_RDATA_SPEC>`"]
pub type EP6_RX_FIFO_RDATA = crate::Reg<ep6_rx_fifo_rdata::EP6_RX_FIFO_RDATA_SPEC>;
#[doc = "ep6_rx_fifo_rdata."]
pub mod ep6_rx_fifo_rdata;
#[doc = "ep7_fifo_config register accessor: an alias for `Reg<EP7_FIFO_CONFIG_SPEC>`"]
pub type EP7_FIFO_CONFIG = crate::Reg<ep7_fifo_config::EP7_FIFO_CONFIG_SPEC>;
#[doc = "ep7_fifo_config."]
pub mod ep7_fifo_config;
#[doc = "ep7_fifo_status register accessor: an alias for `Reg<EP7_FIFO_STATUS_SPEC>`"]
pub type EP7_FIFO_STATUS = crate::Reg<ep7_fifo_status::EP7_FIFO_STATUS_SPEC>;
#[doc = "ep7_fifo_status."]
pub mod ep7_fifo_status;
#[doc = "ep7_tx_fifo_wdata register accessor: an alias for `Reg<EP7_TX_FIFO_WDATA_SPEC>`"]
pub type EP7_TX_FIFO_WDATA = crate::Reg<ep7_tx_fifo_wdata::EP7_TX_FIFO_WDATA_SPEC>;
#[doc = "ep7_tx_fifo_wdata."]
pub mod ep7_tx_fifo_wdata;
#[doc = "ep7_rx_fifo_rdata register accessor: an alias for `Reg<EP7_RX_FIFO_RDATA_SPEC>`"]
pub type EP7_RX_FIFO_RDATA = crate::Reg<ep7_rx_fifo_rdata::EP7_RX_FIFO_RDATA_SPEC>;
#[doc = "ep7_rx_fifo_rdata."]
pub mod ep7_rx_fifo_rdata;
#[doc = "rsvd_0 register accessor: an alias for `Reg<RSVD_0_SPEC>`"]
pub type RSVD_0 = crate::Reg<rsvd_0::RSVD_0_SPEC>;
#[doc = "rsvd_0."]
pub mod rsvd_0;
#[doc = "rsvd_1 register accessor: an alias for `Reg<RSVD_1_SPEC>`"]
pub type RSVD_1 = crate::Reg<rsvd_1::RSVD_1_SPEC>;
#[doc = "rsvd_1."]
pub mod rsvd_1;
#[doc = "xcvr_if_config register accessor: an alias for `Reg<XCVR_IF_CONFIG_SPEC>`"]
pub type XCVR_IF_CONFIG = crate::Reg<xcvr_if_config::XCVR_IF_CONFIG_SPEC>;
#[doc = "xcvr_if_config."]
pub mod xcvr_if_config;
