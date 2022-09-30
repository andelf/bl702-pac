#[doc = "Register `rf_top` reader"]
pub struct R(crate::R<RF_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_top` writer"]
pub struct W(crate::W<RF_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_TOP_SPEC>;
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
impl From<crate::W<RF_TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rfckg_afifo_adpll_inv` reader - "]
pub type RFCKG_AFIFO_ADPLL_INV_R = crate::BitReader<bool>;
#[doc = "Field `rfckg_afifo_adpll_inv` writer - "]
pub type RFCKG_AFIFO_ADPLL_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_SPEC, bool, O>;
#[doc = "Field `rfckg_afifo_rxadc_inv` reader - "]
pub type RFCKG_AFIFO_RXADC_INV_R = crate::BitReader<bool>;
#[doc = "Field `rfckg_afifo_rxadc_inv` writer - "]
pub type RFCKG_AFIFO_RXADC_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_SPEC, bool, O>;
#[doc = "Field `rfckg_afifo_tx_inv` reader - "]
pub type RFCKG_AFIFO_TX_INV_R = crate::BitReader<bool>;
#[doc = "Field `rfckg_afifo_tx_inv` writer - "]
pub type RFCKG_AFIFO_TX_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_SPEC, bool, O>;
#[doc = "Field `rf_mac_lo_time_offset` reader - "]
pub type RF_MAC_LO_TIME_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_mac_lo_time_offset` writer - "]
pub type RF_MAC_LO_TIME_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_TOP_SPEC, u8, u8, 6, O>;
#[doc = "Field `rf_rx_mode_hw` reader - "]
pub type RF_RX_MODE_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_rx_mode_hw` writer - "]
pub type RF_RX_MODE_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RF_TOP_SPEC, u8, u8, 2, O>;
#[doc = "Field `rf_rx_mode_4s` reader - "]
pub type RF_RX_MODE_4S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_rx_mode_4s` writer - "]
pub type RF_RX_MODE_4S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RF_TOP_SPEC, u8, u8, 2, O>;
#[doc = "Field `rf_rx_mode_4s_en` reader - "]
pub type RF_RX_MODE_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_rx_mode_4s_en` writer - "]
pub type RF_RX_MODE_4S_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_SPEC, bool, O>;
#[doc = "Field `rf_rx_en_4s` reader - "]
pub type RF_RX_EN_4S_R = crate::BitReader<bool>;
#[doc = "Field `rf_rx_en_4s` writer - "]
pub type RF_RX_EN_4S_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_SPEC, bool, O>;
#[doc = "Field `rf_rx_en_src` reader - "]
pub type RF_RX_EN_SRC_R = crate::BitReader<bool>;
#[doc = "Field `rf_rx_en_src` writer - "]
pub type RF_RX_EN_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_SPEC, bool, O>;
#[doc = "Field `rf_tx_en_4s` reader - "]
pub type RF_TX_EN_4S_R = crate::BitReader<bool>;
#[doc = "Field `rf_tx_en_4s` writer - "]
pub type RF_TX_EN_4S_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_SPEC, bool, O>;
#[doc = "Field `rf_tx_en_src` reader - "]
pub type RF_TX_EN_SRC_R = crate::BitReader<bool>;
#[doc = "Field `rf_tx_en_src` writer - "]
pub type RF_TX_EN_SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TOP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_afifo_adpll_inv(&self) -> RFCKG_AFIFO_ADPLL_INV_R {
        RFCKG_AFIFO_ADPLL_INV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_afifo_rxadc_inv(&self) -> RFCKG_AFIFO_RXADC_INV_R {
        RFCKG_AFIFO_RXADC_INV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_afifo_tx_inv(&self) -> RFCKG_AFIFO_TX_INV_R {
        RFCKG_AFIFO_TX_INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rf_mac_lo_time_offset(&self) -> RF_MAC_LO_TIME_OFFSET_R {
        RF_MAC_LO_TIME_OFFSET_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn rf_rx_mode_hw(&self) -> RF_RX_MODE_HW_R {
        RF_RX_MODE_HW_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rf_rx_mode_4s(&self) -> RF_RX_MODE_4S_R {
        RF_RX_MODE_4S_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rf_rx_mode_4s_en(&self) -> RF_RX_MODE_4S_EN_R {
        RF_RX_MODE_4S_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rf_rx_en_4s(&self) -> RF_RX_EN_4S_R {
        RF_RX_EN_4S_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rf_rx_en_src(&self) -> RF_RX_EN_SRC_R {
        RF_RX_EN_SRC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_tx_en_4s(&self) -> RF_TX_EN_4S_R {
        RF_TX_EN_4S_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_tx_en_src(&self) -> RF_TX_EN_SRC_R {
        RF_TX_EN_SRC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_afifo_adpll_inv(&mut self) -> RFCKG_AFIFO_ADPLL_INV_W<0> {
        RFCKG_AFIFO_ADPLL_INV_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_afifo_rxadc_inv(&mut self) -> RFCKG_AFIFO_RXADC_INV_W<1> {
        RFCKG_AFIFO_RXADC_INV_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_afifo_tx_inv(&mut self) -> RFCKG_AFIFO_TX_INV_W<2> {
        RFCKG_AFIFO_TX_INV_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rf_mac_lo_time_offset(&mut self) -> RF_MAC_LO_TIME_OFFSET_W<16> {
        RF_MAC_LO_TIME_OFFSET_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn rf_rx_mode_hw(&mut self) -> RF_RX_MODE_HW_W<22> {
        RF_RX_MODE_HW_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rf_rx_mode_4s(&mut self) -> RF_RX_MODE_4S_W<24> {
        RF_RX_MODE_4S_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rf_rx_mode_4s_en(&mut self) -> RF_RX_MODE_4S_EN_W<26> {
        RF_RX_MODE_4S_EN_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rf_rx_en_4s(&mut self) -> RF_RX_EN_4S_W<28> {
        RF_RX_EN_4S_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rf_rx_en_src(&mut self) -> RF_RX_EN_SRC_W<29> {
        RF_RX_EN_SRC_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_tx_en_4s(&mut self) -> RF_TX_EN_4S_W<30> {
        RF_TX_EN_4S_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rf_tx_en_src(&mut self) -> RF_TX_EN_SRC_W<31> {
        RF_TX_EN_SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_top.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_top](index.html) module"]
pub struct RF_TOP_SPEC;
impl crate::RegisterSpec for RF_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_top::R](R) reader structure"]
impl crate::Readable for RF_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_top::W](W) writer structure"]
impl crate::Writable for RF_TOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_top to value 0"]
impl crate::Resettable for RF_TOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
