#[doc = "Register `uart_fifo_config_1` reader"]
pub struct R(crate::R<UART_FIFO_CONFIG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_FIFO_CONFIG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_FIFO_CONFIG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_FIFO_CONFIG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_fifo_config_1` writer"]
pub struct W(crate::W<UART_FIFO_CONFIG_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FIFO_CONFIG_1_SPEC>;
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
impl From<crate::W<UART_FIFO_CONFIG_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_FIFO_CONFIG_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_fifo_cnt` reader - "]
pub type TX_FIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_fifo_cnt` writer - "]
pub type TX_FIFO_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_FIFO_CONFIG_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `rx_fifo_cnt` reader - "]
pub type RX_FIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_fifo_cnt` writer - "]
pub type RX_FIFO_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_FIFO_CONFIG_1_SPEC, u8, u8, 8, O>;
#[doc = "Field `tx_fifo_th` reader - "]
pub type TX_FIFO_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tx_fifo_th` writer - "]
pub type TX_FIFO_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_FIFO_CONFIG_1_SPEC, u8, u8, 7, O>;
#[doc = "Field `rx_fifo_th` reader - "]
pub type RX_FIFO_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rx_fifo_th` writer - "]
pub type RX_FIFO_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UART_FIFO_CONFIG_1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TX_FIFO_TH_R {
        TX_FIFO_TH_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn rx_fifo_th(&self) -> RX_FIFO_TH_R {
        RX_FIFO_TH_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&mut self) -> TX_FIFO_CNT_W<0> {
        TX_FIFO_CNT_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&mut self) -> RX_FIFO_CNT_W<8> {
        RX_FIFO_CNT_W::new(self)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_fifo_th(&mut self) -> TX_FIFO_TH_W<16> {
        TX_FIFO_TH_W::new(self)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn rx_fifo_th(&mut self) -> RX_FIFO_TH_W<24> {
        RX_FIFO_TH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_fifo_config_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo_config_1](index.html) module"]
pub struct UART_FIFO_CONFIG_1_SPEC;
impl crate::RegisterSpec for UART_FIFO_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_fifo_config_1::R](R) reader structure"]
impl crate::Readable for UART_FIFO_CONFIG_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_fifo_config_1::W](W) writer structure"]
impl crate::Writable for UART_FIFO_CONFIG_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets uart_fifo_config_1 to value 0"]
impl crate::Resettable for UART_FIFO_CONFIG_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
