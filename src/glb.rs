#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - clk_cfg0."]
    pub clk_cfg0: crate::Reg<clk_cfg0::CLK_CFG0_SPEC>,
    #[doc = "0x04 - clk_cfg1."]
    pub clk_cfg1: crate::Reg<clk_cfg1::CLK_CFG1_SPEC>,
    #[doc = "0x08 - clk_cfg2."]
    pub clk_cfg2: crate::Reg<clk_cfg2::CLK_CFG2_SPEC>,
    #[doc = "0x0c - clk_cfg3."]
    pub clk_cfg3: crate::Reg<clk_cfg3::CLK_CFG3_SPEC>,
    #[doc = "0x10 - swrst_cfg0."]
    pub swrst_cfg0: crate::Reg<swrst_cfg0::SWRST_CFG0_SPEC>,
    #[doc = "0x14 - swrst_cfg1."]
    pub swrst_cfg1: crate::Reg<swrst_cfg1::SWRST_CFG1_SPEC>,
    #[doc = "0x18 - swrst_cfg2."]
    pub swrst_cfg2: crate::Reg<swrst_cfg2::SWRST_CFG2_SPEC>,
    #[doc = "0x1c - swrst_cfg3."]
    pub swrst_cfg3: crate::Reg<swrst_cfg3::SWRST_CFG3_SPEC>,
    #[doc = "0x20 - cgen_cfg0."]
    pub cgen_cfg0: crate::Reg<cgen_cfg0::CGEN_CFG0_SPEC>,
    #[doc = "0x24 - cgen_cfg1."]
    pub cgen_cfg1: crate::Reg<cgen_cfg1::CGEN_CFG1_SPEC>,
    #[doc = "0x28 - cgen_cfg2."]
    pub cgen_cfg2: crate::Reg<cgen_cfg2::CGEN_CFG2_SPEC>,
    #[doc = "0x2c - cgen_cfg3."]
    pub cgen_cfg3: crate::Reg<cgen_cfg3::CGEN_CFG3_SPEC>,
    #[doc = "0x30 - MBIST_CTL."]
    pub mbist_ctl: crate::Reg<mbist_ctl::MBIST_CTL_SPEC>,
    #[doc = "0x34 - MBIST_STAT."]
    pub mbist_stat: crate::Reg<mbist_stat::MBIST_STAT_SPEC>,
    _reserved14: [u8; 0x18],
    #[doc = "0x50 - bmx_cfg1."]
    pub bmx_cfg1: crate::Reg<bmx_cfg1::BMX_CFG1_SPEC>,
    #[doc = "0x54 - bmx_cfg2."]
    pub bmx_cfg2: crate::Reg<bmx_cfg2::BMX_CFG2_SPEC>,
    #[doc = "0x58 - bmx_err_addr."]
    pub bmx_err_addr: crate::Reg<bmx_err_addr::BMX_ERR_ADDR_SPEC>,
    #[doc = "0x5c - bmx_dbg_out."]
    pub bmx_dbg_out: crate::Reg<bmx_dbg_out::BMX_DBG_OUT_SPEC>,
    #[doc = "0x60 - rsv0."]
    pub rsv0: crate::Reg<rsv0::RSV0_SPEC>,
    #[doc = "0x64 - rsv1."]
    pub rsv1: crate::Reg<rsv1::RSV1_SPEC>,
    #[doc = "0x68 - rsv2."]
    pub rsv2: crate::Reg<rsv2::RSV2_SPEC>,
    #[doc = "0x6c - rsv3."]
    pub rsv3: crate::Reg<rsv3::RSV3_SPEC>,
    #[doc = "0x70 - sram_ret."]
    pub sram_ret: crate::Reg<sram_ret::SRAM_RET_SPEC>,
    #[doc = "0x74 - sram_slp."]
    pub sram_slp: crate::Reg<sram_slp::SRAM_SLP_SPEC>,
    #[doc = "0x78 - sram_parm."]
    pub sram_parm: crate::Reg<sram_parm::SRAM_PARM_SPEC>,
    #[doc = "0x7c - seam_misc."]
    pub seam_misc: crate::Reg<seam_misc::SEAM_MISC_SPEC>,
    #[doc = "0x80 - glb_parm."]
    pub glb_parm: crate::Reg<glb_parm::GLB_PARM_SPEC>,
    #[doc = "0x84 - PDM_CLK_CTRL."]
    pub pdm_clk_ctrl: crate::Reg<pdm_clk_ctrl::PDM_CLK_CTRL_SPEC>,
    #[doc = "0x88 - GPIO_USE_PSRAM__IO."]
    pub gpio_use_psram__io: crate::Reg<gpio_use_psram__io::GPIO_USE_PSRAM__IO_SPEC>,
    _reserved29: [u8; 0x04],
    #[doc = "0x90 - CPU_CLK_CFG."]
    pub cpu_clk_cfg: crate::Reg<cpu_clk_cfg::CPU_CLK_CFG_SPEC>,
    _reserved30: [u8; 0x10],
    #[doc = "0xa4 - GPADC_32M_SRC_CTRL."]
    pub gpadc_32m_src_ctrl: crate::Reg<gpadc_32m_src_ctrl::GPADC_32M_SRC_CTRL_SPEC>,
    #[doc = "0xa8 - DIG32K_WAKEUP_CTRL."]
    pub dig32k_wakeup_ctrl: crate::Reg<dig32k_wakeup_ctrl::DIG32K_WAKEUP_CTRL_SPEC>,
    #[doc = "0xac - WIFI_BT_COEX_CTRL."]
    pub wifi_bt_coex_ctrl: crate::Reg<wifi_bt_coex_ctrl::WIFI_BT_COEX_CTRL_SPEC>,
    #[doc = "0xb0 - BZ_COEX_CTRL."]
    pub bz_coex_ctrl: crate::Reg<bz_coex_ctrl::BZ_COEX_CTRL_SPEC>,
    _reserved34: [u8; 0x0c],
    #[doc = "0xc0 - UART_SIG_SEL_0."]
    pub uart_sig_sel_0: crate::Reg<uart_sig_sel_0::UART_SIG_SEL_0_SPEC>,
    _reserved35: [u8; 0x0c],
    #[doc = "0xd0 - DBG_SEL_LL."]
    pub dbg_sel_ll: crate::Reg<dbg_sel_ll::DBG_SEL_LL_SPEC>,
    #[doc = "0xd4 - DBG_SEL_LH."]
    pub dbg_sel_lh: crate::Reg<dbg_sel_lh::DBG_SEL_LH_SPEC>,
    #[doc = "0xd8 - DBG_SEL_HL."]
    pub dbg_sel_hl: crate::Reg<dbg_sel_hl::DBG_SEL_HL_SPEC>,
    #[doc = "0xdc - DBG_SEL_HH."]
    pub dbg_sel_hh: crate::Reg<dbg_sel_hh::DBG_SEL_HH_SPEC>,
    #[doc = "0xe0 - debug."]
    pub debug: crate::Reg<debug::DEBUG_SPEC>,
    _reserved40: [u8; 0x1c],
    #[doc = "0x100..0x140 - GPIO_CFGCTL%s."]
    pub gpio_pinmode: [crate::Reg<gpio_pinmode::GPIO_PINMODE_SPEC>; 16],
    #[doc = "0x140..0x14c - GPIO_CFGCTL16."]
    pub gpio_pinmode2: [crate::Reg<gpio_pinmode2::GPIO_PINMODE2_SPEC>; 3],
    _reserved42: [u8; 0x34],
    #[doc = "0x180 - GPIO_CFGCTL30."]
    pub gpio_cfgctl30: crate::Reg<gpio_cfgctl30::GPIO_CFGCTL30_SPEC>,
    #[doc = "0x184 - GPIO_CFGCTL31."]
    pub gpio_cfgctl31: crate::Reg<gpio_cfgctl31::GPIO_CFGCTL31_SPEC>,
    #[doc = "0x188 - GPIO_CFGCTL32."]
    pub gpio_cfgctl32: crate::Reg<gpio_cfgctl32::GPIO_CFGCTL32_SPEC>,
    #[doc = "0x18c - GPIO_CFGCTL33."]
    pub gpio_cfgctl33: crate::Reg<gpio_cfgctl33::GPIO_CFGCTL33_SPEC>,
    #[doc = "0x190 - GPIO_CFGCTL34."]
    pub gpio_cfgctl34: crate::Reg<gpio_cfgctl34::GPIO_CFGCTL34_SPEC>,
    #[doc = "0x194 - GPIO_CFGCTL35."]
    pub gpio_cfgctl35: crate::Reg<gpio_cfgctl35::GPIO_CFGCTL35_SPEC>,
    _reserved48: [u8; 0x08],
    #[doc = "0x1a0 - GPIO_INT_MASK1."]
    pub gpio_int_mask1: crate::Reg<gpio_int_mask1::GPIO_INT_MASK1_SPEC>,
    _reserved49: [u8; 0x04],
    #[doc = "0x1a8 - GPIO_INT_STAT1."]
    pub gpio_int_stat1: crate::Reg<gpio_int_stat1::GPIO_INT_STAT1_SPEC>,
    _reserved50: [u8; 0x04],
    #[doc = "0x1b0 - GPIO_INT_CLR1."]
    pub gpio_int_clr1: crate::Reg<gpio_int_clr1::GPIO_INT_CLR1_SPEC>,
    _reserved51: [u8; 0x0c],
    #[doc = "0x1c0 - GPIO_INT_MODE_SET1."]
    pub gpio_int_mode_set1: crate::Reg<gpio_int_mode_set1::GPIO_INT_MODE_SET1_SPEC>,
    #[doc = "0x1c4 - GPIO_INT_MODE_SET2."]
    pub gpio_int_mode_set2: crate::Reg<gpio_int_mode_set2::GPIO_INT_MODE_SET2_SPEC>,
    #[doc = "0x1c8 - GPIO_INT_MODE_SET3."]
    pub gpio_int_mode_set3: crate::Reg<gpio_int_mode_set3::GPIO_INT_MODE_SET3_SPEC>,
    #[doc = "0x1cc - GPIO_INT_MODE_SET4."]
    pub gpio_int_mode_set4: crate::Reg<gpio_int_mode_set4::GPIO_INT_MODE_SET4_SPEC>,
    #[doc = "0x1d0 - GPIO_INT2_MASK1."]
    pub gpio_int2_mask1: crate::Reg<gpio_int2_mask1::GPIO_INT2_MASK1_SPEC>,
    #[doc = "0x1d4 - GPIO_INT2_STAT1."]
    pub gpio_int2_stat1: crate::Reg<gpio_int2_stat1::GPIO_INT2_STAT1_SPEC>,
    #[doc = "0x1d8 - GPIO_INT2_CLR1."]
    pub gpio_int2_clr1: crate::Reg<gpio_int2_clr1::GPIO_INT2_CLR1_SPEC>,
    #[doc = "0x1dc - GPIO_INT2_MODE_SET1."]
    pub gpio_int2_mode_set1: crate::Reg<gpio_int2_mode_set1::GPIO_INT2_MODE_SET1_SPEC>,
    #[doc = "0x1e0 - GPIO_INT2_MODE_SET2."]
    pub gpio_int2_mode_set2: crate::Reg<gpio_int2_mode_set2::GPIO_INT2_MODE_SET2_SPEC>,
    #[doc = "0x1e4 - GPIO_INT2_MODE_SET3."]
    pub gpio_int2_mode_set3: crate::Reg<gpio_int2_mode_set3::GPIO_INT2_MODE_SET3_SPEC>,
    #[doc = "0x1e8 - GPIO_INT2_MODE_SET4."]
    pub gpio_int2_mode_set4: crate::Reg<gpio_int2_mode_set4::GPIO_INT2_MODE_SET4_SPEC>,
    _reserved62: [u8; 0x14],
    #[doc = "0x200 - dll."]
    pub dll: crate::Reg<dll::DLL_SPEC>,
    _reserved63: [u8; 0x20],
    #[doc = "0x224 - led_driver."]
    pub led_driver: crate::Reg<led_driver::LED_DRIVER_SPEC>,
    #[doc = "0x228 - usb_xcvr."]
    pub usb_xcvr: crate::Reg<usb_xcvr::USB_XCVR_SPEC>,
    #[doc = "0x22c - usb_xcvr_config."]
    pub usb_xcvr_config: crate::Reg<usb_xcvr_config::USB_XCVR_CONFIG_SPEC>,
    _reserved66: [u8; 0xd8],
    #[doc = "0x308 - gpdac_ctrl."]
    pub gpdac_ctrl: crate::Reg<gpdac_ctrl::GPDAC_CTRL_SPEC>,
    #[doc = "0x30c - gpdac_actrl."]
    pub gpdac_actrl: crate::Reg<gpdac_actrl::GPDAC_ACTRL_SPEC>,
    #[doc = "0x310 - gpdac_bctrl."]
    pub gpdac_bctrl: crate::Reg<gpdac_bctrl::GPDAC_BCTRL_SPEC>,
    #[doc = "0x314 - gpdac_data."]
    pub gpdac_data: crate::Reg<gpdac_data::GPDAC_DATA_SPEC>,
    _reserved70: [u8; 0x0ae8],
    #[doc = "0xe00 - chip_revision."]
    pub chip_revision: crate::Reg<chip_revision::CHIP_REVISION_SPEC>,
    _reserved71: [u8; 0xfc],
    #[doc = "0xf00 - tzc_glb_ctrl_0."]
    pub tzc_glb_ctrl_0: crate::Reg<tzc_glb_ctrl_0::TZC_GLB_CTRL_0_SPEC>,
    #[doc = "0xf04 - tzc_glb_ctrl_1."]
    pub tzc_glb_ctrl_1: crate::Reg<tzc_glb_ctrl_1::TZC_GLB_CTRL_1_SPEC>,
    #[doc = "0xf08 - tzc_glb_ctrl_2."]
    pub tzc_glb_ctrl_2: crate::Reg<tzc_glb_ctrl_2::TZC_GLB_CTRL_2_SPEC>,
    #[doc = "0xf0c - tzc_glb_ctrl_3."]
    pub tzc_glb_ctrl_3: crate::Reg<tzc_glb_ctrl_3::TZC_GLB_CTRL_3_SPEC>,
}
#[doc = "clk_cfg0 register accessor: an alias for `Reg<CLK_CFG0_SPEC>`"]
pub type CLK_CFG0 = crate::Reg<clk_cfg0::CLK_CFG0_SPEC>;
#[doc = "clk_cfg0."]
pub mod clk_cfg0;
#[doc = "clk_cfg1 register accessor: an alias for `Reg<CLK_CFG1_SPEC>`"]
pub type CLK_CFG1 = crate::Reg<clk_cfg1::CLK_CFG1_SPEC>;
#[doc = "clk_cfg1."]
pub mod clk_cfg1;
#[doc = "clk_cfg2 register accessor: an alias for `Reg<CLK_CFG2_SPEC>`"]
pub type CLK_CFG2 = crate::Reg<clk_cfg2::CLK_CFG2_SPEC>;
#[doc = "clk_cfg2."]
pub mod clk_cfg2;
#[doc = "clk_cfg3 register accessor: an alias for `Reg<CLK_CFG3_SPEC>`"]
pub type CLK_CFG3 = crate::Reg<clk_cfg3::CLK_CFG3_SPEC>;
#[doc = "clk_cfg3."]
pub mod clk_cfg3;
#[doc = "swrst_cfg0 register accessor: an alias for `Reg<SWRST_CFG0_SPEC>`"]
pub type SWRST_CFG0 = crate::Reg<swrst_cfg0::SWRST_CFG0_SPEC>;
#[doc = "swrst_cfg0."]
pub mod swrst_cfg0;
#[doc = "swrst_cfg1 register accessor: an alias for `Reg<SWRST_CFG1_SPEC>`"]
pub type SWRST_CFG1 = crate::Reg<swrst_cfg1::SWRST_CFG1_SPEC>;
#[doc = "swrst_cfg1."]
pub mod swrst_cfg1;
#[doc = "swrst_cfg2 register accessor: an alias for `Reg<SWRST_CFG2_SPEC>`"]
pub type SWRST_CFG2 = crate::Reg<swrst_cfg2::SWRST_CFG2_SPEC>;
#[doc = "swrst_cfg2."]
pub mod swrst_cfg2;
#[doc = "swrst_cfg3 register accessor: an alias for `Reg<SWRST_CFG3_SPEC>`"]
pub type SWRST_CFG3 = crate::Reg<swrst_cfg3::SWRST_CFG3_SPEC>;
#[doc = "swrst_cfg3."]
pub mod swrst_cfg3;
#[doc = "cgen_cfg0 register accessor: an alias for `Reg<CGEN_CFG0_SPEC>`"]
pub type CGEN_CFG0 = crate::Reg<cgen_cfg0::CGEN_CFG0_SPEC>;
#[doc = "cgen_cfg0."]
pub mod cgen_cfg0;
#[doc = "cgen_cfg1 register accessor: an alias for `Reg<CGEN_CFG1_SPEC>`"]
pub type CGEN_CFG1 = crate::Reg<cgen_cfg1::CGEN_CFG1_SPEC>;
#[doc = "cgen_cfg1."]
pub mod cgen_cfg1;
#[doc = "cgen_cfg2 register accessor: an alias for `Reg<CGEN_CFG2_SPEC>`"]
pub type CGEN_CFG2 = crate::Reg<cgen_cfg2::CGEN_CFG2_SPEC>;
#[doc = "cgen_cfg2."]
pub mod cgen_cfg2;
#[doc = "cgen_cfg3 register accessor: an alias for `Reg<CGEN_CFG3_SPEC>`"]
pub type CGEN_CFG3 = crate::Reg<cgen_cfg3::CGEN_CFG3_SPEC>;
#[doc = "cgen_cfg3."]
pub mod cgen_cfg3;
#[doc = "MBIST_CTL register accessor: an alias for `Reg<MBIST_CTL_SPEC>`"]
pub type MBIST_CTL = crate::Reg<mbist_ctl::MBIST_CTL_SPEC>;
#[doc = "MBIST_CTL."]
pub mod mbist_ctl;
#[doc = "MBIST_STAT register accessor: an alias for `Reg<MBIST_STAT_SPEC>`"]
pub type MBIST_STAT = crate::Reg<mbist_stat::MBIST_STAT_SPEC>;
#[doc = "MBIST_STAT."]
pub mod mbist_stat;
#[doc = "bmx_cfg1 register accessor: an alias for `Reg<BMX_CFG1_SPEC>`"]
pub type BMX_CFG1 = crate::Reg<bmx_cfg1::BMX_CFG1_SPEC>;
#[doc = "bmx_cfg1."]
pub mod bmx_cfg1;
#[doc = "bmx_cfg2 register accessor: an alias for `Reg<BMX_CFG2_SPEC>`"]
pub type BMX_CFG2 = crate::Reg<bmx_cfg2::BMX_CFG2_SPEC>;
#[doc = "bmx_cfg2."]
pub mod bmx_cfg2;
#[doc = "bmx_err_addr register accessor: an alias for `Reg<BMX_ERR_ADDR_SPEC>`"]
pub type BMX_ERR_ADDR = crate::Reg<bmx_err_addr::BMX_ERR_ADDR_SPEC>;
#[doc = "bmx_err_addr."]
pub mod bmx_err_addr;
#[doc = "bmx_dbg_out register accessor: an alias for `Reg<BMX_DBG_OUT_SPEC>`"]
pub type BMX_DBG_OUT = crate::Reg<bmx_dbg_out::BMX_DBG_OUT_SPEC>;
#[doc = "bmx_dbg_out."]
pub mod bmx_dbg_out;
#[doc = "rsv0 register accessor: an alias for `Reg<RSV0_SPEC>`"]
pub type RSV0 = crate::Reg<rsv0::RSV0_SPEC>;
#[doc = "rsv0."]
pub mod rsv0;
#[doc = "rsv1 register accessor: an alias for `Reg<RSV1_SPEC>`"]
pub type RSV1 = crate::Reg<rsv1::RSV1_SPEC>;
#[doc = "rsv1."]
pub mod rsv1;
#[doc = "rsv2 register accessor: an alias for `Reg<RSV2_SPEC>`"]
pub type RSV2 = crate::Reg<rsv2::RSV2_SPEC>;
#[doc = "rsv2."]
pub mod rsv2;
#[doc = "rsv3 register accessor: an alias for `Reg<RSV3_SPEC>`"]
pub type RSV3 = crate::Reg<rsv3::RSV3_SPEC>;
#[doc = "rsv3."]
pub mod rsv3;
#[doc = "sram_ret register accessor: an alias for `Reg<SRAM_RET_SPEC>`"]
pub type SRAM_RET = crate::Reg<sram_ret::SRAM_RET_SPEC>;
#[doc = "sram_ret."]
pub mod sram_ret;
#[doc = "sram_slp register accessor: an alias for `Reg<SRAM_SLP_SPEC>`"]
pub type SRAM_SLP = crate::Reg<sram_slp::SRAM_SLP_SPEC>;
#[doc = "sram_slp."]
pub mod sram_slp;
#[doc = "sram_parm register accessor: an alias for `Reg<SRAM_PARM_SPEC>`"]
pub type SRAM_PARM = crate::Reg<sram_parm::SRAM_PARM_SPEC>;
#[doc = "sram_parm."]
pub mod sram_parm;
#[doc = "seam_misc register accessor: an alias for `Reg<SEAM_MISC_SPEC>`"]
pub type SEAM_MISC = crate::Reg<seam_misc::SEAM_MISC_SPEC>;
#[doc = "seam_misc."]
pub mod seam_misc;
#[doc = "glb_parm register accessor: an alias for `Reg<GLB_PARM_SPEC>`"]
pub type GLB_PARM = crate::Reg<glb_parm::GLB_PARM_SPEC>;
#[doc = "glb_parm."]
pub mod glb_parm;
#[doc = "PDM_CLK_CTRL register accessor: an alias for `Reg<PDM_CLK_CTRL_SPEC>`"]
pub type PDM_CLK_CTRL = crate::Reg<pdm_clk_ctrl::PDM_CLK_CTRL_SPEC>;
#[doc = "PDM_CLK_CTRL."]
pub mod pdm_clk_ctrl;
#[doc = "GPIO_USE_PSRAM__IO register accessor: an alias for `Reg<GPIO_USE_PSRAM__IO_SPEC>`"]
pub type GPIO_USE_PSRAM__IO = crate::Reg<gpio_use_psram__io::GPIO_USE_PSRAM__IO_SPEC>;
#[doc = "GPIO_USE_PSRAM__IO."]
pub mod gpio_use_psram__io;
#[doc = "CPU_CLK_CFG register accessor: an alias for `Reg<CPU_CLK_CFG_SPEC>`"]
pub type CPU_CLK_CFG = crate::Reg<cpu_clk_cfg::CPU_CLK_CFG_SPEC>;
#[doc = "CPU_CLK_CFG."]
pub mod cpu_clk_cfg;
#[doc = "GPADC_32M_SRC_CTRL register accessor: an alias for `Reg<GPADC_32M_SRC_CTRL_SPEC>`"]
pub type GPADC_32M_SRC_CTRL = crate::Reg<gpadc_32m_src_ctrl::GPADC_32M_SRC_CTRL_SPEC>;
#[doc = "GPADC_32M_SRC_CTRL."]
pub mod gpadc_32m_src_ctrl;
#[doc = "DIG32K_WAKEUP_CTRL register accessor: an alias for `Reg<DIG32K_WAKEUP_CTRL_SPEC>`"]
pub type DIG32K_WAKEUP_CTRL = crate::Reg<dig32k_wakeup_ctrl::DIG32K_WAKEUP_CTRL_SPEC>;
#[doc = "DIG32K_WAKEUP_CTRL."]
pub mod dig32k_wakeup_ctrl;
#[doc = "WIFI_BT_COEX_CTRL register accessor: an alias for `Reg<WIFI_BT_COEX_CTRL_SPEC>`"]
pub type WIFI_BT_COEX_CTRL = crate::Reg<wifi_bt_coex_ctrl::WIFI_BT_COEX_CTRL_SPEC>;
#[doc = "WIFI_BT_COEX_CTRL."]
pub mod wifi_bt_coex_ctrl;
#[doc = "BZ_COEX_CTRL register accessor: an alias for `Reg<BZ_COEX_CTRL_SPEC>`"]
pub type BZ_COEX_CTRL = crate::Reg<bz_coex_ctrl::BZ_COEX_CTRL_SPEC>;
#[doc = "BZ_COEX_CTRL."]
pub mod bz_coex_ctrl;
#[doc = "UART_SIG_SEL_0 register accessor: an alias for `Reg<UART_SIG_SEL_0_SPEC>`"]
pub type UART_SIG_SEL_0 = crate::Reg<uart_sig_sel_0::UART_SIG_SEL_0_SPEC>;
#[doc = "UART_SIG_SEL_0."]
pub mod uart_sig_sel_0;
#[doc = "DBG_SEL_LL register accessor: an alias for `Reg<DBG_SEL_LL_SPEC>`"]
pub type DBG_SEL_LL = crate::Reg<dbg_sel_ll::DBG_SEL_LL_SPEC>;
#[doc = "DBG_SEL_LL."]
pub mod dbg_sel_ll;
#[doc = "DBG_SEL_LH register accessor: an alias for `Reg<DBG_SEL_LH_SPEC>`"]
pub type DBG_SEL_LH = crate::Reg<dbg_sel_lh::DBG_SEL_LH_SPEC>;
#[doc = "DBG_SEL_LH."]
pub mod dbg_sel_lh;
#[doc = "DBG_SEL_HL register accessor: an alias for `Reg<DBG_SEL_HL_SPEC>`"]
pub type DBG_SEL_HL = crate::Reg<dbg_sel_hl::DBG_SEL_HL_SPEC>;
#[doc = "DBG_SEL_HL."]
pub mod dbg_sel_hl;
#[doc = "DBG_SEL_HH register accessor: an alias for `Reg<DBG_SEL_HH_SPEC>`"]
pub type DBG_SEL_HH = crate::Reg<dbg_sel_hh::DBG_SEL_HH_SPEC>;
#[doc = "DBG_SEL_HH."]
pub mod dbg_sel_hh;
#[doc = "debug register accessor: an alias for `Reg<DEBUG_SPEC>`"]
pub type DEBUG = crate::Reg<debug::DEBUG_SPEC>;
#[doc = "debug."]
pub mod debug;
#[doc = "GPIO_PINMODE register accessor: an alias for `Reg<GPIO_PINMODE_SPEC>`"]
pub type GPIO_PINMODE = crate::Reg<gpio_pinmode::GPIO_PINMODE_SPEC>;
#[doc = "GPIO_CFGCTL%s."]
pub mod gpio_pinmode;
#[doc = "GPIO_PINMODE2 register accessor: an alias for `Reg<GPIO_PINMODE2_SPEC>`"]
pub type GPIO_PINMODE2 = crate::Reg<gpio_pinmode2::GPIO_PINMODE2_SPEC>;
#[doc = "GPIO_CFGCTL16."]
pub mod gpio_pinmode2;
#[doc = "GPIO_CFGCTL30 register accessor: an alias for `Reg<GPIO_CFGCTL30_SPEC>`"]
pub type GPIO_CFGCTL30 = crate::Reg<gpio_cfgctl30::GPIO_CFGCTL30_SPEC>;
#[doc = "GPIO_CFGCTL30."]
pub mod gpio_cfgctl30;
#[doc = "GPIO_CFGCTL31 register accessor: an alias for `Reg<GPIO_CFGCTL31_SPEC>`"]
pub type GPIO_CFGCTL31 = crate::Reg<gpio_cfgctl31::GPIO_CFGCTL31_SPEC>;
#[doc = "GPIO_CFGCTL31."]
pub mod gpio_cfgctl31;
#[doc = "GPIO_CFGCTL32 register accessor: an alias for `Reg<GPIO_CFGCTL32_SPEC>`"]
pub type GPIO_CFGCTL32 = crate::Reg<gpio_cfgctl32::GPIO_CFGCTL32_SPEC>;
#[doc = "GPIO_CFGCTL32."]
pub mod gpio_cfgctl32;
#[doc = "GPIO_CFGCTL33 register accessor: an alias for `Reg<GPIO_CFGCTL33_SPEC>`"]
pub type GPIO_CFGCTL33 = crate::Reg<gpio_cfgctl33::GPIO_CFGCTL33_SPEC>;
#[doc = "GPIO_CFGCTL33."]
pub mod gpio_cfgctl33;
#[doc = "GPIO_CFGCTL34 register accessor: an alias for `Reg<GPIO_CFGCTL34_SPEC>`"]
pub type GPIO_CFGCTL34 = crate::Reg<gpio_cfgctl34::GPIO_CFGCTL34_SPEC>;
#[doc = "GPIO_CFGCTL34."]
pub mod gpio_cfgctl34;
#[doc = "GPIO_CFGCTL35 register accessor: an alias for `Reg<GPIO_CFGCTL35_SPEC>`"]
pub type GPIO_CFGCTL35 = crate::Reg<gpio_cfgctl35::GPIO_CFGCTL35_SPEC>;
#[doc = "GPIO_CFGCTL35."]
pub mod gpio_cfgctl35;
#[doc = "GPIO_INT_MASK1 register accessor: an alias for `Reg<GPIO_INT_MASK1_SPEC>`"]
pub type GPIO_INT_MASK1 = crate::Reg<gpio_int_mask1::GPIO_INT_MASK1_SPEC>;
#[doc = "GPIO_INT_MASK1."]
pub mod gpio_int_mask1;
#[doc = "GPIO_INT_STAT1 register accessor: an alias for `Reg<GPIO_INT_STAT1_SPEC>`"]
pub type GPIO_INT_STAT1 = crate::Reg<gpio_int_stat1::GPIO_INT_STAT1_SPEC>;
#[doc = "GPIO_INT_STAT1."]
pub mod gpio_int_stat1;
#[doc = "GPIO_INT_CLR1 register accessor: an alias for `Reg<GPIO_INT_CLR1_SPEC>`"]
pub type GPIO_INT_CLR1 = crate::Reg<gpio_int_clr1::GPIO_INT_CLR1_SPEC>;
#[doc = "GPIO_INT_CLR1."]
pub mod gpio_int_clr1;
#[doc = "GPIO_INT_MODE_SET1 register accessor: an alias for `Reg<GPIO_INT_MODE_SET1_SPEC>`"]
pub type GPIO_INT_MODE_SET1 = crate::Reg<gpio_int_mode_set1::GPIO_INT_MODE_SET1_SPEC>;
#[doc = "GPIO_INT_MODE_SET1."]
pub mod gpio_int_mode_set1;
#[doc = "GPIO_INT_MODE_SET2 register accessor: an alias for `Reg<GPIO_INT_MODE_SET2_SPEC>`"]
pub type GPIO_INT_MODE_SET2 = crate::Reg<gpio_int_mode_set2::GPIO_INT_MODE_SET2_SPEC>;
#[doc = "GPIO_INT_MODE_SET2."]
pub mod gpio_int_mode_set2;
#[doc = "GPIO_INT_MODE_SET3 register accessor: an alias for `Reg<GPIO_INT_MODE_SET3_SPEC>`"]
pub type GPIO_INT_MODE_SET3 = crate::Reg<gpio_int_mode_set3::GPIO_INT_MODE_SET3_SPEC>;
#[doc = "GPIO_INT_MODE_SET3."]
pub mod gpio_int_mode_set3;
#[doc = "GPIO_INT_MODE_SET4 register accessor: an alias for `Reg<GPIO_INT_MODE_SET4_SPEC>`"]
pub type GPIO_INT_MODE_SET4 = crate::Reg<gpio_int_mode_set4::GPIO_INT_MODE_SET4_SPEC>;
#[doc = "GPIO_INT_MODE_SET4."]
pub mod gpio_int_mode_set4;
#[doc = "GPIO_INT2_MASK1 register accessor: an alias for `Reg<GPIO_INT2_MASK1_SPEC>`"]
pub type GPIO_INT2_MASK1 = crate::Reg<gpio_int2_mask1::GPIO_INT2_MASK1_SPEC>;
#[doc = "GPIO_INT2_MASK1."]
pub mod gpio_int2_mask1;
#[doc = "GPIO_INT2_STAT1 register accessor: an alias for `Reg<GPIO_INT2_STAT1_SPEC>`"]
pub type GPIO_INT2_STAT1 = crate::Reg<gpio_int2_stat1::GPIO_INT2_STAT1_SPEC>;
#[doc = "GPIO_INT2_STAT1."]
pub mod gpio_int2_stat1;
#[doc = "GPIO_INT2_CLR1 register accessor: an alias for `Reg<GPIO_INT2_CLR1_SPEC>`"]
pub type GPIO_INT2_CLR1 = crate::Reg<gpio_int2_clr1::GPIO_INT2_CLR1_SPEC>;
#[doc = "GPIO_INT2_CLR1."]
pub mod gpio_int2_clr1;
#[doc = "GPIO_INT2_MODE_SET1 register accessor: an alias for `Reg<GPIO_INT2_MODE_SET1_SPEC>`"]
pub type GPIO_INT2_MODE_SET1 = crate::Reg<gpio_int2_mode_set1::GPIO_INT2_MODE_SET1_SPEC>;
#[doc = "GPIO_INT2_MODE_SET1."]
pub mod gpio_int2_mode_set1;
#[doc = "GPIO_INT2_MODE_SET2 register accessor: an alias for `Reg<GPIO_INT2_MODE_SET2_SPEC>`"]
pub type GPIO_INT2_MODE_SET2 = crate::Reg<gpio_int2_mode_set2::GPIO_INT2_MODE_SET2_SPEC>;
#[doc = "GPIO_INT2_MODE_SET2."]
pub mod gpio_int2_mode_set2;
#[doc = "GPIO_INT2_MODE_SET3 register accessor: an alias for `Reg<GPIO_INT2_MODE_SET3_SPEC>`"]
pub type GPIO_INT2_MODE_SET3 = crate::Reg<gpio_int2_mode_set3::GPIO_INT2_MODE_SET3_SPEC>;
#[doc = "GPIO_INT2_MODE_SET3."]
pub mod gpio_int2_mode_set3;
#[doc = "GPIO_INT2_MODE_SET4 register accessor: an alias for `Reg<GPIO_INT2_MODE_SET4_SPEC>`"]
pub type GPIO_INT2_MODE_SET4 = crate::Reg<gpio_int2_mode_set4::GPIO_INT2_MODE_SET4_SPEC>;
#[doc = "GPIO_INT2_MODE_SET4."]
pub mod gpio_int2_mode_set4;
#[doc = "dll register accessor: an alias for `Reg<DLL_SPEC>`"]
pub type DLL = crate::Reg<dll::DLL_SPEC>;
#[doc = "dll."]
pub mod dll;
#[doc = "led_driver register accessor: an alias for `Reg<LED_DRIVER_SPEC>`"]
pub type LED_DRIVER = crate::Reg<led_driver::LED_DRIVER_SPEC>;
#[doc = "led_driver."]
pub mod led_driver;
#[doc = "usb_xcvr register accessor: an alias for `Reg<USB_XCVR_SPEC>`"]
pub type USB_XCVR = crate::Reg<usb_xcvr::USB_XCVR_SPEC>;
#[doc = "usb_xcvr."]
pub mod usb_xcvr;
#[doc = "usb_xcvr_config register accessor: an alias for `Reg<USB_XCVR_CONFIG_SPEC>`"]
pub type USB_XCVR_CONFIG = crate::Reg<usb_xcvr_config::USB_XCVR_CONFIG_SPEC>;
#[doc = "usb_xcvr_config."]
pub mod usb_xcvr_config;
#[doc = "gpdac_ctrl register accessor: an alias for `Reg<GPDAC_CTRL_SPEC>`"]
pub type GPDAC_CTRL = crate::Reg<gpdac_ctrl::GPDAC_CTRL_SPEC>;
#[doc = "gpdac_ctrl."]
pub mod gpdac_ctrl;
#[doc = "gpdac_actrl register accessor: an alias for `Reg<GPDAC_ACTRL_SPEC>`"]
pub type GPDAC_ACTRL = crate::Reg<gpdac_actrl::GPDAC_ACTRL_SPEC>;
#[doc = "gpdac_actrl."]
pub mod gpdac_actrl;
#[doc = "gpdac_bctrl register accessor: an alias for `Reg<GPDAC_BCTRL_SPEC>`"]
pub type GPDAC_BCTRL = crate::Reg<gpdac_bctrl::GPDAC_BCTRL_SPEC>;
#[doc = "gpdac_bctrl."]
pub mod gpdac_bctrl;
#[doc = "gpdac_data register accessor: an alias for `Reg<GPDAC_DATA_SPEC>`"]
pub type GPDAC_DATA = crate::Reg<gpdac_data::GPDAC_DATA_SPEC>;
#[doc = "gpdac_data."]
pub mod gpdac_data;
#[doc = "chip_revision register accessor: an alias for `Reg<CHIP_REVISION_SPEC>`"]
pub type CHIP_REVISION = crate::Reg<chip_revision::CHIP_REVISION_SPEC>;
#[doc = "chip_revision."]
pub mod chip_revision;
#[doc = "tzc_glb_ctrl_0 register accessor: an alias for `Reg<TZC_GLB_CTRL_0_SPEC>`"]
pub type TZC_GLB_CTRL_0 = crate::Reg<tzc_glb_ctrl_0::TZC_GLB_CTRL_0_SPEC>;
#[doc = "tzc_glb_ctrl_0."]
pub mod tzc_glb_ctrl_0;
#[doc = "tzc_glb_ctrl_1 register accessor: an alias for `Reg<TZC_GLB_CTRL_1_SPEC>`"]
pub type TZC_GLB_CTRL_1 = crate::Reg<tzc_glb_ctrl_1::TZC_GLB_CTRL_1_SPEC>;
#[doc = "tzc_glb_ctrl_1."]
pub mod tzc_glb_ctrl_1;
#[doc = "tzc_glb_ctrl_2 register accessor: an alias for `Reg<TZC_GLB_CTRL_2_SPEC>`"]
pub type TZC_GLB_CTRL_2 = crate::Reg<tzc_glb_ctrl_2::TZC_GLB_CTRL_2_SPEC>;
#[doc = "tzc_glb_ctrl_2."]
pub mod tzc_glb_ctrl_2;
#[doc = "tzc_glb_ctrl_3 register accessor: an alias for `Reg<TZC_GLB_CTRL_3_SPEC>`"]
pub type TZC_GLB_CTRL_3 = crate::Reg<tzc_glb_ctrl_3::TZC_GLB_CTRL_3_SPEC>;
#[doc = "tzc_glb_ctrl_3."]
pub mod tzc_glb_ctrl_3;
