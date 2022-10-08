#[doc = "Register `keycode_clr` reader"]
pub struct R(crate::R<KEYCODE_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYCODE_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYCODE_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYCODE_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `keycode_clr` writer"]
pub struct W(crate::W<KEYCODE_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYCODE_CLR_SPEC>;
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
impl From<crate::W<KEYCODE_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYCODE_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `keycode_clr` reader - "]
pub type KEYCODE_CLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `keycode_clr` writer - "]
pub type KEYCODE_CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEYCODE_CLR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn keycode_clr(&self) -> KEYCODE_CLR_R {
        KEYCODE_CLR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn keycode_clr(&mut self) -> KEYCODE_CLR_W<0> {
        KEYCODE_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "keycode_clr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keycode_clr](index.html) module"]
pub struct KEYCODE_CLR_SPEC;
impl crate::RegisterSpec for KEYCODE_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keycode_clr::R](R) reader structure"]
impl crate::Readable for KEYCODE_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keycode_clr::W](W) writer structure"]
impl crate::Writable for KEYCODE_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets keycode_clr to value 0"]
impl crate::Resettable for KEYCODE_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
