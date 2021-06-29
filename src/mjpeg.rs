#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - mjpeg_control_1."]
    pub mjpeg_control_1: crate::Reg<mjpeg_control_1::MJPEG_CONTROL_1_SPEC>,
    #[doc = "0x04 - mjpeg_control_2."]
    pub mjpeg_control_2: crate::Reg<mjpeg_control_2::MJPEG_CONTROL_2_SPEC>,
    #[doc = "0x08 - mjpeg_yy_frame_addr."]
    pub mjpeg_yy_frame_addr: crate::Reg<mjpeg_yy_frame_addr::MJPEG_YY_FRAME_ADDR_SPEC>,
    #[doc = "0x0c - mjpeg_uv_frame_addr."]
    pub mjpeg_uv_frame_addr: crate::Reg<mjpeg_uv_frame_addr::MJPEG_UV_FRAME_ADDR_SPEC>,
    #[doc = "0x10 - mjpeg_yuv_mem."]
    pub mjpeg_yuv_mem: crate::Reg<mjpeg_yuv_mem::MJPEG_YUV_MEM_SPEC>,
    #[doc = "0x14 - jpeg_frame_addr."]
    pub jpeg_frame_addr: crate::Reg<jpeg_frame_addr::JPEG_FRAME_ADDR_SPEC>,
    #[doc = "0x18 - jpeg_store_memory."]
    pub jpeg_store_memory: crate::Reg<jpeg_store_memory::JPEG_STORE_MEMORY_SPEC>,
    #[doc = "0x1c - mjpeg_control_3."]
    pub mjpeg_control_3: crate::Reg<mjpeg_control_3::MJPEG_CONTROL_3_SPEC>,
    #[doc = "0x20 - mjpeg_frame_fifo_pop."]
    pub mjpeg_frame_fifo_pop: crate::Reg<mjpeg_frame_fifo_pop::MJPEG_FRAME_FIFO_POP_SPEC>,
    #[doc = "0x24 - mjpeg_frame_size."]
    pub mjpeg_frame_size: crate::Reg<mjpeg_frame_size::MJPEG_FRAME_SIZE_SPEC>,
    #[doc = "0x28 - mjpeg_header_byte."]
    pub mjpeg_header_byte: crate::Reg<mjpeg_header_byte::MJPEG_HEADER_BYTE_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x30 - mjpeg_swap_mode."]
    pub mjpeg_swap_mode: crate::Reg<mjpeg_swap_mode::MJPEG_SWAP_MODE_SPEC>,
    #[doc = "0x34 - mjpeg_swap_bit_cnt."]
    pub mjpeg_swap_bit_cnt: crate::Reg<mjpeg_swap_bit_cnt::MJPEG_SWAP_BIT_CNT_SPEC>,
    #[doc = "0x38 - mjpeg_paket_ctrl."]
    pub mjpeg_paket_ctrl: crate::Reg<mjpeg_paket_ctrl::MJPEG_PAKET_CTRL_SPEC>,
    #[doc = "0x3c - mjpeg_paket_head_tail."]
    pub mjpeg_paket_head_tail: crate::Reg<mjpeg_paket_head_tail::MJPEG_PAKET_HEAD_TAIL_SPEC>,
    #[doc = "0x40 - mjpeg_Y_frame_read_status_1."]
    pub mjpeg_y_frame_read_status_1:
        crate::Reg<mjpeg_y_frame_read_status_1::MJPEG_Y_FRAME_READ_STATUS_1_SPEC>,
    #[doc = "0x44 - mjpeg_Y_frame_read_status_2."]
    pub mjpeg_y_frame_read_status_2:
        crate::Reg<mjpeg_y_frame_read_status_2::MJPEG_Y_FRAME_READ_STATUS_2_SPEC>,
    #[doc = "0x48 - mjpeg_Y_frame_write_status."]
    pub mjpeg_y_frame_write_status:
        crate::Reg<mjpeg_y_frame_write_status::MJPEG_Y_FRAME_WRITE_STATUS_SPEC>,
    #[doc = "0x4c - mjpeg_UV_frame_read_status_1."]
    pub mjpeg_uv_frame_read_status_1:
        crate::Reg<mjpeg_uv_frame_read_status_1::MJPEG_UV_FRAME_READ_STATUS_1_SPEC>,
    #[doc = "0x50 - mjpeg_UV_frame_read_status_2."]
    pub mjpeg_uv_frame_read_status_2:
        crate::Reg<mjpeg_uv_frame_read_status_2::MJPEG_UV_FRAME_READ_STATUS_2_SPEC>,
    #[doc = "0x54 - mjpeg_UV_frame_write_status."]
    pub mjpeg_uv_frame_write_status:
        crate::Reg<mjpeg_uv_frame_write_status::MJPEG_UV_FRAME_WRITE_STATUS_SPEC>,
    _reserved21: [u8; 0x28],
    #[doc = "0x80 - mjpeg_start_addr0."]
    pub mjpeg_start_addr0: crate::Reg<mjpeg_start_addr0::MJPEG_START_ADDR0_SPEC>,
    #[doc = "0x84 - mjpeg_bit_cnt0."]
    pub mjpeg_bit_cnt0: crate::Reg<mjpeg_bit_cnt0::MJPEG_BIT_CNT0_SPEC>,
    #[doc = "0x88 - mjpeg_start_addr1."]
    pub mjpeg_start_addr1: crate::Reg<mjpeg_start_addr1::MJPEG_START_ADDR1_SPEC>,
    #[doc = "0x8c - mjpeg_bit_cnt1."]
    pub mjpeg_bit_cnt1: crate::Reg<mjpeg_bit_cnt1::MJPEG_BIT_CNT1_SPEC>,
    #[doc = "0x90 - mjpeg_start_addr2."]
    pub mjpeg_start_addr2: crate::Reg<mjpeg_start_addr2::MJPEG_START_ADDR2_SPEC>,
    #[doc = "0x94 - mjpeg_bit_cnt2."]
    pub mjpeg_bit_cnt2: crate::Reg<mjpeg_bit_cnt2::MJPEG_BIT_CNT2_SPEC>,
    #[doc = "0x98 - mjpeg_start_addr3."]
    pub mjpeg_start_addr3: crate::Reg<mjpeg_start_addr3::MJPEG_START_ADDR3_SPEC>,
    #[doc = "0x9c - mjpeg_bit_cnt3."]
    pub mjpeg_bit_cnt3: crate::Reg<mjpeg_bit_cnt3::MJPEG_BIT_CNT3_SPEC>,
    #[doc = "0xa0 - mjpeg_start_addr4."]
    pub mjpeg_start_addr4: crate::Reg<mjpeg_start_addr4::MJPEG_START_ADDR4_SPEC>,
    #[doc = "0xa4 - mjpeg_bit_cnt4."]
    pub mjpeg_bit_cnt4: crate::Reg<mjpeg_bit_cnt4::MJPEG_BIT_CNT4_SPEC>,
    #[doc = "0xa8 - mjpeg_start_addr5."]
    pub mjpeg_start_addr5: crate::Reg<mjpeg_start_addr5::MJPEG_START_ADDR5_SPEC>,
    #[doc = "0xac - mjpeg_bit_cnt5."]
    pub mjpeg_bit_cnt5: crate::Reg<mjpeg_bit_cnt5::MJPEG_BIT_CNT5_SPEC>,
    #[doc = "0xb0 - mjpeg_start_addr6."]
    pub mjpeg_start_addr6: crate::Reg<mjpeg_start_addr6::MJPEG_START_ADDR6_SPEC>,
    #[doc = "0xb4 - mjpeg_bit_cnt6."]
    pub mjpeg_bit_cnt6: crate::Reg<mjpeg_bit_cnt6::MJPEG_BIT_CNT6_SPEC>,
    #[doc = "0xb8 - mjpeg_start_addr7."]
    pub mjpeg_start_addr7: crate::Reg<mjpeg_start_addr7::MJPEG_START_ADDR7_SPEC>,
    #[doc = "0xbc - mjpeg_bit_cnt7."]
    pub mjpeg_bit_cnt7: crate::Reg<mjpeg_bit_cnt7::MJPEG_BIT_CNT7_SPEC>,
    #[doc = "0xc0 - mjpeg_start_addr_8."]
    pub mjpeg_start_addr_8: crate::Reg<mjpeg_start_addr_8::MJPEG_START_ADDR_8_SPEC>,
    #[doc = "0xc4 - mjpeg_bit_cnt_8."]
    pub mjpeg_bit_cnt_8: crate::Reg<mjpeg_bit_cnt_8::MJPEG_BIT_CNT_8_SPEC>,
    #[doc = "0xc8 - mjpeg_start_addr_9."]
    pub mjpeg_start_addr_9: crate::Reg<mjpeg_start_addr_9::MJPEG_START_ADDR_9_SPEC>,
    #[doc = "0xcc - mjpeg_bit_cnt_9."]
    pub mjpeg_bit_cnt_9: crate::Reg<mjpeg_bit_cnt_9::MJPEG_BIT_CNT_9_SPEC>,
    #[doc = "0xd0 - mjpeg_start_addr_a."]
    pub mjpeg_start_addr_a: crate::Reg<mjpeg_start_addr_a::MJPEG_START_ADDR_A_SPEC>,
    #[doc = "0xd4 - mjpeg_bit_cnt_a."]
    pub mjpeg_bit_cnt_a: crate::Reg<mjpeg_bit_cnt_a::MJPEG_BIT_CNT_A_SPEC>,
    #[doc = "0xd8 - mjpeg_start_addr_b."]
    pub mjpeg_start_addr_b: crate::Reg<mjpeg_start_addr_b::MJPEG_START_ADDR_B_SPEC>,
    #[doc = "0xdc - mjpeg_bit_cnt_b."]
    pub mjpeg_bit_cnt_b: crate::Reg<mjpeg_bit_cnt_b::MJPEG_BIT_CNT_B_SPEC>,
    #[doc = "0xe0 - mjpeg_start_addr_c."]
    pub mjpeg_start_addr_c: crate::Reg<mjpeg_start_addr_c::MJPEG_START_ADDR_C_SPEC>,
    #[doc = "0xe4 - mjpeg_bit_cnt_c."]
    pub mjpeg_bit_cnt_c: crate::Reg<mjpeg_bit_cnt_c::MJPEG_BIT_CNT_C_SPEC>,
    #[doc = "0xe8 - mjpeg_start_addr_d."]
    pub mjpeg_start_addr_d: crate::Reg<mjpeg_start_addr_d::MJPEG_START_ADDR_D_SPEC>,
    #[doc = "0xec - mjpeg_bit_cnt_d."]
    pub mjpeg_bit_cnt_d: crate::Reg<mjpeg_bit_cnt_d::MJPEG_BIT_CNT_D_SPEC>,
    #[doc = "0xf0 - mjpeg_start_addr_e."]
    pub mjpeg_start_addr_e: crate::Reg<mjpeg_start_addr_e::MJPEG_START_ADDR_E_SPEC>,
    #[doc = "0xf4 - mjpeg_bit_cnt_e."]
    pub mjpeg_bit_cnt_e: crate::Reg<mjpeg_bit_cnt_e::MJPEG_BIT_CNT_E_SPEC>,
    #[doc = "0xf8 - mjpeg_start_addr_f."]
    pub mjpeg_start_addr_f: crate::Reg<mjpeg_start_addr_f::MJPEG_START_ADDR_F_SPEC>,
    #[doc = "0xfc - mjpeg_bit_cnt_f."]
    pub mjpeg_bit_cnt_f: crate::Reg<mjpeg_bit_cnt_f::MJPEG_BIT_CNT_F_SPEC>,
    #[doc = "0x100 - mjpeg_q_mode0."]
    pub mjpeg_q_mode0: crate::Reg<mjpeg_q_mode0::MJPEG_Q_MODE0_SPEC>,
    #[doc = "0x104 - mjpeg_q_mode1."]
    pub mjpeg_q_mode1: crate::Reg<mjpeg_q_mode1::MJPEG_Q_MODE1_SPEC>,
    #[doc = "0x108 - mjpeg_q_mode2."]
    pub mjpeg_q_mode2: crate::Reg<mjpeg_q_mode2::MJPEG_Q_MODE2_SPEC>,
    #[doc = "0x10c - mjpeg_q_mode3."]
    pub mjpeg_q_mode3: crate::Reg<mjpeg_q_mode3::MJPEG_Q_MODE3_SPEC>,
    #[doc = "0x110 - mjpeg_q_mode4."]
    pub mjpeg_q_mode4: crate::Reg<mjpeg_q_mode4::MJPEG_Q_MODE4_SPEC>,
    #[doc = "0x114 - mjpeg_q_mode5."]
    pub mjpeg_q_mode5: crate::Reg<mjpeg_q_mode5::MJPEG_Q_MODE5_SPEC>,
    #[doc = "0x118 - mjpeg_q_mode6."]
    pub mjpeg_q_mode6: crate::Reg<mjpeg_q_mode6::MJPEG_Q_MODE6_SPEC>,
    #[doc = "0x11c - mjpeg_q_mode7."]
    pub mjpeg_q_mode7: crate::Reg<mjpeg_q_mode7::MJPEG_Q_MODE7_SPEC>,
    #[doc = "0x120 - mjpeg_q_mode_8."]
    pub mjpeg_q_mode_8: crate::Reg<mjpeg_q_mode_8::MJPEG_Q_MODE_8_SPEC>,
    #[doc = "0x124 - mjpeg_q_mode_9."]
    pub mjpeg_q_mode_9: crate::Reg<mjpeg_q_mode_9::MJPEG_Q_MODE_9_SPEC>,
    #[doc = "0x128 - mjpeg_q_mode_a."]
    pub mjpeg_q_mode_a: crate::Reg<mjpeg_q_mode_a::MJPEG_Q_MODE_A_SPEC>,
    #[doc = "0x12c - mjpeg_q_mode_b."]
    pub mjpeg_q_mode_b: crate::Reg<mjpeg_q_mode_b::MJPEG_Q_MODE_B_SPEC>,
    #[doc = "0x130 - mjpeg_q_mode_c."]
    pub mjpeg_q_mode_c: crate::Reg<mjpeg_q_mode_c::MJPEG_Q_MODE_C_SPEC>,
    #[doc = "0x134 - mjpeg_q_mode_d."]
    pub mjpeg_q_mode_d: crate::Reg<mjpeg_q_mode_d::MJPEG_Q_MODE_D_SPEC>,
    #[doc = "0x138 - mjpeg_q_mode_e."]
    pub mjpeg_q_mode_e: crate::Reg<mjpeg_q_mode_e::MJPEG_Q_MODE_E_SPEC>,
    #[doc = "0x13c - mjpeg_q_mode_f."]
    pub mjpeg_q_mode_f: crate::Reg<mjpeg_q_mode_f::MJPEG_Q_MODE_F_SPEC>,
    _reserved69: [u8; 0xb0],
    #[doc = "0x1f0 - mjpeg_debug."]
    pub mjpeg_debug: crate::Reg<mjpeg_debug::MJPEG_DEBUG_SPEC>,
    _reserved70: [u8; 0x08],
    #[doc = "0x1fc - mjpeg_dummy_reg."]
    pub mjpeg_dummy_reg: crate::Reg<mjpeg_dummy_reg::MJPEG_DUMMY_REG_SPEC>,
}
#[doc = "mjpeg_control_1 register accessor: an alias for `Reg<MJPEG_CONTROL_1_SPEC>`"]
pub type MJPEG_CONTROL_1 = crate::Reg<mjpeg_control_1::MJPEG_CONTROL_1_SPEC>;
#[doc = "mjpeg_control_1."]
pub mod mjpeg_control_1;
#[doc = "mjpeg_control_2 register accessor: an alias for `Reg<MJPEG_CONTROL_2_SPEC>`"]
pub type MJPEG_CONTROL_2 = crate::Reg<mjpeg_control_2::MJPEG_CONTROL_2_SPEC>;
#[doc = "mjpeg_control_2."]
pub mod mjpeg_control_2;
#[doc = "mjpeg_yy_frame_addr register accessor: an alias for `Reg<MJPEG_YY_FRAME_ADDR_SPEC>`"]
pub type MJPEG_YY_FRAME_ADDR = crate::Reg<mjpeg_yy_frame_addr::MJPEG_YY_FRAME_ADDR_SPEC>;
#[doc = "mjpeg_yy_frame_addr."]
pub mod mjpeg_yy_frame_addr;
#[doc = "mjpeg_uv_frame_addr register accessor: an alias for `Reg<MJPEG_UV_FRAME_ADDR_SPEC>`"]
pub type MJPEG_UV_FRAME_ADDR = crate::Reg<mjpeg_uv_frame_addr::MJPEG_UV_FRAME_ADDR_SPEC>;
#[doc = "mjpeg_uv_frame_addr."]
pub mod mjpeg_uv_frame_addr;
#[doc = "mjpeg_yuv_mem register accessor: an alias for `Reg<MJPEG_YUV_MEM_SPEC>`"]
pub type MJPEG_YUV_MEM = crate::Reg<mjpeg_yuv_mem::MJPEG_YUV_MEM_SPEC>;
#[doc = "mjpeg_yuv_mem."]
pub mod mjpeg_yuv_mem;
#[doc = "jpeg_frame_addr register accessor: an alias for `Reg<JPEG_FRAME_ADDR_SPEC>`"]
pub type JPEG_FRAME_ADDR = crate::Reg<jpeg_frame_addr::JPEG_FRAME_ADDR_SPEC>;
#[doc = "jpeg_frame_addr."]
pub mod jpeg_frame_addr;
#[doc = "jpeg_store_memory register accessor: an alias for `Reg<JPEG_STORE_MEMORY_SPEC>`"]
pub type JPEG_STORE_MEMORY = crate::Reg<jpeg_store_memory::JPEG_STORE_MEMORY_SPEC>;
#[doc = "jpeg_store_memory."]
pub mod jpeg_store_memory;
#[doc = "mjpeg_control_3 register accessor: an alias for `Reg<MJPEG_CONTROL_3_SPEC>`"]
pub type MJPEG_CONTROL_3 = crate::Reg<mjpeg_control_3::MJPEG_CONTROL_3_SPEC>;
#[doc = "mjpeg_control_3."]
pub mod mjpeg_control_3;
#[doc = "mjpeg_frame_fifo_pop register accessor: an alias for `Reg<MJPEG_FRAME_FIFO_POP_SPEC>`"]
pub type MJPEG_FRAME_FIFO_POP = crate::Reg<mjpeg_frame_fifo_pop::MJPEG_FRAME_FIFO_POP_SPEC>;
#[doc = "mjpeg_frame_fifo_pop."]
pub mod mjpeg_frame_fifo_pop;
#[doc = "mjpeg_frame_size register accessor: an alias for `Reg<MJPEG_FRAME_SIZE_SPEC>`"]
pub type MJPEG_FRAME_SIZE = crate::Reg<mjpeg_frame_size::MJPEG_FRAME_SIZE_SPEC>;
#[doc = "mjpeg_frame_size."]
pub mod mjpeg_frame_size;
#[doc = "mjpeg_header_byte register accessor: an alias for `Reg<MJPEG_HEADER_BYTE_SPEC>`"]
pub type MJPEG_HEADER_BYTE = crate::Reg<mjpeg_header_byte::MJPEG_HEADER_BYTE_SPEC>;
#[doc = "mjpeg_header_byte."]
pub mod mjpeg_header_byte;
#[doc = "mjpeg_swap_mode register accessor: an alias for `Reg<MJPEG_SWAP_MODE_SPEC>`"]
pub type MJPEG_SWAP_MODE = crate::Reg<mjpeg_swap_mode::MJPEG_SWAP_MODE_SPEC>;
#[doc = "mjpeg_swap_mode."]
pub mod mjpeg_swap_mode;
#[doc = "mjpeg_swap_bit_cnt register accessor: an alias for `Reg<MJPEG_SWAP_BIT_CNT_SPEC>`"]
pub type MJPEG_SWAP_BIT_CNT = crate::Reg<mjpeg_swap_bit_cnt::MJPEG_SWAP_BIT_CNT_SPEC>;
#[doc = "mjpeg_swap_bit_cnt."]
pub mod mjpeg_swap_bit_cnt;
#[doc = "mjpeg_paket_ctrl register accessor: an alias for `Reg<MJPEG_PAKET_CTRL_SPEC>`"]
pub type MJPEG_PAKET_CTRL = crate::Reg<mjpeg_paket_ctrl::MJPEG_PAKET_CTRL_SPEC>;
#[doc = "mjpeg_paket_ctrl."]
pub mod mjpeg_paket_ctrl;
#[doc = "mjpeg_paket_head_tail register accessor: an alias for `Reg<MJPEG_PAKET_HEAD_TAIL_SPEC>`"]
pub type MJPEG_PAKET_HEAD_TAIL = crate::Reg<mjpeg_paket_head_tail::MJPEG_PAKET_HEAD_TAIL_SPEC>;
#[doc = "mjpeg_paket_head_tail."]
pub mod mjpeg_paket_head_tail;
#[doc = "mjpeg_Y_frame_read_status_1 register accessor: an alias for `Reg<MJPEG_Y_FRAME_READ_STATUS_1_SPEC>`"]
pub type MJPEG_Y_FRAME_READ_STATUS_1 =
    crate::Reg<mjpeg_y_frame_read_status_1::MJPEG_Y_FRAME_READ_STATUS_1_SPEC>;
#[doc = "mjpeg_Y_frame_read_status_1."]
pub mod mjpeg_y_frame_read_status_1;
#[doc = "mjpeg_Y_frame_read_status_2 register accessor: an alias for `Reg<MJPEG_Y_FRAME_READ_STATUS_2_SPEC>`"]
pub type MJPEG_Y_FRAME_READ_STATUS_2 =
    crate::Reg<mjpeg_y_frame_read_status_2::MJPEG_Y_FRAME_READ_STATUS_2_SPEC>;
#[doc = "mjpeg_Y_frame_read_status_2."]
pub mod mjpeg_y_frame_read_status_2;
#[doc = "mjpeg_Y_frame_write_status register accessor: an alias for `Reg<MJPEG_Y_FRAME_WRITE_STATUS_SPEC>`"]
pub type MJPEG_Y_FRAME_WRITE_STATUS =
    crate::Reg<mjpeg_y_frame_write_status::MJPEG_Y_FRAME_WRITE_STATUS_SPEC>;
#[doc = "mjpeg_Y_frame_write_status."]
pub mod mjpeg_y_frame_write_status;
#[doc = "mjpeg_UV_frame_read_status_1 register accessor: an alias for `Reg<MJPEG_UV_FRAME_READ_STATUS_1_SPEC>`"]
pub type MJPEG_UV_FRAME_READ_STATUS_1 =
    crate::Reg<mjpeg_uv_frame_read_status_1::MJPEG_UV_FRAME_READ_STATUS_1_SPEC>;
#[doc = "mjpeg_UV_frame_read_status_1."]
pub mod mjpeg_uv_frame_read_status_1;
#[doc = "mjpeg_UV_frame_read_status_2 register accessor: an alias for `Reg<MJPEG_UV_FRAME_READ_STATUS_2_SPEC>`"]
pub type MJPEG_UV_FRAME_READ_STATUS_2 =
    crate::Reg<mjpeg_uv_frame_read_status_2::MJPEG_UV_FRAME_READ_STATUS_2_SPEC>;
#[doc = "mjpeg_UV_frame_read_status_2."]
pub mod mjpeg_uv_frame_read_status_2;
#[doc = "mjpeg_UV_frame_write_status register accessor: an alias for `Reg<MJPEG_UV_FRAME_WRITE_STATUS_SPEC>`"]
pub type MJPEG_UV_FRAME_WRITE_STATUS =
    crate::Reg<mjpeg_uv_frame_write_status::MJPEG_UV_FRAME_WRITE_STATUS_SPEC>;
#[doc = "mjpeg_UV_frame_write_status."]
pub mod mjpeg_uv_frame_write_status;
#[doc = "mjpeg_start_addr0 register accessor: an alias for `Reg<MJPEG_START_ADDR0_SPEC>`"]
pub type MJPEG_START_ADDR0 = crate::Reg<mjpeg_start_addr0::MJPEG_START_ADDR0_SPEC>;
#[doc = "mjpeg_start_addr0."]
pub mod mjpeg_start_addr0;
#[doc = "mjpeg_bit_cnt0 register accessor: an alias for `Reg<MJPEG_BIT_CNT0_SPEC>`"]
pub type MJPEG_BIT_CNT0 = crate::Reg<mjpeg_bit_cnt0::MJPEG_BIT_CNT0_SPEC>;
#[doc = "mjpeg_bit_cnt0."]
pub mod mjpeg_bit_cnt0;
#[doc = "mjpeg_start_addr1 register accessor: an alias for `Reg<MJPEG_START_ADDR1_SPEC>`"]
pub type MJPEG_START_ADDR1 = crate::Reg<mjpeg_start_addr1::MJPEG_START_ADDR1_SPEC>;
#[doc = "mjpeg_start_addr1."]
pub mod mjpeg_start_addr1;
#[doc = "mjpeg_bit_cnt1 register accessor: an alias for `Reg<MJPEG_BIT_CNT1_SPEC>`"]
pub type MJPEG_BIT_CNT1 = crate::Reg<mjpeg_bit_cnt1::MJPEG_BIT_CNT1_SPEC>;
#[doc = "mjpeg_bit_cnt1."]
pub mod mjpeg_bit_cnt1;
#[doc = "mjpeg_start_addr2 register accessor: an alias for `Reg<MJPEG_START_ADDR2_SPEC>`"]
pub type MJPEG_START_ADDR2 = crate::Reg<mjpeg_start_addr2::MJPEG_START_ADDR2_SPEC>;
#[doc = "mjpeg_start_addr2."]
pub mod mjpeg_start_addr2;
#[doc = "mjpeg_bit_cnt2 register accessor: an alias for `Reg<MJPEG_BIT_CNT2_SPEC>`"]
pub type MJPEG_BIT_CNT2 = crate::Reg<mjpeg_bit_cnt2::MJPEG_BIT_CNT2_SPEC>;
#[doc = "mjpeg_bit_cnt2."]
pub mod mjpeg_bit_cnt2;
#[doc = "mjpeg_start_addr3 register accessor: an alias for `Reg<MJPEG_START_ADDR3_SPEC>`"]
pub type MJPEG_START_ADDR3 = crate::Reg<mjpeg_start_addr3::MJPEG_START_ADDR3_SPEC>;
#[doc = "mjpeg_start_addr3."]
pub mod mjpeg_start_addr3;
#[doc = "mjpeg_bit_cnt3 register accessor: an alias for `Reg<MJPEG_BIT_CNT3_SPEC>`"]
pub type MJPEG_BIT_CNT3 = crate::Reg<mjpeg_bit_cnt3::MJPEG_BIT_CNT3_SPEC>;
#[doc = "mjpeg_bit_cnt3."]
pub mod mjpeg_bit_cnt3;
#[doc = "mjpeg_start_addr4 register accessor: an alias for `Reg<MJPEG_START_ADDR4_SPEC>`"]
pub type MJPEG_START_ADDR4 = crate::Reg<mjpeg_start_addr4::MJPEG_START_ADDR4_SPEC>;
#[doc = "mjpeg_start_addr4."]
pub mod mjpeg_start_addr4;
#[doc = "mjpeg_bit_cnt4 register accessor: an alias for `Reg<MJPEG_BIT_CNT4_SPEC>`"]
pub type MJPEG_BIT_CNT4 = crate::Reg<mjpeg_bit_cnt4::MJPEG_BIT_CNT4_SPEC>;
#[doc = "mjpeg_bit_cnt4."]
pub mod mjpeg_bit_cnt4;
#[doc = "mjpeg_start_addr5 register accessor: an alias for `Reg<MJPEG_START_ADDR5_SPEC>`"]
pub type MJPEG_START_ADDR5 = crate::Reg<mjpeg_start_addr5::MJPEG_START_ADDR5_SPEC>;
#[doc = "mjpeg_start_addr5."]
pub mod mjpeg_start_addr5;
#[doc = "mjpeg_bit_cnt5 register accessor: an alias for `Reg<MJPEG_BIT_CNT5_SPEC>`"]
pub type MJPEG_BIT_CNT5 = crate::Reg<mjpeg_bit_cnt5::MJPEG_BIT_CNT5_SPEC>;
#[doc = "mjpeg_bit_cnt5."]
pub mod mjpeg_bit_cnt5;
#[doc = "mjpeg_start_addr6 register accessor: an alias for `Reg<MJPEG_START_ADDR6_SPEC>`"]
pub type MJPEG_START_ADDR6 = crate::Reg<mjpeg_start_addr6::MJPEG_START_ADDR6_SPEC>;
#[doc = "mjpeg_start_addr6."]
pub mod mjpeg_start_addr6;
#[doc = "mjpeg_bit_cnt6 register accessor: an alias for `Reg<MJPEG_BIT_CNT6_SPEC>`"]
pub type MJPEG_BIT_CNT6 = crate::Reg<mjpeg_bit_cnt6::MJPEG_BIT_CNT6_SPEC>;
#[doc = "mjpeg_bit_cnt6."]
pub mod mjpeg_bit_cnt6;
#[doc = "mjpeg_start_addr7 register accessor: an alias for `Reg<MJPEG_START_ADDR7_SPEC>`"]
pub type MJPEG_START_ADDR7 = crate::Reg<mjpeg_start_addr7::MJPEG_START_ADDR7_SPEC>;
#[doc = "mjpeg_start_addr7."]
pub mod mjpeg_start_addr7;
#[doc = "mjpeg_bit_cnt7 register accessor: an alias for `Reg<MJPEG_BIT_CNT7_SPEC>`"]
pub type MJPEG_BIT_CNT7 = crate::Reg<mjpeg_bit_cnt7::MJPEG_BIT_CNT7_SPEC>;
#[doc = "mjpeg_bit_cnt7."]
pub mod mjpeg_bit_cnt7;
#[doc = "mjpeg_start_addr_8 register accessor: an alias for `Reg<MJPEG_START_ADDR_8_SPEC>`"]
pub type MJPEG_START_ADDR_8 = crate::Reg<mjpeg_start_addr_8::MJPEG_START_ADDR_8_SPEC>;
#[doc = "mjpeg_start_addr_8."]
pub mod mjpeg_start_addr_8;
#[doc = "mjpeg_bit_cnt_8 register accessor: an alias for `Reg<MJPEG_BIT_CNT_8_SPEC>`"]
pub type MJPEG_BIT_CNT_8 = crate::Reg<mjpeg_bit_cnt_8::MJPEG_BIT_CNT_8_SPEC>;
#[doc = "mjpeg_bit_cnt_8."]
pub mod mjpeg_bit_cnt_8;
#[doc = "mjpeg_start_addr_9 register accessor: an alias for `Reg<MJPEG_START_ADDR_9_SPEC>`"]
pub type MJPEG_START_ADDR_9 = crate::Reg<mjpeg_start_addr_9::MJPEG_START_ADDR_9_SPEC>;
#[doc = "mjpeg_start_addr_9."]
pub mod mjpeg_start_addr_9;
#[doc = "mjpeg_bit_cnt_9 register accessor: an alias for `Reg<MJPEG_BIT_CNT_9_SPEC>`"]
pub type MJPEG_BIT_CNT_9 = crate::Reg<mjpeg_bit_cnt_9::MJPEG_BIT_CNT_9_SPEC>;
#[doc = "mjpeg_bit_cnt_9."]
pub mod mjpeg_bit_cnt_9;
#[doc = "mjpeg_start_addr_a register accessor: an alias for `Reg<MJPEG_START_ADDR_A_SPEC>`"]
pub type MJPEG_START_ADDR_A = crate::Reg<mjpeg_start_addr_a::MJPEG_START_ADDR_A_SPEC>;
#[doc = "mjpeg_start_addr_a."]
pub mod mjpeg_start_addr_a;
#[doc = "mjpeg_bit_cnt_a register accessor: an alias for `Reg<MJPEG_BIT_CNT_A_SPEC>`"]
pub type MJPEG_BIT_CNT_A = crate::Reg<mjpeg_bit_cnt_a::MJPEG_BIT_CNT_A_SPEC>;
#[doc = "mjpeg_bit_cnt_a."]
pub mod mjpeg_bit_cnt_a;
#[doc = "mjpeg_start_addr_b register accessor: an alias for `Reg<MJPEG_START_ADDR_B_SPEC>`"]
pub type MJPEG_START_ADDR_B = crate::Reg<mjpeg_start_addr_b::MJPEG_START_ADDR_B_SPEC>;
#[doc = "mjpeg_start_addr_b."]
pub mod mjpeg_start_addr_b;
#[doc = "mjpeg_bit_cnt_b register accessor: an alias for `Reg<MJPEG_BIT_CNT_B_SPEC>`"]
pub type MJPEG_BIT_CNT_B = crate::Reg<mjpeg_bit_cnt_b::MJPEG_BIT_CNT_B_SPEC>;
#[doc = "mjpeg_bit_cnt_b."]
pub mod mjpeg_bit_cnt_b;
#[doc = "mjpeg_start_addr_c register accessor: an alias for `Reg<MJPEG_START_ADDR_C_SPEC>`"]
pub type MJPEG_START_ADDR_C = crate::Reg<mjpeg_start_addr_c::MJPEG_START_ADDR_C_SPEC>;
#[doc = "mjpeg_start_addr_c."]
pub mod mjpeg_start_addr_c;
#[doc = "mjpeg_bit_cnt_c register accessor: an alias for `Reg<MJPEG_BIT_CNT_C_SPEC>`"]
pub type MJPEG_BIT_CNT_C = crate::Reg<mjpeg_bit_cnt_c::MJPEG_BIT_CNT_C_SPEC>;
#[doc = "mjpeg_bit_cnt_c."]
pub mod mjpeg_bit_cnt_c;
#[doc = "mjpeg_start_addr_d register accessor: an alias for `Reg<MJPEG_START_ADDR_D_SPEC>`"]
pub type MJPEG_START_ADDR_D = crate::Reg<mjpeg_start_addr_d::MJPEG_START_ADDR_D_SPEC>;
#[doc = "mjpeg_start_addr_d."]
pub mod mjpeg_start_addr_d;
#[doc = "mjpeg_bit_cnt_d register accessor: an alias for `Reg<MJPEG_BIT_CNT_D_SPEC>`"]
pub type MJPEG_BIT_CNT_D = crate::Reg<mjpeg_bit_cnt_d::MJPEG_BIT_CNT_D_SPEC>;
#[doc = "mjpeg_bit_cnt_d."]
pub mod mjpeg_bit_cnt_d;
#[doc = "mjpeg_start_addr_e register accessor: an alias for `Reg<MJPEG_START_ADDR_E_SPEC>`"]
pub type MJPEG_START_ADDR_E = crate::Reg<mjpeg_start_addr_e::MJPEG_START_ADDR_E_SPEC>;
#[doc = "mjpeg_start_addr_e."]
pub mod mjpeg_start_addr_e;
#[doc = "mjpeg_bit_cnt_e register accessor: an alias for `Reg<MJPEG_BIT_CNT_E_SPEC>`"]
pub type MJPEG_BIT_CNT_E = crate::Reg<mjpeg_bit_cnt_e::MJPEG_BIT_CNT_E_SPEC>;
#[doc = "mjpeg_bit_cnt_e."]
pub mod mjpeg_bit_cnt_e;
#[doc = "mjpeg_start_addr_f register accessor: an alias for `Reg<MJPEG_START_ADDR_F_SPEC>`"]
pub type MJPEG_START_ADDR_F = crate::Reg<mjpeg_start_addr_f::MJPEG_START_ADDR_F_SPEC>;
#[doc = "mjpeg_start_addr_f."]
pub mod mjpeg_start_addr_f;
#[doc = "mjpeg_bit_cnt_f register accessor: an alias for `Reg<MJPEG_BIT_CNT_F_SPEC>`"]
pub type MJPEG_BIT_CNT_F = crate::Reg<mjpeg_bit_cnt_f::MJPEG_BIT_CNT_F_SPEC>;
#[doc = "mjpeg_bit_cnt_f."]
pub mod mjpeg_bit_cnt_f;
#[doc = "mjpeg_q_mode0 register accessor: an alias for `Reg<MJPEG_Q_MODE0_SPEC>`"]
pub type MJPEG_Q_MODE0 = crate::Reg<mjpeg_q_mode0::MJPEG_Q_MODE0_SPEC>;
#[doc = "mjpeg_q_mode0."]
pub mod mjpeg_q_mode0;
#[doc = "mjpeg_q_mode1 register accessor: an alias for `Reg<MJPEG_Q_MODE1_SPEC>`"]
pub type MJPEG_Q_MODE1 = crate::Reg<mjpeg_q_mode1::MJPEG_Q_MODE1_SPEC>;
#[doc = "mjpeg_q_mode1."]
pub mod mjpeg_q_mode1;
#[doc = "mjpeg_q_mode2 register accessor: an alias for `Reg<MJPEG_Q_MODE2_SPEC>`"]
pub type MJPEG_Q_MODE2 = crate::Reg<mjpeg_q_mode2::MJPEG_Q_MODE2_SPEC>;
#[doc = "mjpeg_q_mode2."]
pub mod mjpeg_q_mode2;
#[doc = "mjpeg_q_mode3 register accessor: an alias for `Reg<MJPEG_Q_MODE3_SPEC>`"]
pub type MJPEG_Q_MODE3 = crate::Reg<mjpeg_q_mode3::MJPEG_Q_MODE3_SPEC>;
#[doc = "mjpeg_q_mode3."]
pub mod mjpeg_q_mode3;
#[doc = "mjpeg_q_mode4 register accessor: an alias for `Reg<MJPEG_Q_MODE4_SPEC>`"]
pub type MJPEG_Q_MODE4 = crate::Reg<mjpeg_q_mode4::MJPEG_Q_MODE4_SPEC>;
#[doc = "mjpeg_q_mode4."]
pub mod mjpeg_q_mode4;
#[doc = "mjpeg_q_mode5 register accessor: an alias for `Reg<MJPEG_Q_MODE5_SPEC>`"]
pub type MJPEG_Q_MODE5 = crate::Reg<mjpeg_q_mode5::MJPEG_Q_MODE5_SPEC>;
#[doc = "mjpeg_q_mode5."]
pub mod mjpeg_q_mode5;
#[doc = "mjpeg_q_mode6 register accessor: an alias for `Reg<MJPEG_Q_MODE6_SPEC>`"]
pub type MJPEG_Q_MODE6 = crate::Reg<mjpeg_q_mode6::MJPEG_Q_MODE6_SPEC>;
#[doc = "mjpeg_q_mode6."]
pub mod mjpeg_q_mode6;
#[doc = "mjpeg_q_mode7 register accessor: an alias for `Reg<MJPEG_Q_MODE7_SPEC>`"]
pub type MJPEG_Q_MODE7 = crate::Reg<mjpeg_q_mode7::MJPEG_Q_MODE7_SPEC>;
#[doc = "mjpeg_q_mode7."]
pub mod mjpeg_q_mode7;
#[doc = "mjpeg_q_mode_8 register accessor: an alias for `Reg<MJPEG_Q_MODE_8_SPEC>`"]
pub type MJPEG_Q_MODE_8 = crate::Reg<mjpeg_q_mode_8::MJPEG_Q_MODE_8_SPEC>;
#[doc = "mjpeg_q_mode_8."]
pub mod mjpeg_q_mode_8;
#[doc = "mjpeg_q_mode_9 register accessor: an alias for `Reg<MJPEG_Q_MODE_9_SPEC>`"]
pub type MJPEG_Q_MODE_9 = crate::Reg<mjpeg_q_mode_9::MJPEG_Q_MODE_9_SPEC>;
#[doc = "mjpeg_q_mode_9."]
pub mod mjpeg_q_mode_9;
#[doc = "mjpeg_q_mode_a register accessor: an alias for `Reg<MJPEG_Q_MODE_A_SPEC>`"]
pub type MJPEG_Q_MODE_A = crate::Reg<mjpeg_q_mode_a::MJPEG_Q_MODE_A_SPEC>;
#[doc = "mjpeg_q_mode_a."]
pub mod mjpeg_q_mode_a;
#[doc = "mjpeg_q_mode_b register accessor: an alias for `Reg<MJPEG_Q_MODE_B_SPEC>`"]
pub type MJPEG_Q_MODE_B = crate::Reg<mjpeg_q_mode_b::MJPEG_Q_MODE_B_SPEC>;
#[doc = "mjpeg_q_mode_b."]
pub mod mjpeg_q_mode_b;
#[doc = "mjpeg_q_mode_c register accessor: an alias for `Reg<MJPEG_Q_MODE_C_SPEC>`"]
pub type MJPEG_Q_MODE_C = crate::Reg<mjpeg_q_mode_c::MJPEG_Q_MODE_C_SPEC>;
#[doc = "mjpeg_q_mode_c."]
pub mod mjpeg_q_mode_c;
#[doc = "mjpeg_q_mode_d register accessor: an alias for `Reg<MJPEG_Q_MODE_D_SPEC>`"]
pub type MJPEG_Q_MODE_D = crate::Reg<mjpeg_q_mode_d::MJPEG_Q_MODE_D_SPEC>;
#[doc = "mjpeg_q_mode_d."]
pub mod mjpeg_q_mode_d;
#[doc = "mjpeg_q_mode_e register accessor: an alias for `Reg<MJPEG_Q_MODE_E_SPEC>`"]
pub type MJPEG_Q_MODE_E = crate::Reg<mjpeg_q_mode_e::MJPEG_Q_MODE_E_SPEC>;
#[doc = "mjpeg_q_mode_e."]
pub mod mjpeg_q_mode_e;
#[doc = "mjpeg_q_mode_f register accessor: an alias for `Reg<MJPEG_Q_MODE_F_SPEC>`"]
pub type MJPEG_Q_MODE_F = crate::Reg<mjpeg_q_mode_f::MJPEG_Q_MODE_F_SPEC>;
#[doc = "mjpeg_q_mode_f."]
pub mod mjpeg_q_mode_f;
#[doc = "mjpeg_debug register accessor: an alias for `Reg<MJPEG_DEBUG_SPEC>`"]
pub type MJPEG_DEBUG = crate::Reg<mjpeg_debug::MJPEG_DEBUG_SPEC>;
#[doc = "mjpeg_debug."]
pub mod mjpeg_debug;
#[doc = "mjpeg_dummy_reg register accessor: an alias for `Reg<MJPEG_DUMMY_REG_SPEC>`"]
pub type MJPEG_DUMMY_REG = crate::Reg<mjpeg_dummy_reg::MJPEG_DUMMY_REG_SPEC>;
#[doc = "mjpeg_dummy_reg."]
pub mod mjpeg_dummy_reg;
