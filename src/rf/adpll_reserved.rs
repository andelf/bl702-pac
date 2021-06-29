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
#[doc = "Field `adpll_resv0` reader - "]
pub struct ADPLL_RESV0_R(crate::FieldReader<u16, u16>);
impl ADPLL_RESV0_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADPLL_RESV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_RESV0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_resv0` writer - "]
pub struct ADPLL_RESV0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_RESV0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `adpll_resv1` reader - "]
pub struct ADPLL_RESV1_R(crate::FieldReader<u16, u16>);
impl ADPLL_RESV1_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADPLL_RESV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_RESV1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_resv1` writer - "]
pub struct ADPLL_RESV1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_RESV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn adpll_resv0(&self) -> ADPLL_RESV0_R {
        ADPLL_RESV0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn adpll_resv1(&self) -> ADPLL_RESV1_R {
        ADPLL_RESV1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn adpll_resv0(&mut self) -> ADPLL_RESV0_W {
        ADPLL_RESV0_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn adpll_resv1(&mut self) -> ADPLL_RESV1_W {
        ADPLL_RESV1_W { w: self }
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
