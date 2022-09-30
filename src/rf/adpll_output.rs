#[doc = "Register `adpll_output` reader"]
pub struct R(crate::R<ADPLL_OUTPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_OUTPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_OUTPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_OUTPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_output` writer"]
pub struct W(crate::W<ADPLL_OUTPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_OUTPUT_SPEC>;
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
impl From<crate::W<ADPLL_OUTPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_OUTPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_unlock_intrpt` reader - "]
pub type ADPLL_UNLOCK_INTRPT_R = crate::BitReader<bool>;
#[doc = "Field `adpll_unlock_intrpt` writer - "]
pub type ADPLL_UNLOCK_INTRPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_lo_lock` reader - "]
pub type ADPLL_LO_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lo_lock` writer - "]
pub type ADPLL_LO_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_fsm_state` reader - "]
pub type ADPLL_FSM_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_fsm_state` writer - "]
pub type ADPLL_FSM_STATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_OUTPUT_SPEC, u8, u8, 4, O>;
#[doc = "Field `adpll_spd_unlock_sign` reader - "]
pub type ADPLL_SPD_UNLOCK_SIGN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_spd_unlock_sign` writer - "]
pub type ADPLL_SPD_UNLOCK_SIGN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_vctrl_out_range_fsm` reader - "]
pub type ADPLL_VCTRL_OUT_RANGE_FSM_R = crate::BitReader<bool>;
#[doc = "Field `adpll_vctrl_out_range_fsm` writer - "]
pub type ADPLL_VCTRL_OUT_RANGE_FSM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_mom_update_fail_fsm` reader - "]
pub type ADPLL_MOM_UPDATE_FAIL_FSM_R = crate::BitReader<bool>;
#[doc = "Field `adpll_mom_update_fail_fsm` writer - "]
pub type ADPLL_MOM_UPDATE_FAIL_FSM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_mom_update_ou_fsm` reader - "]
pub type ADPLL_MOM_UPDATE_OU_FSM_R = crate::BitReader<bool>;
#[doc = "Field `adpll_mom_update_ou_fsm` writer - "]
pub type ADPLL_MOM_UPDATE_OU_FSM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_spd_unlock_fsm` reader - "]
pub type ADPLL_SPD_UNLOCK_FSM_R = crate::BitReader<bool>;
#[doc = "Field `adpll_spd_unlock_fsm` writer - "]
pub type ADPLL_SPD_UNLOCK_FSM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_spd_lock_fsm` reader - "]
pub type ADPLL_SPD_LOCK_FSM_R = crate::BitReader<bool>;
#[doc = "Field `adpll_spd_lock_fsm` writer - "]
pub type ADPLL_SPD_LOCK_FSM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_fcal_done_fsm` reader - "]
pub type ADPLL_FCAL_DONE_FSM_R = crate::BitReader<bool>;
#[doc = "Field `adpll_fcal_done_fsm` writer - "]
pub type ADPLL_FCAL_DONE_FSM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_capcode_out_range` reader - "]
pub type ADPLL_CAPCODE_OUT_RANGE_R = crate::BitReader<bool>;
#[doc = "Field `adpll_capcode_out_range` writer - "]
pub type ADPLL_CAPCODE_OUT_RANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_mom_update_total_ou` reader - "]
pub type ADPLL_MOM_UPDATE_TOTAL_OU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_mom_update_total_ou` writer - "]
pub type ADPLL_MOM_UPDATE_TOTAL_OU_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_OUTPUT_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_capcode_ud` reader - "]
pub type ADPLL_CAPCODE_UD_R = crate::BitReader<bool>;
#[doc = "Field `adpll_capcode_ud` writer - "]
pub type ADPLL_CAPCODE_UD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_vctrl_det_done` reader - "]
pub type ADPLL_VCTRL_DET_DONE_R = crate::BitReader<bool>;
#[doc = "Field `adpll_vctrl_det_done` writer - "]
pub type ADPLL_VCTRL_DET_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_freqerr_sign` reader - "]
pub type ADPLL_FREQERR_SIGN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_freqerr_sign` writer - "]
pub type ADPLL_FREQERR_SIGN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_freqerr_ou` reader - "]
pub type ADPLL_FREQERR_OU_R = crate::BitReader<bool>;
#[doc = "Field `adpll_freqerr_ou` writer - "]
pub type ADPLL_FREQERR_OU_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
#[doc = "Field `adpll_freqerr_det_done` reader - "]
pub type ADPLL_FREQERR_DET_DONE_R = crate::BitReader<bool>;
#[doc = "Field `adpll_freqerr_det_done` writer - "]
pub type ADPLL_FREQERR_DET_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_OUTPUT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_unlock_intrpt(&self) -> ADPLL_UNLOCK_INTRPT_R {
        ADPLL_UNLOCK_INTRPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adpll_lo_lock(&self) -> ADPLL_LO_LOCK_R {
        ADPLL_LO_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn adpll_fsm_state(&self) -> ADPLL_FSM_STATE_R {
        ADPLL_FSM_STATE_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn adpll_spd_unlock_sign(&self) -> ADPLL_SPD_UNLOCK_SIGN_R {
        ADPLL_SPD_UNLOCK_SIGN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adpll_vctrl_out_range_fsm(&self) -> ADPLL_VCTRL_OUT_RANGE_FSM_R {
        ADPLL_VCTRL_OUT_RANGE_FSM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adpll_mom_update_fail_fsm(&self) -> ADPLL_MOM_UPDATE_FAIL_FSM_R {
        ADPLL_MOM_UPDATE_FAIL_FSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adpll_mom_update_ou_fsm(&self) -> ADPLL_MOM_UPDATE_OU_FSM_R {
        ADPLL_MOM_UPDATE_OU_FSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn adpll_spd_unlock_fsm(&self) -> ADPLL_SPD_UNLOCK_FSM_R {
        ADPLL_SPD_UNLOCK_FSM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_spd_lock_fsm(&self) -> ADPLL_SPD_LOCK_FSM_R {
        ADPLL_SPD_LOCK_FSM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_fcal_done_fsm(&self) -> ADPLL_FCAL_DONE_FSM_R {
        ADPLL_FCAL_DONE_FSM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_capcode_out_range(&self) -> ADPLL_CAPCODE_OUT_RANGE_R {
        ADPLL_CAPCODE_OUT_RANGE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn adpll_mom_update_total_ou(&self) -> ADPLL_MOM_UPDATE_TOTAL_OU_R {
        ADPLL_MOM_UPDATE_TOTAL_OU_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adpll_capcode_ud(&self) -> ADPLL_CAPCODE_UD_R {
        ADPLL_CAPCODE_UD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adpll_vctrl_det_done(&self) -> ADPLL_VCTRL_DET_DONE_R {
        ADPLL_VCTRL_DET_DONE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adpll_freqerr_sign(&self) -> ADPLL_FREQERR_SIGN_R {
        ADPLL_FREQERR_SIGN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adpll_freqerr_ou(&self) -> ADPLL_FREQERR_OU_R {
        ADPLL_FREQERR_OU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adpll_freqerr_det_done(&self) -> ADPLL_FREQERR_DET_DONE_R {
        ADPLL_FREQERR_DET_DONE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_unlock_intrpt(&mut self) -> ADPLL_UNLOCK_INTRPT_W<0> {
        ADPLL_UNLOCK_INTRPT_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adpll_lo_lock(&mut self) -> ADPLL_LO_LOCK_W<1> {
        ADPLL_LO_LOCK_W::new(self)
    }
    #[doc = "Bits 3:6"]
    #[inline(always)]
    pub fn adpll_fsm_state(&mut self) -> ADPLL_FSM_STATE_W<3> {
        ADPLL_FSM_STATE_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn adpll_spd_unlock_sign(&mut self) -> ADPLL_SPD_UNLOCK_SIGN_W<7> {
        ADPLL_SPD_UNLOCK_SIGN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adpll_vctrl_out_range_fsm(&mut self) -> ADPLL_VCTRL_OUT_RANGE_FSM_W<8> {
        ADPLL_VCTRL_OUT_RANGE_FSM_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adpll_mom_update_fail_fsm(&mut self) -> ADPLL_MOM_UPDATE_FAIL_FSM_W<9> {
        ADPLL_MOM_UPDATE_FAIL_FSM_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adpll_mom_update_ou_fsm(&mut self) -> ADPLL_MOM_UPDATE_OU_FSM_W<10> {
        ADPLL_MOM_UPDATE_OU_FSM_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn adpll_spd_unlock_fsm(&mut self) -> ADPLL_SPD_UNLOCK_FSM_W<11> {
        ADPLL_SPD_UNLOCK_FSM_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_spd_lock_fsm(&mut self) -> ADPLL_SPD_LOCK_FSM_W<12> {
        ADPLL_SPD_LOCK_FSM_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_fcal_done_fsm(&mut self) -> ADPLL_FCAL_DONE_FSM_W<13> {
        ADPLL_FCAL_DONE_FSM_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_capcode_out_range(&mut self) -> ADPLL_CAPCODE_OUT_RANGE_W<14> {
        ADPLL_CAPCODE_OUT_RANGE_W::new(self)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn adpll_mom_update_total_ou(&mut self) -> ADPLL_MOM_UPDATE_TOTAL_OU_W<15> {
        ADPLL_MOM_UPDATE_TOTAL_OU_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adpll_capcode_ud(&mut self) -> ADPLL_CAPCODE_UD_W<17> {
        ADPLL_CAPCODE_UD_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adpll_vctrl_det_done(&mut self) -> ADPLL_VCTRL_DET_DONE_W<18> {
        ADPLL_VCTRL_DET_DONE_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adpll_freqerr_sign(&mut self) -> ADPLL_FREQERR_SIGN_W<19> {
        ADPLL_FREQERR_SIGN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adpll_freqerr_ou(&mut self) -> ADPLL_FREQERR_OU_W<20> {
        ADPLL_FREQERR_OU_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn adpll_freqerr_det_done(&mut self) -> ADPLL_FREQERR_DET_DONE_W<21> {
        ADPLL_FREQERR_DET_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_output.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_output](index.html) module"]
pub struct ADPLL_OUTPUT_SPEC;
impl crate::RegisterSpec for ADPLL_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_output::R](R) reader structure"]
impl crate::Readable for ADPLL_OUTPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_output::W](W) writer structure"]
impl crate::Writable for ADPLL_OUTPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_output to value 0"]
impl crate::Resettable for ADPLL_OUTPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
