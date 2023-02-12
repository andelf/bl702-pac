#[doc = "Register `rf_ctrl_source` reader"]
pub struct R(crate::R<RF_CTRL_SOURCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_CTRL_SOURCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_CTRL_SOURCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_CTRL_SOURCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_ctrl_source` writer"]
pub struct W(crate::W<RF_CTRL_SOURCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_CTRL_SOURCE_SPEC>;
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
impl From<crate::W<RF_CTRL_SOURCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_CTRL_SOURCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_ctrl_hw` reader - "]
pub type PU_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_ctrl_hw` writer - "]
pub type PU_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_CTRL_SOURCE_SPEC, bool, O>;
#[doc = "Field `gain_ctrl_tx_hw` reader - "]
pub type GAIN_CTRL_TX_HW_R = crate::BitReader<bool>;
#[doc = "Field `gain_ctrl_tx_hw` writer - "]
pub type GAIN_CTRL_TX_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CTRL_SOURCE_SPEC, bool, O>;
#[doc = "Field `gain_ctrl_rx_hw` reader - "]
pub type GAIN_CTRL_RX_HW_R = crate::BitReader<bool>;
#[doc = "Field `gain_ctrl_rx_hw` writer - "]
pub type GAIN_CTRL_RX_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CTRL_SOURCE_SPEC, bool, O>;
#[doc = "Field `rosdac_ctrl_hw` reader - "]
pub type ROSDAC_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rosdac_ctrl_hw` writer - "]
pub type ROSDAC_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CTRL_SOURCE_SPEC, bool, O>;
#[doc = "Field `rosdac_ctrl_rccal` reader - "]
pub type ROSDAC_CTRL_RCCAL_R = crate::BitReader<bool>;
#[doc = "Field `rosdac_ctrl_rccal` writer - "]
pub type ROSDAC_CTRL_RCCAL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CTRL_SOURCE_SPEC, bool, O>;
#[doc = "Field `kcal_ratio_ctrl_hw` reader - "]
pub type KCAL_RATIO_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `kcal_ratio_ctrl_hw` writer - "]
pub type KCAL_RATIO_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CTRL_SOURCE_SPEC, bool, O>;
#[doc = "Field `rbb_bw_ctrl_hw` reader - "]
pub type RBB_BW_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bw_ctrl_hw` writer - "]
pub type RBB_BW_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CTRL_SOURCE_SPEC, bool, O>;
#[doc = "Field `lo_fcw_ctrl_hw` reader - "]
pub type LO_FCW_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `lo_fcw_ctrl_hw` writer - "]
pub type LO_FCW_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CTRL_SOURCE_SPEC, bool, O>;
#[doc = "Field `inc_fcal_en_ctrl_hw` reader - "]
pub type INC_FCAL_EN_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `inc_fcal_en_ctrl_hw` writer - "]
pub type INC_FCAL_EN_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CTRL_SOURCE_SPEC, bool, O>;
#[doc = "Field `vco_idac_ctrl_hw` reader - "]
pub type VCO_IDAC_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `vco_idac_ctrl_hw` writer - "]
pub type VCO_IDAC_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_CTRL_SOURCE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ctrl_hw(&self) -> PU_CTRL_HW_R {
        PU_CTRL_HW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gain_ctrl_tx_hw(&self) -> GAIN_CTRL_TX_HW_R {
        GAIN_CTRL_TX_HW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gain_ctrl_rx_hw(&self) -> GAIN_CTRL_RX_HW_R {
        GAIN_CTRL_RX_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rosdac_ctrl_hw(&self) -> ROSDAC_CTRL_HW_R {
        ROSDAC_CTRL_HW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rosdac_ctrl_rccal(&self) -> ROSDAC_CTRL_RCCAL_R {
        ROSDAC_CTRL_RCCAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn kcal_ratio_ctrl_hw(&self) -> KCAL_RATIO_CTRL_HW_R {
        KCAL_RATIO_CTRL_HW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rbb_bw_ctrl_hw(&self) -> RBB_BW_CTRL_HW_R {
        RBB_BW_CTRL_HW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_fcw_ctrl_hw(&self) -> LO_FCW_CTRL_HW_R {
        LO_FCW_CTRL_HW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn inc_fcal_en_ctrl_hw(&self) -> INC_FCAL_EN_CTRL_HW_R {
        INC_FCAL_EN_CTRL_HW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn vco_idac_ctrl_hw(&self) -> VCO_IDAC_CTRL_HW_R {
        VCO_IDAC_CTRL_HW_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pu_ctrl_hw(&mut self) -> PU_CTRL_HW_W<0> {
        PU_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl_tx_hw(&mut self) -> GAIN_CTRL_TX_HW_W<3> {
        GAIN_CTRL_TX_HW_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl_rx_hw(&mut self) -> GAIN_CTRL_RX_HW_W<4> {
        GAIN_CTRL_RX_HW_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_ctrl_hw(&mut self) -> ROSDAC_CTRL_HW_W<8> {
        ROSDAC_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_ctrl_rccal(&mut self) -> ROSDAC_CTRL_RCCAL_W<9> {
        ROSDAC_CTRL_RCCAL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn kcal_ratio_ctrl_hw(&mut self) -> KCAL_RATIO_CTRL_HW_W<12> {
        KCAL_RATIO_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_bw_ctrl_hw(&mut self) -> RBB_BW_CTRL_HW_W<16> {
        RBB_BW_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn lo_fcw_ctrl_hw(&mut self) -> LO_FCW_CTRL_HW_W<20> {
        LO_FCW_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn inc_fcal_en_ctrl_hw(&mut self) -> INC_FCAL_EN_CTRL_HW_W<24> {
        INC_FCAL_EN_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn vco_idac_ctrl_hw(&mut self) -> VCO_IDAC_CTRL_HW_W<28> {
        VCO_IDAC_CTRL_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control logic switch\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ctrl_source](index.html) module"]
pub struct RF_CTRL_SOURCE_SPEC;
impl crate::RegisterSpec for RF_CTRL_SOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ctrl_source::R](R) reader structure"]
impl crate::Readable for RF_CTRL_SOURCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ctrl_source::W](W) writer structure"]
impl crate::Writable for RF_CTRL_SOURCE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_ctrl_source to value 0"]
impl crate::Resettable for RF_CTRL_SOURCE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
