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
pub struct KEYCODE_CLR_R(crate::FieldReader<u8, u8>);
impl KEYCODE_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEYCODE_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYCODE_CLR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `keycode_clr` writer - "]
pub struct KEYCODE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYCODE_CLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
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
    pub fn keycode_clr(&mut self) -> KEYCODE_CLR_W {
        KEYCODE_CLR_W { w: self }
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
