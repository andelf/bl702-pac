#[doc = "Register `chip_revision` reader"]
pub struct R(crate::R<CHIP_REVISION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIP_REVISION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIP_REVISION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIP_REVISION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `chip_revision` writer"]
pub struct W(crate::W<CHIP_REVISION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIP_REVISION_SPEC>;
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
impl From<crate::W<CHIP_REVISION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIP_REVISION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `chip_rev` reader - "]
pub type CHIP_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `chip_rev` writer - "]
pub type CHIP_REV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHIP_REVISION_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn chip_rev(&self) -> CHIP_REV_R {
        CHIP_REV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn chip_rev(&mut self) -> CHIP_REV_W<0> {
        CHIP_REV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "chip_revision.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chip_revision](index.html) module"]
pub struct CHIP_REVISION_SPEC;
impl crate::RegisterSpec for CHIP_REVISION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chip_revision::R](R) reader structure"]
impl crate::Readable for CHIP_REVISION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chip_revision::W](W) writer structure"]
impl crate::Writable for CHIP_REVISION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets chip_revision to value 0"]
impl crate::Resettable for CHIP_REVISION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
