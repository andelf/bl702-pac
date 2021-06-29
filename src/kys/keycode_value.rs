#[doc = "Register `keycode_value` reader"]
pub struct R(crate::R<KEYCODE_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYCODE_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYCODE_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYCODE_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `keycode_value` writer"]
pub struct W(crate::W<KEYCODE_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYCODE_VALUE_SPEC>;
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
impl From<crate::W<KEYCODE_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYCODE_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `keycode3` reader - "]
pub struct KEYCODE3_R(crate::FieldReader<u8, u8>);
impl KEYCODE3_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYCODE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYCODE3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `keycode3` writer - "]
pub struct KEYCODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYCODE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `keycode2` reader - "]
pub struct KEYCODE2_R(crate::FieldReader<u8, u8>);
impl KEYCODE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYCODE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYCODE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `keycode2` writer - "]
pub struct KEYCODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYCODE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `keycode1` reader - "]
pub struct KEYCODE1_R(crate::FieldReader<u8, u8>);
impl KEYCODE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYCODE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYCODE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `keycode1` writer - "]
pub struct KEYCODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYCODE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `keycode0` reader - "]
pub struct KEYCODE0_R(crate::FieldReader<u8, u8>);
impl KEYCODE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYCODE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYCODE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `keycode0` writer - "]
pub struct KEYCODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYCODE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn keycode3(&self) -> KEYCODE3_R {
        KEYCODE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn keycode2(&self) -> KEYCODE2_R {
        KEYCODE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn keycode1(&self) -> KEYCODE1_R {
        KEYCODE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn keycode0(&self) -> KEYCODE0_R {
        KEYCODE0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn keycode3(&mut self) -> KEYCODE3_W {
        KEYCODE3_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn keycode2(&mut self) -> KEYCODE2_W {
        KEYCODE2_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn keycode1(&mut self) -> KEYCODE1_W {
        KEYCODE1_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn keycode0(&mut self) -> KEYCODE0_W {
        KEYCODE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "keycode_value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keycode_value](index.html) module"]
pub struct KEYCODE_VALUE_SPEC;
impl crate::RegisterSpec for KEYCODE_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keycode_value::R](R) reader structure"]
impl crate::Readable for KEYCODE_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keycode_value::W](W) writer structure"]
impl crate::Writable for KEYCODE_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets keycode_value to value 0"]
impl crate::Resettable for KEYCODE_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
