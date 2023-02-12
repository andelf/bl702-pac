#[doc = "Register `rf_fsm` reader"]
pub struct R(crate::R<RF_FSM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_FSM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_FSM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm` writer"]
pub struct W(crate::W<RF_FSM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_SPEC>;
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
impl From<crate::W<RF_FSM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_FSM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_fsm_lo_time` reader - "]
pub type RF_FSM_LO_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_fsm_lo_time` writer - "]
pub type RF_FSM_LO_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_SPEC, u16, u16, 12, O>;
#[doc = "Field `rf_fsm_state` reader - "]
pub type RF_FSM_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_fsm_state` writer - "]
pub type RF_FSM_STATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RF_FSM_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_fsm_st_4s` reader - "]
pub type RF_FSM_ST_4S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_fsm_st_4s` writer - "]
pub type RF_FSM_ST_4S_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RF_FSM_SPEC, u8, u8, 3, O>;
#[doc = "Field `rf_fsm_st_4s_en` reader - "]
pub type RF_FSM_ST_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_st_4s_en` writer - "]
pub type RF_FSM_ST_4S_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_FSM_SPEC, bool, O>;
#[doc = "Field `rf_fsm_en` reader - "]
pub type RF_FSM_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_en` writer - "]
pub type RF_FSM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_FSM_SPEC, bool, O>;
#[doc = "Field `rf_fsm_rx_afifo_4s` reader - "]
pub type RF_FSM_RX_AFIFO_4S_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_rx_afifo_4s` writer - "]
pub type RF_FSM_RX_AFIFO_4S_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_FSM_SPEC, bool, O>;
#[doc = "Field `rf_fsm_rx_afifo_4s_en` reader - "]
pub type RF_FSM_RX_AFIFO_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_rx_afifo_4s_en` writer - "]
pub type RF_FSM_RX_AFIFO_4S_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_FSM_SPEC, bool, O>;
#[doc = "Field `rf_fsm_tx_afifo_4s` reader - "]
pub type RF_FSM_TX_AFIFO_4S_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_tx_afifo_4s` writer - "]
pub type RF_FSM_TX_AFIFO_4S_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_FSM_SPEC, bool, O>;
#[doc = "Field `rf_fsm_tx_afifo_4s_en` reader - "]
pub type RF_FSM_TX_AFIFO_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_fsm_tx_afifo_4s_en` writer - "]
pub type RF_FSM_TX_AFIFO_4S_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_FSM_SPEC, bool, O>;
#[doc = "Field `rf_fsm_afifo_dly_time` reader - "]
pub type RF_FSM_AFIFO_DLY_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_fsm_afifo_dly_time` writer - "]
pub type RF_FSM_AFIFO_DLY_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_FSM_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn rf_fsm_lo_time(&self) -> RF_FSM_LO_TIME_R {
        RF_FSM_LO_TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_fsm_state(&self) -> RF_FSM_STATE_R {
        RF_FSM_STATE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_fsm_st_4s(&self) -> RF_FSM_ST_4S_R {
        RF_FSM_ST_4S_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rf_fsm_st_4s_en(&self) -> RF_FSM_ST_4S_EN_R {
        RF_FSM_ST_4S_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rf_fsm_en(&self) -> RF_FSM_EN_R {
        RF_FSM_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rf_fsm_rx_afifo_4s(&self) -> RF_FSM_RX_AFIFO_4S_R {
        RF_FSM_RX_AFIFO_4S_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rf_fsm_rx_afifo_4s_en(&self) -> RF_FSM_RX_AFIFO_4S_EN_R {
        RF_FSM_RX_AFIFO_4S_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rf_fsm_tx_afifo_4s(&self) -> RF_FSM_TX_AFIFO_4S_R {
        RF_FSM_TX_AFIFO_4S_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rf_fsm_tx_afifo_4s_en(&self) -> RF_FSM_TX_AFIFO_4S_EN_R {
        RF_FSM_TX_AFIFO_4S_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rf_fsm_afifo_dly_time(&self) -> RF_FSM_AFIFO_DLY_TIME_R {
        RF_FSM_AFIFO_DLY_TIME_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_lo_time(&mut self) -> RF_FSM_LO_TIME_W<0> {
        RF_FSM_LO_TIME_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_state(&mut self) -> RF_FSM_STATE_W<12> {
        RF_FSM_STATE_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_st_4s(&mut self) -> RF_FSM_ST_4S_W<16> {
        RF_FSM_ST_4S_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_st_4s_en(&mut self) -> RF_FSM_ST_4S_EN_W<20> {
        RF_FSM_ST_4S_EN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_en(&mut self) -> RF_FSM_EN_W<21> {
        RF_FSM_EN_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_rx_afifo_4s(&mut self) -> RF_FSM_RX_AFIFO_4S_W<22> {
        RF_FSM_RX_AFIFO_4S_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_rx_afifo_4s_en(&mut self) -> RF_FSM_RX_AFIFO_4S_EN_W<23> {
        RF_FSM_RX_AFIFO_4S_EN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_tx_afifo_4s(&mut self) -> RF_FSM_TX_AFIFO_4S_W<24> {
        RF_FSM_TX_AFIFO_4S_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_tx_afifo_4s_en(&mut self) -> RF_FSM_TX_AFIFO_4S_EN_W<25> {
        RF_FSM_TX_AFIFO_4S_EN_W::new(self)
    }
    #[doc = "Bits 26:31"]
    #[inline(always)]
    #[must_use]
    pub fn rf_fsm_afifo_dly_time(&mut self) -> RF_FSM_AFIFO_DLY_TIME_W<26> {
        RF_FSM_AFIFO_DLY_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_fsm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm](index.html) module"]
pub struct RF_FSM_SPEC;
impl crate::RegisterSpec for RF_FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm::R](R) reader structure"]
impl crate::Readable for RF_FSM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm::W](W) writer structure"]
impl crate::Writable for RF_FSM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_fsm to value 0"]
impl crate::Resettable for RF_FSM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
