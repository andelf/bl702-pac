#[doc = "Register `TCVSYN3` reader"]
pub struct R(crate::R<TCVSYN3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCVSYN3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCVSYN3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCVSYN3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCVSYN3` writer"]
pub struct W(crate::W<TCVSYN3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCVSYN3_SPEC>;
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
impl From<crate::W<TCVSYN3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCVSYN3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tcvsyn3` reader - "]
pub type TCVSYN3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `tcvsyn3` writer - "]
pub type TCVSYN3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCVSYN3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn3(&self) -> TCVSYN3_R {
        TCVSYN3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn3(&mut self) -> TCVSYN3_W<0> {
        TCVSYN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCVSYN3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvsyn3](index.html) module"]
pub struct TCVSYN3_SPEC;
impl crate::RegisterSpec for TCVSYN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcvsyn3::R](R) reader structure"]
impl crate::Readable for TCVSYN3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcvsyn3::W](W) writer structure"]
impl crate::Writable for TCVSYN3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCVSYN3 to value 0"]
impl crate::Resettable for TCVSYN3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
