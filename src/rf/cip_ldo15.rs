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
pub struct VG11_SEL_R(crate::FieldReader<u8, u8>);
impl VG11_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VG11_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VG11_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vg11_sel` writer - "]
pub struct VG11_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VG11_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vg11_sel(&self) -> VG11_SEL_R {
        VG11_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vg11_sel(&mut self) -> VG11_SEL_W {
        VG11_SEL_W { w: self }
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
}
#[doc = "`reset()` method sets cip_ldo15 to value 0"]
impl crate::Resettable for CIP_LDO15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
