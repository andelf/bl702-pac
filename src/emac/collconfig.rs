#[doc = "Register `COLLCONFIG` reader"]
pub struct R(crate::R<COLLCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLLCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLLCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLLCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLLCONFIG` writer"]
pub struct W(crate::W<COLLCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLLCONFIG_SPEC>;
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
impl From<crate::W<COLLCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLLCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXRET` reader - "]
pub struct MAXRET_R(crate::FieldReader<u8, u8>);
impl MAXRET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAXRET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAXRET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXRET` writer - "]
pub struct MAXRET_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXRET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `COLLVALID` reader - "]
pub struct COLLVALID_R(crate::FieldReader<u8, u8>);
impl COLLVALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        COLLVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLLVALID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLLVALID` writer - "]
pub struct COLLVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLVALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn maxret(&self) -> MAXRET_R {
        MAXRET_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn collvalid(&self) -> COLLVALID_R {
        COLLVALID_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn maxret(&mut self) -> MAXRET_W {
        MAXRET_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn collvalid(&mut self) -> COLLVALID_W {
        COLLVALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COLLCONFIG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [collconfig](index.html) module"]
pub struct COLLCONFIG_SPEC;
impl crate::RegisterSpec for COLLCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [collconfig::R](R) reader structure"]
impl crate::Readable for COLLCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [collconfig::W](W) writer structure"]
impl crate::Writable for COLLCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COLLCONFIG to value 0"]
impl crate::Resettable for COLLCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
