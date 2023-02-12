#[doc = "Register `rsvd_0` reader"]
pub struct R(crate::R<RSVD_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSVD_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSVD_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSVD_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rsvd_0` writer"]
pub struct W(crate::W<RSVD_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSVD_0_SPEC>;
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
impl From<crate::W<RSVD_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSVD_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rsvd_0` reader - "]
pub type RSVD_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `rsvd_0` writer - "]
pub type RSVD_0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSVD_0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rsvd_0(&self) -> RSVD_0_R {
        RSVD_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_0(&mut self) -> RSVD_0_W<0> {
        RSVD_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rsvd_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsvd_0](index.html) module"]
pub struct RSVD_0_SPEC;
impl crate::RegisterSpec for RSVD_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsvd_0::R](R) reader structure"]
impl crate::Readable for RSVD_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsvd_0::W](W) writer structure"]
impl crate::Writable for RSVD_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rsvd_0 to value 0"]
impl crate::Resettable for RSVD_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
