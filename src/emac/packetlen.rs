#[doc = "Register `PACKETLEN` reader"]
pub struct R(crate::R<PACKETLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKETLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACKETLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACKETLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PACKETLEN` writer"]
pub struct W(crate::W<PACKETLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PACKETLEN_SPEC>;
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
impl From<crate::W<PACKETLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PACKETLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXFL` reader - "]
pub type MAXFL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAXFL` writer - "]
pub type MAXFL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PACKETLEN_SPEC, u16, u16, 16, O>;
#[doc = "Field `MINFL` reader - "]
pub type MINFL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MINFL` writer - "]
pub type MINFL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PACKETLEN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn maxfl(&self) -> MAXFL_R {
        MAXFL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn minfl(&self) -> MINFL_R {
        MINFL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn maxfl(&mut self) -> MAXFL_W<0> {
        MAXFL_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn minfl(&mut self) -> MINFL_W<16> {
        MINFL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PACKETLEN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [packetlen](index.html) module"]
pub struct PACKETLEN_SPEC;
impl crate::RegisterSpec for PACKETLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [packetlen::R](R) reader structure"]
impl crate::Readable for PACKETLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [packetlen::W](W) writer structure"]
impl crate::Writable for PACKETLEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PACKETLEN to value 0"]
impl crate::Resettable for PACKETLEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
