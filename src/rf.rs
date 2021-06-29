#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Silicon revision"]
    pub rf_rev: crate::Reg<rf_rev::RF_REV_SPEC>,
    #[doc = "0x04 - dsp_readback."]
    pub dsp_readback: crate::Reg<dsp_readback::DSP_READBACK_SPEC>,
    #[doc = "0x08 - Control logic switch"]
    pub rf_ctrl_source: crate::Reg<rf_ctrl_source::RF_CTRL_SOURCE_SPEC>,
    #[doc = "0x0c - rf calibration state enable in full cal list"]
    pub rf_cal_state_ctrl: crate::Reg<rf_cal_state_ctrl::RF_CAL_STATE_CTRL_SPEC>,
    #[doc = "0x10 - Calibration mode register"]
    pub rf_cal_switch_ctrl: crate::Reg<rf_cal_switch_ctrl::RF_CAL_SWITCH_CTRL_SPEC>,
    #[doc = "0x14 - rf_cal_status."]
    pub rf_cal_status: crate::Reg<rf_cal_status::RF_CAL_STATUS_SPEC>,
    #[doc = "0x18 - pu_delay_confg."]
    pub pu_delay_confg: crate::Reg<pu_delay_confg::PU_DELAY_CONFG_SPEC>,
    _reserved7: [u8; 0xe4],
    #[doc = "0x100 - Register control of power up signals"]
    pub pucr_reg: crate::Reg<pucr_reg::PUCR_REG_SPEC>,
    #[doc = "0x104 - Power up setting in SB state"]
    pub pucr_sb: crate::Reg<pucr_sb::PUCR_SB_SPEC>,
    #[doc = "0x108 - Power up in LOTX state"]
    pub pucr_lotx: crate::Reg<pucr_lotx::PUCR_LOTX_SPEC>,
    #[doc = "0x10c - Power up in LORX state"]
    pub pucr_lorx: crate::Reg<pucr_lorx::PUCR_LORX_SPEC>,
    #[doc = "0x110 - Power up in TX state"]
    pub pucr_tx: crate::Reg<pucr_tx::PUCR_TX_SPEC>,
    #[doc = "0x114 - Power up in RX state"]
    pub pucr_rx: crate::Reg<pucr_rx::PUCR_RX_SPEC>,
    #[doc = "0x118 - Hardware read back of power up signals"]
    pub pucr_hw: crate::Reg<pucr_hw::PUCR_HW_SPEC>,
    #[doc = "0x11c - non_reg_readback."]
    pub non_reg_readback: crate::Reg<non_reg_readback::NON_REG_READBACK_SPEC>,
    #[doc = "0x120 - Register control of TX/RX gain"]
    pub trx_gain_bw: crate::Reg<trx_gain_bw::TRX_GAIN_BW_SPEC>,
    #[doc = "0x124 - Hardware read back of TX/RX gain"]
    pub trx_gain_bw_hw: crate::Reg<trx_gain_bw_hw::TRX_GAIN_BW_HW_SPEC>,
    #[doc = "0x128 - DC Test register 0"]
    pub dctest_actest: crate::Reg<dctest_actest::DCTEST_ACTEST_SPEC>,
    #[doc = "0x12c - LO Bias Mode registers"]
    pub dtest: crate::Reg<dtest::DTEST_SPEC>,
    #[doc = "0x130 - adpll_test."]
    pub adpll_test: crate::Reg<adpll_test::ADPLL_TEST_SPEC>,
    #[doc = "0x134 - rf_ext_pa."]
    pub rf_ext_pa: crate::Reg<rf_ext_pa::RF_EXT_PA_SPEC>,
    _reserved21: [u8; 0xc8],
    #[doc = "0x200 - cip_ldo15."]
    pub cip_ldo15: crate::Reg<cip_ldo15::CIP_LDO15_SPEC>,
    #[doc = "0x204 - PA register"]
    pub pa: crate::Reg<pa::PA_SPEC>,
    #[doc = "0x208 - LNA mixer register"]
    pub lna_mx: crate::Reg<lna_mx::LNA_MX_SPEC>,
    #[doc = "0x20c - rbb_rosdac."]
    pub rbb_rosdac: crate::Reg<rbb_rosdac::RBB_ROSDAC_SPEC>,
    #[doc = "0x210 - rbb_cap_1."]
    pub rbb_cap_1: crate::Reg<rbb_cap_1::RBB_CAP_1_SPEC>,
    #[doc = "0x214 - rbb_cap_2."]
    pub rbb_cap_2: crate::Reg<rbb_cap_2::RBB_CAP_2_SPEC>,
    #[doc = "0x218 - rbb_cap_3."]
    pub rbb_cap_3: crate::Reg<rbb_cap_3::RBB_CAP_3_SPEC>,
    #[doc = "0x21c - rbb_cap4."]
    pub rbb_cap4: crate::Reg<rbb_cap4::RBB_CAP4_SPEC>,
    #[doc = "0x220 - rbb_rx."]
    pub rbb_rx: crate::Reg<rbb_rx::RBB_RX_SPEC>,
    #[doc = "0x224 - rbb."]
    pub rbb: crate::Reg<rbb::RBB_SPEC>,
    #[doc = "0x228 - rxadc."]
    pub rxadc: crate::Reg<rxadc::RXADC_SPEC>,
    #[doc = "0x22c - rxadc_readback."]
    pub rxadc_readback: crate::Reg<rxadc_readback::RXADC_READBACK_SPEC>,
    #[doc = "0x230 - rf_adc_osdata."]
    pub rf_adc_osdata: crate::Reg<rf_adc_osdata::RF_ADC_OSDATA_SPEC>,
    #[doc = "0x234 - testbuf."]
    pub testbuf: crate::Reg<testbuf::TESTBUF_SPEC>,
    #[doc = "0x238 - vco."]
    pub vco: crate::Reg<vco::VCO_SPEC>,
    #[doc = "0x23c - lodist."]
    pub lodist: crate::Reg<lodist::LODIST_SPEC>,
    #[doc = "0x240 - fbdv."]
    pub fbdv: crate::Reg<fbdv::FBDV_SPEC>,
    #[doc = "0x244 - kcal1."]
    pub kcal1: crate::Reg<kcal1::KCAL1_SPEC>,
    #[doc = "0x248 - kcal2."]
    pub kcal2: crate::Reg<kcal2::KCAL2_SPEC>,
    #[doc = "0x24c - adpll_slope_gen."]
    pub adpll_slope_gen: crate::Reg<adpll_slope_gen::ADPLL_SLOPE_GEN_SPEC>,
    #[doc = "0x250 - adpll_adc."]
    pub adpll_adc: crate::Reg<adpll_adc::ADPLL_ADC_SPEC>,
    #[doc = "0x254 - adpll_dtc."]
    pub adpll_dtc: crate::Reg<adpll_dtc::ADPLL_DTC_SPEC>,
    #[doc = "0x258 - lo_fc_config1."]
    pub lo_fc_config1: crate::Reg<lo_fc_config1::LO_FC_CONFIG1_SPEC>,
    #[doc = "0x25c - lo_fcw_config2."]
    pub lo_fcw_config2: crate::Reg<lo_fcw_config2::LO_FCW_CONFIG2_SPEC>,
    #[doc = "0x260 - lo_fcw3."]
    pub lo_fcw3: crate::Reg<lo_fcw3::LO_FCW3_SPEC>,
    #[doc = "0x264 - lotpm."]
    pub lotpm: crate::Reg<lotpm::LOTPM_SPEC>,
    #[doc = "0x268 - adpll1."]
    pub adpll1: crate::Reg<adpll1::ADPLL1_SPEC>,
    #[doc = "0x26c - adpll_lf_reg."]
    pub adpll_lf_reg: crate::Reg<adpll_lf_reg::ADPLL_LF_REG_SPEC>,
    #[doc = "0x270 - adpll_lf_tx."]
    pub adpll_lf_tx: crate::Reg<adpll_lf_tx::ADPLL_LF_TX_SPEC>,
    #[doc = "0x274 - adpll_lf_rx."]
    pub adpll_lf_rx: crate::Reg<adpll_lf_rx::ADPLL_LF_RX_SPEC>,
    #[doc = "0x278 - adpll_lf_hw."]
    pub adpll_lf_hw: crate::Reg<adpll_lf_hw::ADPLL_LF_HW_SPEC>,
    #[doc = "0x27c - adpll_vctrl."]
    pub adpll_vctrl: crate::Reg<adpll_vctrl::ADPLL_VCTRL_SPEC>,
    #[doc = "0x280 - adpll_lms."]
    pub adpll_lms: crate::Reg<adpll_lms::ADPLL_LMS_SPEC>,
    #[doc = "0x284 - adpll_spd."]
    pub adpll_spd: crate::Reg<adpll_spd::ADPLL_SPD_SPEC>,
    #[doc = "0x288 - fcal."]
    pub fcal: crate::Reg<fcal::FCAL_SPEC>,
    #[doc = "0x28c - adpll_polarity."]
    pub adpll_polarity: crate::Reg<adpll_polarity::ADPLL_POLARITY_SPEC>,
    #[doc = "0x290 - adpll_output."]
    pub adpll_output: crate::Reg<adpll_output::ADPLL_OUTPUT_SPEC>,
    #[doc = "0x294 - adpll_reserved."]
    pub adpll_reserved: crate::Reg<adpll_reserved::ADPLL_RESERVED_SPEC>,
    #[doc = "0x298 - rf_reserved."]
    pub rf_reserved: crate::Reg<rf_reserved::RF_RESERVED_SPEC>,
    #[doc = "0x29c - rf_reserved_2."]
    pub rf_reserved_2: crate::Reg<rf_reserved_2::RF_RESERVED_2_SPEC>,
    _reserved61: [u8; 0x60],
    #[doc = "0x300 - rbb_gain_ctrl0."]
    pub rbb_gain_ctrl0: crate::Reg<rbb_gain_ctrl0::RBB_GAIN_CTRL0_SPEC>,
    #[doc = "0x304 - rbb_gain_ctrl1."]
    pub rbb_gain_ctrl1: crate::Reg<rbb_gain_ctrl1::RBB_GAIN_CTRL1_SPEC>,
    #[doc = "0x308 - rbb_gain_ctrl2."]
    pub rbb_gain_ctrl2: crate::Reg<rbb_gain_ctrl2::RBB_GAIN_CTRL2_SPEC>,
    #[doc = "0x30c - rbb_gain_ctrl3."]
    pub rbb_gain_ctrl3: crate::Reg<rbb_gain_ctrl3::RBB_GAIN_CTRL3_SPEC>,
    #[doc = "0x310 - rbb_gain_ctrl4."]
    pub rbb_gain_ctrl4: crate::Reg<rbb_gain_ctrl4::RBB_GAIN_CTRL4_SPEC>,
    #[doc = "0x314 - rbb_gain_ctrl5."]
    pub rbb_gain_ctrl5: crate::Reg<rbb_gain_ctrl5::RBB_GAIN_CTRL5_SPEC>,
    #[doc = "0x318 - rbb_gain_ctrl6."]
    pub rbb_gain_ctrl6: crate::Reg<rbb_gain_ctrl6::RBB_GAIN_CTRL6_SPEC>,
    #[doc = "0x31c - rbb_gain_ctrl7."]
    pub rbb_gain_ctrl7: crate::Reg<rbb_gain_ctrl7::RBB_GAIN_CTRL7_SPEC>,
    #[doc = "0x320 - rbb_gain_ctrl8."]
    pub rbb_gain_ctrl8: crate::Reg<rbb_gain_ctrl8::RBB_GAIN_CTRL8_SPEC>,
    #[doc = "0x324 - rbb_gain_ctrl9."]
    pub rbb_gain_ctrl9: crate::Reg<rbb_gain_ctrl9::RBB_GAIN_CTRL9_SPEC>,
    #[doc = "0x328 - rbb_gain_ctrl10."]
    pub rbb_gain_ctrl10: crate::Reg<rbb_gain_ctrl10::RBB_GAIN_CTRL10_SPEC>,
    #[doc = "0x32c - rbb_gain_ctrl11."]
    pub rbb_gain_ctrl11: crate::Reg<rbb_gain_ctrl11::RBB_GAIN_CTRL11_SPEC>,
    #[doc = "0x330 - rbb_gain_ctrl12."]
    pub rbb_gain_ctrl12: crate::Reg<rbb_gain_ctrl12::RBB_GAIN_CTRL12_SPEC>,
    #[doc = "0x334 - rbb_gain_ctrl13."]
    pub rbb_gain_ctrl13: crate::Reg<rbb_gain_ctrl13::RBB_GAIN_CTRL13_SPEC>,
    #[doc = "0x338 - rbb_gain_ctrl14."]
    pub rbb_gain_ctrl14: crate::Reg<rbb_gain_ctrl14::RBB_GAIN_CTRL14_SPEC>,
    #[doc = "0x33c - rbb_gain_ctrl15."]
    pub rbb_gain_ctrl15: crate::Reg<rbb_gain_ctrl15::RBB_GAIN_CTRL15_SPEC>,
    _reserved77: [u8; 0xc0],
    #[doc = "0x400 - acal_config."]
    pub acal_config: crate::Reg<acal_config::ACAL_CONFIG_SPEC>,
    #[doc = "0x404 - lo_config_2402."]
    pub lo_config_2402: crate::Reg<lo_config_2402::LO_CONFIG_2402_SPEC>,
    #[doc = "0x408 - lo_config_2404."]
    pub lo_config_2404: crate::Reg<lo_config_2404::LO_CONFIG_2404_SPEC>,
    #[doc = "0x40c - lo_config_2406."]
    pub lo_config_2406: crate::Reg<lo_config_2406::LO_CONFIG_2406_SPEC>,
    #[doc = "0x410 - lo_config_2408."]
    pub lo_config_2408: crate::Reg<lo_config_2408::LO_CONFIG_2408_SPEC>,
    #[doc = "0x414 - lo_config_2410."]
    pub lo_config_2410: crate::Reg<lo_config_2410::LO_CONFIG_2410_SPEC>,
    #[doc = "0x418 - lo_config_2412."]
    pub lo_config_2412: crate::Reg<lo_config_2412::LO_CONFIG_2412_SPEC>,
    #[doc = "0x41c - lo_config_2414."]
    pub lo_config_2414: crate::Reg<lo_config_2414::LO_CONFIG_2414_SPEC>,
    #[doc = "0x420 - lo_config_2416."]
    pub lo_config_2416: crate::Reg<lo_config_2416::LO_CONFIG_2416_SPEC>,
    #[doc = "0x424 - lo_config_2418."]
    pub lo_config_2418: crate::Reg<lo_config_2418::LO_CONFIG_2418_SPEC>,
    #[doc = "0x428 - lo_config_2420."]
    pub lo_config_2420: crate::Reg<lo_config_2420::LO_CONFIG_2420_SPEC>,
    #[doc = "0x42c - lo_config_2422."]
    pub lo_config_2422: crate::Reg<lo_config_2422::LO_CONFIG_2422_SPEC>,
    #[doc = "0x430 - lo_config_2424."]
    pub lo_config_2424: crate::Reg<lo_config_2424::LO_CONFIG_2424_SPEC>,
    #[doc = "0x434 - lo_config_2426."]
    pub lo_config_2426: crate::Reg<lo_config_2426::LO_CONFIG_2426_SPEC>,
    #[doc = "0x438 - lo_config_2428."]
    pub lo_config_2428: crate::Reg<lo_config_2428::LO_CONFIG_2428_SPEC>,
    #[doc = "0x43c - lo_config_2430."]
    pub lo_config_2430: crate::Reg<lo_config_2430::LO_CONFIG_2430_SPEC>,
    #[doc = "0x440 - lo_config_2432."]
    pub lo_config_2432: crate::Reg<lo_config_2432::LO_CONFIG_2432_SPEC>,
    #[doc = "0x444 - lo_config_2434."]
    pub lo_config_2434: crate::Reg<lo_config_2434::LO_CONFIG_2434_SPEC>,
    #[doc = "0x448 - lo_config_2436."]
    pub lo_config_2436: crate::Reg<lo_config_2436::LO_CONFIG_2436_SPEC>,
    #[doc = "0x44c - lo_config_2438."]
    pub lo_config_2438: crate::Reg<lo_config_2438::LO_CONFIG_2438_SPEC>,
    #[doc = "0x450 - lo_config_2440."]
    pub lo_config_2440: crate::Reg<lo_config_2440::LO_CONFIG_2440_SPEC>,
    #[doc = "0x454 - lo_config_2442."]
    pub lo_config_2442: crate::Reg<lo_config_2442::LO_CONFIG_2442_SPEC>,
    #[doc = "0x458 - lo_config_2444."]
    pub lo_config_2444: crate::Reg<lo_config_2444::LO_CONFIG_2444_SPEC>,
    #[doc = "0x45c - lo_config_2446."]
    pub lo_config_2446: crate::Reg<lo_config_2446::LO_CONFIG_2446_SPEC>,
    #[doc = "0x460 - lo_config_2448."]
    pub lo_config_2448: crate::Reg<lo_config_2448::LO_CONFIG_2448_SPEC>,
    #[doc = "0x464 - lo_config_2450."]
    pub lo_config_2450: crate::Reg<lo_config_2450::LO_CONFIG_2450_SPEC>,
    #[doc = "0x468 - lo_config_2452."]
    pub lo_config_2452: crate::Reg<lo_config_2452::LO_CONFIG_2452_SPEC>,
    #[doc = "0x46c - lo_config_2454."]
    pub lo_config_2454: crate::Reg<lo_config_2454::LO_CONFIG_2454_SPEC>,
    #[doc = "0x470 - lo_config_2456."]
    pub lo_config_2456: crate::Reg<lo_config_2456::LO_CONFIG_2456_SPEC>,
    #[doc = "0x474 - lo_config_2458."]
    pub lo_config_2458: crate::Reg<lo_config_2458::LO_CONFIG_2458_SPEC>,
    #[doc = "0x478 - lo_config_2460."]
    pub lo_config_2460: crate::Reg<lo_config_2460::LO_CONFIG_2460_SPEC>,
    #[doc = "0x47c - lo_config_2462."]
    pub lo_config_2462: crate::Reg<lo_config_2462::LO_CONFIG_2462_SPEC>,
    #[doc = "0x480 - lo_config_2464."]
    pub lo_config_2464: crate::Reg<lo_config_2464::LO_CONFIG_2464_SPEC>,
    #[doc = "0x484 - lo_config_2466."]
    pub lo_config_2466: crate::Reg<lo_config_2466::LO_CONFIG_2466_SPEC>,
    #[doc = "0x488 - lo_config_2468."]
    pub lo_config_2468: crate::Reg<lo_config_2468::LO_CONFIG_2468_SPEC>,
    #[doc = "0x48c - lo_config_2470."]
    pub lo_config_2470: crate::Reg<lo_config_2470::LO_CONFIG_2470_SPEC>,
    #[doc = "0x490 - lo_config_2472."]
    pub lo_config_2472: crate::Reg<lo_config_2472::LO_CONFIG_2472_SPEC>,
    #[doc = "0x494 - lo_config_2474."]
    pub lo_config_2474: crate::Reg<lo_config_2474::LO_CONFIG_2474_SPEC>,
    #[doc = "0x498 - lo_config_2476."]
    pub lo_config_2476: crate::Reg<lo_config_2476::LO_CONFIG_2476_SPEC>,
    #[doc = "0x49c - lo_config_2478."]
    pub lo_config_2478: crate::Reg<lo_config_2478::LO_CONFIG_2478_SPEC>,
    #[doc = "0x4a0 - lo_config_2480."]
    pub lo_config_2480: crate::Reg<lo_config_2480::LO_CONFIG_2480_SPEC>,
    #[doc = "0x4a4 - lo_config_2405."]
    pub lo_config_2405: crate::Reg<lo_config_2405::LO_CONFIG_2405_SPEC>,
    #[doc = "0x4a8 - lo_config_2415."]
    pub lo_config_2415: crate::Reg<lo_config_2415::LO_CONFIG_2415_SPEC>,
    #[doc = "0x4ac - lo_config_2425."]
    pub lo_config_2425: crate::Reg<lo_config_2425::LO_CONFIG_2425_SPEC>,
    #[doc = "0x4b0 - lo_config_2435."]
    pub lo_config_2435: crate::Reg<lo_config_2435::LO_CONFIG_2435_SPEC>,
    #[doc = "0x4b4 - lo_config_2445."]
    pub lo_config_2445: crate::Reg<lo_config_2445::LO_CONFIG_2445_SPEC>,
    #[doc = "0x4b8 - lo_config_2455."]
    pub lo_config_2455: crate::Reg<lo_config_2455::LO_CONFIG_2455_SPEC>,
    #[doc = "0x4bc - lo_config_2465."]
    pub lo_config_2465: crate::Reg<lo_config_2465::LO_CONFIG_2465_SPEC>,
    #[doc = "0x4c0 - lo_config_2475."]
    pub lo_config_2475: crate::Reg<lo_config_2475::LO_CONFIG_2475_SPEC>,
    _reserved126: [u8; 0x3c],
    #[doc = "0x500 - dg_testbus_0."]
    pub dg_testbus_0: crate::Reg<dg_testbus_0::DG_TESTBUS_0_SPEC>,
    #[doc = "0x504 - dg_testbus_1."]
    pub dg_testbus_1: crate::Reg<dg_testbus_1::DG_TESTBUS_1_SPEC>,
    #[doc = "0x508 - dg_ppud_0."]
    pub dg_ppud_0: crate::Reg<dg_ppud_0::DG_PPUD_0_SPEC>,
    #[doc = "0x50c - rf_top."]
    pub rf_top: crate::Reg<rf_top::RF_TOP_SPEC>,
    #[doc = "0x510 - rf_fsm."]
    pub rf_fsm: crate::Reg<rf_fsm::RF_FSM_SPEC>,
    #[doc = "0x514 - rf_singen_0."]
    pub rf_singen_0: crate::Reg<rf_singen_0::RF_SINGEN_0_SPEC>,
    #[doc = "0x518 - rf_singen_1."]
    pub rf_singen_1: crate::Reg<rf_singen_1::RF_SINGEN_1_SPEC>,
    #[doc = "0x51c - rf_singen_2."]
    pub rf_singen_2: crate::Reg<rf_singen_2::RF_SINGEN_2_SPEC>,
    #[doc = "0x520 - rf_singen_3."]
    pub rf_singen_3: crate::Reg<rf_singen_3::RF_SINGEN_3_SPEC>,
    #[doc = "0x524 - rf_singen_4."]
    pub rf_singen_4: crate::Reg<rf_singen_4::RF_SINGEN_4_SPEC>,
    #[doc = "0x528 - rf_sram_ctrl0."]
    pub rf_sram_ctrl0: crate::Reg<rf_sram_ctrl0::RF_SRAM_CTRL0_SPEC>,
    #[doc = "0x52c - rf_sram_ctrl1."]
    pub rf_sram_ctrl1: crate::Reg<rf_sram_ctrl1::RF_SRAM_CTRL1_SPEC>,
    #[doc = "0x530 - rf_sram_ctrl2."]
    pub rf_sram_ctrl2: crate::Reg<rf_sram_ctrl2::RF_SRAM_CTRL2_SPEC>,
    #[doc = "0x534 - rf_test_mode."]
    pub rf_test_mode: crate::Reg<rf_test_mode::RF_TEST_MODE_SPEC>,
    #[doc = "0x538 - rf_rx_pulse_filter."]
    pub rf_rx_pulse_filter: crate::Reg<rf_rx_pulse_filter::RF_RX_PULSE_FILTER_SPEC>,
}
#[doc = "rf_rev register accessor: an alias for `Reg<RF_REV_SPEC>`"]
pub type RF_REV = crate::Reg<rf_rev::RF_REV_SPEC>;
#[doc = "Silicon revision"]
pub mod rf_rev;
#[doc = "dsp_readback register accessor: an alias for `Reg<DSP_READBACK_SPEC>`"]
pub type DSP_READBACK = crate::Reg<dsp_readback::DSP_READBACK_SPEC>;
#[doc = "dsp_readback."]
pub mod dsp_readback;
#[doc = "rf_ctrl_source register accessor: an alias for `Reg<RF_CTRL_SOURCE_SPEC>`"]
pub type RF_CTRL_SOURCE = crate::Reg<rf_ctrl_source::RF_CTRL_SOURCE_SPEC>;
#[doc = "Control logic switch"]
pub mod rf_ctrl_source;
#[doc = "rf_cal_state_ctrl register accessor: an alias for `Reg<RF_CAL_STATE_CTRL_SPEC>`"]
pub type RF_CAL_STATE_CTRL = crate::Reg<rf_cal_state_ctrl::RF_CAL_STATE_CTRL_SPEC>;
#[doc = "rf calibration state enable in full cal list"]
pub mod rf_cal_state_ctrl;
#[doc = "rf_cal_switch_ctrl register accessor: an alias for `Reg<RF_CAL_SWITCH_CTRL_SPEC>`"]
pub type RF_CAL_SWITCH_CTRL = crate::Reg<rf_cal_switch_ctrl::RF_CAL_SWITCH_CTRL_SPEC>;
#[doc = "Calibration mode register"]
pub mod rf_cal_switch_ctrl;
#[doc = "rf_cal_status register accessor: an alias for `Reg<RF_CAL_STATUS_SPEC>`"]
pub type RF_CAL_STATUS = crate::Reg<rf_cal_status::RF_CAL_STATUS_SPEC>;
#[doc = "rf_cal_status."]
pub mod rf_cal_status;
#[doc = "pu_delay_confg register accessor: an alias for `Reg<PU_DELAY_CONFG_SPEC>`"]
pub type PU_DELAY_CONFG = crate::Reg<pu_delay_confg::PU_DELAY_CONFG_SPEC>;
#[doc = "pu_delay_confg."]
pub mod pu_delay_confg;
#[doc = "pucr_reg register accessor: an alias for `Reg<PUCR_REG_SPEC>`"]
pub type PUCR_REG = crate::Reg<pucr_reg::PUCR_REG_SPEC>;
#[doc = "Register control of power up signals"]
pub mod pucr_reg;
#[doc = "pucr_sb register accessor: an alias for `Reg<PUCR_SB_SPEC>`"]
pub type PUCR_SB = crate::Reg<pucr_sb::PUCR_SB_SPEC>;
#[doc = "Power up setting in SB state"]
pub mod pucr_sb;
#[doc = "pucr_lotx register accessor: an alias for `Reg<PUCR_LOTX_SPEC>`"]
pub type PUCR_LOTX = crate::Reg<pucr_lotx::PUCR_LOTX_SPEC>;
#[doc = "Power up in LOTX state"]
pub mod pucr_lotx;
#[doc = "pucr_lorx register accessor: an alias for `Reg<PUCR_LORX_SPEC>`"]
pub type PUCR_LORX = crate::Reg<pucr_lorx::PUCR_LORX_SPEC>;
#[doc = "Power up in LORX state"]
pub mod pucr_lorx;
#[doc = "pucr_tx register accessor: an alias for `Reg<PUCR_TX_SPEC>`"]
pub type PUCR_TX = crate::Reg<pucr_tx::PUCR_TX_SPEC>;
#[doc = "Power up in TX state"]
pub mod pucr_tx;
#[doc = "pucr_rx register accessor: an alias for `Reg<PUCR_RX_SPEC>`"]
pub type PUCR_RX = crate::Reg<pucr_rx::PUCR_RX_SPEC>;
#[doc = "Power up in RX state"]
pub mod pucr_rx;
#[doc = "pucr_hw register accessor: an alias for `Reg<PUCR_HW_SPEC>`"]
pub type PUCR_HW = crate::Reg<pucr_hw::PUCR_HW_SPEC>;
#[doc = "Hardware read back of power up signals"]
pub mod pucr_hw;
#[doc = "non_reg_readback register accessor: an alias for `Reg<NON_REG_READBACK_SPEC>`"]
pub type NON_REG_READBACK = crate::Reg<non_reg_readback::NON_REG_READBACK_SPEC>;
#[doc = "non_reg_readback."]
pub mod non_reg_readback;
#[doc = "trx_gain_bw register accessor: an alias for `Reg<TRX_GAIN_BW_SPEC>`"]
pub type TRX_GAIN_BW = crate::Reg<trx_gain_bw::TRX_GAIN_BW_SPEC>;
#[doc = "Register control of TX/RX gain"]
pub mod trx_gain_bw;
#[doc = "trx_gain_bw_hw register accessor: an alias for `Reg<TRX_GAIN_BW_HW_SPEC>`"]
pub type TRX_GAIN_BW_HW = crate::Reg<trx_gain_bw_hw::TRX_GAIN_BW_HW_SPEC>;
#[doc = "Hardware read back of TX/RX gain"]
pub mod trx_gain_bw_hw;
#[doc = "dctest_actest register accessor: an alias for `Reg<DCTEST_ACTEST_SPEC>`"]
pub type DCTEST_ACTEST = crate::Reg<dctest_actest::DCTEST_ACTEST_SPEC>;
#[doc = "DC Test register 0"]
pub mod dctest_actest;
#[doc = "dtest register accessor: an alias for `Reg<DTEST_SPEC>`"]
pub type DTEST = crate::Reg<dtest::DTEST_SPEC>;
#[doc = "LO Bias Mode registers"]
pub mod dtest;
#[doc = "adpll_test register accessor: an alias for `Reg<ADPLL_TEST_SPEC>`"]
pub type ADPLL_TEST = crate::Reg<adpll_test::ADPLL_TEST_SPEC>;
#[doc = "adpll_test."]
pub mod adpll_test;
#[doc = "rf_ext_pa register accessor: an alias for `Reg<RF_EXT_PA_SPEC>`"]
pub type RF_EXT_PA = crate::Reg<rf_ext_pa::RF_EXT_PA_SPEC>;
#[doc = "rf_ext_pa."]
pub mod rf_ext_pa;
#[doc = "cip_ldo15 register accessor: an alias for `Reg<CIP_LDO15_SPEC>`"]
pub type CIP_LDO15 = crate::Reg<cip_ldo15::CIP_LDO15_SPEC>;
#[doc = "cip_ldo15."]
pub mod cip_ldo15;
#[doc = "pa register accessor: an alias for `Reg<PA_SPEC>`"]
pub type PA = crate::Reg<pa::PA_SPEC>;
#[doc = "PA register"]
pub mod pa;
#[doc = "lna_mx register accessor: an alias for `Reg<LNA_MX_SPEC>`"]
pub type LNA_MX = crate::Reg<lna_mx::LNA_MX_SPEC>;
#[doc = "LNA mixer register"]
pub mod lna_mx;
#[doc = "rbb_rosdac register accessor: an alias for `Reg<RBB_ROSDAC_SPEC>`"]
pub type RBB_ROSDAC = crate::Reg<rbb_rosdac::RBB_ROSDAC_SPEC>;
#[doc = "rbb_rosdac."]
pub mod rbb_rosdac;
#[doc = "rbb_cap_1 register accessor: an alias for `Reg<RBB_CAP_1_SPEC>`"]
pub type RBB_CAP_1 = crate::Reg<rbb_cap_1::RBB_CAP_1_SPEC>;
#[doc = "rbb_cap_1."]
pub mod rbb_cap_1;
#[doc = "rbb_cap_2 register accessor: an alias for `Reg<RBB_CAP_2_SPEC>`"]
pub type RBB_CAP_2 = crate::Reg<rbb_cap_2::RBB_CAP_2_SPEC>;
#[doc = "rbb_cap_2."]
pub mod rbb_cap_2;
#[doc = "rbb_cap_3 register accessor: an alias for `Reg<RBB_CAP_3_SPEC>`"]
pub type RBB_CAP_3 = crate::Reg<rbb_cap_3::RBB_CAP_3_SPEC>;
#[doc = "rbb_cap_3."]
pub mod rbb_cap_3;
#[doc = "rbb_cap4 register accessor: an alias for `Reg<RBB_CAP4_SPEC>`"]
pub type RBB_CAP4 = crate::Reg<rbb_cap4::RBB_CAP4_SPEC>;
#[doc = "rbb_cap4."]
pub mod rbb_cap4;
#[doc = "rbb_rx register accessor: an alias for `Reg<RBB_RX_SPEC>`"]
pub type RBB_RX = crate::Reg<rbb_rx::RBB_RX_SPEC>;
#[doc = "rbb_rx."]
pub mod rbb_rx;
#[doc = "rbb register accessor: an alias for `Reg<RBB_SPEC>`"]
pub type RBB = crate::Reg<rbb::RBB_SPEC>;
#[doc = "rbb."]
pub mod rbb;
#[doc = "rxadc register accessor: an alias for `Reg<RXADC_SPEC>`"]
pub type RXADC = crate::Reg<rxadc::RXADC_SPEC>;
#[doc = "rxadc."]
pub mod rxadc;
#[doc = "rxadc_readback register accessor: an alias for `Reg<RXADC_READBACK_SPEC>`"]
pub type RXADC_READBACK = crate::Reg<rxadc_readback::RXADC_READBACK_SPEC>;
#[doc = "rxadc_readback."]
pub mod rxadc_readback;
#[doc = "rf_adc_osdata register accessor: an alias for `Reg<RF_ADC_OSDATA_SPEC>`"]
pub type RF_ADC_OSDATA = crate::Reg<rf_adc_osdata::RF_ADC_OSDATA_SPEC>;
#[doc = "rf_adc_osdata."]
pub mod rf_adc_osdata;
#[doc = "testbuf register accessor: an alias for `Reg<TESTBUF_SPEC>`"]
pub type TESTBUF = crate::Reg<testbuf::TESTBUF_SPEC>;
#[doc = "testbuf."]
pub mod testbuf;
#[doc = "vco register accessor: an alias for `Reg<VCO_SPEC>`"]
pub type VCO = crate::Reg<vco::VCO_SPEC>;
#[doc = "vco."]
pub mod vco;
#[doc = "lodist register accessor: an alias for `Reg<LODIST_SPEC>`"]
pub type LODIST = crate::Reg<lodist::LODIST_SPEC>;
#[doc = "lodist."]
pub mod lodist;
#[doc = "fbdv register accessor: an alias for `Reg<FBDV_SPEC>`"]
pub type FBDV = crate::Reg<fbdv::FBDV_SPEC>;
#[doc = "fbdv."]
pub mod fbdv;
#[doc = "kcal1 register accessor: an alias for `Reg<KCAL1_SPEC>`"]
pub type KCAL1 = crate::Reg<kcal1::KCAL1_SPEC>;
#[doc = "kcal1."]
pub mod kcal1;
#[doc = "kcal2 register accessor: an alias for `Reg<KCAL2_SPEC>`"]
pub type KCAL2 = crate::Reg<kcal2::KCAL2_SPEC>;
#[doc = "kcal2."]
pub mod kcal2;
#[doc = "adpll_slope_gen register accessor: an alias for `Reg<ADPLL_SLOPE_GEN_SPEC>`"]
pub type ADPLL_SLOPE_GEN = crate::Reg<adpll_slope_gen::ADPLL_SLOPE_GEN_SPEC>;
#[doc = "adpll_slope_gen."]
pub mod adpll_slope_gen;
#[doc = "adpll_adc register accessor: an alias for `Reg<ADPLL_ADC_SPEC>`"]
pub type ADPLL_ADC = crate::Reg<adpll_adc::ADPLL_ADC_SPEC>;
#[doc = "adpll_adc."]
pub mod adpll_adc;
#[doc = "adpll_dtc register accessor: an alias for `Reg<ADPLL_DTC_SPEC>`"]
pub type ADPLL_DTC = crate::Reg<adpll_dtc::ADPLL_DTC_SPEC>;
#[doc = "adpll_dtc."]
pub mod adpll_dtc;
#[doc = "lo_fc_config1 register accessor: an alias for `Reg<LO_FC_CONFIG1_SPEC>`"]
pub type LO_FC_CONFIG1 = crate::Reg<lo_fc_config1::LO_FC_CONFIG1_SPEC>;
#[doc = "lo_fc_config1."]
pub mod lo_fc_config1;
#[doc = "lo_fcw_config2 register accessor: an alias for `Reg<LO_FCW_CONFIG2_SPEC>`"]
pub type LO_FCW_CONFIG2 = crate::Reg<lo_fcw_config2::LO_FCW_CONFIG2_SPEC>;
#[doc = "lo_fcw_config2."]
pub mod lo_fcw_config2;
#[doc = "lo_fcw3 register accessor: an alias for `Reg<LO_FCW3_SPEC>`"]
pub type LO_FCW3 = crate::Reg<lo_fcw3::LO_FCW3_SPEC>;
#[doc = "lo_fcw3."]
pub mod lo_fcw3;
#[doc = "lotpm register accessor: an alias for `Reg<LOTPM_SPEC>`"]
pub type LOTPM = crate::Reg<lotpm::LOTPM_SPEC>;
#[doc = "lotpm."]
pub mod lotpm;
#[doc = "adpll1 register accessor: an alias for `Reg<ADPLL1_SPEC>`"]
pub type ADPLL1 = crate::Reg<adpll1::ADPLL1_SPEC>;
#[doc = "adpll1."]
pub mod adpll1;
#[doc = "adpll_lf_reg register accessor: an alias for `Reg<ADPLL_LF_REG_SPEC>`"]
pub type ADPLL_LF_REG = crate::Reg<adpll_lf_reg::ADPLL_LF_REG_SPEC>;
#[doc = "adpll_lf_reg."]
pub mod adpll_lf_reg;
#[doc = "adpll_lf_tx register accessor: an alias for `Reg<ADPLL_LF_TX_SPEC>`"]
pub type ADPLL_LF_TX = crate::Reg<adpll_lf_tx::ADPLL_LF_TX_SPEC>;
#[doc = "adpll_lf_tx."]
pub mod adpll_lf_tx;
#[doc = "adpll_lf_rx register accessor: an alias for `Reg<ADPLL_LF_RX_SPEC>`"]
pub type ADPLL_LF_RX = crate::Reg<adpll_lf_rx::ADPLL_LF_RX_SPEC>;
#[doc = "adpll_lf_rx."]
pub mod adpll_lf_rx;
#[doc = "adpll_lf_hw register accessor: an alias for `Reg<ADPLL_LF_HW_SPEC>`"]
pub type ADPLL_LF_HW = crate::Reg<adpll_lf_hw::ADPLL_LF_HW_SPEC>;
#[doc = "adpll_lf_hw."]
pub mod adpll_lf_hw;
#[doc = "adpll_vctrl register accessor: an alias for `Reg<ADPLL_VCTRL_SPEC>`"]
pub type ADPLL_VCTRL = crate::Reg<adpll_vctrl::ADPLL_VCTRL_SPEC>;
#[doc = "adpll_vctrl."]
pub mod adpll_vctrl;
#[doc = "adpll_lms register accessor: an alias for `Reg<ADPLL_LMS_SPEC>`"]
pub type ADPLL_LMS = crate::Reg<adpll_lms::ADPLL_LMS_SPEC>;
#[doc = "adpll_lms."]
pub mod adpll_lms;
#[doc = "adpll_spd register accessor: an alias for `Reg<ADPLL_SPD_SPEC>`"]
pub type ADPLL_SPD = crate::Reg<adpll_spd::ADPLL_SPD_SPEC>;
#[doc = "adpll_spd."]
pub mod adpll_spd;
#[doc = "fcal register accessor: an alias for `Reg<FCAL_SPEC>`"]
pub type FCAL = crate::Reg<fcal::FCAL_SPEC>;
#[doc = "fcal."]
pub mod fcal;
#[doc = "adpll_polarity register accessor: an alias for `Reg<ADPLL_POLARITY_SPEC>`"]
pub type ADPLL_POLARITY = crate::Reg<adpll_polarity::ADPLL_POLARITY_SPEC>;
#[doc = "adpll_polarity."]
pub mod adpll_polarity;
#[doc = "adpll_output register accessor: an alias for `Reg<ADPLL_OUTPUT_SPEC>`"]
pub type ADPLL_OUTPUT = crate::Reg<adpll_output::ADPLL_OUTPUT_SPEC>;
#[doc = "adpll_output."]
pub mod adpll_output;
#[doc = "adpll_reserved register accessor: an alias for `Reg<ADPLL_RESERVED_SPEC>`"]
pub type ADPLL_RESERVED = crate::Reg<adpll_reserved::ADPLL_RESERVED_SPEC>;
#[doc = "adpll_reserved."]
pub mod adpll_reserved;
#[doc = "rf_reserved register accessor: an alias for `Reg<RF_RESERVED_SPEC>`"]
pub type RF_RESERVED = crate::Reg<rf_reserved::RF_RESERVED_SPEC>;
#[doc = "rf_reserved."]
pub mod rf_reserved;
#[doc = "rf_reserved_2 register accessor: an alias for `Reg<RF_RESERVED_2_SPEC>`"]
pub type RF_RESERVED_2 = crate::Reg<rf_reserved_2::RF_RESERVED_2_SPEC>;
#[doc = "rf_reserved_2."]
pub mod rf_reserved_2;
#[doc = "rbb_gain_ctrl0 register accessor: an alias for `Reg<RBB_GAIN_CTRL0_SPEC>`"]
pub type RBB_GAIN_CTRL0 = crate::Reg<rbb_gain_ctrl0::RBB_GAIN_CTRL0_SPEC>;
#[doc = "rbb_gain_ctrl0."]
pub mod rbb_gain_ctrl0;
#[doc = "rbb_gain_ctrl1 register accessor: an alias for `Reg<RBB_GAIN_CTRL1_SPEC>`"]
pub type RBB_GAIN_CTRL1 = crate::Reg<rbb_gain_ctrl1::RBB_GAIN_CTRL1_SPEC>;
#[doc = "rbb_gain_ctrl1."]
pub mod rbb_gain_ctrl1;
#[doc = "rbb_gain_ctrl2 register accessor: an alias for `Reg<RBB_GAIN_CTRL2_SPEC>`"]
pub type RBB_GAIN_CTRL2 = crate::Reg<rbb_gain_ctrl2::RBB_GAIN_CTRL2_SPEC>;
#[doc = "rbb_gain_ctrl2."]
pub mod rbb_gain_ctrl2;
#[doc = "rbb_gain_ctrl3 register accessor: an alias for `Reg<RBB_GAIN_CTRL3_SPEC>`"]
pub type RBB_GAIN_CTRL3 = crate::Reg<rbb_gain_ctrl3::RBB_GAIN_CTRL3_SPEC>;
#[doc = "rbb_gain_ctrl3."]
pub mod rbb_gain_ctrl3;
#[doc = "rbb_gain_ctrl4 register accessor: an alias for `Reg<RBB_GAIN_CTRL4_SPEC>`"]
pub type RBB_GAIN_CTRL4 = crate::Reg<rbb_gain_ctrl4::RBB_GAIN_CTRL4_SPEC>;
#[doc = "rbb_gain_ctrl4."]
pub mod rbb_gain_ctrl4;
#[doc = "rbb_gain_ctrl5 register accessor: an alias for `Reg<RBB_GAIN_CTRL5_SPEC>`"]
pub type RBB_GAIN_CTRL5 = crate::Reg<rbb_gain_ctrl5::RBB_GAIN_CTRL5_SPEC>;
#[doc = "rbb_gain_ctrl5."]
pub mod rbb_gain_ctrl5;
#[doc = "rbb_gain_ctrl6 register accessor: an alias for `Reg<RBB_GAIN_CTRL6_SPEC>`"]
pub type RBB_GAIN_CTRL6 = crate::Reg<rbb_gain_ctrl6::RBB_GAIN_CTRL6_SPEC>;
#[doc = "rbb_gain_ctrl6."]
pub mod rbb_gain_ctrl6;
#[doc = "rbb_gain_ctrl7 register accessor: an alias for `Reg<RBB_GAIN_CTRL7_SPEC>`"]
pub type RBB_GAIN_CTRL7 = crate::Reg<rbb_gain_ctrl7::RBB_GAIN_CTRL7_SPEC>;
#[doc = "rbb_gain_ctrl7."]
pub mod rbb_gain_ctrl7;
#[doc = "rbb_gain_ctrl8 register accessor: an alias for `Reg<RBB_GAIN_CTRL8_SPEC>`"]
pub type RBB_GAIN_CTRL8 = crate::Reg<rbb_gain_ctrl8::RBB_GAIN_CTRL8_SPEC>;
#[doc = "rbb_gain_ctrl8."]
pub mod rbb_gain_ctrl8;
#[doc = "rbb_gain_ctrl9 register accessor: an alias for `Reg<RBB_GAIN_CTRL9_SPEC>`"]
pub type RBB_GAIN_CTRL9 = crate::Reg<rbb_gain_ctrl9::RBB_GAIN_CTRL9_SPEC>;
#[doc = "rbb_gain_ctrl9."]
pub mod rbb_gain_ctrl9;
#[doc = "rbb_gain_ctrl10 register accessor: an alias for `Reg<RBB_GAIN_CTRL10_SPEC>`"]
pub type RBB_GAIN_CTRL10 = crate::Reg<rbb_gain_ctrl10::RBB_GAIN_CTRL10_SPEC>;
#[doc = "rbb_gain_ctrl10."]
pub mod rbb_gain_ctrl10;
#[doc = "rbb_gain_ctrl11 register accessor: an alias for `Reg<RBB_GAIN_CTRL11_SPEC>`"]
pub type RBB_GAIN_CTRL11 = crate::Reg<rbb_gain_ctrl11::RBB_GAIN_CTRL11_SPEC>;
#[doc = "rbb_gain_ctrl11."]
pub mod rbb_gain_ctrl11;
#[doc = "rbb_gain_ctrl12 register accessor: an alias for `Reg<RBB_GAIN_CTRL12_SPEC>`"]
pub type RBB_GAIN_CTRL12 = crate::Reg<rbb_gain_ctrl12::RBB_GAIN_CTRL12_SPEC>;
#[doc = "rbb_gain_ctrl12."]
pub mod rbb_gain_ctrl12;
#[doc = "rbb_gain_ctrl13 register accessor: an alias for `Reg<RBB_GAIN_CTRL13_SPEC>`"]
pub type RBB_GAIN_CTRL13 = crate::Reg<rbb_gain_ctrl13::RBB_GAIN_CTRL13_SPEC>;
#[doc = "rbb_gain_ctrl13."]
pub mod rbb_gain_ctrl13;
#[doc = "rbb_gain_ctrl14 register accessor: an alias for `Reg<RBB_GAIN_CTRL14_SPEC>`"]
pub type RBB_GAIN_CTRL14 = crate::Reg<rbb_gain_ctrl14::RBB_GAIN_CTRL14_SPEC>;
#[doc = "rbb_gain_ctrl14."]
pub mod rbb_gain_ctrl14;
#[doc = "rbb_gain_ctrl15 register accessor: an alias for `Reg<RBB_GAIN_CTRL15_SPEC>`"]
pub type RBB_GAIN_CTRL15 = crate::Reg<rbb_gain_ctrl15::RBB_GAIN_CTRL15_SPEC>;
#[doc = "rbb_gain_ctrl15."]
pub mod rbb_gain_ctrl15;
#[doc = "acal_config register accessor: an alias for `Reg<ACAL_CONFIG_SPEC>`"]
pub type ACAL_CONFIG = crate::Reg<acal_config::ACAL_CONFIG_SPEC>;
#[doc = "acal_config."]
pub mod acal_config;
#[doc = "lo_config_2402 register accessor: an alias for `Reg<LO_CONFIG_2402_SPEC>`"]
pub type LO_CONFIG_2402 = crate::Reg<lo_config_2402::LO_CONFIG_2402_SPEC>;
#[doc = "lo_config_2402."]
pub mod lo_config_2402;
#[doc = "lo_config_2404 register accessor: an alias for `Reg<LO_CONFIG_2404_SPEC>`"]
pub type LO_CONFIG_2404 = crate::Reg<lo_config_2404::LO_CONFIG_2404_SPEC>;
#[doc = "lo_config_2404."]
pub mod lo_config_2404;
#[doc = "lo_config_2406 register accessor: an alias for `Reg<LO_CONFIG_2406_SPEC>`"]
pub type LO_CONFIG_2406 = crate::Reg<lo_config_2406::LO_CONFIG_2406_SPEC>;
#[doc = "lo_config_2406."]
pub mod lo_config_2406;
#[doc = "lo_config_2408 register accessor: an alias for `Reg<LO_CONFIG_2408_SPEC>`"]
pub type LO_CONFIG_2408 = crate::Reg<lo_config_2408::LO_CONFIG_2408_SPEC>;
#[doc = "lo_config_2408."]
pub mod lo_config_2408;
#[doc = "lo_config_2410 register accessor: an alias for `Reg<LO_CONFIG_2410_SPEC>`"]
pub type LO_CONFIG_2410 = crate::Reg<lo_config_2410::LO_CONFIG_2410_SPEC>;
#[doc = "lo_config_2410."]
pub mod lo_config_2410;
#[doc = "lo_config_2412 register accessor: an alias for `Reg<LO_CONFIG_2412_SPEC>`"]
pub type LO_CONFIG_2412 = crate::Reg<lo_config_2412::LO_CONFIG_2412_SPEC>;
#[doc = "lo_config_2412."]
pub mod lo_config_2412;
#[doc = "lo_config_2414 register accessor: an alias for `Reg<LO_CONFIG_2414_SPEC>`"]
pub type LO_CONFIG_2414 = crate::Reg<lo_config_2414::LO_CONFIG_2414_SPEC>;
#[doc = "lo_config_2414."]
pub mod lo_config_2414;
#[doc = "lo_config_2416 register accessor: an alias for `Reg<LO_CONFIG_2416_SPEC>`"]
pub type LO_CONFIG_2416 = crate::Reg<lo_config_2416::LO_CONFIG_2416_SPEC>;
#[doc = "lo_config_2416."]
pub mod lo_config_2416;
#[doc = "lo_config_2418 register accessor: an alias for `Reg<LO_CONFIG_2418_SPEC>`"]
pub type LO_CONFIG_2418 = crate::Reg<lo_config_2418::LO_CONFIG_2418_SPEC>;
#[doc = "lo_config_2418."]
pub mod lo_config_2418;
#[doc = "lo_config_2420 register accessor: an alias for `Reg<LO_CONFIG_2420_SPEC>`"]
pub type LO_CONFIG_2420 = crate::Reg<lo_config_2420::LO_CONFIG_2420_SPEC>;
#[doc = "lo_config_2420."]
pub mod lo_config_2420;
#[doc = "lo_config_2422 register accessor: an alias for `Reg<LO_CONFIG_2422_SPEC>`"]
pub type LO_CONFIG_2422 = crate::Reg<lo_config_2422::LO_CONFIG_2422_SPEC>;
#[doc = "lo_config_2422."]
pub mod lo_config_2422;
#[doc = "lo_config_2424 register accessor: an alias for `Reg<LO_CONFIG_2424_SPEC>`"]
pub type LO_CONFIG_2424 = crate::Reg<lo_config_2424::LO_CONFIG_2424_SPEC>;
#[doc = "lo_config_2424."]
pub mod lo_config_2424;
#[doc = "lo_config_2426 register accessor: an alias for `Reg<LO_CONFIG_2426_SPEC>`"]
pub type LO_CONFIG_2426 = crate::Reg<lo_config_2426::LO_CONFIG_2426_SPEC>;
#[doc = "lo_config_2426."]
pub mod lo_config_2426;
#[doc = "lo_config_2428 register accessor: an alias for `Reg<LO_CONFIG_2428_SPEC>`"]
pub type LO_CONFIG_2428 = crate::Reg<lo_config_2428::LO_CONFIG_2428_SPEC>;
#[doc = "lo_config_2428."]
pub mod lo_config_2428;
#[doc = "lo_config_2430 register accessor: an alias for `Reg<LO_CONFIG_2430_SPEC>`"]
pub type LO_CONFIG_2430 = crate::Reg<lo_config_2430::LO_CONFIG_2430_SPEC>;
#[doc = "lo_config_2430."]
pub mod lo_config_2430;
#[doc = "lo_config_2432 register accessor: an alias for `Reg<LO_CONFIG_2432_SPEC>`"]
pub type LO_CONFIG_2432 = crate::Reg<lo_config_2432::LO_CONFIG_2432_SPEC>;
#[doc = "lo_config_2432."]
pub mod lo_config_2432;
#[doc = "lo_config_2434 register accessor: an alias for `Reg<LO_CONFIG_2434_SPEC>`"]
pub type LO_CONFIG_2434 = crate::Reg<lo_config_2434::LO_CONFIG_2434_SPEC>;
#[doc = "lo_config_2434."]
pub mod lo_config_2434;
#[doc = "lo_config_2436 register accessor: an alias for `Reg<LO_CONFIG_2436_SPEC>`"]
pub type LO_CONFIG_2436 = crate::Reg<lo_config_2436::LO_CONFIG_2436_SPEC>;
#[doc = "lo_config_2436."]
pub mod lo_config_2436;
#[doc = "lo_config_2438 register accessor: an alias for `Reg<LO_CONFIG_2438_SPEC>`"]
pub type LO_CONFIG_2438 = crate::Reg<lo_config_2438::LO_CONFIG_2438_SPEC>;
#[doc = "lo_config_2438."]
pub mod lo_config_2438;
#[doc = "lo_config_2440 register accessor: an alias for `Reg<LO_CONFIG_2440_SPEC>`"]
pub type LO_CONFIG_2440 = crate::Reg<lo_config_2440::LO_CONFIG_2440_SPEC>;
#[doc = "lo_config_2440."]
pub mod lo_config_2440;
#[doc = "lo_config_2442 register accessor: an alias for `Reg<LO_CONFIG_2442_SPEC>`"]
pub type LO_CONFIG_2442 = crate::Reg<lo_config_2442::LO_CONFIG_2442_SPEC>;
#[doc = "lo_config_2442."]
pub mod lo_config_2442;
#[doc = "lo_config_2444 register accessor: an alias for `Reg<LO_CONFIG_2444_SPEC>`"]
pub type LO_CONFIG_2444 = crate::Reg<lo_config_2444::LO_CONFIG_2444_SPEC>;
#[doc = "lo_config_2444."]
pub mod lo_config_2444;
#[doc = "lo_config_2446 register accessor: an alias for `Reg<LO_CONFIG_2446_SPEC>`"]
pub type LO_CONFIG_2446 = crate::Reg<lo_config_2446::LO_CONFIG_2446_SPEC>;
#[doc = "lo_config_2446."]
pub mod lo_config_2446;
#[doc = "lo_config_2448 register accessor: an alias for `Reg<LO_CONFIG_2448_SPEC>`"]
pub type LO_CONFIG_2448 = crate::Reg<lo_config_2448::LO_CONFIG_2448_SPEC>;
#[doc = "lo_config_2448."]
pub mod lo_config_2448;
#[doc = "lo_config_2450 register accessor: an alias for `Reg<LO_CONFIG_2450_SPEC>`"]
pub type LO_CONFIG_2450 = crate::Reg<lo_config_2450::LO_CONFIG_2450_SPEC>;
#[doc = "lo_config_2450."]
pub mod lo_config_2450;
#[doc = "lo_config_2452 register accessor: an alias for `Reg<LO_CONFIG_2452_SPEC>`"]
pub type LO_CONFIG_2452 = crate::Reg<lo_config_2452::LO_CONFIG_2452_SPEC>;
#[doc = "lo_config_2452."]
pub mod lo_config_2452;
#[doc = "lo_config_2454 register accessor: an alias for `Reg<LO_CONFIG_2454_SPEC>`"]
pub type LO_CONFIG_2454 = crate::Reg<lo_config_2454::LO_CONFIG_2454_SPEC>;
#[doc = "lo_config_2454."]
pub mod lo_config_2454;
#[doc = "lo_config_2456 register accessor: an alias for `Reg<LO_CONFIG_2456_SPEC>`"]
pub type LO_CONFIG_2456 = crate::Reg<lo_config_2456::LO_CONFIG_2456_SPEC>;
#[doc = "lo_config_2456."]
pub mod lo_config_2456;
#[doc = "lo_config_2458 register accessor: an alias for `Reg<LO_CONFIG_2458_SPEC>`"]
pub type LO_CONFIG_2458 = crate::Reg<lo_config_2458::LO_CONFIG_2458_SPEC>;
#[doc = "lo_config_2458."]
pub mod lo_config_2458;
#[doc = "lo_config_2460 register accessor: an alias for `Reg<LO_CONFIG_2460_SPEC>`"]
pub type LO_CONFIG_2460 = crate::Reg<lo_config_2460::LO_CONFIG_2460_SPEC>;
#[doc = "lo_config_2460."]
pub mod lo_config_2460;
#[doc = "lo_config_2462 register accessor: an alias for `Reg<LO_CONFIG_2462_SPEC>`"]
pub type LO_CONFIG_2462 = crate::Reg<lo_config_2462::LO_CONFIG_2462_SPEC>;
#[doc = "lo_config_2462."]
pub mod lo_config_2462;
#[doc = "lo_config_2464 register accessor: an alias for `Reg<LO_CONFIG_2464_SPEC>`"]
pub type LO_CONFIG_2464 = crate::Reg<lo_config_2464::LO_CONFIG_2464_SPEC>;
#[doc = "lo_config_2464."]
pub mod lo_config_2464;
#[doc = "lo_config_2466 register accessor: an alias for `Reg<LO_CONFIG_2466_SPEC>`"]
pub type LO_CONFIG_2466 = crate::Reg<lo_config_2466::LO_CONFIG_2466_SPEC>;
#[doc = "lo_config_2466."]
pub mod lo_config_2466;
#[doc = "lo_config_2468 register accessor: an alias for `Reg<LO_CONFIG_2468_SPEC>`"]
pub type LO_CONFIG_2468 = crate::Reg<lo_config_2468::LO_CONFIG_2468_SPEC>;
#[doc = "lo_config_2468."]
pub mod lo_config_2468;
#[doc = "lo_config_2470 register accessor: an alias for `Reg<LO_CONFIG_2470_SPEC>`"]
pub type LO_CONFIG_2470 = crate::Reg<lo_config_2470::LO_CONFIG_2470_SPEC>;
#[doc = "lo_config_2470."]
pub mod lo_config_2470;
#[doc = "lo_config_2472 register accessor: an alias for `Reg<LO_CONFIG_2472_SPEC>`"]
pub type LO_CONFIG_2472 = crate::Reg<lo_config_2472::LO_CONFIG_2472_SPEC>;
#[doc = "lo_config_2472."]
pub mod lo_config_2472;
#[doc = "lo_config_2474 register accessor: an alias for `Reg<LO_CONFIG_2474_SPEC>`"]
pub type LO_CONFIG_2474 = crate::Reg<lo_config_2474::LO_CONFIG_2474_SPEC>;
#[doc = "lo_config_2474."]
pub mod lo_config_2474;
#[doc = "lo_config_2476 register accessor: an alias for `Reg<LO_CONFIG_2476_SPEC>`"]
pub type LO_CONFIG_2476 = crate::Reg<lo_config_2476::LO_CONFIG_2476_SPEC>;
#[doc = "lo_config_2476."]
pub mod lo_config_2476;
#[doc = "lo_config_2478 register accessor: an alias for `Reg<LO_CONFIG_2478_SPEC>`"]
pub type LO_CONFIG_2478 = crate::Reg<lo_config_2478::LO_CONFIG_2478_SPEC>;
#[doc = "lo_config_2478."]
pub mod lo_config_2478;
#[doc = "lo_config_2480 register accessor: an alias for `Reg<LO_CONFIG_2480_SPEC>`"]
pub type LO_CONFIG_2480 = crate::Reg<lo_config_2480::LO_CONFIG_2480_SPEC>;
#[doc = "lo_config_2480."]
pub mod lo_config_2480;
#[doc = "lo_config_2405 register accessor: an alias for `Reg<LO_CONFIG_2405_SPEC>`"]
pub type LO_CONFIG_2405 = crate::Reg<lo_config_2405::LO_CONFIG_2405_SPEC>;
#[doc = "lo_config_2405."]
pub mod lo_config_2405;
#[doc = "lo_config_2415 register accessor: an alias for `Reg<LO_CONFIG_2415_SPEC>`"]
pub type LO_CONFIG_2415 = crate::Reg<lo_config_2415::LO_CONFIG_2415_SPEC>;
#[doc = "lo_config_2415."]
pub mod lo_config_2415;
#[doc = "lo_config_2425 register accessor: an alias for `Reg<LO_CONFIG_2425_SPEC>`"]
pub type LO_CONFIG_2425 = crate::Reg<lo_config_2425::LO_CONFIG_2425_SPEC>;
#[doc = "lo_config_2425."]
pub mod lo_config_2425;
#[doc = "lo_config_2435 register accessor: an alias for `Reg<LO_CONFIG_2435_SPEC>`"]
pub type LO_CONFIG_2435 = crate::Reg<lo_config_2435::LO_CONFIG_2435_SPEC>;
#[doc = "lo_config_2435."]
pub mod lo_config_2435;
#[doc = "lo_config_2445 register accessor: an alias for `Reg<LO_CONFIG_2445_SPEC>`"]
pub type LO_CONFIG_2445 = crate::Reg<lo_config_2445::LO_CONFIG_2445_SPEC>;
#[doc = "lo_config_2445."]
pub mod lo_config_2445;
#[doc = "lo_config_2455 register accessor: an alias for `Reg<LO_CONFIG_2455_SPEC>`"]
pub type LO_CONFIG_2455 = crate::Reg<lo_config_2455::LO_CONFIG_2455_SPEC>;
#[doc = "lo_config_2455."]
pub mod lo_config_2455;
#[doc = "lo_config_2465 register accessor: an alias for `Reg<LO_CONFIG_2465_SPEC>`"]
pub type LO_CONFIG_2465 = crate::Reg<lo_config_2465::LO_CONFIG_2465_SPEC>;
#[doc = "lo_config_2465."]
pub mod lo_config_2465;
#[doc = "lo_config_2475 register accessor: an alias for `Reg<LO_CONFIG_2475_SPEC>`"]
pub type LO_CONFIG_2475 = crate::Reg<lo_config_2475::LO_CONFIG_2475_SPEC>;
#[doc = "lo_config_2475."]
pub mod lo_config_2475;
#[doc = "dg_testbus_0 register accessor: an alias for `Reg<DG_TESTBUS_0_SPEC>`"]
pub type DG_TESTBUS_0 = crate::Reg<dg_testbus_0::DG_TESTBUS_0_SPEC>;
#[doc = "dg_testbus_0."]
pub mod dg_testbus_0;
#[doc = "dg_testbus_1 register accessor: an alias for `Reg<DG_TESTBUS_1_SPEC>`"]
pub type DG_TESTBUS_1 = crate::Reg<dg_testbus_1::DG_TESTBUS_1_SPEC>;
#[doc = "dg_testbus_1."]
pub mod dg_testbus_1;
#[doc = "dg_ppud_0 register accessor: an alias for `Reg<DG_PPUD_0_SPEC>`"]
pub type DG_PPUD_0 = crate::Reg<dg_ppud_0::DG_PPUD_0_SPEC>;
#[doc = "dg_ppud_0."]
pub mod dg_ppud_0;
#[doc = "rf_top register accessor: an alias for `Reg<RF_TOP_SPEC>`"]
pub type RF_TOP = crate::Reg<rf_top::RF_TOP_SPEC>;
#[doc = "rf_top."]
pub mod rf_top;
#[doc = "rf_fsm register accessor: an alias for `Reg<RF_FSM_SPEC>`"]
pub type RF_FSM = crate::Reg<rf_fsm::RF_FSM_SPEC>;
#[doc = "rf_fsm."]
pub mod rf_fsm;
#[doc = "rf_singen_0 register accessor: an alias for `Reg<RF_SINGEN_0_SPEC>`"]
pub type RF_SINGEN_0 = crate::Reg<rf_singen_0::RF_SINGEN_0_SPEC>;
#[doc = "rf_singen_0."]
pub mod rf_singen_0;
#[doc = "rf_singen_1 register accessor: an alias for `Reg<RF_SINGEN_1_SPEC>`"]
pub type RF_SINGEN_1 = crate::Reg<rf_singen_1::RF_SINGEN_1_SPEC>;
#[doc = "rf_singen_1."]
pub mod rf_singen_1;
#[doc = "rf_singen_2 register accessor: an alias for `Reg<RF_SINGEN_2_SPEC>`"]
pub type RF_SINGEN_2 = crate::Reg<rf_singen_2::RF_SINGEN_2_SPEC>;
#[doc = "rf_singen_2."]
pub mod rf_singen_2;
#[doc = "rf_singen_3 register accessor: an alias for `Reg<RF_SINGEN_3_SPEC>`"]
pub type RF_SINGEN_3 = crate::Reg<rf_singen_3::RF_SINGEN_3_SPEC>;
#[doc = "rf_singen_3."]
pub mod rf_singen_3;
#[doc = "rf_singen_4 register accessor: an alias for `Reg<RF_SINGEN_4_SPEC>`"]
pub type RF_SINGEN_4 = crate::Reg<rf_singen_4::RF_SINGEN_4_SPEC>;
#[doc = "rf_singen_4."]
pub mod rf_singen_4;
#[doc = "rf_sram_ctrl0 register accessor: an alias for `Reg<RF_SRAM_CTRL0_SPEC>`"]
pub type RF_SRAM_CTRL0 = crate::Reg<rf_sram_ctrl0::RF_SRAM_CTRL0_SPEC>;
#[doc = "rf_sram_ctrl0."]
pub mod rf_sram_ctrl0;
#[doc = "rf_sram_ctrl1 register accessor: an alias for `Reg<RF_SRAM_CTRL1_SPEC>`"]
pub type RF_SRAM_CTRL1 = crate::Reg<rf_sram_ctrl1::RF_SRAM_CTRL1_SPEC>;
#[doc = "rf_sram_ctrl1."]
pub mod rf_sram_ctrl1;
#[doc = "rf_sram_ctrl2 register accessor: an alias for `Reg<RF_SRAM_CTRL2_SPEC>`"]
pub type RF_SRAM_CTRL2 = crate::Reg<rf_sram_ctrl2::RF_SRAM_CTRL2_SPEC>;
#[doc = "rf_sram_ctrl2."]
pub mod rf_sram_ctrl2;
#[doc = "rf_test_mode register accessor: an alias for `Reg<RF_TEST_MODE_SPEC>`"]
pub type RF_TEST_MODE = crate::Reg<rf_test_mode::RF_TEST_MODE_SPEC>;
#[doc = "rf_test_mode."]
pub mod rf_test_mode;
#[doc = "rf_rx_pulse_filter register accessor: an alias for `Reg<RF_RX_PULSE_FILTER_SPEC>`"]
pub type RF_RX_PULSE_FILTER = crate::Reg<rf_rx_pulse_filter::RF_RX_PULSE_FILTER_SPEC>;
#[doc = "rf_rx_pulse_filter."]
pub mod rf_rx_pulse_filter;
