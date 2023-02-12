#[doc = "Register `lo_config_2474` reader"]
pub struct R(crate::R<LO_CONFIG_2474_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_CONFIG_2474_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_CONFIG_2474_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_CONFIG_2474_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_config_2474` writer"]
pub struct W(crate::W<LO_CONFIG_2474_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_CONFIG_2474_SPEC>;
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
impl From<crate::W<LO_CONFIG_2474_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_CONFIG_2474_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `kcal_ratio_2474` reader - "]
pub type KCAL_RATIO_2474_R = crate::FieldReader<u16, u16>;
#[doc = "Field `kcal_ratio_2474` writer - "]
pub type KCAL_RATIO_2474_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_CONFIG_2474_SPEC, u16, u16, 10, O>;
#[doc = "Field `adpll_sdm_dither_en_2474` reader - "]
pub type ADPLL_SDM_DITHER_EN_2474_R = crate::BitReader<bool>;
#[doc = "Field `adpll_sdm_dither_en_2474` writer - "]
pub type ADPLL_SDM_DITHER_EN_2474_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LO_CONFIG_2474_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn kcal_ratio_2474(&self) -> KCAL_RATIO_2474_R {
        KCAL_RATIO_2474_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en_2474(&self) -> ADPLL_SDM_DITHER_EN_2474_R {
        ADPLL_SDM_DITHER_EN_2474_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn kcal_ratio_2474(&mut self) -> KCAL_RATIO_2474_W<0> {
        KCAL_RATIO_2474_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_sdm_dither_en_2474(&mut self) -> ADPLL_SDM_DITHER_EN_2474_W<12> {
        ADPLL_SDM_DITHER_EN_2474_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_config_2474.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_config_2474](index.html) module"]
pub struct LO_CONFIG_2474_SPEC;
impl crate::RegisterSpec for LO_CONFIG_2474_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_config_2474::R](R) reader structure"]
impl crate::Readable for LO_CONFIG_2474_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_config_2474::W](W) writer structure"]
impl crate::Writable for LO_CONFIG_2474_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lo_config_2474 to value 0"]
impl crate::Resettable for LO_CONFIG_2474_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
