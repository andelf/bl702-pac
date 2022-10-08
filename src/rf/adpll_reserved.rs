#[doc = "Register `adpll_reserved` reader"]
pub struct R(crate::R<ADPLL_RESERVED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_RESERVED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_RESERVED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_RESERVED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_reserved` writer"]
pub struct W(crate::W<ADPLL_RESERVED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_RESERVED_SPEC>;
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
impl From<crate::W<ADPLL_RESERVED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_RESERVED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_resv1` reader - "]
pub type ADPLL_RESV1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `adpll_resv1` writer - "]
pub type ADPLL_RESV1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_RESERVED_SPEC, u16, u16, 16, O>;
#[doc = "Field `adpll_resv0` reader - "]
pub type ADPLL_RESV0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `adpll_resv0` writer - "]
pub type ADPLL_RESV0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_RESERVED_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn adpll_resv1(&self) -> ADPLL_RESV1_R {
        ADPLL_RESV1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn adpll_resv0(&self) -> ADPLL_RESV0_R {
        ADPLL_RESV0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn adpll_resv1(&mut self) -> ADPLL_RESV1_W<0> {
        ADPLL_RESV1_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn adpll_resv0(&mut self) -> ADPLL_RESV0_W<16> {
        ADPLL_RESV0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_reserved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_reserved](index.html) module"]
pub struct ADPLL_RESERVED_SPEC;
impl crate::RegisterSpec for ADPLL_RESERVED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_reserved::R](R) reader structure"]
impl crate::Readable for ADPLL_RESERVED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_reserved::W](W) writer structure"]
impl crate::Writable for ADPLL_RESERVED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_reserved to value 0"]
impl crate::Resettable for ADPLL_RESERVED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
