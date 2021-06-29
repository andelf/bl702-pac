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
pub struct CHIP_REV_R(crate::FieldReader<u8, u8>);
impl CHIP_REV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHIP_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIP_REV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `chip_rev` writer - "]
pub struct CHIP_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIP_REV_W<'a> {
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
    pub fn chip_rev(&self) -> CHIP_REV_R {
        CHIP_REV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn chip_rev(&mut self) -> CHIP_REV_W {
        CHIP_REV_W { w: self }
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
}
#[doc = "`reset()` method sets chip_revision to value 0"]
impl crate::Resettable for CHIP_REVISION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
