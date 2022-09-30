#[doc = "Register `rf_ext_pa` reader"]
pub struct R(crate::R<RF_EXT_PA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_EXT_PA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_EXT_PA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_EXT_PA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_ext_pa` writer"]
pub struct W(crate::W<RF_EXT_PA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_EXT_PA_SPEC>;
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
impl From<crate::W<RF_EXT_PA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_EXT_PA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_ext_pa_sb` reader - "]
pub type RF_EXT_PA_SB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_ext_pa_sb` writer - "]
pub type RF_EXT_PA_SB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_EXT_PA_SPEC, u8, u8, 5, O>;
#[doc = "Field `rf_ext_pa_lotx` reader - "]
pub type RF_EXT_PA_LOTX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_ext_pa_lotx` writer - "]
pub type RF_EXT_PA_LOTX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_EXT_PA_SPEC, u8, u8, 5, O>;
#[doc = "Field `rf_ext_pa_tx` reader - "]
pub type RF_EXT_PA_TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_ext_pa_tx` writer - "]
pub type RF_EXT_PA_TX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_EXT_PA_SPEC, u8, u8, 5, O>;
#[doc = "Field `rf_ext_pa_lorx` reader - "]
pub type RF_EXT_PA_LORX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_ext_pa_lorx` writer - "]
pub type RF_EXT_PA_LORX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_EXT_PA_SPEC, u8, u8, 5, O>;
#[doc = "Field `rf_ext_pa_rx` reader - "]
pub type RF_EXT_PA_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rf_ext_pa_rx` writer - "]
pub type RF_EXT_PA_RX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_EXT_PA_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ext_pa_sb(&self) -> RF_EXT_PA_SB_R {
        RF_EXT_PA_SB_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn rf_ext_pa_lotx(&self) -> RF_EXT_PA_LOTX_R {
        RF_EXT_PA_LOTX_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn rf_ext_pa_tx(&self) -> RF_EXT_PA_TX_R {
        RF_EXT_PA_TX_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn rf_ext_pa_lorx(&self) -> RF_EXT_PA_LORX_R {
        RF_EXT_PA_LORX_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn rf_ext_pa_rx(&self) -> RF_EXT_PA_RX_R {
        RF_EXT_PA_RX_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ext_pa_sb(&mut self) -> RF_EXT_PA_SB_W<0> {
        RF_EXT_PA_SB_W::new(self)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn rf_ext_pa_lotx(&mut self) -> RF_EXT_PA_LOTX_W<5> {
        RF_EXT_PA_LOTX_W::new(self)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn rf_ext_pa_tx(&mut self) -> RF_EXT_PA_TX_W<10> {
        RF_EXT_PA_TX_W::new(self)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn rf_ext_pa_lorx(&mut self) -> RF_EXT_PA_LORX_W<15> {
        RF_EXT_PA_LORX_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn rf_ext_pa_rx(&mut self) -> RF_EXT_PA_RX_W<20> {
        RF_EXT_PA_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_ext_pa.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ext_pa](index.html) module"]
pub struct RF_EXT_PA_SPEC;
impl crate::RegisterSpec for RF_EXT_PA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ext_pa::R](R) reader structure"]
impl crate::Readable for RF_EXT_PA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ext_pa::W](W) writer structure"]
impl crate::Writable for RF_EXT_PA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_ext_pa to value 0"]
impl crate::Resettable for RF_EXT_PA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
