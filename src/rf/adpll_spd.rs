#[doc = "Register `adpll_spd` reader"]
pub struct R(crate::R<ADPLL_SPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_SPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_SPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_SPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_spd` writer"]
pub struct W(crate::W<ADPLL_SPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_SPD_SPEC>;
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
impl From<crate::W<ADPLL_SPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_SPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_spd_in_range_delay` reader - "]
pub type ADPLL_SPD_IN_RANGE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_spd_in_range_delay` writer - "]
pub type ADPLL_SPD_IN_RANGE_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SPD_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_spd_out_range_delay` reader - "]
pub type ADPLL_SPD_OUT_RANGE_DELAY_R = crate::BitReader<bool>;
#[doc = "Field `adpll_spd_out_range_delay` writer - "]
pub type ADPLL_SPD_OUT_RANGE_DELAY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_SPD_SPEC, bool, O>;
#[doc = "Field `adpll_spd_in_range_cons` reader - "]
pub type ADPLL_SPD_IN_RANGE_CONS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_spd_in_range_cons` writer - "]
pub type ADPLL_SPD_IN_RANGE_CONS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SPD_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_coarse_path_turnoff` reader - "]
pub type ADPLL_COARSE_PATH_TURNOFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_coarse_path_turnoff` writer - "]
pub type ADPLL_COARSE_PATH_TURNOFF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SPD_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_spd_threshold` reader - "]
pub type ADPLL_SPD_THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_spd_threshold` writer - "]
pub type ADPLL_SPD_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SPD_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_coarse_in_range_cons` reader - "]
pub type ADPLL_COARSE_IN_RANGE_CONS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_coarse_in_range_cons` writer - "]
pub type ADPLL_COARSE_IN_RANGE_CONS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SPD_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_spd_gain` reader - "]
pub type ADPLL_SPD_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_spd_gain` writer - "]
pub type ADPLL_SPD_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SPD_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_coarse_gain` reader - "]
pub type ADPLL_COARSE_GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_coarse_gain` writer - "]
pub type ADPLL_COARSE_GAIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SPD_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_force_lf_fast_mode_ctrl_hw` reader - "]
pub type ADPLL_FORCE_LF_FAST_MODE_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `adpll_force_lf_fast_mode_ctrl_hw` writer - "]
pub type ADPLL_FORCE_LF_FAST_MODE_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_SPD_SPEC, bool, O>;
#[doc = "Field `adpll_force_lf_fast_mode` reader - "]
pub type ADPLL_FORCE_LF_FAST_MODE_R = crate::BitReader<bool>;
#[doc = "Field `adpll_force_lf_fast_mode` writer - "]
pub type ADPLL_FORCE_LF_FAST_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_SPD_SPEC, bool, O>;
#[doc = "Field `adpll_force_lf_fast_mode_hw` reader - "]
pub type ADPLL_FORCE_LF_FAST_MODE_HW_R = crate::BitReader<bool>;
#[doc = "Field `adpll_force_lf_fast_mode_hw` writer - "]
pub type ADPLL_FORCE_LF_FAST_MODE_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_SPD_SPEC, bool, O>;
#[doc = "Field `adpll_coarse_phaerr_en` reader - "]
pub type ADPLL_COARSE_PHAERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_coarse_phaerr_en` writer - "]
pub type ADPLL_COARSE_PHAERR_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_SPD_SPEC, bool, O>;
#[doc = "Field `adpll_coarse_path_offtime_sel` reader - "]
pub type ADPLL_COARSE_PATH_OFFTIME_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_coarse_path_offtime_sel` writer - "]
pub type ADPLL_COARSE_PATH_OFFTIME_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_SPD_SPEC, bool, O>;
#[doc = "Field `adpll_spd_outrange_dly_sel_ext` reader - "]
pub type ADPLL_SPD_OUTRANGE_DLY_SEL_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_spd_outrange_dly_sel_ext` writer - "]
pub type ADPLL_SPD_OUTRANGE_DLY_SEL_EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SPD_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_spd_lms_sstp_win_sel` reader - "]
pub type ADPLL_SPD_LMS_SSTP_WIN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_spd_lms_sstp_win_sel` writer - "]
pub type ADPLL_SPD_LMS_SSTP_WIN_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_SPD_SPEC, bool, O>;
#[doc = "Field `adpll_force_coarse_path_on` reader - "]
pub type ADPLL_FORCE_COARSE_PATH_ON_R = crate::BitReader<bool>;
#[doc = "Field `adpll_force_coarse_path_on` writer - "]
pub type ADPLL_FORCE_COARSE_PATH_ON_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_SPD_SPEC, bool, O>;
#[doc = "Field `adpll_coarsepha_dly_sel` reader - "]
pub type ADPLL_COARSEPHA_DLY_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_coarsepha_dly_sel` writer - "]
pub type ADPLL_COARSEPHA_DLY_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_SPD_SPEC, bool, O>;
#[doc = "Field `adpll_spd_in_range_delay_1` reader - "]
pub type ADPLL_SPD_IN_RANGE_DELAY_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_spd_in_range_delay_1` writer - "]
pub type ADPLL_SPD_IN_RANGE_DELAY_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_SPD_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adpll_spd_in_range_delay(&self) -> ADPLL_SPD_IN_RANGE_DELAY_R {
        ADPLL_SPD_IN_RANGE_DELAY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adpll_spd_out_range_delay(&self) -> ADPLL_SPD_OUT_RANGE_DELAY_R {
        ADPLL_SPD_OUT_RANGE_DELAY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_spd_in_range_cons(&self) -> ADPLL_SPD_IN_RANGE_CONS_R {
        ADPLL_SPD_IN_RANGE_CONS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adpll_coarse_path_turnoff(&self) -> ADPLL_COARSE_PATH_TURNOFF_R {
        ADPLL_COARSE_PATH_TURNOFF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn adpll_spd_threshold(&self) -> ADPLL_SPD_THRESHOLD_R {
        ADPLL_SPD_THRESHOLD_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn adpll_coarse_in_range_cons(&self) -> ADPLL_COARSE_IN_RANGE_CONS_R {
        ADPLL_COARSE_IN_RANGE_CONS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adpll_spd_gain(&self) -> ADPLL_SPD_GAIN_R {
        ADPLL_SPD_GAIN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn adpll_coarse_gain(&self) -> ADPLL_COARSE_GAIN_R {
        ADPLL_COARSE_GAIN_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adpll_force_lf_fast_mode_ctrl_hw(&self) -> ADPLL_FORCE_LF_FAST_MODE_CTRL_HW_R {
        ADPLL_FORCE_LF_FAST_MODE_CTRL_HW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adpll_force_lf_fast_mode(&self) -> ADPLL_FORCE_LF_FAST_MODE_R {
        ADPLL_FORCE_LF_FAST_MODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn adpll_force_lf_fast_mode_hw(&self) -> ADPLL_FORCE_LF_FAST_MODE_HW_R {
        ADPLL_FORCE_LF_FAST_MODE_HW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn adpll_coarse_phaerr_en(&self) -> ADPLL_COARSE_PHAERR_EN_R {
        ADPLL_COARSE_PHAERR_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_coarse_path_offtime_sel(&self) -> ADPLL_COARSE_PATH_OFFTIME_SEL_R {
        ADPLL_COARSE_PATH_OFFTIME_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn adpll_spd_outrange_dly_sel_ext(&self) -> ADPLL_SPD_OUTRANGE_DLY_SEL_EXT_R {
        ADPLL_SPD_OUTRANGE_DLY_SEL_EXT_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_spd_lms_sstp_win_sel(&self) -> ADPLL_SPD_LMS_SSTP_WIN_SEL_R {
        ADPLL_SPD_LMS_SSTP_WIN_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adpll_force_coarse_path_on(&self) -> ADPLL_FORCE_COARSE_PATH_ON_R {
        ADPLL_FORCE_COARSE_PATH_ON_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adpll_coarsepha_dly_sel(&self) -> ADPLL_COARSEPHA_DLY_SEL_R {
        ADPLL_COARSEPHA_DLY_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn adpll_spd_in_range_delay_1(&self) -> ADPLL_SPD_IN_RANGE_DELAY_1_R {
        ADPLL_SPD_IN_RANGE_DELAY_1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adpll_spd_in_range_delay(&mut self) -> ADPLL_SPD_IN_RANGE_DELAY_W<0> {
        ADPLL_SPD_IN_RANGE_DELAY_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adpll_spd_out_range_delay(&mut self) -> ADPLL_SPD_OUT_RANGE_DELAY_W<4> {
        ADPLL_SPD_OUT_RANGE_DELAY_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_spd_in_range_cons(&mut self) -> ADPLL_SPD_IN_RANGE_CONS_W<8> {
        ADPLL_SPD_IN_RANGE_CONS_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adpll_coarse_path_turnoff(&mut self) -> ADPLL_COARSE_PATH_TURNOFF_W<10> {
        ADPLL_COARSE_PATH_TURNOFF_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn adpll_spd_threshold(&mut self) -> ADPLL_SPD_THRESHOLD_W<12> {
        ADPLL_SPD_THRESHOLD_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn adpll_coarse_in_range_cons(&mut self) -> ADPLL_COARSE_IN_RANGE_CONS_W<14> {
        ADPLL_COARSE_IN_RANGE_CONS_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn adpll_spd_gain(&mut self) -> ADPLL_SPD_GAIN_W<16> {
        ADPLL_SPD_GAIN_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn adpll_coarse_gain(&mut self) -> ADPLL_COARSE_GAIN_W<18> {
        ADPLL_COARSE_GAIN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adpll_force_lf_fast_mode_ctrl_hw(&mut self) -> ADPLL_FORCE_LF_FAST_MODE_CTRL_HW_W<20> {
        ADPLL_FORCE_LF_FAST_MODE_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adpll_force_lf_fast_mode(&mut self) -> ADPLL_FORCE_LF_FAST_MODE_W<21> {
        ADPLL_FORCE_LF_FAST_MODE_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn adpll_force_lf_fast_mode_hw(&mut self) -> ADPLL_FORCE_LF_FAST_MODE_HW_W<22> {
        ADPLL_FORCE_LF_FAST_MODE_HW_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn adpll_coarse_phaerr_en(&mut self) -> ADPLL_COARSE_PHAERR_EN_W<23> {
        ADPLL_COARSE_PHAERR_EN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_coarse_path_offtime_sel(&mut self) -> ADPLL_COARSE_PATH_OFFTIME_SEL_W<24> {
        ADPLL_COARSE_PATH_OFFTIME_SEL_W::new(self)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn adpll_spd_outrange_dly_sel_ext(&mut self) -> ADPLL_SPD_OUTRANGE_DLY_SEL_EXT_W<25> {
        ADPLL_SPD_OUTRANGE_DLY_SEL_EXT_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_spd_lms_sstp_win_sel(&mut self) -> ADPLL_SPD_LMS_SSTP_WIN_SEL_W<27> {
        ADPLL_SPD_LMS_SSTP_WIN_SEL_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adpll_force_coarse_path_on(&mut self) -> ADPLL_FORCE_COARSE_PATH_ON_W<28> {
        ADPLL_FORCE_COARSE_PATH_ON_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn adpll_coarsepha_dly_sel(&mut self) -> ADPLL_COARSEPHA_DLY_SEL_W<29> {
        ADPLL_COARSEPHA_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn adpll_spd_in_range_delay_1(&mut self) -> ADPLL_SPD_IN_RANGE_DELAY_1_W<30> {
        ADPLL_SPD_IN_RANGE_DELAY_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_spd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_spd](index.html) module"]
pub struct ADPLL_SPD_SPEC;
impl crate::RegisterSpec for ADPLL_SPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_spd::R](R) reader structure"]
impl crate::Readable for ADPLL_SPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_spd::W](W) writer structure"]
impl crate::Writable for ADPLL_SPD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_spd to value 0"]
impl crate::Resettable for ADPLL_SPD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
