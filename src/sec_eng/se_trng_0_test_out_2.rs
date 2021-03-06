#[doc = "Register `se_trng_0_test_out_2` reader"]
pub struct R(crate::R<SE_TRNG_0_TEST_OUT_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_TRNG_0_TEST_OUT_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_TRNG_0_TEST_OUT_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_TRNG_0_TEST_OUT_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_trng_0_test_out_2` writer"]
pub struct W(crate::W<SE_TRNG_0_TEST_OUT_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_TRNG_0_TEST_OUT_2_SPEC>;
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
impl From<crate::W<SE_TRNG_0_TEST_OUT_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_TRNG_0_TEST_OUT_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_trng_0_test_out_2` reader - "]
pub struct SE_TRNG_0_TEST_OUT_2_R(crate::FieldReader<u32, u32>);
impl SE_TRNG_0_TEST_OUT_2_R {
    pub(crate) fn new(bits: u32) -> Self {
        SE_TRNG_0_TEST_OUT_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_TEST_OUT_2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_test_out_2` writer - "]
pub struct SE_TRNG_0_TEST_OUT_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_TEST_OUT_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_test_out_2(&self) -> SE_TRNG_0_TEST_OUT_2_R {
        SE_TRNG_0_TEST_OUT_2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_trng_0_test_out_2(&mut self) -> SE_TRNG_0_TEST_OUT_2_W {
        SE_TRNG_0_TEST_OUT_2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_trng_0_test_out_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_test_out_2](index.html) module"]
pub struct SE_TRNG_0_TEST_OUT_2_SPEC;
impl crate::RegisterSpec for SE_TRNG_0_TEST_OUT_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_trng_0_test_out_2::R](R) reader structure"]
impl crate::Readable for SE_TRNG_0_TEST_OUT_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_trng_0_test_out_2::W](W) writer structure"]
impl crate::Writable for SE_TRNG_0_TEST_OUT_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_trng_0_test_out_2 to value 0"]
impl crate::Resettable for SE_TRNG_0_TEST_OUT_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
