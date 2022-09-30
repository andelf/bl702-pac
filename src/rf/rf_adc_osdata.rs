#[doc = "Register `rf_adc_osdata` reader"]
pub struct R(crate::R<RF_ADC_OSDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ADC_OSDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_ADC_OSDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_ADC_OSDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_adc_osdata` writer"]
pub struct W(crate::W<RF_ADC_OSDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ADC_OSDATA_SPEC>;
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
impl From<crate::W<RF_ADC_OSDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_ADC_OSDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxadc_os_q` reader - "]
pub type RXADC_OS_Q_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rxadc_os_q` writer - "]
pub type RXADC_OS_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_ADC_OSDATA_SPEC, u16, u16, 9, O>;
#[doc = "Field `rxadc_os_i` reader - "]
pub type RXADC_OS_I_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rxadc_os_i` writer - "]
pub type RXADC_OS_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_ADC_OSDATA_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rxadc_os_q(&self) -> RXADC_OS_Q_R {
        RXADC_OS_Q_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rxadc_os_i(&self) -> RXADC_OS_I_R {
        RXADC_OS_I_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rxadc_os_q(&mut self) -> RXADC_OS_Q_W<0> {
        RXADC_OS_Q_W::new(self)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rxadc_os_i(&mut self) -> RXADC_OS_I_W<16> {
        RXADC_OS_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_adc_osdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_adc_osdata](index.html) module"]
pub struct RF_ADC_OSDATA_SPEC;
impl crate::RegisterSpec for RF_ADC_OSDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_adc_osdata::R](R) reader structure"]
impl crate::Readable for RF_ADC_OSDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_adc_osdata::W](W) writer structure"]
impl crate::Writable for RF_ADC_OSDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_adc_osdata to value 0"]
impl crate::Resettable for RF_ADC_OSDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
