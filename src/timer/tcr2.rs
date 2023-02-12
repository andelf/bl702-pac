#[doc = "Register `TCR2` reader"]
pub struct R(crate::R<TCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR2` writer"]
pub struct W(crate::W<TCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR2_SPEC>;
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
impl From<crate::W<TCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tcr` reader - "]
pub type TCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `tcr` writer - "]
pub type TCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TCR_W<0> {
        TCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr2](index.html) module"]
pub struct TCR2_SPEC;
impl crate::RegisterSpec for TCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr2::R](R) reader structure"]
impl crate::Readable for TCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr2::W](W) writer structure"]
impl crate::Writable for TCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR2 to value 0"]
impl crate::Resettable for TCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
