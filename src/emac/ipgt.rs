#[doc = "Register `IPGT` reader"]
pub struct R(crate::R<IPGT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPGT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPGT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPGT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPGT` writer"]
pub struct W(crate::W<IPGT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPGT_SPEC>;
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
impl From<crate::W<IPGT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPGT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPGT` reader - "]
pub type IPGT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPGT` writer - "]
pub type IPGT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPGT_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn ipgt(&self) -> IPGT_R {
        IPGT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn ipgt(&mut self) -> IPGT_W<0> {
        IPGT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPGT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipgt](index.html) module"]
pub struct IPGT_SPEC;
impl crate::RegisterSpec for IPGT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipgt::R](R) reader structure"]
impl crate::Readable for IPGT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipgt::W](W) writer structure"]
impl crate::Writable for IPGT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPGT to value 0"]
impl crate::Resettable for IPGT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
