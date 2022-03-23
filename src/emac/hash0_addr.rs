#[doc = "Register `HASH0_ADDR` reader"]
pub struct R(crate::R<HASH0_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH0_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH0_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH0_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH0_ADDR` writer"]
pub struct W(crate::W<HASH0_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH0_ADDR_SPEC>;
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
impl From<crate::W<HASH0_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH0_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HASH0` reader - "]
pub struct HASH0_R(crate::FieldReader<u32, u32>);
impl HASH0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        HASH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASH0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH0` writer - "]
pub struct HASH0_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hash0(&self) -> HASH0_R {
        HASH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hash0(&mut self) -> HASH0_W {
        HASH0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH0_ADDR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash0_addr](index.html) module"]
pub struct HASH0_ADDR_SPEC;
impl crate::RegisterSpec for HASH0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash0_addr::R](R) reader structure"]
impl crate::Readable for HASH0_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash0_addr::W](W) writer structure"]
impl crate::Writable for HASH0_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH0_ADDR to value 0"]
impl crate::Resettable for HASH0_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
