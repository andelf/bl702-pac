#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ks_ctrl."]
    pub ks_ctrl: KS_CTRL,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - ks_int_en."]
    pub ks_int_en: KS_INT_EN,
    #[doc = "0x14 - ks_int_sts."]
    pub ks_int_sts: KS_INT_STS,
    #[doc = "0x18 - keycode_clr."]
    pub keycode_clr: KEYCODE_CLR,
    #[doc = "0x1c - keycode_value."]
    pub keycode_value: KEYCODE_VALUE,
}
#[doc = "ks_ctrl (rw) register accessor: an alias for `Reg<KS_CTRL_SPEC>`"]
pub type KS_CTRL = crate::Reg<ks_ctrl::KS_CTRL_SPEC>;
#[doc = "ks_ctrl."]
pub mod ks_ctrl;
#[doc = "ks_int_en (rw) register accessor: an alias for `Reg<KS_INT_EN_SPEC>`"]
pub type KS_INT_EN = crate::Reg<ks_int_en::KS_INT_EN_SPEC>;
#[doc = "ks_int_en."]
pub mod ks_int_en;
#[doc = "ks_int_sts (rw) register accessor: an alias for `Reg<KS_INT_STS_SPEC>`"]
pub type KS_INT_STS = crate::Reg<ks_int_sts::KS_INT_STS_SPEC>;
#[doc = "ks_int_sts."]
pub mod ks_int_sts;
#[doc = "keycode_clr (rw) register accessor: an alias for `Reg<KEYCODE_CLR_SPEC>`"]
pub type KEYCODE_CLR = crate::Reg<keycode_clr::KEYCODE_CLR_SPEC>;
#[doc = "keycode_clr."]
pub mod keycode_clr;
#[doc = "keycode_value (rw) register accessor: an alias for `Reg<KEYCODE_VALUE_SPEC>`"]
pub type KEYCODE_VALUE = crate::Reg<keycode_value::KEYCODE_VALUE_SPEC>;
#[doc = "keycode_value."]
pub mod keycode_value;
