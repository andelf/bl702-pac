#[doc = "Register `TCVSYN2` reader"]
pub struct R(crate::R<TCVSYN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCVSYN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCVSYN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCVSYN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCVSYN2` writer"]
pub struct W(crate::W<TCVSYN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCVSYN2_SPEC>;
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
impl From<crate::W<TCVSYN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCVSYN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tcvsyn2` reader - "]
pub type TCVSYN2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `tcvsyn2` writer - "]
pub type TCVSYN2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCVSYN2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn2(&self) -> TCVSYN2_R {
        TCVSYN2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn tcvsyn2(&mut self) -> TCVSYN2_W<0> {
        TCVSYN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCVSYN2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvsyn2](index.html) module"]
pub struct TCVSYN2_SPEC;
impl crate::RegisterSpec for TCVSYN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcvsyn2::R](R) reader structure"]
impl crate::Readable for TCVSYN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcvsyn2::W](W) writer structure"]
impl crate::Writable for TCVSYN2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCVSYN2 to value 0"]
impl crate::Resettable for TCVSYN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
