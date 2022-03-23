#[doc = "Register `acal_config` reader"]
pub struct R(crate::R<ACAL_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACAL_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACAL_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACAL_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `acal_config` writer"]
pub struct W(crate::W<ACAL_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACAL_CONFIG_SPEC>;
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
impl From<crate::W<ACAL_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACAL_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vco_idac_hh` reader - "]
pub struct VCO_IDAC_HH_R(crate::FieldReader<u8, u8>);
impl VCO_IDAC_HH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_IDAC_HH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_IDAC_HH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_idac_hh` writer - "]
pub struct VCO_IDAC_HH_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_IDAC_HH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `vco_idac_hl` reader - "]
pub struct VCO_IDAC_HL_R(crate::FieldReader<u8, u8>);
impl VCO_IDAC_HL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_IDAC_HL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_IDAC_HL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_idac_hl` writer - "]
pub struct VCO_IDAC_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_IDAC_HL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `vco_idac_lh` reader - "]
pub struct VCO_IDAC_LH_R(crate::FieldReader<u8, u8>);
impl VCO_IDAC_LH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_IDAC_LH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_IDAC_LH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_idac_lh` writer - "]
pub struct VCO_IDAC_LH_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_IDAC_LH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `vco_idac_ll` reader - "]
pub struct VCO_IDAC_LL_R(crate::FieldReader<u8, u8>);
impl VCO_IDAC_LL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_IDAC_LL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_IDAC_LL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_idac_ll` writer - "]
pub struct VCO_IDAC_LL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_IDAC_LL_W<'a> {
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
    pub fn vco_idac_hh(&self) -> VCO_IDAC_HH_R {
        VCO_IDAC_HH_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn vco_idac_hl(&self) -> VCO_IDAC_HL_R {
        VCO_IDAC_HL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn vco_idac_lh(&self) -> VCO_IDAC_LH_R {
        VCO_IDAC_LH_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn vco_idac_ll(&self) -> VCO_IDAC_LL_R {
        VCO_IDAC_LL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn vco_idac_hh(&mut self) -> VCO_IDAC_HH_W {
        VCO_IDAC_HH_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn vco_idac_hl(&mut self) -> VCO_IDAC_HL_W {
        VCO_IDAC_HL_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn vco_idac_lh(&mut self) -> VCO_IDAC_LH_W {
        VCO_IDAC_LH_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn vco_idac_ll(&mut self) -> VCO_IDAC_LL_W {
        VCO_IDAC_LL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "acal_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acal_config](index.html) module"]
pub struct ACAL_CONFIG_SPEC;
impl crate::RegisterSpec for ACAL_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acal_config::R](R) reader structure"]
impl crate::Readable for ACAL_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acal_config::W](W) writer structure"]
impl crate::Writable for ACAL_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets acal_config to value 0"]
impl crate::Resettable for ACAL_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
