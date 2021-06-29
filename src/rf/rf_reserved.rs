#[doc = "Register `rf_reserved` reader"]
pub struct R(crate::R<RF_RESERVED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_RESERVED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_RESERVED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_RESERVED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_reserved` writer"]
pub struct W(crate::W<RF_RESERVED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_RESERVED_SPEC>;
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
impl From<crate::W<RF_RESERVED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_RESERVED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_resv0` reader - "]
pub struct RF_RESV0_R(crate::FieldReader<u16, u16>);
impl RF_RESV0_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_RESV0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RESV0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_resv0` writer - "]
pub struct RF_RESV0_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RESV0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `rf_resv1` reader - "]
pub struct RF_RESV1_R(crate::FieldReader<u16, u16>);
impl RF_RESV1_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_RESV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RESV1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_resv1` writer - "]
pub struct RF_RESV1_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RESV1_W<'a> {
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
    pub fn rf_resv0(&self) -> RF_RESV0_R {
        RF_RESV0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_resv1(&self) -> RF_RESV1_R {
        RF_RESV1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rf_resv0(&mut self) -> RF_RESV0_W {
        RF_RESV0_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_resv1(&mut self) -> RF_RESV1_W {
        RF_RESV1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_reserved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_reserved](index.html) module"]
pub struct RF_RESERVED_SPEC;
impl crate::RegisterSpec for RF_RESERVED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_reserved::R](R) reader structure"]
impl crate::Readable for RF_RESERVED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_reserved::W](W) writer structure"]
impl crate::Writable for RF_RESERVED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_reserved to value 0"]
impl crate::Resettable for RF_RESERVED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
