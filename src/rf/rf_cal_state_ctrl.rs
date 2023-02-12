#[doc = "Register `rf_cal_state_ctrl` reader"]
pub struct R(crate::R<RF_CAL_STATE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_CAL_STATE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_CAL_STATE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_CAL_STATE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_cal_state_ctrl` writer"]
pub struct W(crate::W<RF_CAL_STATE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_CAL_STATE_CTRL_SPEC>;
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
impl From<crate::W<RF_CAL_STATE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_CAL_STATE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fcal_state_en` reader - "]
pub type FCAL_STATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `fcal_state_en` writer - "]
pub type FCAL_STATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_STATE_CTRL_SPEC, bool, O>;
#[doc = "Field `acal_state_en` reader - "]
pub type ACAL_STATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `acal_state_en` writer - "]
pub type ACAL_STATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_STATE_CTRL_SPEC, bool, O>;
#[doc = "Field `kcal_state_en` reader - "]
pub type KCAL_STATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `kcal_state_en` writer - "]
pub type KCAL_STATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_STATE_CTRL_SPEC, bool, O>;
#[doc = "Field `roscal_state_en` reader - "]
pub type ROSCAL_STATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `roscal_state_en` writer - "]
pub type ROSCAL_STATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_STATE_CTRL_SPEC, bool, O>;
#[doc = "Field `rccal_state_en` reader - "]
pub type RCCAL_STATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `rccal_state_en` writer - "]
pub type RCCAL_STATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_STATE_CTRL_SPEC, bool, O>;
#[doc = "Field `inc_fcal_state_en` reader - "]
pub type INC_FCAL_STATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `inc_fcal_state_en` writer - "]
pub type INC_FCAL_STATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_STATE_CTRL_SPEC, bool, O>;
#[doc = "Field `inc_acal_state_en` reader - "]
pub type INC_ACAL_STATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `inc_acal_state_en` writer - "]
pub type INC_ACAL_STATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_STATE_CTRL_SPEC, bool, O>;
#[doc = "Field `inc_roscal_state_en` reader - "]
pub type INC_ROSCAL_STATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `inc_roscal_state_en` writer - "]
pub type INC_ROSCAL_STATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_STATE_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fcal_state_en(&self) -> FCAL_STATE_EN_R {
        FCAL_STATE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acal_state_en(&self) -> ACAL_STATE_EN_R {
        ACAL_STATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn kcal_state_en(&self) -> KCAL_STATE_EN_R {
        KCAL_STATE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn roscal_state_en(&self) -> ROSCAL_STATE_EN_R {
        ROSCAL_STATE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rccal_state_en(&self) -> RCCAL_STATE_EN_R {
        RCCAL_STATE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inc_fcal_state_en(&self) -> INC_FCAL_STATE_EN_R {
        INC_FCAL_STATE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inc_acal_state_en(&self) -> INC_ACAL_STATE_EN_R {
        INC_ACAL_STATE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inc_roscal_state_en(&self) -> INC_ROSCAL_STATE_EN_R {
        INC_ROSCAL_STATE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fcal_state_en(&mut self) -> FCAL_STATE_EN_W<0> {
        FCAL_STATE_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn acal_state_en(&mut self) -> ACAL_STATE_EN_W<1> {
        ACAL_STATE_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn kcal_state_en(&mut self) -> KCAL_STATE_EN_W<2> {
        KCAL_STATE_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn roscal_state_en(&mut self) -> ROSCAL_STATE_EN_W<3> {
        ROSCAL_STATE_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rccal_state_en(&mut self) -> RCCAL_STATE_EN_W<4> {
        RCCAL_STATE_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn inc_fcal_state_en(&mut self) -> INC_FCAL_STATE_EN_W<5> {
        INC_FCAL_STATE_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn inc_acal_state_en(&mut self) -> INC_ACAL_STATE_EN_W<6> {
        INC_ACAL_STATE_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn inc_roscal_state_en(&mut self) -> INC_ROSCAL_STATE_EN_W<7> {
        INC_ROSCAL_STATE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf calibration state enable in full cal list\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_cal_state_ctrl](index.html) module"]
pub struct RF_CAL_STATE_CTRL_SPEC;
impl crate::RegisterSpec for RF_CAL_STATE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_cal_state_ctrl::R](R) reader structure"]
impl crate::Readable for RF_CAL_STATE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_cal_state_ctrl::W](W) writer structure"]
impl crate::Writable for RF_CAL_STATE_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_cal_state_ctrl to value 0"]
impl crate::Resettable for RF_CAL_STATE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
