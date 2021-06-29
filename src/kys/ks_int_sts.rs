#[doc = "Register `ks_int_sts` reader"]
pub struct R(crate::R<KS_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KS_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KS_INT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KS_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ks_int_sts` writer"]
pub struct W(crate::W<KS_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KS_INT_STS_SPEC>;
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
impl From<crate::W<KS_INT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KS_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `keycode_valid` reader - "]
pub struct KEYCODE_VALID_R(crate::FieldReader<u8, u8>);
impl KEYCODE_VALID_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYCODE_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYCODE_VALID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `keycode_valid` writer - "]
pub struct KEYCODE_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYCODE_VALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn keycode_valid(&self) -> KEYCODE_VALID_R {
        KEYCODE_VALID_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn keycode_valid(&mut self) -> KEYCODE_VALID_W {
        KEYCODE_VALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ks_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ks_int_sts](index.html) module"]
pub struct KS_INT_STS_SPEC;
impl crate::RegisterSpec for KS_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ks_int_sts::R](R) reader structure"]
impl crate::Readable for KS_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ks_int_sts::W](W) writer structure"]
impl crate::Writable for KS_INT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ks_int_sts to value 0"]
impl crate::Resettable for KS_INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
