#[doc = "Register `rf_singen_1` reader"]
pub struct R(crate::R<RF_SINGEN_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_SINGEN_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_SINGEN_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_SINGEN_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_singen_1` writer"]
pub struct W(crate::W<RF_SINGEN_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_SINGEN_1_SPEC>;
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
impl From<crate::W<RF_SINGEN_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_SINGEN_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `singen_clkdiv_q` reader - "]
pub type SINGEN_CLKDIV_Q_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_clkdiv_q` writer - "]
pub type SINGEN_CLKDIV_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SINGEN_1_SPEC, u16, u16, 10, O>;
#[doc = "Field `singen_mode_q` reader - "]
pub type SINGEN_MODE_Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `singen_mode_q` writer - "]
pub type SINGEN_MODE_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SINGEN_1_SPEC, u8, u8, 4, O>;
#[doc = "Field `singen_clkdiv_i` reader - "]
pub type SINGEN_CLKDIV_I_R = crate::FieldReader<u16, u16>;
#[doc = "Field `singen_clkdiv_i` writer - "]
pub type SINGEN_CLKDIV_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SINGEN_1_SPEC, u16, u16, 10, O>;
#[doc = "Field `singen_mode_i` reader - "]
pub type SINGEN_MODE_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `singen_mode_i` writer - "]
pub type SINGEN_MODE_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_SINGEN_1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_clkdiv_q(&self) -> SINGEN_CLKDIV_Q_R {
        SINGEN_CLKDIV_Q_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn singen_mode_q(&self) -> SINGEN_MODE_Q_R {
        SINGEN_MODE_Q_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_clkdiv_i(&self) -> SINGEN_CLKDIV_I_R {
        SINGEN_CLKDIV_I_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn singen_mode_i(&self) -> SINGEN_MODE_I_R {
        SINGEN_MODE_I_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn singen_clkdiv_q(&mut self) -> SINGEN_CLKDIV_Q_W<0> {
        SINGEN_CLKDIV_Q_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn singen_mode_q(&mut self) -> SINGEN_MODE_Q_W<12> {
        SINGEN_MODE_Q_W::new(self)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn singen_clkdiv_i(&mut self) -> SINGEN_CLKDIV_I_W<16> {
        SINGEN_CLKDIV_I_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn singen_mode_i(&mut self) -> SINGEN_MODE_I_W<28> {
        SINGEN_MODE_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_singen_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_singen_1](index.html) module"]
pub struct RF_SINGEN_1_SPEC;
impl crate::RegisterSpec for RF_SINGEN_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_singen_1::R](R) reader structure"]
impl crate::Readable for RF_SINGEN_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_singen_1::W](W) writer structure"]
impl crate::Writable for RF_SINGEN_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_singen_1 to value 0"]
impl crate::Resettable for RF_SINGEN_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
