#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - dvp2axi_configue."]
    pub dvp2axi_configue: crate::Reg<dvp2axi_configue::DVP2AXI_CONFIGUE_SPEC>,
    #[doc = "0x04 - dvp2ahb_addr_start_0."]
    pub dvp2ahb_addr_start_0: crate::Reg<dvp2ahb_addr_start_0::DVP2AHB_ADDR_START_0_SPEC>,
    #[doc = "0x08 - dvp2ahb_mem_bcnt_0."]
    pub dvp2ahb_mem_bcnt_0: crate::Reg<dvp2ahb_mem_bcnt_0::DVP2AHB_MEM_BCNT_0_SPEC>,
    #[doc = "0x0c - dvp2ahb_frame_bcnt_0."]
    pub dvp2ahb_frame_bcnt_0: crate::Reg<dvp2ahb_frame_bcnt_0::DVP2AHB_FRAME_BCNT_0_SPEC>,
    #[doc = "0x10 - dvp2ahb_addr_start_1."]
    pub dvp2ahb_addr_start_1: crate::Reg<dvp2ahb_addr_start_1::DVP2AHB_ADDR_START_1_SPEC>,
    #[doc = "0x14 - dvp2ahb_mem_bcnt_1."]
    pub dvp2ahb_mem_bcnt_1: crate::Reg<dvp2ahb_mem_bcnt_1::DVP2AHB_MEM_BCNT_1_SPEC>,
    #[doc = "0x18 - dvp2ahb_frame_bcnt_1."]
    pub dvp2ahb_frame_bcnt_1: crate::Reg<dvp2ahb_frame_bcnt_1::DVP2AHB_FRAME_BCNT_1_SPEC>,
    #[doc = "0x1c - dvp_status_and_error."]
    pub dvp_status_and_error: crate::Reg<dvp_status_and_error::DVP_STATUS_AND_ERROR_SPEC>,
    #[doc = "0x20 - dvp_frame_fifo_pop."]
    pub dvp_frame_fifo_pop: crate::Reg<dvp_frame_fifo_pop::DVP_FRAME_FIFO_POP_SPEC>,
    #[doc = "0x24 - snsr_control."]
    pub snsr_control: crate::Reg<snsr_control::SNSR_CONTROL_SPEC>,
    #[doc = "0x28 - int_control."]
    pub int_control: crate::Reg<int_control::INT_CONTROL_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - hsync_control."]
    pub hsync_control: crate::Reg<hsync_control::HSYNC_CONTROL_SPEC>,
    #[doc = "0x34 - vsync_control."]
    pub vsync_control: crate::Reg<vsync_control::VSYNC_CONTROL_SPEC>,
    #[doc = "0x38 - frame_size_control."]
    pub frame_size_control: crate::Reg<frame_size_control::FRAME_SIZE_CONTROL_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x40 - frame_start_addr0_0."]
    pub frame_start_addr0_0: crate::Reg<frame_start_addr0_0::FRAME_START_ADDR0_0_SPEC>,
    #[doc = "0x44 - frame_byte_cnt0_0."]
    pub frame_byte_cnt0_0: crate::Reg<frame_byte_cnt0_0::FRAME_BYTE_CNT0_0_SPEC>,
    #[doc = "0x48 - frame_start_addr0_1."]
    pub frame_start_addr0_1: crate::Reg<frame_start_addr0_1::FRAME_START_ADDR0_1_SPEC>,
    #[doc = "0x4c - frame_byte_cnt0_1."]
    pub frame_byte_cnt0_1: crate::Reg<frame_byte_cnt0_1::FRAME_BYTE_CNT0_1_SPEC>,
    #[doc = "0x50 - frame_start_addr0_2."]
    pub frame_start_addr0_2: crate::Reg<frame_start_addr0_2::FRAME_START_ADDR0_2_SPEC>,
    #[doc = "0x54 - frame_byte_cnt0_2."]
    pub frame_byte_cnt0_2: crate::Reg<frame_byte_cnt0_2::FRAME_BYTE_CNT0_2_SPEC>,
    #[doc = "0x58 - frame_start_addr0_3."]
    pub frame_start_addr0_3: crate::Reg<frame_start_addr0_3::FRAME_START_ADDR0_3_SPEC>,
    #[doc = "0x5c - frame_byte_cnt0_3."]
    pub frame_byte_cnt0_3: crate::Reg<frame_byte_cnt0_3::FRAME_BYTE_CNT0_3_SPEC>,
    #[doc = "0x60 - frame_start_addr0_4."]
    pub frame_start_addr0_4: crate::Reg<frame_start_addr0_4::FRAME_START_ADDR0_4_SPEC>,
    #[doc = "0x64 - frame_byte_cnt0_4."]
    pub frame_byte_cnt0_4: crate::Reg<frame_byte_cnt0_4::FRAME_BYTE_CNT0_4_SPEC>,
    #[doc = "0x68 - frame_start_addr0_5."]
    pub frame_start_addr0_5: crate::Reg<frame_start_addr0_5::FRAME_START_ADDR0_5_SPEC>,
    #[doc = "0x6c - frame_byte_cnt0_5."]
    pub frame_byte_cnt0_5: crate::Reg<frame_byte_cnt0_5::FRAME_BYTE_CNT0_5_SPEC>,
    #[doc = "0x70 - frame_start_addr0_6."]
    pub frame_start_addr0_6: crate::Reg<frame_start_addr0_6::FRAME_START_ADDR0_6_SPEC>,
    #[doc = "0x74 - frame_byte_cnt0_6."]
    pub frame_byte_cnt0_6: crate::Reg<frame_byte_cnt0_6::FRAME_BYTE_CNT0_6_SPEC>,
    #[doc = "0x78 - frame_start_addr0_7."]
    pub frame_start_addr0_7: crate::Reg<frame_start_addr0_7::FRAME_START_ADDR0_7_SPEC>,
    #[doc = "0x7c - frame_byte_cnt0_7."]
    pub frame_byte_cnt0_7: crate::Reg<frame_byte_cnt0_7::FRAME_BYTE_CNT0_7_SPEC>,
    #[doc = "0x80 - frame_start_addr1_0."]
    pub frame_start_addr1_0: crate::Reg<frame_start_addr1_0::FRAME_START_ADDR1_0_SPEC>,
    #[doc = "0x84 - frame_byte_cnt1_0."]
    pub frame_byte_cnt1_0: crate::Reg<frame_byte_cnt1_0::FRAME_BYTE_CNT1_0_SPEC>,
    #[doc = "0x88 - frame_start_addr1_1."]
    pub frame_start_addr1_1: crate::Reg<frame_start_addr1_1::FRAME_START_ADDR1_1_SPEC>,
    #[doc = "0x8c - frame_byte_cnt1_1."]
    pub frame_byte_cnt1_1: crate::Reg<frame_byte_cnt1_1::FRAME_BYTE_CNT1_1_SPEC>,
    #[doc = "0x90 - frame_start_addr1_2."]
    pub frame_start_addr1_2: crate::Reg<frame_start_addr1_2::FRAME_START_ADDR1_2_SPEC>,
    #[doc = "0x94 - frame_byte_cnt1_2."]
    pub frame_byte_cnt1_2: crate::Reg<frame_byte_cnt1_2::FRAME_BYTE_CNT1_2_SPEC>,
    #[doc = "0x98 - frame_start_addr1_3."]
    pub frame_start_addr1_3: crate::Reg<frame_start_addr1_3::FRAME_START_ADDR1_3_SPEC>,
    #[doc = "0x9c - frame_byte_cnt1_3."]
    pub frame_byte_cnt1_3: crate::Reg<frame_byte_cnt1_3::FRAME_BYTE_CNT1_3_SPEC>,
    #[doc = "0xa0 - frame_start_addr1_4."]
    pub frame_start_addr1_4: crate::Reg<frame_start_addr1_4::FRAME_START_ADDR1_4_SPEC>,
    #[doc = "0xa4 - frame_byte_cnt1_4."]
    pub frame_byte_cnt1_4: crate::Reg<frame_byte_cnt1_4::FRAME_BYTE_CNT1_4_SPEC>,
    #[doc = "0xa8 - frame_start_addr1_5."]
    pub frame_start_addr1_5: crate::Reg<frame_start_addr1_5::FRAME_START_ADDR1_5_SPEC>,
    #[doc = "0xac - frame_byte_cnt1_5."]
    pub frame_byte_cnt1_5: crate::Reg<frame_byte_cnt1_5::FRAME_BYTE_CNT1_5_SPEC>,
    #[doc = "0xb0 - frame_start_addr1_6."]
    pub frame_start_addr1_6: crate::Reg<frame_start_addr1_6::FRAME_START_ADDR1_6_SPEC>,
    #[doc = "0xb4 - frame_byte_cnt1_6."]
    pub frame_byte_cnt1_6: crate::Reg<frame_byte_cnt1_6::FRAME_BYTE_CNT1_6_SPEC>,
    #[doc = "0xb8 - frame_start_addr1_7."]
    pub frame_start_addr1_7: crate::Reg<frame_start_addr1_7::FRAME_START_ADDR1_7_SPEC>,
    #[doc = "0xbc - frame_byte_cnt1_7."]
    pub frame_byte_cnt1_7: crate::Reg<frame_byte_cnt1_7::FRAME_BYTE_CNT1_7_SPEC>,
    _reserved46: [u8; 0x0f30],
    #[doc = "0xff0 - dvp_debug."]
    pub dvp_debug: crate::Reg<dvp_debug::DVP_DEBUG_SPEC>,
    _reserved47: [u8; 0x08],
    #[doc = "0xffc - dvp_dummy_reg."]
    pub dvp_dummy_reg: crate::Reg<dvp_dummy_reg::DVP_DUMMY_REG_SPEC>,
}
#[doc = "dvp2axi_configue register accessor: an alias for `Reg<DVP2AXI_CONFIGUE_SPEC>`"]
pub type DVP2AXI_CONFIGUE = crate::Reg<dvp2axi_configue::DVP2AXI_CONFIGUE_SPEC>;
#[doc = "dvp2axi_configue."]
pub mod dvp2axi_configue;
#[doc = "dvp2ahb_addr_start_0 register accessor: an alias for `Reg<DVP2AHB_ADDR_START_0_SPEC>`"]
pub type DVP2AHB_ADDR_START_0 = crate::Reg<dvp2ahb_addr_start_0::DVP2AHB_ADDR_START_0_SPEC>;
#[doc = "dvp2ahb_addr_start_0."]
pub mod dvp2ahb_addr_start_0;
#[doc = "dvp2ahb_mem_bcnt_0 register accessor: an alias for `Reg<DVP2AHB_MEM_BCNT_0_SPEC>`"]
pub type DVP2AHB_MEM_BCNT_0 = crate::Reg<dvp2ahb_mem_bcnt_0::DVP2AHB_MEM_BCNT_0_SPEC>;
#[doc = "dvp2ahb_mem_bcnt_0."]
pub mod dvp2ahb_mem_bcnt_0;
#[doc = "dvp2ahb_frame_bcnt_0 register accessor: an alias for `Reg<DVP2AHB_FRAME_BCNT_0_SPEC>`"]
pub type DVP2AHB_FRAME_BCNT_0 = crate::Reg<dvp2ahb_frame_bcnt_0::DVP2AHB_FRAME_BCNT_0_SPEC>;
#[doc = "dvp2ahb_frame_bcnt_0."]
pub mod dvp2ahb_frame_bcnt_0;
#[doc = "dvp2ahb_addr_start_1 register accessor: an alias for `Reg<DVP2AHB_ADDR_START_1_SPEC>`"]
pub type DVP2AHB_ADDR_START_1 = crate::Reg<dvp2ahb_addr_start_1::DVP2AHB_ADDR_START_1_SPEC>;
#[doc = "dvp2ahb_addr_start_1."]
pub mod dvp2ahb_addr_start_1;
#[doc = "dvp2ahb_mem_bcnt_1 register accessor: an alias for `Reg<DVP2AHB_MEM_BCNT_1_SPEC>`"]
pub type DVP2AHB_MEM_BCNT_1 = crate::Reg<dvp2ahb_mem_bcnt_1::DVP2AHB_MEM_BCNT_1_SPEC>;
#[doc = "dvp2ahb_mem_bcnt_1."]
pub mod dvp2ahb_mem_bcnt_1;
#[doc = "dvp2ahb_frame_bcnt_1 register accessor: an alias for `Reg<DVP2AHB_FRAME_BCNT_1_SPEC>`"]
pub type DVP2AHB_FRAME_BCNT_1 = crate::Reg<dvp2ahb_frame_bcnt_1::DVP2AHB_FRAME_BCNT_1_SPEC>;
#[doc = "dvp2ahb_frame_bcnt_1."]
pub mod dvp2ahb_frame_bcnt_1;
#[doc = "dvp_status_and_error register accessor: an alias for `Reg<DVP_STATUS_AND_ERROR_SPEC>`"]
pub type DVP_STATUS_AND_ERROR = crate::Reg<dvp_status_and_error::DVP_STATUS_AND_ERROR_SPEC>;
#[doc = "dvp_status_and_error."]
pub mod dvp_status_and_error;
#[doc = "dvp_frame_fifo_pop register accessor: an alias for `Reg<DVP_FRAME_FIFO_POP_SPEC>`"]
pub type DVP_FRAME_FIFO_POP = crate::Reg<dvp_frame_fifo_pop::DVP_FRAME_FIFO_POP_SPEC>;
#[doc = "dvp_frame_fifo_pop."]
pub mod dvp_frame_fifo_pop;
#[doc = "snsr_control register accessor: an alias for `Reg<SNSR_CONTROL_SPEC>`"]
pub type SNSR_CONTROL = crate::Reg<snsr_control::SNSR_CONTROL_SPEC>;
#[doc = "snsr_control."]
pub mod snsr_control;
#[doc = "int_control register accessor: an alias for `Reg<INT_CONTROL_SPEC>`"]
pub type INT_CONTROL = crate::Reg<int_control::INT_CONTROL_SPEC>;
#[doc = "int_control."]
pub mod int_control;
#[doc = "hsync_control register accessor: an alias for `Reg<HSYNC_CONTROL_SPEC>`"]
pub type HSYNC_CONTROL = crate::Reg<hsync_control::HSYNC_CONTROL_SPEC>;
#[doc = "hsync_control."]
pub mod hsync_control;
#[doc = "vsync_control register accessor: an alias for `Reg<VSYNC_CONTROL_SPEC>`"]
pub type VSYNC_CONTROL = crate::Reg<vsync_control::VSYNC_CONTROL_SPEC>;
#[doc = "vsync_control."]
pub mod vsync_control;
#[doc = "frame_size_control register accessor: an alias for `Reg<FRAME_SIZE_CONTROL_SPEC>`"]
pub type FRAME_SIZE_CONTROL = crate::Reg<frame_size_control::FRAME_SIZE_CONTROL_SPEC>;
#[doc = "frame_size_control."]
pub mod frame_size_control;
#[doc = "frame_start_addr0_0 register accessor: an alias for `Reg<FRAME_START_ADDR0_0_SPEC>`"]
pub type FRAME_START_ADDR0_0 = crate::Reg<frame_start_addr0_0::FRAME_START_ADDR0_0_SPEC>;
#[doc = "frame_start_addr0_0."]
pub mod frame_start_addr0_0;
#[doc = "frame_byte_cnt0_0 register accessor: an alias for `Reg<FRAME_BYTE_CNT0_0_SPEC>`"]
pub type FRAME_BYTE_CNT0_0 = crate::Reg<frame_byte_cnt0_0::FRAME_BYTE_CNT0_0_SPEC>;
#[doc = "frame_byte_cnt0_0."]
pub mod frame_byte_cnt0_0;
#[doc = "frame_start_addr0_1 register accessor: an alias for `Reg<FRAME_START_ADDR0_1_SPEC>`"]
pub type FRAME_START_ADDR0_1 = crate::Reg<frame_start_addr0_1::FRAME_START_ADDR0_1_SPEC>;
#[doc = "frame_start_addr0_1."]
pub mod frame_start_addr0_1;
#[doc = "frame_byte_cnt0_1 register accessor: an alias for `Reg<FRAME_BYTE_CNT0_1_SPEC>`"]
pub type FRAME_BYTE_CNT0_1 = crate::Reg<frame_byte_cnt0_1::FRAME_BYTE_CNT0_1_SPEC>;
#[doc = "frame_byte_cnt0_1."]
pub mod frame_byte_cnt0_1;
#[doc = "frame_start_addr0_2 register accessor: an alias for `Reg<FRAME_START_ADDR0_2_SPEC>`"]
pub type FRAME_START_ADDR0_2 = crate::Reg<frame_start_addr0_2::FRAME_START_ADDR0_2_SPEC>;
#[doc = "frame_start_addr0_2."]
pub mod frame_start_addr0_2;
#[doc = "frame_byte_cnt0_2 register accessor: an alias for `Reg<FRAME_BYTE_CNT0_2_SPEC>`"]
pub type FRAME_BYTE_CNT0_2 = crate::Reg<frame_byte_cnt0_2::FRAME_BYTE_CNT0_2_SPEC>;
#[doc = "frame_byte_cnt0_2."]
pub mod frame_byte_cnt0_2;
#[doc = "frame_start_addr0_3 register accessor: an alias for `Reg<FRAME_START_ADDR0_3_SPEC>`"]
pub type FRAME_START_ADDR0_3 = crate::Reg<frame_start_addr0_3::FRAME_START_ADDR0_3_SPEC>;
#[doc = "frame_start_addr0_3."]
pub mod frame_start_addr0_3;
#[doc = "frame_byte_cnt0_3 register accessor: an alias for `Reg<FRAME_BYTE_CNT0_3_SPEC>`"]
pub type FRAME_BYTE_CNT0_3 = crate::Reg<frame_byte_cnt0_3::FRAME_BYTE_CNT0_3_SPEC>;
#[doc = "frame_byte_cnt0_3."]
pub mod frame_byte_cnt0_3;
#[doc = "frame_start_addr0_4 register accessor: an alias for `Reg<FRAME_START_ADDR0_4_SPEC>`"]
pub type FRAME_START_ADDR0_4 = crate::Reg<frame_start_addr0_4::FRAME_START_ADDR0_4_SPEC>;
#[doc = "frame_start_addr0_4."]
pub mod frame_start_addr0_4;
#[doc = "frame_byte_cnt0_4 register accessor: an alias for `Reg<FRAME_BYTE_CNT0_4_SPEC>`"]
pub type FRAME_BYTE_CNT0_4 = crate::Reg<frame_byte_cnt0_4::FRAME_BYTE_CNT0_4_SPEC>;
#[doc = "frame_byte_cnt0_4."]
pub mod frame_byte_cnt0_4;
#[doc = "frame_start_addr0_5 register accessor: an alias for `Reg<FRAME_START_ADDR0_5_SPEC>`"]
pub type FRAME_START_ADDR0_5 = crate::Reg<frame_start_addr0_5::FRAME_START_ADDR0_5_SPEC>;
#[doc = "frame_start_addr0_5."]
pub mod frame_start_addr0_5;
#[doc = "frame_byte_cnt0_5 register accessor: an alias for `Reg<FRAME_BYTE_CNT0_5_SPEC>`"]
pub type FRAME_BYTE_CNT0_5 = crate::Reg<frame_byte_cnt0_5::FRAME_BYTE_CNT0_5_SPEC>;
#[doc = "frame_byte_cnt0_5."]
pub mod frame_byte_cnt0_5;
#[doc = "frame_start_addr0_6 register accessor: an alias for `Reg<FRAME_START_ADDR0_6_SPEC>`"]
pub type FRAME_START_ADDR0_6 = crate::Reg<frame_start_addr0_6::FRAME_START_ADDR0_6_SPEC>;
#[doc = "frame_start_addr0_6."]
pub mod frame_start_addr0_6;
#[doc = "frame_byte_cnt0_6 register accessor: an alias for `Reg<FRAME_BYTE_CNT0_6_SPEC>`"]
pub type FRAME_BYTE_CNT0_6 = crate::Reg<frame_byte_cnt0_6::FRAME_BYTE_CNT0_6_SPEC>;
#[doc = "frame_byte_cnt0_6."]
pub mod frame_byte_cnt0_6;
#[doc = "frame_start_addr0_7 register accessor: an alias for `Reg<FRAME_START_ADDR0_7_SPEC>`"]
pub type FRAME_START_ADDR0_7 = crate::Reg<frame_start_addr0_7::FRAME_START_ADDR0_7_SPEC>;
#[doc = "frame_start_addr0_7."]
pub mod frame_start_addr0_7;
#[doc = "frame_byte_cnt0_7 register accessor: an alias for `Reg<FRAME_BYTE_CNT0_7_SPEC>`"]
pub type FRAME_BYTE_CNT0_7 = crate::Reg<frame_byte_cnt0_7::FRAME_BYTE_CNT0_7_SPEC>;
#[doc = "frame_byte_cnt0_7."]
pub mod frame_byte_cnt0_7;
#[doc = "frame_start_addr1_0 register accessor: an alias for `Reg<FRAME_START_ADDR1_0_SPEC>`"]
pub type FRAME_START_ADDR1_0 = crate::Reg<frame_start_addr1_0::FRAME_START_ADDR1_0_SPEC>;
#[doc = "frame_start_addr1_0."]
pub mod frame_start_addr1_0;
#[doc = "frame_byte_cnt1_0 register accessor: an alias for `Reg<FRAME_BYTE_CNT1_0_SPEC>`"]
pub type FRAME_BYTE_CNT1_0 = crate::Reg<frame_byte_cnt1_0::FRAME_BYTE_CNT1_0_SPEC>;
#[doc = "frame_byte_cnt1_0."]
pub mod frame_byte_cnt1_0;
#[doc = "frame_start_addr1_1 register accessor: an alias for `Reg<FRAME_START_ADDR1_1_SPEC>`"]
pub type FRAME_START_ADDR1_1 = crate::Reg<frame_start_addr1_1::FRAME_START_ADDR1_1_SPEC>;
#[doc = "frame_start_addr1_1."]
pub mod frame_start_addr1_1;
#[doc = "frame_byte_cnt1_1 register accessor: an alias for `Reg<FRAME_BYTE_CNT1_1_SPEC>`"]
pub type FRAME_BYTE_CNT1_1 = crate::Reg<frame_byte_cnt1_1::FRAME_BYTE_CNT1_1_SPEC>;
#[doc = "frame_byte_cnt1_1."]
pub mod frame_byte_cnt1_1;
#[doc = "frame_start_addr1_2 register accessor: an alias for `Reg<FRAME_START_ADDR1_2_SPEC>`"]
pub type FRAME_START_ADDR1_2 = crate::Reg<frame_start_addr1_2::FRAME_START_ADDR1_2_SPEC>;
#[doc = "frame_start_addr1_2."]
pub mod frame_start_addr1_2;
#[doc = "frame_byte_cnt1_2 register accessor: an alias for `Reg<FRAME_BYTE_CNT1_2_SPEC>`"]
pub type FRAME_BYTE_CNT1_2 = crate::Reg<frame_byte_cnt1_2::FRAME_BYTE_CNT1_2_SPEC>;
#[doc = "frame_byte_cnt1_2."]
pub mod frame_byte_cnt1_2;
#[doc = "frame_start_addr1_3 register accessor: an alias for `Reg<FRAME_START_ADDR1_3_SPEC>`"]
pub type FRAME_START_ADDR1_3 = crate::Reg<frame_start_addr1_3::FRAME_START_ADDR1_3_SPEC>;
#[doc = "frame_start_addr1_3."]
pub mod frame_start_addr1_3;
#[doc = "frame_byte_cnt1_3 register accessor: an alias for `Reg<FRAME_BYTE_CNT1_3_SPEC>`"]
pub type FRAME_BYTE_CNT1_3 = crate::Reg<frame_byte_cnt1_3::FRAME_BYTE_CNT1_3_SPEC>;
#[doc = "frame_byte_cnt1_3."]
pub mod frame_byte_cnt1_3;
#[doc = "frame_start_addr1_4 register accessor: an alias for `Reg<FRAME_START_ADDR1_4_SPEC>`"]
pub type FRAME_START_ADDR1_4 = crate::Reg<frame_start_addr1_4::FRAME_START_ADDR1_4_SPEC>;
#[doc = "frame_start_addr1_4."]
pub mod frame_start_addr1_4;
#[doc = "frame_byte_cnt1_4 register accessor: an alias for `Reg<FRAME_BYTE_CNT1_4_SPEC>`"]
pub type FRAME_BYTE_CNT1_4 = crate::Reg<frame_byte_cnt1_4::FRAME_BYTE_CNT1_4_SPEC>;
#[doc = "frame_byte_cnt1_4."]
pub mod frame_byte_cnt1_4;
#[doc = "frame_start_addr1_5 register accessor: an alias for `Reg<FRAME_START_ADDR1_5_SPEC>`"]
pub type FRAME_START_ADDR1_5 = crate::Reg<frame_start_addr1_5::FRAME_START_ADDR1_5_SPEC>;
#[doc = "frame_start_addr1_5."]
pub mod frame_start_addr1_5;
#[doc = "frame_byte_cnt1_5 register accessor: an alias for `Reg<FRAME_BYTE_CNT1_5_SPEC>`"]
pub type FRAME_BYTE_CNT1_5 = crate::Reg<frame_byte_cnt1_5::FRAME_BYTE_CNT1_5_SPEC>;
#[doc = "frame_byte_cnt1_5."]
pub mod frame_byte_cnt1_5;
#[doc = "frame_start_addr1_6 register accessor: an alias for `Reg<FRAME_START_ADDR1_6_SPEC>`"]
pub type FRAME_START_ADDR1_6 = crate::Reg<frame_start_addr1_6::FRAME_START_ADDR1_6_SPEC>;
#[doc = "frame_start_addr1_6."]
pub mod frame_start_addr1_6;
#[doc = "frame_byte_cnt1_6 register accessor: an alias for `Reg<FRAME_BYTE_CNT1_6_SPEC>`"]
pub type FRAME_BYTE_CNT1_6 = crate::Reg<frame_byte_cnt1_6::FRAME_BYTE_CNT1_6_SPEC>;
#[doc = "frame_byte_cnt1_6."]
pub mod frame_byte_cnt1_6;
#[doc = "frame_start_addr1_7 register accessor: an alias for `Reg<FRAME_START_ADDR1_7_SPEC>`"]
pub type FRAME_START_ADDR1_7 = crate::Reg<frame_start_addr1_7::FRAME_START_ADDR1_7_SPEC>;
#[doc = "frame_start_addr1_7."]
pub mod frame_start_addr1_7;
#[doc = "frame_byte_cnt1_7 register accessor: an alias for `Reg<FRAME_BYTE_CNT1_7_SPEC>`"]
pub type FRAME_BYTE_CNT1_7 = crate::Reg<frame_byte_cnt1_7::FRAME_BYTE_CNT1_7_SPEC>;
#[doc = "frame_byte_cnt1_7."]
pub mod frame_byte_cnt1_7;
#[doc = "dvp_debug register accessor: an alias for `Reg<DVP_DEBUG_SPEC>`"]
pub type DVP_DEBUG = crate::Reg<dvp_debug::DVP_DEBUG_SPEC>;
#[doc = "dvp_debug."]
pub mod dvp_debug;
#[doc = "dvp_dummy_reg register accessor: an alias for `Reg<DVP_DUMMY_REG_SPEC>`"]
pub type DVP_DUMMY_REG = crate::Reg<dvp_dummy_reg::DVP_DUMMY_REG_SPEC>;
#[doc = "dvp_dummy_reg."]
pub mod dvp_dummy_reg;
