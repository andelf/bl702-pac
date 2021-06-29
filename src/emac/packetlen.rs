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
#[doc = "Field `MINFL` reader - "]
pub struct MINFL_R(crate::FieldReader<u16, u16>);
impl MINFL_R {
    pub(crate) fn new(bits: u16) -> Self {
        MINFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINFL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINFL` writer - "]
pub struct MINFL_W<'a> {
    w: &'a mut W,
}
impl<'a> MINFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `MAXFL` reader - "]
pub struct MAXFL_R(crate::FieldReader<u16, u16>);
impl MAXFL_R {
    pub(crate) fn new(bits: u16) -> Self {
        MAXFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXFL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXFL` writer - "]
pub struct MAXFL_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXFL_W<'a> {
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
    pub fn minfl(&self) -> MINFL_R {
        MINFL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn maxfl(&self) -> MAXFL_R {
        MAXFL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn minfl(&mut self) -> MINFL_W {
        MINFL_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn maxfl(&mut self) -> MAXFL_W {
        MAXFL_W { w: self }
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
}
#[doc = "`reset()` method sets PACKETLEN to value 0"]
impl crate::Resettable for PACKETLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
