#[doc = "Register `vco` reader"]
pub struct R(crate::R<VCO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vco` writer"]
pub struct W(crate::W<VCO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCO_SPEC>;
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
impl From<crate::W<VCO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vco_short_vbias_filter` reader - "]
pub type VCO_SHORT_VBIAS_FILTER_R = crate::BitReader<bool>;
#[doc = "Field `vco_short_vbias_filter` writer - "]
pub type VCO_SHORT_VBIAS_FILTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO_SPEC, bool, O>;
#[doc = "Field `vco_short_idac_filter` reader - "]
pub type VCO_SHORT_IDAC_FILTER_R = crate::BitReader<bool>;
#[doc = "Field `vco_short_idac_filter` writer - "]
pub type VCO_SHORT_IDAC_FILTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO_SPEC, bool, O>;
#[doc = "Field `vco_modcap_sel` reader - "]
pub type VCO_MODCAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vco_modcap_sel` writer - "]
pub type VCO_MODCAP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCO_SPEC, u8, u8, 2, O>;
#[doc = "Field `vco_acal_vref` reader - "]
pub type VCO_ACAL_VREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vco_acal_vref` writer - "]
pub type VCO_ACAL_VREF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCO_SPEC, u8, u8, 3, O>;
#[doc = "Field `vco_vbias` reader - "]
pub type VCO_VBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vco_vbias` writer - "]
pub type VCO_VBIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCO_SPEC, u8, u8, 2, O>;
#[doc = "Field `vco_idac_boost` reader - "]
pub type VCO_IDAC_BOOST_R = crate::BitReader<bool>;
#[doc = "Field `vco_idac_boost` writer - "]
pub type VCO_IDAC_BOOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO_SPEC, bool, O>;
#[doc = "Field `vco_ldo_vsel` reader - "]
pub type VCO_LDO_VSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vco_ldo_vsel` writer - "]
pub type VCO_LDO_VSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCO_SPEC, u8, u8, 2, O>;
#[doc = "Field `vco_ldo_bypass` reader - "]
pub type VCO_LDO_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `vco_ldo_bypass` writer - "]
pub type VCO_LDO_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO_SPEC, bool, O>;
#[doc = "Field `vco_idac` reader - "]
pub type VCO_IDAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vco_idac` writer - "]
pub type VCO_IDAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCO_SPEC, u8, u8, 6, O>;
#[doc = "Field `vco_idac_hw` reader - "]
pub type VCO_IDAC_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `vco_idac_hw` writer - "]
pub type VCO_IDAC_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VCO_SPEC, u8, u8, 6, O>;
#[doc = "Field `vco_acal_ud` reader - "]
pub type VCO_ACAL_UD_R = crate::BitReader<bool>;
#[doc = "Field `vco_acal_ud` writer - "]
pub type VCO_ACAL_UD_W<'a, const O: u8> = crate::BitWriter<'a, u32, VCO_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vco_short_vbias_filter(&self) -> VCO_SHORT_VBIAS_FILTER_R {
        VCO_SHORT_VBIAS_FILTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vco_short_idac_filter(&self) -> VCO_SHORT_IDAC_FILTER_R {
        VCO_SHORT_IDAC_FILTER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vco_modcap_sel(&self) -> VCO_MODCAP_SEL_R {
        VCO_MODCAP_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn vco_acal_vref(&self) -> VCO_ACAL_VREF_R {
        VCO_ACAL_VREF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn vco_vbias(&self) -> VCO_VBIAS_R {
        VCO_VBIAS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn vco_idac_boost(&self) -> VCO_IDAC_BOOST_R {
        VCO_IDAC_BOOST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn vco_ldo_vsel(&self) -> VCO_LDO_VSEL_R {
        VCO_LDO_VSEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn vco_ldo_bypass(&self) -> VCO_LDO_BYPASS_R {
        VCO_LDO_BYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn vco_idac(&self) -> VCO_IDAC_R {
        VCO_IDAC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn vco_idac_hw(&self) -> VCO_IDAC_HW_R {
        VCO_IDAC_HW_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn vco_acal_ud(&self) -> VCO_ACAL_UD_R {
        VCO_ACAL_UD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vco_short_vbias_filter(&mut self) -> VCO_SHORT_VBIAS_FILTER_W<0> {
        VCO_SHORT_VBIAS_FILTER_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vco_short_idac_filter(&mut self) -> VCO_SHORT_IDAC_FILTER_W<1> {
        VCO_SHORT_IDAC_FILTER_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vco_modcap_sel(&mut self) -> VCO_MODCAP_SEL_W<2> {
        VCO_MODCAP_SEL_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn vco_acal_vref(&mut self) -> VCO_ACAL_VREF_W<4> {
        VCO_ACAL_VREF_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn vco_vbias(&mut self) -> VCO_VBIAS_W<8> {
        VCO_VBIAS_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn vco_idac_boost(&mut self) -> VCO_IDAC_BOOST_W<12> {
        VCO_IDAC_BOOST_W::new(self)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn vco_ldo_vsel(&mut self) -> VCO_LDO_VSEL_W<13> {
        VCO_LDO_VSEL_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn vco_ldo_bypass(&mut self) -> VCO_LDO_BYPASS_W<15> {
        VCO_LDO_BYPASS_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn vco_idac(&mut self) -> VCO_IDAC_W<16> {
        VCO_IDAC_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn vco_idac_hw(&mut self) -> VCO_IDAC_HW_W<24> {
        VCO_IDAC_HW_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn vco_acal_ud(&mut self) -> VCO_ACAL_UD_W<31> {
        VCO_ACAL_UD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "vco.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco](index.html) module"]
pub struct VCO_SPEC;
impl crate::RegisterSpec for VCO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vco::R](R) reader structure"]
impl crate::Readable for VCO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vco::W](W) writer structure"]
impl crate::Writable for VCO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets vco to value 0"]
impl crate::Resettable for VCO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
