#[doc = "Register `rf_cal_switch_ctrl` reader"]
pub struct R(crate::R<RF_CAL_SWITCH_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_CAL_SWITCH_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_CAL_SWITCH_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_CAL_SWITCH_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_cal_switch_ctrl` writer"]
pub struct W(crate::W<RF_CAL_SWITCH_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_CAL_SWITCH_CTRL_SPEC>;
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
impl From<crate::W<RF_CAL_SWITCH_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_CAL_SWITCH_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `acal_en` reader - "]
pub type ACAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `acal_en` writer - "]
pub type ACAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_CAL_SWITCH_CTRL_SPEC, bool, O>;
#[doc = "Field `kcal_en` reader - "]
pub type KCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `kcal_en` writer - "]
pub type KCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_CAL_SWITCH_CTRL_SPEC, bool, O>;
#[doc = "Field `rccal_en` reader - "]
pub type RCCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `rccal_en` writer - "]
pub type RCCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_CAL_SWITCH_CTRL_SPEC, bool, O>;
#[doc = "Field `inc_acal_en` reader - "]
pub type INC_ACAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `inc_acal_en` writer - "]
pub type INC_ACAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_SWITCH_CTRL_SPEC, bool, O>;
#[doc = "Field `inc_fcal_en` reader - "]
pub type INC_FCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `inc_fcal_en` writer - "]
pub type INC_FCAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_SWITCH_CTRL_SPEC, bool, O>;
#[doc = "Field `inc_fcal_en_hw` reader - "]
pub type INC_FCAL_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `inc_fcal_en_hw` writer - "]
pub type INC_FCAL_EN_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CAL_SWITCH_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acal_en(&self) -> ACAL_EN_R {
        ACAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn kcal_en(&self) -> KCAL_EN_R {
        KCAL_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rccal_en(&self) -> RCCAL_EN_R {
        RCCAL_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn inc_acal_en(&self) -> INC_ACAL_EN_R {
        INC_ACAL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_fcal_en(&self) -> INC_FCAL_EN_R {
        INC_FCAL_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn inc_fcal_en_hw(&self) -> INC_FCAL_EN_HW_R {
        INC_FCAL_EN_HW_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acal_en(&mut self) -> ACAL_EN_W<0> {
        ACAL_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn kcal_en(&mut self) -> KCAL_EN_W<4> {
        KCAL_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rccal_en(&mut self) -> RCCAL_EN_W<8> {
        RCCAL_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn inc_acal_en(&mut self) -> INC_ACAL_EN_W<12> {
        INC_ACAL_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_fcal_en(&mut self) -> INC_FCAL_EN_W<16> {
        INC_FCAL_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn inc_fcal_en_hw(&mut self) -> INC_FCAL_EN_HW_W<17> {
        INC_FCAL_EN_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_cal_switch_ctrl](index.html) module"]
pub struct RF_CAL_SWITCH_CTRL_SPEC;
impl crate::RegisterSpec for RF_CAL_SWITCH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_cal_switch_ctrl::R](R) reader structure"]
impl crate::Readable for RF_CAL_SWITCH_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_cal_switch_ctrl::W](W) writer structure"]
impl crate::Writable for RF_CAL_SWITCH_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_cal_switch_ctrl to value 0"]
impl crate::Resettable for RF_CAL_SWITCH_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
