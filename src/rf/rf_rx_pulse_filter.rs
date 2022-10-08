#[doc = "Register `rf_rx_pulse_filter` reader"]
pub struct R(crate::R<RF_RX_PULSE_FILTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_RX_PULSE_FILTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_RX_PULSE_FILTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_RX_PULSE_FILTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_rx_pulse_filter` writer"]
pub struct W(crate::W<RF_RX_PULSE_FILTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_RX_PULSE_FILTER_SPEC>;
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
impl From<crate::W<RF_RX_PULSE_FILTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_RX_PULSE_FILTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pf_th2` reader - "]
pub type PF_TH2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pf_th2` writer - "]
pub type PF_TH2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_RX_PULSE_FILTER_SPEC, u16, u16, 9, O>;
#[doc = "Field `pf_th1` reader - "]
pub type PF_TH1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `pf_th1` writer - "]
pub type PF_TH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_RX_PULSE_FILTER_SPEC, u16, u16, 9, O>;
#[doc = "Field `pf_en_q` reader - "]
pub type PF_EN_Q_R = crate::BitReader<bool>;
#[doc = "Field `pf_en_q` writer - "]
pub type PF_EN_Q_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_RX_PULSE_FILTER_SPEC, bool, O>;
#[doc = "Field `pf_en_i` reader - "]
pub type PF_EN_I_R = crate::BitReader<bool>;
#[doc = "Field `pf_en_i` writer - "]
pub type PF_EN_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_RX_PULSE_FILTER_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pf_th2(&self) -> PF_TH2_R {
        PF_TH2_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    pub fn pf_th1(&self) -> PF_TH1_R {
        PF_TH1_R::new(((self.bits >> 10) & 0x01ff) as u16)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pf_en_q(&self) -> PF_EN_Q_R {
        PF_EN_Q_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pf_en_i(&self) -> PF_EN_I_R {
        PF_EN_I_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn pf_th2(&mut self) -> PF_TH2_W<0> {
        PF_TH2_W::new(self)
    }
    #[doc = "Bits 10:18"]
    #[inline(always)]
    pub fn pf_th1(&mut self) -> PF_TH1_W<10> {
        PF_TH1_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pf_en_q(&mut self) -> PF_EN_Q_W<20> {
        PF_EN_Q_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pf_en_i(&mut self) -> PF_EN_I_W<21> {
        PF_EN_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_rx_pulse_filter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_rx_pulse_filter](index.html) module"]
pub struct RF_RX_PULSE_FILTER_SPEC;
impl crate::RegisterSpec for RF_RX_PULSE_FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_rx_pulse_filter::R](R) reader structure"]
impl crate::Readable for RF_RX_PULSE_FILTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_rx_pulse_filter::W](W) writer structure"]
impl crate::Writable for RF_RX_PULSE_FILTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_rx_pulse_filter to value 0"]
impl crate::Resettable for RF_RX_PULSE_FILTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
