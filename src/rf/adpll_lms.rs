#[doc = "Register `adpll_lms` reader"]
pub struct R(crate::R<ADPLL_LMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_LMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_LMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_LMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_lms` writer"]
pub struct W(crate::W<ADPLL_LMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_LMS_SPEC>;
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
impl From<crate::W<ADPLL_LMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_LMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_pha_cancel_delay` reader - "]
pub type ADPLL_PHA_CANCEL_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_pha_cancel_delay` writer - "]
pub type ADPLL_PHA_CANCEL_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LMS_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_pha_cancel_en` reader - "]
pub type ADPLL_PHA_CANCEL_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_pha_cancel_en` writer - "]
pub type ADPLL_PHA_CANCEL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LMS_SPEC, bool, O>;
#[doc = "Field `adpll_lms_q_delay` reader - "]
pub type ADPLL_LMS_Q_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lms_q_delay` writer - "]
pub type ADPLL_LMS_Q_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LMS_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_pha_prbs_sel` reader - "]
pub type ADPLL_PHA_PRBS_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_pha_prbs_sel` writer - "]
pub type ADPLL_PHA_PRBS_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LMS_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_lms_step_enlarge` reader - "]
pub type ADPLL_LMS_STEP_ENLARGE_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lms_step_enlarge` writer - "]
pub type ADPLL_LMS_STEP_ENLARGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LMS_SPEC, bool, O>;
#[doc = "Field `adpll_pha_dither_en` reader - "]
pub type ADPLL_PHA_DITHER_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_pha_dither_en` writer - "]
pub type ADPLL_PHA_DITHER_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LMS_SPEC, bool, O>;
#[doc = "Field `adpll_pha_dem_en` reader - "]
pub type ADPLL_PHA_DEM_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_pha_dem_en` writer - "]
pub type ADPLL_PHA_DEM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_LMS_SPEC, bool, O>;
#[doc = "Field `adpll_sdm_dither_prbs_en` reader - "]
pub type ADPLL_SDM_DITHER_PRBS_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_sdm_dither_prbs_en` writer - "]
pub type ADPLL_SDM_DITHER_PRBS_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LMS_SPEC, bool, O>;
#[doc = "Field `adpll_lms_step` reader - "]
pub type ADPLL_LMS_STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lms_step` writer - "]
pub type ADPLL_LMS_STEP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LMS_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_sdm_dither_en_ctrl_hw` reader - "]
pub type ADPLL_SDM_DITHER_EN_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `adpll_sdm_dither_en_ctrl_hw` writer - "]
pub type ADPLL_SDM_DITHER_EN_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LMS_SPEC, bool, O>;
#[doc = "Field `adpll_sdm_dither_en` reader - "]
pub type ADPLL_SDM_DITHER_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_sdm_dither_en` writer - "]
pub type ADPLL_SDM_DITHER_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LMS_SPEC, bool, O>;
#[doc = "Field `adpll_lms_ext_value` reader - "]
pub type ADPLL_LMS_EXT_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `adpll_lms_ext_value` writer - "]
pub type ADPLL_LMS_EXT_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LMS_SPEC, u16, u16, 9, O>;
#[doc = "Field `adpll_lms_ext_value_en` reader - "]
pub type ADPLL_LMS_EXT_VALUE_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lms_ext_value_en` writer - "]
pub type ADPLL_LMS_EXT_VALUE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LMS_SPEC, bool, O>;
#[doc = "Field `adpll_fref_div2_en` reader - "]
pub type ADPLL_FREF_DIV2_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_fref_div2_en` writer - "]
pub type ADPLL_FREF_DIV2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_LMS_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adpll_pha_cancel_delay(&self) -> ADPLL_PHA_CANCEL_DELAY_R {
        ADPLL_PHA_CANCEL_DELAY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adpll_pha_cancel_en(&self) -> ADPLL_PHA_CANCEL_EN_R {
        ADPLL_PHA_CANCEL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_lms_q_delay(&self) -> ADPLL_LMS_Q_DELAY_R {
        ADPLL_LMS_Q_DELAY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adpll_pha_prbs_sel(&self) -> ADPLL_PHA_PRBS_SEL_R {
        ADPLL_PHA_PRBS_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_lms_step_enlarge(&self) -> ADPLL_LMS_STEP_ENLARGE_R {
        ADPLL_LMS_STEP_ENLARGE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_pha_dither_en(&self) -> ADPLL_PHA_DITHER_EN_R {
        ADPLL_PHA_DITHER_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_pha_dem_en(&self) -> ADPLL_PHA_DEM_EN_R {
        ADPLL_PHA_DEM_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adpll_sdm_dither_prbs_en(&self) -> ADPLL_SDM_DITHER_PRBS_EN_R {
        ADPLL_SDM_DITHER_PRBS_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adpll_lms_step(&self) -> ADPLL_LMS_STEP_R {
        ADPLL_LMS_STEP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en_ctrl_hw(&self) -> ADPLL_SDM_DITHER_EN_CTRL_HW_R {
        ADPLL_SDM_DITHER_EN_CTRL_HW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adpll_sdm_dither_en(&self) -> ADPLL_SDM_DITHER_EN_R {
        ADPLL_SDM_DITHER_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn adpll_lms_ext_value(&self) -> ADPLL_LMS_EXT_VALUE_R {
        ADPLL_LMS_EXT_VALUE_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adpll_lms_ext_value_en(&self) -> ADPLL_LMS_EXT_VALUE_EN_R {
        ADPLL_LMS_EXT_VALUE_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adpll_fref_div2_en(&self) -> ADPLL_FREF_DIV2_EN_R {
        ADPLL_FREF_DIV2_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_pha_cancel_delay(&mut self) -> ADPLL_PHA_CANCEL_DELAY_W<0> {
        ADPLL_PHA_CANCEL_DELAY_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_pha_cancel_en(&mut self) -> ADPLL_PHA_CANCEL_EN_W<4> {
        ADPLL_PHA_CANCEL_EN_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lms_q_delay(&mut self) -> ADPLL_LMS_Q_DELAY_W<8> {
        ADPLL_LMS_Q_DELAY_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_pha_prbs_sel(&mut self) -> ADPLL_PHA_PRBS_SEL_W<10> {
        ADPLL_PHA_PRBS_SEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lms_step_enlarge(&mut self) -> ADPLL_LMS_STEP_ENLARGE_W<12> {
        ADPLL_LMS_STEP_ENLARGE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_pha_dither_en(&mut self) -> ADPLL_PHA_DITHER_EN_W<13> {
        ADPLL_PHA_DITHER_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_pha_dem_en(&mut self) -> ADPLL_PHA_DEM_EN_W<14> {
        ADPLL_PHA_DEM_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_sdm_dither_prbs_en(&mut self) -> ADPLL_SDM_DITHER_PRBS_EN_W<15> {
        ADPLL_SDM_DITHER_PRBS_EN_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lms_step(&mut self) -> ADPLL_LMS_STEP_W<16> {
        ADPLL_LMS_STEP_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_sdm_dither_en_ctrl_hw(&mut self) -> ADPLL_SDM_DITHER_EN_CTRL_HW_W<18> {
        ADPLL_SDM_DITHER_EN_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_sdm_dither_en(&mut self) -> ADPLL_SDM_DITHER_EN_W<19> {
        ADPLL_SDM_DITHER_EN_W::new(self)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lms_ext_value(&mut self) -> ADPLL_LMS_EXT_VALUE_W<20> {
        ADPLL_LMS_EXT_VALUE_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lms_ext_value_en(&mut self) -> ADPLL_LMS_EXT_VALUE_EN_W<29> {
        ADPLL_LMS_EXT_VALUE_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_fref_div2_en(&mut self) -> ADPLL_FREF_DIV2_EN_W<31> {
        ADPLL_FREF_DIV2_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_lms.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_lms](index.html) module"]
pub struct ADPLL_LMS_SPEC;
impl crate::RegisterSpec for ADPLL_LMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_lms::R](R) reader structure"]
impl crate::Readable for ADPLL_LMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_lms::W](W) writer structure"]
impl crate::Writable for ADPLL_LMS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets adpll_lms to value 0"]
impl crate::Resettable for ADPLL_LMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
