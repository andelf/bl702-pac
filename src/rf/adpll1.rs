#[doc = "Register `adpll1` reader"]
pub struct R(crate::R<ADPLL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll1` writer"]
pub struct W(crate::W<ADPLL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL1_SPEC>;
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
impl From<crate::W<ADPLL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_lo_unlock_intrpt_clear` reader - "]
pub type ADPLL_LO_UNLOCK_INTRPT_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lo_unlock_intrpt_clear` writer - "]
pub type ADPLL_LO_UNLOCK_INTRPT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_lo_lock_sel` reader - "]
pub type ADPLL_LO_LOCK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lo_lock_sel` writer - "]
pub type ADPLL_LO_LOCK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_timeout_cnt1_sel` reader - "]
pub type ADPLL_TIMEOUT_CNT1_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_timeout_cnt1_sel` writer - "]
pub type ADPLL_TIMEOUT_CNT1_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_timeout_cnt_sel` reader - "]
pub type ADPLL_TIMEOUT_CNT_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_timeout_cnt_sel` writer - "]
pub type ADPLL_TIMEOUT_CNT_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_momhold_lmsenb_ext` reader - "]
pub type ADPLL_MOMHOLD_LMSENB_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_momhold_lmsenb_ext` writer - "]
pub type ADPLL_MOMHOLD_LMSENB_EXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_rst_coarse_det_ext` reader - "]
pub type ADPLL_RST_COARSE_DET_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_rst_coarse_det_ext` writer - "]
pub type ADPLL_RST_COARSE_DET_EXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_rst_spd_det_ext` reader - "]
pub type ADPLL_RST_SPD_DET_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_rst_spd_det_ext` writer - "]
pub type ADPLL_RST_SPD_DET_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_loop_lock_ext` reader - "]
pub type ADPLL_LOOP_LOCK_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_loop_lock_ext` writer - "]
pub type ADPLL_LOOP_LOCK_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_fcal_done_ext` reader - "]
pub type ADPLL_FCAL_DONE_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_fcal_done_ext` writer - "]
pub type ADPLL_FCAL_DONE_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_fcal_start_ext` reader - "]
pub type ADPLL_FCAL_START_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_fcal_start_ext` writer - "]
pub type ADPLL_FCAL_START_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_lo_lock_directly` reader - "]
pub type ADPLL_LO_LOCK_DIRECTLY_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lo_lock_directly` writer - "]
pub type ADPLL_LO_LOCK_DIRECTLY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_lo_fsm_ext` reader - "]
pub type ADPLL_LO_FSM_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lo_fsm_ext` writer - "]
pub type ADPLL_LO_FSM_EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_fsm_en` reader - "]
pub type ADPLL_FSM_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_fsm_en` writer - "]
pub type ADPLL_FSM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_lock_fail_en` reader - "]
pub type ADPLL_LOCK_FAIL_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lock_fail_en` writer - "]
pub type ADPLL_LOCK_FAIL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_abnormal_dealed` reader - "]
pub type ADPLL_ABNORMAL_DEALED_R = crate::BitReader<bool>;
#[doc = "Field `adpll_abnormal_dealed` writer - "]
pub type ADPLL_ABNORMAL_DEALED_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_vctrl_det_start_ext` reader - "]
pub type ADPLL_VCTRL_DET_START_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_vctrl_det_start_ext` writer - "]
pub type ADPLL_VCTRL_DET_START_EXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_vctrl_det_en_ext` reader - "]
pub type ADPLL_VCTRL_DET_EN_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_vctrl_det_en_ext` writer - "]
pub type ADPLL_VCTRL_DET_EN_EXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_mom_update_en_ext` reader - "]
pub type ADPLL_MOM_UPDATE_EN_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_mom_update_en_ext` writer - "]
pub type ADPLL_MOM_UPDATE_EN_EXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_freqerr_det_start_ext` reader - "]
pub type ADPLL_FREQERR_DET_START_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_freqerr_det_start_ext` writer - "]
pub type ADPLL_FREQERR_DET_START_EXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_mom_search_en_ext` reader - "]
pub type ADPLL_MOM_SEARCH_EN_EXT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_mom_search_en_ext` writer - "]
pub type ADPLL_MOM_SEARCH_EN_EXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_lo_open` reader - "]
pub type ADPLL_LO_OPEN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lo_open` writer - "]
pub type ADPLL_LO_OPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_sfreg_sel` reader - "]
pub type ADPLL_SFREG_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_sfreg_sel` writer - "]
pub type ADPLL_SFREG_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_lo_unlock_intrpt_clear_sel` reader - "]
pub type ADPLL_LO_UNLOCK_INTRPT_CLEAR_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lo_unlock_intrpt_clear_sel` writer - "]
pub type ADPLL_LO_UNLOCK_INTRPT_CLEAR_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
#[doc = "Field `adpll_force_inc_fcal_en` reader - "]
pub type ADPLL_FORCE_INC_FCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_force_inc_fcal_en` writer - "]
pub type ADPLL_FORCE_INC_FCAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_lo_unlock_intrpt_clear(&self) -> ADPLL_LO_UNLOCK_INTRPT_CLEAR_R {
        ADPLL_LO_UNLOCK_INTRPT_CLEAR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adpll_lo_lock_sel(&self) -> ADPLL_LO_LOCK_SEL_R {
        ADPLL_LO_LOCK_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adpll_timeout_cnt1_sel(&self) -> ADPLL_TIMEOUT_CNT1_SEL_R {
        ADPLL_TIMEOUT_CNT1_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adpll_timeout_cnt_sel(&self) -> ADPLL_TIMEOUT_CNT_SEL_R {
        ADPLL_TIMEOUT_CNT_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adpll_momhold_lmsenb_ext(&self) -> ADPLL_MOMHOLD_LMSENB_EXT_R {
        ADPLL_MOMHOLD_LMSENB_EXT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adpll_rst_coarse_det_ext(&self) -> ADPLL_RST_COARSE_DET_EXT_R {
        ADPLL_RST_COARSE_DET_EXT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_rst_spd_det_ext(&self) -> ADPLL_RST_SPD_DET_EXT_R {
        ADPLL_RST_SPD_DET_EXT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn adpll_loop_lock_ext(&self) -> ADPLL_LOOP_LOCK_EXT_R {
        ADPLL_LOOP_LOCK_EXT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adpll_fcal_done_ext(&self) -> ADPLL_FCAL_DONE_EXT_R {
        ADPLL_FCAL_DONE_EXT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adpll_fcal_start_ext(&self) -> ADPLL_FCAL_START_EXT_R {
        ADPLL_FCAL_START_EXT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adpll_lo_lock_directly(&self) -> ADPLL_LO_LOCK_DIRECTLY_R {
        ADPLL_LO_LOCK_DIRECTLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn adpll_lo_fsm_ext(&self) -> ADPLL_LO_FSM_EXT_R {
        ADPLL_LO_FSM_EXT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_fsm_en(&self) -> ADPLL_FSM_EN_R {
        ADPLL_FSM_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_lock_fail_en(&self) -> ADPLL_LOCK_FAIL_EN_R {
        ADPLL_LOCK_FAIL_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adpll_abnormal_dealed(&self) -> ADPLL_ABNORMAL_DEALED_R {
        ADPLL_ABNORMAL_DEALED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adpll_vctrl_det_start_ext(&self) -> ADPLL_VCTRL_DET_START_EXT_R {
        ADPLL_VCTRL_DET_START_EXT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adpll_vctrl_det_en_ext(&self) -> ADPLL_VCTRL_DET_EN_EXT_R {
        ADPLL_VCTRL_DET_EN_EXT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adpll_mom_update_en_ext(&self) -> ADPLL_MOM_UPDATE_EN_EXT_R {
        ADPLL_MOM_UPDATE_EN_EXT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adpll_freqerr_det_start_ext(&self) -> ADPLL_FREQERR_DET_START_EXT_R {
        ADPLL_FREQERR_DET_START_EXT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn adpll_mom_search_en_ext(&self) -> ADPLL_MOM_SEARCH_EN_EXT_R {
        ADPLL_MOM_SEARCH_EN_EXT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn adpll_lo_open(&self) -> ADPLL_LO_OPEN_R {
        ADPLL_LO_OPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_sfreg_sel(&self) -> ADPLL_SFREG_SEL_R {
        ADPLL_SFREG_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adpll_lo_unlock_intrpt_clear_sel(&self) -> ADPLL_LO_UNLOCK_INTRPT_CLEAR_SEL_R {
        ADPLL_LO_UNLOCK_INTRPT_CLEAR_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn adpll_force_inc_fcal_en(&self) -> ADPLL_FORCE_INC_FCAL_EN_R {
        ADPLL_FORCE_INC_FCAL_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_lo_unlock_intrpt_clear(&mut self) -> ADPLL_LO_UNLOCK_INTRPT_CLEAR_W<0> {
        ADPLL_LO_UNLOCK_INTRPT_CLEAR_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adpll_lo_lock_sel(&mut self) -> ADPLL_LO_LOCK_SEL_W<1> {
        ADPLL_LO_LOCK_SEL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adpll_timeout_cnt1_sel(&mut self) -> ADPLL_TIMEOUT_CNT1_SEL_W<2> {
        ADPLL_TIMEOUT_CNT1_SEL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adpll_timeout_cnt_sel(&mut self) -> ADPLL_TIMEOUT_CNT_SEL_W<3> {
        ADPLL_TIMEOUT_CNT_SEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adpll_momhold_lmsenb_ext(&mut self) -> ADPLL_MOMHOLD_LMSENB_EXT_W<4> {
        ADPLL_MOMHOLD_LMSENB_EXT_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adpll_rst_coarse_det_ext(&mut self) -> ADPLL_RST_COARSE_DET_EXT_W<5> {
        ADPLL_RST_COARSE_DET_EXT_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_rst_spd_det_ext(&mut self) -> ADPLL_RST_SPD_DET_EXT_W<6> {
        ADPLL_RST_SPD_DET_EXT_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn adpll_loop_lock_ext(&mut self) -> ADPLL_LOOP_LOCK_EXT_W<7> {
        ADPLL_LOOP_LOCK_EXT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adpll_fcal_done_ext(&mut self) -> ADPLL_FCAL_DONE_EXT_W<8> {
        ADPLL_FCAL_DONE_EXT_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adpll_fcal_start_ext(&mut self) -> ADPLL_FCAL_START_EXT_W<9> {
        ADPLL_FCAL_START_EXT_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adpll_lo_lock_directly(&mut self) -> ADPLL_LO_LOCK_DIRECTLY_W<10> {
        ADPLL_LO_LOCK_DIRECTLY_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn adpll_lo_fsm_ext(&mut self) -> ADPLL_LO_FSM_EXT_W<11> {
        ADPLL_LO_FSM_EXT_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_fsm_en(&mut self) -> ADPLL_FSM_EN_W<12> {
        ADPLL_FSM_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_lock_fail_en(&mut self) -> ADPLL_LOCK_FAIL_EN_W<14> {
        ADPLL_LOCK_FAIL_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adpll_abnormal_dealed(&mut self) -> ADPLL_ABNORMAL_DEALED_W<15> {
        ADPLL_ABNORMAL_DEALED_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adpll_vctrl_det_start_ext(&mut self) -> ADPLL_VCTRL_DET_START_EXT_W<18> {
        ADPLL_VCTRL_DET_START_EXT_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adpll_vctrl_det_en_ext(&mut self) -> ADPLL_VCTRL_DET_EN_EXT_W<19> {
        ADPLL_VCTRL_DET_EN_EXT_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adpll_mom_update_en_ext(&mut self) -> ADPLL_MOM_UPDATE_EN_EXT_W<20> {
        ADPLL_MOM_UPDATE_EN_EXT_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adpll_freqerr_det_start_ext(&mut self) -> ADPLL_FREQERR_DET_START_EXT_W<21> {
        ADPLL_FREQERR_DET_START_EXT_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn adpll_mom_search_en_ext(&mut self) -> ADPLL_MOM_SEARCH_EN_EXT_W<22> {
        ADPLL_MOM_SEARCH_EN_EXT_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn adpll_lo_open(&mut self) -> ADPLL_LO_OPEN_W<23> {
        ADPLL_LO_OPEN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_sfreg_sel(&mut self) -> ADPLL_SFREG_SEL_W<24> {
        ADPLL_SFREG_SEL_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adpll_lo_unlock_intrpt_clear_sel(&mut self) -> ADPLL_LO_UNLOCK_INTRPT_CLEAR_SEL_W<25> {
        ADPLL_LO_UNLOCK_INTRPT_CLEAR_SEL_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn adpll_force_inc_fcal_en(&mut self) -> ADPLL_FORCE_INC_FCAL_EN_W<26> {
        ADPLL_FORCE_INC_FCAL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll1](index.html) module"]
pub struct ADPLL1_SPEC;
impl crate::RegisterSpec for ADPLL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll1::R](R) reader structure"]
impl crate::Readable for ADPLL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll1::W](W) writer structure"]
impl crate::Writable for ADPLL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll1 to value 0"]
impl crate::Resettable for ADPLL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
