#[doc = "Register `lo_config_2418` reader"]
pub struct R(crate::R<LO_CONFIG_2418_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_CONFIG_2418_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_CONFIG_2418_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_CONFIG_2418_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_config_2418` writer"]
pub struct W(crate::W<LO_CONFIG_2418_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_CONFIG_2418_SPEC>;
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
impl From<crate::W<LO_CONFIG_2418_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_CONFIG_2418_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `kcal_ratio_2418` reader - "]
pub type KCAL_RATIO_2418_R = crate::FieldReader<u16, u16>;
#[doc = "Field `kcal_ratio_2418` writer - "]
pub type KCAL_RATIO_2418_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LO_CONFIG_2418_SPEC, u16, u16, 10, O>;
#[doc = "Field `adpll_sdm_dither_en_2418` reader - "]
pub type ADPLL_SDM_DITHER_EN_2418_R = crate::BitReader<bool>;
#[doc = "Field `adpll_sdm_dither_en_2418` writer - "]
pub type ADPLL_SDM_DITHER_EN_2418_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LO_CONFIG_2418_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn kcal_ratio_2418(&self) -> KCAL_RATIO_2418_R {
        KCAL_RATIO_2418_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en_2418(&self) -> ADPLL_SDM_DITHER_EN_2418_R {
        ADPLL_SDM_DITHER_EN_2418_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn kcal_ratio_2418(&mut self) -> KCAL_RATIO_2418_W<0> {
        KCAL_RATIO_2418_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en_2418(&mut self) -> ADPLL_SDM_DITHER_EN_2418_W<12> {
        ADPLL_SDM_DITHER_EN_2418_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_config_2418.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_config_2418](index.html) module"]
pub struct LO_CONFIG_2418_SPEC;
impl crate::RegisterSpec for LO_CONFIG_2418_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_config_2418::R](R) reader structure"]
impl crate::Readable for LO_CONFIG_2418_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_config_2418::W](W) writer structure"]
impl crate::Writable for LO_CONFIG_2418_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_config_2418 to value 0"]
impl crate::Resettable for LO_CONFIG_2418_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
