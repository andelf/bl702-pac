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
#[doc = "Field `vco_idac_ll` reader - "]
pub type VCO_IDAC_LL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vco_idac_ll` writer - "]
pub type VCO_IDAC_LL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACAL_CONFIG_SPEC, u8, u8, 6, O>;
#[doc = "Field `vco_idac_lh` reader - "]
pub type VCO_IDAC_LH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vco_idac_lh` writer - "]
pub type VCO_IDAC_LH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACAL_CONFIG_SPEC, u8, u8, 6, O>;
#[doc = "Field `vco_idac_hl` reader - "]
pub type VCO_IDAC_HL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vco_idac_hl` writer - "]
pub type VCO_IDAC_HL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACAL_CONFIG_SPEC, u8, u8, 6, O>;
#[doc = "Field `vco_idac_hh` reader - "]
pub type VCO_IDAC_HH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vco_idac_hh` writer - "]
pub type VCO_IDAC_HH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ACAL_CONFIG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn vco_idac_ll(&self) -> VCO_IDAC_LL_R {
        VCO_IDAC_LL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn vco_idac_lh(&self) -> VCO_IDAC_LH_R {
        VCO_IDAC_LH_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn vco_idac_hl(&self) -> VCO_IDAC_HL_R {
        VCO_IDAC_HL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn vco_idac_hh(&self) -> VCO_IDAC_HH_R {
        VCO_IDAC_HH_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn vco_idac_ll(&mut self) -> VCO_IDAC_LL_W<0> {
        VCO_IDAC_LL_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn vco_idac_lh(&mut self) -> VCO_IDAC_LH_W<8> {
        VCO_IDAC_LH_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn vco_idac_hl(&mut self) -> VCO_IDAC_HL_W<16> {
        VCO_IDAC_HL_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn vco_idac_hh(&mut self) -> VCO_IDAC_HH_W<24> {
        VCO_IDAC_HH_W::new(self)
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
