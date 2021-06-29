#[doc = "Register `lo_config_2414` reader"]
pub struct R(crate::R<LO_CONFIG_2414_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_CONFIG_2414_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_CONFIG_2414_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_CONFIG_2414_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_config_2414` writer"]
pub struct W(crate::W<LO_CONFIG_2414_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_CONFIG_2414_SPEC>;
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
impl From<crate::W<LO_CONFIG_2414_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_CONFIG_2414_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_sdm_dither_en_2414` reader - "]
pub struct ADPLL_SDM_DITHER_EN_2414_R(crate::FieldReader<bool, bool>);
impl ADPLL_SDM_DITHER_EN_2414_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_SDM_DITHER_EN_2414_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_SDM_DITHER_EN_2414_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_sdm_dither_en_2414` writer - "]
pub struct ADPLL_SDM_DITHER_EN_2414_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_SDM_DITHER_EN_2414_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `kcal_ratio_2414` reader - "]
pub struct KCAL_RATIO_2414_R(crate::FieldReader<u16, u16>);
impl KCAL_RATIO_2414_R {
    pub(crate) fn new(bits: u16) -> Self {
        KCAL_RATIO_2414_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_RATIO_2414_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_ratio_2414` writer - "]
pub struct KCAL_RATIO_2414_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_RATIO_2414_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en_2414(&self) -> ADPLL_SDM_DITHER_EN_2414_R {
        ADPLL_SDM_DITHER_EN_2414_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn kcal_ratio_2414(&self) -> KCAL_RATIO_2414_R {
        KCAL_RATIO_2414_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en_2414(&mut self) -> ADPLL_SDM_DITHER_EN_2414_W {
        ADPLL_SDM_DITHER_EN_2414_W { w: self }
    }
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn kcal_ratio_2414(&mut self) -> KCAL_RATIO_2414_W {
        KCAL_RATIO_2414_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_config_2414.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_config_2414](index.html) module"]
pub struct LO_CONFIG_2414_SPEC;
impl crate::RegisterSpec for LO_CONFIG_2414_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_config_2414::R](R) reader structure"]
impl crate::Readable for LO_CONFIG_2414_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_config_2414::W](W) writer structure"]
impl crate::Writable for LO_CONFIG_2414_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_config_2414 to value 0"]
impl crate::Resettable for LO_CONFIG_2414_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
