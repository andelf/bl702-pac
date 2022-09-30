#[doc = "Register `adpll_vctrl` reader"]
pub struct R(crate::R<ADPLL_VCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_VCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_VCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_VCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_vctrl` writer"]
pub struct W(crate::W<ADPLL_VCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_VCTRL_SPEC>;
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
impl From<crate::W<ADPLL_VCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_VCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sdmout_dly_sel` reader - "]
pub type SDMOUT_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sdmout_dly_sel` writer - "]
pub type SDMOUT_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_VCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `sdm_bypass` reader - "]
pub type SDM_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `sdm_bypass` writer - "]
pub type SDM_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_VCTRL_SPEC, bool, O>;
#[doc = "Field `sdm_dither` reader - "]
pub type SDM_DITHER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sdm_dither` writer - "]
pub type SDM_DITHER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_VCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `sdm_order` reader - "]
pub type SDM_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `sdm_order` writer - "]
pub type SDM_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_VCTRL_SPEC, bool, O>;
#[doc = "Field `adpll_capcode_bypass` reader - "]
pub type ADPLL_CAPCODE_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `adpll_capcode_bypass` writer - "]
pub type ADPLL_CAPCODE_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_VCTRL_SPEC, bool, O>;
#[doc = "Field `adpll_dco_mash_bypass` reader - "]
pub type ADPLL_DCO_MASH_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `adpll_dco_mash_bypass` writer - "]
pub type ADPLL_DCO_MASH_BYPASS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_VCTRL_SPEC, bool, O>;
#[doc = "Field `adpll_force_mom_hold` reader - "]
pub type ADPLL_FORCE_MOM_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `adpll_force_mom_hold` writer - "]
pub type ADPLL_FORCE_MOM_HOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_VCTRL_SPEC, bool, O>;
#[doc = "Field `adpll_mom_update_period` reader - "]
pub type ADPLL_MOM_UPDATE_PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_mom_update_period` writer - "]
pub type ADPLL_MOM_UPDATE_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_VCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_vctrl_det_cons_en` reader - "]
pub type ADPLL_VCTRL_DET_CONS_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_vctrl_det_cons_en` writer - "]
pub type ADPLL_VCTRL_DET_CONS_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_VCTRL_SPEC, bool, O>;
#[doc = "Field `adpll_vctrl_moni_win_sel` reader - "]
pub type ADPLL_VCTRL_MONI_WIN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_vctrl_moni_win_sel` writer - "]
pub type ADPLL_VCTRL_MONI_WIN_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_VCTRL_SPEC, bool, O>;
#[doc = "Field `adpll_vctrl_lock_win_sel` reader - "]
pub type ADPLL_VCTRL_LOCK_WIN_SEL_R = crate::BitReader<bool>;
#[doc = "Field `adpll_vctrl_lock_win_sel` writer - "]
pub type ADPLL_VCTRL_LOCK_WIN_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_VCTRL_SPEC, bool, O>;
#[doc = "Field `adpll_vctrl_range_sel_ext_en` reader - "]
pub type ADPLL_VCTRL_RANGE_SEL_EXT_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_vctrl_range_sel_ext_en` writer - "]
pub type ADPLL_VCTRL_RANGE_SEL_EXT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_VCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sdmout_dly_sel(&self) -> SDMOUT_DLY_SEL_R {
        SDMOUT_DLY_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sdm_bypass(&self) -> SDM_BYPASS_R {
        SDM_BYPASS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sdm_dither(&self) -> SDM_DITHER_R {
        SDM_DITHER_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sdm_order(&self) -> SDM_ORDER_R {
        SDM_ORDER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_capcode_bypass(&self) -> ADPLL_CAPCODE_BYPASS_R {
        ADPLL_CAPCODE_BYPASS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adpll_dco_mash_bypass(&self) -> ADPLL_DCO_MASH_BYPASS_R {
        ADPLL_DCO_MASH_BYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adpll_force_mom_hold(&self) -> ADPLL_FORCE_MOM_HOLD_R {
        ADPLL_FORCE_MOM_HOLD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_mom_update_period(&self) -> ADPLL_MOM_UPDATE_PERIOD_R {
        ADPLL_MOM_UPDATE_PERIOD_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_vctrl_det_cons_en(&self) -> ADPLL_VCTRL_DET_CONS_EN_R {
        ADPLL_VCTRL_DET_CONS_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adpll_vctrl_moni_win_sel(&self) -> ADPLL_VCTRL_MONI_WIN_SEL_R {
        ADPLL_VCTRL_MONI_WIN_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn adpll_vctrl_lock_win_sel(&self) -> ADPLL_VCTRL_LOCK_WIN_SEL_R {
        ADPLL_VCTRL_LOCK_WIN_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_vctrl_range_sel_ext_en(&self) -> ADPLL_VCTRL_RANGE_SEL_EXT_EN_R {
        ADPLL_VCTRL_RANGE_SEL_EXT_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sdmout_dly_sel(&mut self) -> SDMOUT_DLY_SEL_W<0> {
        SDMOUT_DLY_SEL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sdm_bypass(&mut self) -> SDM_BYPASS_W<4> {
        SDM_BYPASS_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sdm_dither(&mut self) -> SDM_DITHER_W<8> {
        SDM_DITHER_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sdm_order(&mut self) -> SDM_ORDER_W<12> {
        SDM_ORDER_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn adpll_capcode_bypass(&mut self) -> ADPLL_CAPCODE_BYPASS_W<14> {
        ADPLL_CAPCODE_BYPASS_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adpll_dco_mash_bypass(&mut self) -> ADPLL_DCO_MASH_BYPASS_W<15> {
        ADPLL_DCO_MASH_BYPASS_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adpll_force_mom_hold(&mut self) -> ADPLL_FORCE_MOM_HOLD_W<16> {
        ADPLL_FORCE_MOM_HOLD_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_mom_update_period(&mut self) -> ADPLL_MOM_UPDATE_PERIOD_W<20> {
        ADPLL_MOM_UPDATE_PERIOD_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_vctrl_det_cons_en(&mut self) -> ADPLL_VCTRL_DET_CONS_EN_W<24> {
        ADPLL_VCTRL_DET_CONS_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adpll_vctrl_moni_win_sel(&mut self) -> ADPLL_VCTRL_MONI_WIN_SEL_W<25> {
        ADPLL_VCTRL_MONI_WIN_SEL_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn adpll_vctrl_lock_win_sel(&mut self) -> ADPLL_VCTRL_LOCK_WIN_SEL_W<26> {
        ADPLL_VCTRL_LOCK_WIN_SEL_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_vctrl_range_sel_ext_en(&mut self) -> ADPLL_VCTRL_RANGE_SEL_EXT_EN_W<27> {
        ADPLL_VCTRL_RANGE_SEL_EXT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_vctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_vctrl](index.html) module"]
pub struct ADPLL_VCTRL_SPEC;
impl crate::RegisterSpec for ADPLL_VCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_vctrl::R](R) reader structure"]
impl crate::Readable for ADPLL_VCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_vctrl::W](W) writer structure"]
impl crate::Writable for ADPLL_VCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_vctrl to value 0"]
impl crate::Resettable for ADPLL_VCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
