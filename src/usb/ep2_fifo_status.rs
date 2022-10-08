#[doc = "Register `ep2_fifo_status` reader"]
pub struct R(crate::R<EP2_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP2_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP2_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP2_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep2_fifo_status` writer"]
pub struct W(crate::W<EP2_FIFO_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP2_FIFO_STATUS_SPEC>;
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
impl From<crate::W<EP2_FIFO_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP2_FIFO_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ep2_tx_fifo_cnt` reader - "]
pub type EP2_TX_FIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ep2_tx_fifo_cnt` writer - "]
pub type EP2_TX_FIFO_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP2_FIFO_STATUS_SPEC, u8, u8, 7, O>;
#[doc = "Field `ep2_tx_fifo_empty` reader - "]
pub type EP2_TX_FIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `ep2_tx_fifo_empty` writer - "]
pub type EP2_TX_FIFO_EMPTY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP2_FIFO_STATUS_SPEC, bool, O>;
#[doc = "Field `ep2_tx_fifo_full` reader - "]
pub type EP2_TX_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `ep2_tx_fifo_full` writer - "]
pub type EP2_TX_FIFO_FULL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP2_FIFO_STATUS_SPEC, bool, O>;
#[doc = "Field `ep2_rx_fifo_cnt` reader - "]
pub type EP2_RX_FIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ep2_rx_fifo_cnt` writer - "]
pub type EP2_RX_FIFO_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP2_FIFO_STATUS_SPEC, u8, u8, 7, O>;
#[doc = "Field `ep2_rx_fifo_empty` reader - "]
pub type EP2_RX_FIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `ep2_rx_fifo_empty` writer - "]
pub type EP2_RX_FIFO_EMPTY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP2_FIFO_STATUS_SPEC, bool, O>;
#[doc = "Field `ep2_rx_fifo_full` reader - "]
pub type EP2_RX_FIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `ep2_rx_fifo_full` writer - "]
pub type EP2_RX_FIFO_FULL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EP2_FIFO_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn ep2_tx_fifo_cnt(&self) -> EP2_TX_FIFO_CNT_R {
        EP2_TX_FIFO_CNT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep2_tx_fifo_empty(&self) -> EP2_TX_FIFO_EMPTY_R {
        EP2_TX_FIFO_EMPTY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep2_tx_fifo_full(&self) -> EP2_TX_FIFO_FULL_R {
        EP2_TX_FIFO_FULL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn ep2_rx_fifo_cnt(&self) -> EP2_RX_FIFO_CNT_R {
        EP2_RX_FIFO_CNT_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ep2_rx_fifo_empty(&self) -> EP2_RX_FIFO_EMPTY_R {
        EP2_RX_FIFO_EMPTY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ep2_rx_fifo_full(&self) -> EP2_RX_FIFO_FULL_R {
        EP2_RX_FIFO_FULL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn ep2_tx_fifo_cnt(&mut self) -> EP2_TX_FIFO_CNT_W<0> {
        EP2_TX_FIFO_CNT_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep2_tx_fifo_empty(&mut self) -> EP2_TX_FIFO_EMPTY_W<14> {
        EP2_TX_FIFO_EMPTY_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep2_tx_fifo_full(&mut self) -> EP2_TX_FIFO_FULL_W<15> {
        EP2_TX_FIFO_FULL_W::new(self)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn ep2_rx_fifo_cnt(&mut self) -> EP2_RX_FIFO_CNT_W<16> {
        EP2_RX_FIFO_CNT_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ep2_rx_fifo_empty(&mut self) -> EP2_RX_FIFO_EMPTY_W<30> {
        EP2_RX_FIFO_EMPTY_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ep2_rx_fifo_full(&mut self) -> EP2_RX_FIFO_FULL_W<31> {
        EP2_RX_FIFO_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep2_fifo_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2_fifo_status](index.html) module"]
pub struct EP2_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for EP2_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep2_fifo_status::R](R) reader structure"]
impl crate::Readable for EP2_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep2_fifo_status::W](W) writer structure"]
impl crate::Writable for EP2_FIFO_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ep2_fifo_status to value 0"]
impl crate::Resettable for EP2_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
