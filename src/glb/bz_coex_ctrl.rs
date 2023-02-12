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
#[doc = "Field `coex_en` reader - "]
pub type COEX_EN_R = crate::BitReader<bool>;
#[doc = "Field `coex_en` writer - "]
pub type COEX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `wlan_en` reader - "]
pub type WLAN_EN_R = crate::BitReader<bool>;
#[doc = "Field `wlan_en` writer - "]
pub type WLAN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `ble_rx_ignore` reader - "]
pub type BLE_RX_IGNORE_R = crate::BitReader<bool>;
#[doc = "Field `ble_rx_ignore` writer - "]
pub type BLE_RX_IGNORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `m154_rx_ignore` reader - "]
pub type M154_RX_IGNORE_R = crate::BitReader<bool>;
#[doc = "Field `m154_rx_ignore` writer - "]
pub type M154_RX_IGNORE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `bz_pri_thr` reader - "]
pub type BZ_PRI_THR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bz_pri_thr` writer - "]
pub type BZ_PRI_THR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BZ_COEX_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `bz_pri_en` reader - "]
pub type BZ_PRI_EN_R = crate::BitReader<bool>;
#[doc = "Field `bz_pri_en` writer - "]
pub type BZ_PRI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `bz_pri_pol` reader - "]
pub type BZ_PRI_POL_R = crate::BitReader<bool>;
#[doc = "Field `bz_pri_pol` writer - "]
pub type BZ_PRI_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `bz_active_pol` reader - "]
pub type BZ_ACTIVE_POL_R = crate::BitReader<bool>;
#[doc = "Field `bz_active_pol` writer - "]
pub type BZ_ACTIVE_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `bz_abort_pol` reader - "]
pub type BZ_ABORT_POL_R = crate::BitReader<bool>;
#[doc = "Field `bz_abort_pol` writer - "]
pub type BZ_ABORT_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `coex_pri` reader - "]
pub type COEX_PRI_R = crate::BitReader<bool>;
#[doc = "Field `coex_pri` writer - "]
pub type COEX_PRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `force_m154_win` reader - "]
pub type FORCE_M154_WIN_R = crate::BitReader<bool>;
#[doc = "Field `force_m154_win` writer - "]
pub type FORCE_M154_WIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `force_ble_win` reader - "]
pub type FORCE_BLE_WIN_R = crate::BitReader<bool>;
#[doc = "Field `force_ble_win` writer - "]
pub type FORCE_BLE_WIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `coex_option` reader - "]
pub type COEX_OPTION_R = crate::BitReader<bool>;
#[doc = "Field `coex_option` writer - "]
pub type COEX_OPTION_W<'a, const O: u8> = crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `coex_force_ch` reader - "]
pub type COEX_FORCE_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `coex_force_ch` writer - "]
pub type COEX_FORCE_CH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BZ_COEX_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `m154_rx_abort_dis` reader - "]
pub type M154_RX_ABORT_DIS_R = crate::BitReader<bool>;
#[doc = "Field `m154_rx_abort_dis` writer - "]
pub type M154_RX_ABORT_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `m154_tx_abort_dis` reader - "]
pub type M154_TX_ABORT_DIS_R = crate::BitReader<bool>;
#[doc = "Field `m154_tx_abort_dis` writer - "]
pub type M154_TX_ABORT_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `ble_rx_abort_dis` reader - "]
pub type BLE_RX_ABORT_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ble_rx_abort_dis` writer - "]
pub type BLE_RX_ABORT_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `ble_tx_abort_dis` reader - "]
pub type BLE_TX_ABORT_DIS_R = crate::BitReader<bool>;
#[doc = "Field `ble_tx_abort_dis` writer - "]
pub type BLE_TX_ABORT_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, BZ_COEX_CTRL_SPEC, bool, O>;
#[doc = "Field `coex_arb` reader - "]
pub type COEX_ARB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `coex_arb` writer - "]
pub type COEX_ARB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BZ_COEX_CTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn coex_en(&self) -> COEX_EN_R {
        COEX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wlan_en(&self) -> WLAN_EN_R {
        WLAN_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ble_rx_ignore(&self) -> BLE_RX_IGNORE_R {
        BLE_RX_IGNORE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn m154_rx_ignore(&self) -> M154_RX_IGNORE_R {
        M154_RX_IGNORE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn bz_pri_thr(&self) -> BZ_PRI_THR_R {
        BZ_PRI_THR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bz_pri_en(&self) -> BZ_PRI_EN_R {
        BZ_PRI_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bz_pri_pol(&self) -> BZ_PRI_POL_R {
        BZ_PRI_POL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bz_active_pol(&self) -> BZ_ACTIVE_POL_R {
        BZ_ACTIVE_POL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bz_abort_pol(&self) -> BZ_ABORT_POL_R {
        BZ_ABORT_POL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn coex_pri(&self) -> COEX_PRI_R {
        COEX_PRI_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn force_m154_win(&self) -> FORCE_M154_WIN_R {
        FORCE_M154_WIN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn force_ble_win(&self) -> FORCE_BLE_WIN_R {
        FORCE_BLE_WIN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn coex_option(&self) -> COEX_OPTION_R {
        COEX_OPTION_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn coex_force_ch(&self) -> COEX_FORCE_CH_R {
        COEX_FORCE_CH_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn m154_rx_abort_dis(&self) -> M154_RX_ABORT_DIS_R {
        M154_RX_ABORT_DIS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn m154_tx_abort_dis(&self) -> M154_TX_ABORT_DIS_R {
        M154_TX_ABORT_DIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ble_rx_abort_dis(&self) -> BLE_RX_ABORT_DIS_R {
        BLE_RX_ABORT_DIS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ble_tx_abort_dis(&self) -> BLE_TX_ABORT_DIS_R {
        BLE_TX_ABORT_DIS_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn coex_arb(&self) -> COEX_ARB_R {
        COEX_ARB_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn coex_en(&mut self) -> COEX_EN_W<0> {
        COEX_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn wlan_en(&mut self) -> WLAN_EN_W<1> {
        WLAN_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ble_rx_ignore(&mut self) -> BLE_RX_IGNORE_W<2> {
        BLE_RX_IGNORE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn m154_rx_ignore(&mut self) -> M154_RX_IGNORE_W<3> {
        M154_RX_IGNORE_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn bz_pri_thr(&mut self) -> BZ_PRI_THR_W<4> {
        BZ_PRI_THR_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn bz_pri_en(&mut self) -> BZ_PRI_EN_W<8> {
        BZ_PRI_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bz_pri_pol(&mut self) -> BZ_PRI_POL_W<9> {
        BZ_PRI_POL_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn bz_active_pol(&mut self) -> BZ_ACTIVE_POL_W<10> {
        BZ_ACTIVE_POL_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn bz_abort_pol(&mut self) -> BZ_ABORT_POL_W<11> {
        BZ_ABORT_POL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn coex_pri(&mut self) -> COEX_PRI_W<12> {
        COEX_PRI_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn force_m154_win(&mut self) -> FORCE_M154_WIN_W<13> {
        FORCE_M154_WIN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn force_ble_win(&mut self) -> FORCE_BLE_WIN_W<14> {
        FORCE_BLE_WIN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn coex_option(&mut self) -> COEX_OPTION_W<15> {
        COEX_OPTION_W::new(self)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    #[must_use]
    pub fn coex_force_ch(&mut self) -> COEX_FORCE_CH_W<16> {
        COEX_FORCE_CH_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn m154_rx_abort_dis(&mut self) -> M154_RX_ABORT_DIS_W<24> {
        M154_RX_ABORT_DIS_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn m154_tx_abort_dis(&mut self) -> M154_TX_ABORT_DIS_W<25> {
        M154_TX_ABORT_DIS_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn ble_rx_abort_dis(&mut self) -> BLE_RX_ABORT_DIS_W<26> {
        BLE_RX_ABORT_DIS_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn ble_tx_abort_dis(&mut self) -> BLE_TX_ABORT_DIS_W<27> {
        BLE_TX_ABORT_DIS_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn coex_arb(&mut self) -> COEX_ARB_W<28> {
        COEX_ARB_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BZ_COEX_CTRL to value 0"]
impl crate::Resettable for BZ_COEX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
