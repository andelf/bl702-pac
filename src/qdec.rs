#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - qdec_ctrl."]
    pub qdec_ctrl: QDEC_CTRL,
    #[doc = "0x04 - qdec_value."]
    pub qdec_value: QDEC_VALUE,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - qdec_int_en."]
    pub qdec_int_en: QDEC_INT_EN,
    #[doc = "0x14 - qdec_int_sts."]
    pub qdec_int_sts: QDEC_INT_STS,
    #[doc = "0x18 - qdec_int_clr."]
    pub qdec_int_clr: QDEC_INT_CLR,
}
#[doc = "qdec_ctrl (rw) register accessor: an alias for `Reg<QDEC_CTRL_SPEC>`"]
pub type QDEC_CTRL = crate::Reg<qdec_ctrl::QDEC_CTRL_SPEC>;
#[doc = "qdec_ctrl."]
pub mod qdec_ctrl;
#[doc = "qdec_value (rw) register accessor: an alias for `Reg<QDEC_VALUE_SPEC>`"]
pub type QDEC_VALUE = crate::Reg<qdec_value::QDEC_VALUE_SPEC>;
#[doc = "qdec_value."]
pub mod qdec_value;
#[doc = "qdec_int_en (rw) register accessor: an alias for `Reg<QDEC_INT_EN_SPEC>`"]
pub type QDEC_INT_EN = crate::Reg<qdec_int_en::QDEC_INT_EN_SPEC>;
#[doc = "qdec_int_en."]
pub mod qdec_int_en;
#[doc = "qdec_int_sts (rw) register accessor: an alias for `Reg<QDEC_INT_STS_SPEC>`"]
pub type QDEC_INT_STS = crate::Reg<qdec_int_sts::QDEC_INT_STS_SPEC>;
#[doc = "qdec_int_sts."]
pub mod qdec_int_sts;
#[doc = "qdec_int_clr (rw) register accessor: an alias for `Reg<QDEC_INT_CLR_SPEC>`"]
pub type QDEC_INT_CLR = crate::Reg<qdec_int_clr::QDEC_INT_CLR_SPEC>;
#[doc = "qdec_int_clr."]
pub mod qdec_int_clr;
