#[doc = "Register `rf_reserved` reader"]
pub struct R(crate::R<RF_RESERVED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_RESERVED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_RESERVED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_RESERVED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_reserved` writer"]
pub struct W(crate::W<RF_RESERVED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_RESERVED_SPEC>;
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
impl From<crate::W<RF_RESERVED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_RESERVED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_resv1` reader - "]
pub type RF_RESV1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_resv1` writer - "]
pub type RF_RESV1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_RESERVED_SPEC, u16, u16, 16, O>;
#[doc = "Field `rf_resv0` reader - "]
pub type RF_RESV0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `rf_resv0` writer - "]
pub type RF_RESV0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_RESERVED_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_resv1(&self) -> RF_RESV1_R {
        RF_RESV1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_resv0(&self) -> RF_RESV0_R {
        RF_RESV0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn rf_resv1(&mut self) -> RF_RESV1_W<0> {
        RF_RESV1_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn rf_resv0(&mut self) -> RF_RESV0_W<16> {
        RF_RESV0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_reserved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reserved](index.html) module"]
pub struct RF_RESERVED_SPEC;
impl crate::RegisterSpec for RF_RESERVED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_reserved::R](R) reader structure"]
impl crate::Readable for RF_RESERVED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_reserved::W](W) writer structure"]
impl crate::Writable for RF_RESERVED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_reserved to value 0"]
impl crate::Resettable for RF_RESERVED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
