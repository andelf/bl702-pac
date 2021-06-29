#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MODE."]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x04 - INT_SOURCE."]
    pub int_source: crate::Reg<int_source::INT_SOURCE_SPEC>,
    #[doc = "0x08 - INT_MASK."]
    pub int_mask: crate::Reg<int_mask::INT_MASK_SPEC>,
    #[doc = "0x0c - IPGT."]
    pub ipgt: crate::Reg<ipgt::IPGT_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - PACKETLEN."]
    pub packetlen: crate::Reg<packetlen::PACKETLEN_SPEC>,
    #[doc = "0x1c - COLLCONFIG."]
    pub collconfig: crate::Reg<collconfig::COLLCONFIG_SPEC>,
    #[doc = "0x20 - TX_BD_NUM."]
    pub tx_bd_num: crate::Reg<tx_bd_num::TX_BD_NUM_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x28 - MIIMODE."]
    pub miimode: crate::Reg<miimode::MIIMODE_SPEC>,
    #[doc = "0x2c - MIICOMMAND."]
    pub miicommand: crate::Reg<miicommand::MIICOMMAND_SPEC>,
    #[doc = "0x30 - MIIADDRESS."]
    pub miiaddress: crate::Reg<miiaddress::MIIADDRESS_SPEC>,
    #[doc = "0x34 - MIITX_DATA."]
    pub miitx_data: crate::Reg<miitx_data::MIITX_DATA_SPEC>,
    #[doc = "0x38 - MIIRX_DATA."]
    pub miirx_data: crate::Reg<miirx_data::MIIRX_DATA_SPEC>,
    #[doc = "0x3c - MIISTATUS."]
    pub miistatus: crate::Reg<miistatus::MIISTATUS_SPEC>,
    #[doc = "0x40 - MAC_ADDR0."]
    pub mac_addr0: crate::Reg<mac_addr0::MAC_ADDR0_SPEC>,
    #[doc = "0x44 - MAC_ADDR1."]
    pub mac_addr1: crate::Reg<mac_addr1::MAC_ADDR1_SPEC>,
    #[doc = "0x48 - HASH0_ADDR."]
    pub hash0_addr: crate::Reg<hash0_addr::HASH0_ADDR_SPEC>,
    #[doc = "0x4c - HASH1_ADDR."]
    pub hash1_addr: crate::Reg<hash1_addr::HASH1_ADDR_SPEC>,
    #[doc = "0x50 - TXCTRL."]
    pub txctrl: crate::Reg<txctrl::TXCTRL_SPEC>,
}
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "MODE."]
pub mod mode;
#[doc = "INT_SOURCE register accessor: an alias for `Reg<INT_SOURCE_SPEC>`"]
pub type INT_SOURCE = crate::Reg<int_source::INT_SOURCE_SPEC>;
#[doc = "INT_SOURCE."]
pub mod int_source;
#[doc = "INT_MASK register accessor: an alias for `Reg<INT_MASK_SPEC>`"]
pub type INT_MASK = crate::Reg<int_mask::INT_MASK_SPEC>;
#[doc = "INT_MASK."]
pub mod int_mask;
#[doc = "IPGT register accessor: an alias for `Reg<IPGT_SPEC>`"]
pub type IPGT = crate::Reg<ipgt::IPGT_SPEC>;
#[doc = "IPGT."]
pub mod ipgt;
#[doc = "PACKETLEN register accessor: an alias for `Reg<PACKETLEN_SPEC>`"]
pub type PACKETLEN = crate::Reg<packetlen::PACKETLEN_SPEC>;
#[doc = "PACKETLEN."]
pub mod packetlen;
#[doc = "COLLCONFIG register accessor: an alias for `Reg<COLLCONFIG_SPEC>`"]
pub type COLLCONFIG = crate::Reg<collconfig::COLLCONFIG_SPEC>;
#[doc = "COLLCONFIG."]
pub mod collconfig;
#[doc = "TX_BD_NUM register accessor: an alias for `Reg<TX_BD_NUM_SPEC>`"]
pub type TX_BD_NUM = crate::Reg<tx_bd_num::TX_BD_NUM_SPEC>;
#[doc = "TX_BD_NUM."]
pub mod tx_bd_num;
#[doc = "MIIMODE register accessor: an alias for `Reg<MIIMODE_SPEC>`"]
pub type MIIMODE = crate::Reg<miimode::MIIMODE_SPEC>;
#[doc = "MIIMODE."]
pub mod miimode;
#[doc = "MIICOMMAND register accessor: an alias for `Reg<MIICOMMAND_SPEC>`"]
pub type MIICOMMAND = crate::Reg<miicommand::MIICOMMAND_SPEC>;
#[doc = "MIICOMMAND."]
pub mod miicommand;
#[doc = "MIIADDRESS register accessor: an alias for `Reg<MIIADDRESS_SPEC>`"]
pub type MIIADDRESS = crate::Reg<miiaddress::MIIADDRESS_SPEC>;
#[doc = "MIIADDRESS."]
pub mod miiaddress;
#[doc = "MIITX_DATA register accessor: an alias for `Reg<MIITX_DATA_SPEC>`"]
pub type MIITX_DATA = crate::Reg<miitx_data::MIITX_DATA_SPEC>;
#[doc = "MIITX_DATA."]
pub mod miitx_data;
#[doc = "MIIRX_DATA register accessor: an alias for `Reg<MIIRX_DATA_SPEC>`"]
pub type MIIRX_DATA = crate::Reg<miirx_data::MIIRX_DATA_SPEC>;
#[doc = "MIIRX_DATA."]
pub mod miirx_data;
#[doc = "MIISTATUS register accessor: an alias for `Reg<MIISTATUS_SPEC>`"]
pub type MIISTATUS = crate::Reg<miistatus::MIISTATUS_SPEC>;
#[doc = "MIISTATUS."]
pub mod miistatus;
#[doc = "MAC_ADDR0 register accessor: an alias for `Reg<MAC_ADDR0_SPEC>`"]
pub type MAC_ADDR0 = crate::Reg<mac_addr0::MAC_ADDR0_SPEC>;
#[doc = "MAC_ADDR0."]
pub mod mac_addr0;
#[doc = "MAC_ADDR1 register accessor: an alias for `Reg<MAC_ADDR1_SPEC>`"]
pub type MAC_ADDR1 = crate::Reg<mac_addr1::MAC_ADDR1_SPEC>;
#[doc = "MAC_ADDR1."]
pub mod mac_addr1;
#[doc = "HASH0_ADDR register accessor: an alias for `Reg<HASH0_ADDR_SPEC>`"]
pub type HASH0_ADDR = crate::Reg<hash0_addr::HASH0_ADDR_SPEC>;
#[doc = "HASH0_ADDR."]
pub mod hash0_addr;
#[doc = "HASH1_ADDR register accessor: an alias for `Reg<HASH1_ADDR_SPEC>`"]
pub type HASH1_ADDR = crate::Reg<hash1_addr::HASH1_ADDR_SPEC>;
#[doc = "HASH1_ADDR."]
pub mod hash1_addr;
#[doc = "TXCTRL register accessor: an alias for `Reg<TXCTRL_SPEC>`"]
pub type TXCTRL = crate::Reg<txctrl::TXCTRL_SPEC>;
#[doc = "TXCTRL."]
pub mod txctrl;
