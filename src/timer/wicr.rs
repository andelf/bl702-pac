#[doc = "Register `WICR` reader"]
pub struct R(crate::R<WICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WICR` writer"]
pub struct W(crate::W<WICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WICR_SPEC>;
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
impl From<crate::W<WICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wiclr` reader - "]
pub type WICLR_R = crate::BitReader<bool>;
#[doc = "Field `wiclr` writer - "]
pub type WICLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wiclr(&self) -> WICLR_R {
        WICLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wiclr(&mut self) -> WICLR_W<0> {
        WICLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WICR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wicr](index.html) module"]
pub struct WICR_SPEC;
impl crate::RegisterSpec for WICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wicr::R](R) reader structure"]
impl crate::Readable for WICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wicr::W](W) writer structure"]
impl crate::Writable for WICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WICR to value 0"]
impl crate::Resettable for WICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
