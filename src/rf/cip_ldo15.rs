#[doc = "Register `cip_ldo15` reader"]
pub struct R(crate::R<CIP_LDO15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIP_LDO15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIP_LDO15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIP_LDO15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cip_ldo15` writer"]
pub struct W(crate::W<CIP_LDO15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIP_LDO15_SPEC>;
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
impl From<crate::W<CIP_LDO15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIP_LDO15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vg11_sel` reader - "]
pub type VG11_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vg11_sel` writer - "]
pub type VG11_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CIP_LDO15_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vg11_sel(&self) -> VG11_SEL_R {
        VG11_SEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn vg11_sel(&mut self) -> VG11_SEL_W<0> {
        VG11_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cip_ldo15.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cip_ldo15](index.html) module"]
pub struct CIP_LDO15_SPEC;
impl crate::RegisterSpec for CIP_LDO15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cip_ldo15::R](R) reader structure"]
impl crate::Readable for CIP_LDO15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cip_ldo15::W](W) writer structure"]
impl crate::Writable for CIP_LDO15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets cip_ldo15 to value 0"]
impl crate::Resettable for CIP_LDO15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
