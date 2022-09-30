#[doc = "Register `TCVWR2` reader"]
pub struct R(crate::R<TCVWR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCVWR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCVWR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCVWR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCVWR2` writer"]
pub struct W(crate::W<TCVWR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCVWR2_SPEC>;
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
impl From<crate::W<TCVWR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCVWR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tcvwr` reader - "]
pub type TCVWR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `tcvwr` writer - "]
pub type TCVWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCVWR2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvwr(&self) -> TCVWR_R {
        TCVWR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvwr(&mut self) -> TCVWR_W<0> {
        TCVWR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCVWR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvwr2](index.html) module"]
pub struct TCVWR2_SPEC;
impl crate::RegisterSpec for TCVWR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcvwr2::R](R) reader structure"]
impl crate::Readable for TCVWR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcvwr2::W](W) writer structure"]
impl crate::Writable for TCVWR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCVWR2 to value 0"]
impl crate::Resettable for TCVWR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
