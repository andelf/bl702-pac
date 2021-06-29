#[doc = "Register `MIIADDRESS` reader"]
pub struct R(crate::R<MIIADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIIADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIIADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIIADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIIADDRESS` writer"]
pub struct W(crate::W<MIIADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIIADDRESS_SPEC>;
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
impl From<crate::W<MIIADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIIADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RGAD` reader - "]
pub struct RGAD_R(crate::FieldReader<u8, u8>);
impl RGAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RGAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RGAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RGAD` writer - "]
pub struct RGAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RGAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `FIAD` reader - "]
pub struct FIAD_R(crate::FieldReader<u8, u8>);
impl FIAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIAD` writer - "]
pub struct FIAD_W<'a> {
    w: &'a mut W,
}
impl<'a> FIAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rgad(&self) -> RGAD_R {
        RGAD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn fiad(&self) -> FIAD_R {
        FIAD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rgad(&mut self) -> RGAD_W {
        RGAD_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn fiad(&mut self) -> FIAD_W {
        FIAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIIADDRESS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miiaddress](index.html) module"]
pub struct MIIADDRESS_SPEC;
impl crate::RegisterSpec for MIIADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miiaddress::R](R) reader structure"]
impl crate::Readable for MIIADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miiaddress::W](W) writer structure"]
impl crate::Writable for MIIADDRESS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIIADDRESS to value 0"]
impl crate::Resettable for MIIADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
