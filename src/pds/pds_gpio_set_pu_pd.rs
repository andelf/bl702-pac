#[doc = "Register `pds_gpio_set_pu_pd` reader"]
pub struct R(crate::R<PDS_GPIO_SET_PU_PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_GPIO_SET_PU_PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_GPIO_SET_PU_PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_GPIO_SET_PU_PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_gpio_set_pu_pd` writer"]
pub struct W(crate::W<PDS_GPIO_SET_PU_PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_GPIO_SET_PU_PD_SPEC>;
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
impl From<crate::W<PDS_GPIO_SET_PU_PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_GPIO_SET_PU_PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_pds_gpio_28_23_pu` reader - "]
pub struct CR_PDS_GPIO_28_23_PU_R(crate::FieldReader<u8, u8>);
impl CR_PDS_GPIO_28_23_PU_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_PDS_GPIO_28_23_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_GPIO_28_23_PU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_gpio_28_23_pu` writer - "]
pub struct CR_PDS_GPIO_28_23_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_GPIO_28_23_PU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `cr_pds_gpio_28_23_pd` reader - "]
pub struct CR_PDS_GPIO_28_23_PD_R(crate::FieldReader<u8, u8>);
impl CR_PDS_GPIO_28_23_PD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_PDS_GPIO_28_23_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_GPIO_28_23_PD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_gpio_28_23_pd` writer - "]
pub struct CR_PDS_GPIO_28_23_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_GPIO_28_23_PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `cr_pds_gpio_22_17_pu` reader - "]
pub struct CR_PDS_GPIO_22_17_PU_R(crate::FieldReader<u8, u8>);
impl CR_PDS_GPIO_22_17_PU_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_PDS_GPIO_22_17_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_GPIO_22_17_PU_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_gpio_22_17_pu` writer - "]
pub struct CR_PDS_GPIO_22_17_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_GPIO_22_17_PU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `cr_pds_gpio_22_17_pd` reader - "]
pub struct CR_PDS_GPIO_22_17_PD_R(crate::FieldReader<u8, u8>);
impl CR_PDS_GPIO_22_17_PD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_PDS_GPIO_22_17_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_GPIO_22_17_PD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_gpio_22_17_pd` writer - "]
pub struct CR_PDS_GPIO_22_17_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_GPIO_22_17_PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn cr_pds_gpio_28_23_pu(&self) -> CR_PDS_GPIO_28_23_PU_R {
        CR_PDS_GPIO_28_23_PU_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn cr_pds_gpio_28_23_pd(&self) -> CR_PDS_GPIO_28_23_PD_R {
        CR_PDS_GPIO_28_23_PD_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn cr_pds_gpio_22_17_pu(&self) -> CR_PDS_GPIO_22_17_PU_R {
        CR_PDS_GPIO_22_17_PU_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn cr_pds_gpio_22_17_pd(&self) -> CR_PDS_GPIO_22_17_PD_R {
        CR_PDS_GPIO_22_17_PD_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn cr_pds_gpio_28_23_pu(&mut self) -> CR_PDS_GPIO_28_23_PU_W {
        CR_PDS_GPIO_28_23_PU_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn cr_pds_gpio_28_23_pd(&mut self) -> CR_PDS_GPIO_28_23_PD_W {
        CR_PDS_GPIO_28_23_PD_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn cr_pds_gpio_22_17_pu(&mut self) -> CR_PDS_GPIO_22_17_PU_W {
        CR_PDS_GPIO_22_17_PU_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn cr_pds_gpio_22_17_pd(&mut self) -> CR_PDS_GPIO_22_17_PD_W {
        CR_PDS_GPIO_22_17_PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_gpio_set_pu_pd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_gpio_set_pu_pd](index.html) module"]
pub struct PDS_GPIO_SET_PU_PD_SPEC;
impl crate::RegisterSpec for PDS_GPIO_SET_PU_PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_gpio_set_pu_pd::R](R) reader structure"]
impl crate::Readable for PDS_GPIO_SET_PU_PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_gpio_set_pu_pd::W](W) writer structure"]
impl crate::Writable for PDS_GPIO_SET_PU_PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pds_gpio_set_pu_pd to value 0"]
impl crate::Resettable for PDS_GPIO_SET_PU_PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
