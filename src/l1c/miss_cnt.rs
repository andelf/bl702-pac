#[doc = "Register `miss_cnt` reader"]
pub struct R(crate::R<MISS_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISS_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISS_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISS_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `miss_cnt` writer"]
pub struct W(crate::W<MISS_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISS_CNT_SPEC>;
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
impl From<crate::W<MISS_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISS_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `miss_cnt` reader - "]
pub type MISS_CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `miss_cnt` writer - "]
pub type MISS_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MISS_CNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn miss_cnt(&self) -> MISS_CNT_R {
        MISS_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn miss_cnt(&mut self) -> MISS_CNT_W<0> {
        MISS_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "miss_cnt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miss_cnt](index.html) module"]
pub struct MISS_CNT_SPEC;
impl crate::RegisterSpec for MISS_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miss_cnt::R](R) reader structure"]
impl crate::Readable for MISS_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miss_cnt::W](W) writer structure"]
impl crate::Writable for MISS_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets miss_cnt to value 0"]
impl crate::Resettable for MISS_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
