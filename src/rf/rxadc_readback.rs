#[doc = "Register `rxadc_readback` reader"]
pub struct R(crate::R<RXADC_READBACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXADC_READBACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXADC_READBACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXADC_READBACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxadc_readback` writer"]
pub struct W(crate::W<RXADC_READBACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXADC_READBACK_SPEC>;
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
impl From<crate::W<RXADC_READBACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXADC_READBACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxadc_dout_q` reader - "]
pub type RXADC_DOUT_Q_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rxadc_dout_q` writer - "]
pub type RXADC_DOUT_Q_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXADC_READBACK_SPEC, u16, u16, 9, O>;
#[doc = "Field `rxadc_dout_i` reader - "]
pub type RXADC_DOUT_I_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rxadc_dout_i` writer - "]
pub type RXADC_DOUT_I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RXADC_READBACK_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rxadc_dout_q(&self) -> RXADC_DOUT_Q_R {
        RXADC_DOUT_Q_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rxadc_dout_i(&self) -> RXADC_DOUT_I_R {
        RXADC_DOUT_I_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rxadc_dout_q(&mut self) -> RXADC_DOUT_Q_W<0> {
        RXADC_DOUT_Q_W::new(self)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rxadc_dout_i(&mut self) -> RXADC_DOUT_I_W<16> {
        RXADC_DOUT_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rxadc_readback.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxadc_readback](index.html) module"]
pub struct RXADC_READBACK_SPEC;
impl crate::RegisterSpec for RXADC_READBACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxadc_readback::R](R) reader structure"]
impl crate::Readable for RXADC_READBACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxadc_readback::W](W) writer structure"]
impl crate::Writable for RXADC_READBACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rxadc_readback to value 0"]
impl crate::Resettable for RXADC_READBACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
