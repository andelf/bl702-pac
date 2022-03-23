#[doc = "Register `BZ_COEX_CTRL` reader"]
pub struct R(crate::R<BZ_COEX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BZ_COEX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BZ_COEX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BZ_COEX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BZ_COEX_CTRL` writer"]
pub struct W(crate::W<BZ_COEX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BZ_COEX_CTRL_SPEC>;
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
impl From<crate::W<BZ_COEX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BZ_COEX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `coex_arb` reader - "]
pub struct COEX_ARB_R(crate::FieldReader<u8, u8>);
impl COEX_ARB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COEX_ARB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COEX_ARB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coex_arb` writer - "]
pub struct COEX_ARB_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_ARB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `ble_tx_abort_dis` reader - "]
pub struct BLE_TX_ABORT_DIS_R(crate::FieldReader<bool, bool>);
impl BLE_TX_ABORT_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_TX_ABORT_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_TX_ABORT_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ble_tx_abort_dis` writer - "]
pub struct BLE_TX_ABORT_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_TX_ABORT_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `ble_rx_abort_dis` reader - "]
pub struct BLE_RX_ABORT_DIS_R(crate::FieldReader<bool, bool>);
impl BLE_RX_ABORT_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_RX_ABORT_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_RX_ABORT_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ble_rx_abort_dis` writer - "]
pub struct BLE_RX_ABORT_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_RX_ABORT_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `m154_tx_abort_dis` reader - "]
pub struct M154_TX_ABORT_DIS_R(crate::FieldReader<bool, bool>);
impl M154_TX_ABORT_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M154_TX_ABORT_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M154_TX_ABORT_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `m154_tx_abort_dis` writer - "]
pub struct M154_TX_ABORT_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> M154_TX_ABORT_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `m154_rx_abort_dis` reader - "]
pub struct M154_RX_ABORT_DIS_R(crate::FieldReader<bool, bool>);
impl M154_RX_ABORT_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M154_RX_ABORT_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M154_RX_ABORT_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `m154_rx_abort_dis` writer - "]
pub struct M154_RX_ABORT_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> M154_RX_ABORT_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `coex_force_ch` reader - "]
pub struct COEX_FORCE_CH_R(crate::FieldReader<u8, u8>);
impl COEX_FORCE_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COEX_FORCE_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COEX_FORCE_CH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coex_force_ch` writer - "]
pub struct COEX_FORCE_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_FORCE_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `coex_option` reader - "]
pub struct COEX_OPTION_R(crate::FieldReader<bool, bool>);
impl COEX_OPTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COEX_OPTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COEX_OPTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coex_option` writer - "]
pub struct COEX_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_OPTION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `force_ble_win` reader - "]
pub struct FORCE_BLE_WIN_R(crate::FieldReader<bool, bool>);
impl FORCE_BLE_WIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_BLE_WIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_BLE_WIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `force_ble_win` writer - "]
pub struct FORCE_BLE_WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_BLE_WIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `force_m154_win` reader - "]
pub struct FORCE_M154_WIN_R(crate::FieldReader<bool, bool>);
impl FORCE_M154_WIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_M154_WIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_M154_WIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `force_m154_win` writer - "]
pub struct FORCE_M154_WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_M154_WIN_W<'a> {
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
#[doc = "Field `coex_pri` reader - "]
pub struct COEX_PRI_R(crate::FieldReader<bool, bool>);
impl COEX_PRI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COEX_PRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COEX_PRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coex_pri` writer - "]
pub struct COEX_PRI_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_PRI_W<'a> {
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
#[doc = "Field `bz_abort_pol` reader - "]
pub struct BZ_ABORT_POL_R(crate::FieldReader<bool, bool>);
impl BZ_ABORT_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BZ_ABORT_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BZ_ABORT_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bz_abort_pol` writer - "]
pub struct BZ_ABORT_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> BZ_ABORT_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `bz_active_pol` reader - "]
pub struct BZ_ACTIVE_POL_R(crate::FieldReader<bool, bool>);
impl BZ_ACTIVE_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BZ_ACTIVE_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BZ_ACTIVE_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bz_active_pol` writer - "]
pub struct BZ_ACTIVE_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> BZ_ACTIVE_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `bz_pri_pol` reader - "]
pub struct BZ_PRI_POL_R(crate::FieldReader<bool, bool>);
impl BZ_PRI_POL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BZ_PRI_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BZ_PRI_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bz_pri_pol` writer - "]
pub struct BZ_PRI_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> BZ_PRI_POL_W<'a> {
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
#[doc = "Field `bz_pri_en` reader - "]
pub struct BZ_PRI_EN_R(crate::FieldReader<bool, bool>);
impl BZ_PRI_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BZ_PRI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BZ_PRI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bz_pri_en` writer - "]
pub struct BZ_PRI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BZ_PRI_EN_W<'a> {
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
#[doc = "Field `bz_pri_thr` reader - "]
pub struct BZ_PRI_THR_R(crate::FieldReader<u8, u8>);
impl BZ_PRI_THR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BZ_PRI_THR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BZ_PRI_THR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bz_pri_thr` writer - "]
pub struct BZ_PRI_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> BZ_PRI_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `m154_rx_ignore` reader - "]
pub struct M154_RX_IGNORE_R(crate::FieldReader<bool, bool>);
impl M154_RX_IGNORE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M154_RX_IGNORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M154_RX_IGNORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `m154_rx_ignore` writer - "]
pub struct M154_RX_IGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> M154_RX_IGNORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ble_rx_ignore` reader - "]
pub struct BLE_RX_IGNORE_R(crate::FieldReader<bool, bool>);
impl BLE_RX_IGNORE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_RX_IGNORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_RX_IGNORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ble_rx_ignore` writer - "]
pub struct BLE_RX_IGNORE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_RX_IGNORE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `wlan_en` reader - "]
pub struct WLAN_EN_R(crate::FieldReader<bool, bool>);
impl WLAN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WLAN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLAN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wlan_en` writer - "]
pub struct WLAN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLAN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `coex_en` reader - "]
pub struct COEX_EN_R(crate::FieldReader<bool, bool>);
impl COEX_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COEX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COEX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coex_en` writer - "]
pub struct COEX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> COEX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn coex_arb(&self) -> COEX_ARB_R {
        COEX_ARB_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ble_tx_abort_dis(&self) -> BLE_TX_ABORT_DIS_R {
        BLE_TX_ABORT_DIS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ble_rx_abort_dis(&self) -> BLE_RX_ABORT_DIS_R {
        BLE_RX_ABORT_DIS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn m154_tx_abort_dis(&self) -> M154_TX_ABORT_DIS_R {
        M154_TX_ABORT_DIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn m154_rx_abort_dis(&self) -> M154_RX_ABORT_DIS_R {
        M154_RX_ABORT_DIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn coex_force_ch(&self) -> COEX_FORCE_CH_R {
        COEX_FORCE_CH_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn coex_option(&self) -> COEX_OPTION_R {
        COEX_OPTION_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn force_ble_win(&self) -> FORCE_BLE_WIN_R {
        FORCE_BLE_WIN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn force_m154_win(&self) -> FORCE_M154_WIN_R {
        FORCE_M154_WIN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn coex_pri(&self) -> COEX_PRI_R {
        COEX_PRI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bz_abort_pol(&self) -> BZ_ABORT_POL_R {
        BZ_ABORT_POL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bz_active_pol(&self) -> BZ_ACTIVE_POL_R {
        BZ_ACTIVE_POL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bz_pri_pol(&self) -> BZ_PRI_POL_R {
        BZ_PRI_POL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bz_pri_en(&self) -> BZ_PRI_EN_R {
        BZ_PRI_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn bz_pri_thr(&self) -> BZ_PRI_THR_R {
        BZ_PRI_THR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn m154_rx_ignore(&self) -> M154_RX_IGNORE_R {
        M154_RX_IGNORE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ble_rx_ignore(&self) -> BLE_RX_IGNORE_R {
        BLE_RX_IGNORE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wlan_en(&self) -> WLAN_EN_R {
        WLAN_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn coex_en(&self) -> COEX_EN_R {
        COEX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn coex_arb(&mut self) -> COEX_ARB_W {
        COEX_ARB_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ble_tx_abort_dis(&mut self) -> BLE_TX_ABORT_DIS_W {
        BLE_TX_ABORT_DIS_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ble_rx_abort_dis(&mut self) -> BLE_RX_ABORT_DIS_W {
        BLE_RX_ABORT_DIS_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn m154_tx_abort_dis(&mut self) -> M154_TX_ABORT_DIS_W {
        M154_TX_ABORT_DIS_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn m154_rx_abort_dis(&mut self) -> M154_RX_ABORT_DIS_W {
        M154_RX_ABORT_DIS_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn coex_force_ch(&mut self) -> COEX_FORCE_CH_W {
        COEX_FORCE_CH_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn coex_option(&mut self) -> COEX_OPTION_W {
        COEX_OPTION_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn force_ble_win(&mut self) -> FORCE_BLE_WIN_W {
        FORCE_BLE_WIN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn force_m154_win(&mut self) -> FORCE_M154_WIN_W {
        FORCE_M154_WIN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn coex_pri(&mut self) -> COEX_PRI_W {
        COEX_PRI_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bz_abort_pol(&mut self) -> BZ_ABORT_POL_W {
        BZ_ABORT_POL_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bz_active_pol(&mut self) -> BZ_ACTIVE_POL_W {
        BZ_ACTIVE_POL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bz_pri_pol(&mut self) -> BZ_PRI_POL_W {
        BZ_PRI_POL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bz_pri_en(&mut self) -> BZ_PRI_EN_W {
        BZ_PRI_EN_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn bz_pri_thr(&mut self) -> BZ_PRI_THR_W {
        BZ_PRI_THR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn m154_rx_ignore(&mut self) -> M154_RX_IGNORE_W {
        M154_RX_IGNORE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ble_rx_ignore(&mut self) -> BLE_RX_IGNORE_W {
        BLE_RX_IGNORE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wlan_en(&mut self) -> WLAN_EN_W {
        WLAN_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn coex_en(&mut self) -> COEX_EN_W {
        COEX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BZ_COEX_CTRL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bz_coex_ctrl](index.html) module"]
pub struct BZ_COEX_CTRL_SPEC;
impl crate::RegisterSpec for BZ_COEX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bz_coex_ctrl::R](R) reader structure"]
impl crate::Readable for BZ_COEX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bz_coex_ctrl::W](W) writer structure"]
impl crate::Writable for BZ_COEX_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BZ_COEX_CTRL to value 0"]
impl crate::Resettable for BZ_COEX_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
