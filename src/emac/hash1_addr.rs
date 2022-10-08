#[doc = "Register `HASH1_ADDR` reader"]
pub struct R(crate::R<HASH1_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH1_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH1_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH1_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH1_ADDR` writer"]
pub struct W(crate::W<HASH1_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH1_ADDR_SPEC>;
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
impl From<crate::W<HASH1_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH1_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HASH1` reader - "]
pub type HASH1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HASH1` writer - "]
pub type HASH1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HASH1_ADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hash1(&self) -> HASH1_R {
        HASH1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hash1(&mut self) -> HASH1_W<0> {
        HASH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH1_ADDR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash1_addr](index.html) module"]
pub struct HASH1_ADDR_SPEC;
impl crate::RegisterSpec for HASH1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash1_addr::R](R) reader structure"]
impl crate::Readable for HASH1_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash1_addr::W](W) writer structure"]
impl crate::Writable for HASH1_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH1_ADDR to value 0"]
impl crate::Resettable for HASH1_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
