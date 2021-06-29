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
pub struct IPGT_R(crate::FieldReader<u8, u8>);
impl IPGT_R {
    pub(crate) fn new(bits: u8) -> Self {
        IPGT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPGT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPGT` writer - "]
pub struct IPGT_W<'a> {
    w: &'a mut W,
}
impl<'a> IPGT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
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
    pub fn ipgt(&mut self) -> IPGT_W {
        IPGT_W { w: self }
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
}
#[doc = "`reset()` method sets IPGT to value 0"]
impl crate::Resettable for IPGT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
