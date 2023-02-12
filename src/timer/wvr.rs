#[doc = "Register `WVR` reader"]
pub struct R(crate::R<WVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WVR` writer"]
pub struct W(crate::W<WVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WVR_SPEC>;
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
impl From<crate::W<WVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wvr` reader - "]
pub type WVR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wvr` writer - "]
pub type WVR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WVR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wvr(&self) -> WVR_R {
        WVR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn wvr(&mut self) -> WVR_W<0> {
        WVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WVR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvr](index.html) module"]
pub struct WVR_SPEC;
impl crate::RegisterSpec for WVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wvr::R](R) reader structure"]
impl crate::Readable for WVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wvr::W](W) writer structure"]
impl crate::Writable for WVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WVR to value 0"]
impl crate::Resettable for WVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
