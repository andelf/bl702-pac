#[doc = "Register `pucr_reg` reader"]
pub struct R(crate::R<PUCR_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCR_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCR_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pucr_reg` writer"]
pub struct W(crate::W<PUCR_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR_REG_SPEC>;
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
impl From<crate::W<PUCR_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCR_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lodist_tx_en` reader - "]
pub type LODIST_TX_EN_R = crate::BitReader<bool>;
#[doc = "Field `lodist_tx_en` writer - "]
pub type LODIST_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_txbuf` reader - "]
pub type PU_TXBUF_R = crate::BitReader<bool>;
#[doc = "Field `pu_txbuf` writer - "]
pub type PU_TXBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_rxbuf` reader - "]
pub type PU_RXBUF_R = crate::BitReader<bool>;
#[doc = "Field `pu_rxbuf` writer - "]
pub type PU_RXBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_dtc` reader - "]
pub type PU_DTC_R = crate::BitReader<bool>;
#[doc = "Field `pu_dtc` writer - "]
pub type PU_DTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_adpll_sfreg` reader - "]
pub type PU_ADPLL_SFREG_R = crate::BitReader<bool>;
#[doc = "Field `pu_adpll_sfreg` writer - "]
pub type PU_ADPLL_SFREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_adpll_adc` reader - "]
pub type PU_ADPLL_ADC_R = crate::BitReader<bool>;
#[doc = "Field `pu_adpll_adc` writer - "]
pub type PU_ADPLL_ADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `adpll_clk_en` reader - "]
pub type ADPLL_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_clk_en` writer - "]
pub type ADPLL_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `lotpm_hfp_bypass` reader - "]
pub type LOTPM_HFP_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_hfp_bypass` writer - "]
pub type LOTPM_HFP_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `lotpm_lfp_bypass` reader - "]
pub type LOTPM_LFP_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_lfp_bypass` writer - "]
pub type LOTPM_LFP_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `lotpm_hfp_clk_en` reader - "]
pub type LOTPM_HFP_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_hfp_clk_en` writer - "]
pub type LOTPM_HFP_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_fbdv_buf` reader - "]
pub type PU_FBDV_BUF_R = crate::BitReader<bool>;
#[doc = "Field `pu_fbdv_buf` writer - "]
pub type PU_FBDV_BUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_fbdv` reader - "]
pub type PU_FBDV_R = crate::BitReader<bool>;
#[doc = "Field `pu_fbdv` writer - "]
pub type PU_FBDV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_vco` reader - "]
pub type PU_VCO_R = crate::BitReader<bool>;
#[doc = "Field `pu_vco` writer - "]
pub type PU_VCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_vco_ldo` reader - "]
pub type PU_VCO_LDO_R = crate::BitReader<bool>;
#[doc = "Field `pu_vco_ldo` writer - "]
pub type PU_VCO_LDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_lodist_body_bias` reader - "]
pub type PU_LODIST_BODY_BIAS_R = crate::BitReader<bool>;
#[doc = "Field `pu_lodist_body_bias` writer - "]
pub type PU_LODIST_BODY_BIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `rxadc_clk_en` reader - "]
pub type RXADC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `rxadc_clk_en` writer - "]
pub type RXADC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_rxadc` reader - "]
pub type PU_RXADC_R = crate::BitReader<bool>;
#[doc = "Field `pu_rxadc` writer - "]
pub type PU_RXADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_rosdac` reader - "]
pub type PU_ROSDAC_R = crate::BitReader<bool>;
#[doc = "Field `pu_rosdac` writer - "]
pub type PU_ROSDAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_rbb_pkdet` reader - "]
pub type PU_RBB_PKDET_R = crate::BitReader<bool>;
#[doc = "Field `pu_rbb_pkdet` writer - "]
pub type PU_RBB_PKDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_rbb` reader - "]
pub type PU_RBB_R = crate::BitReader<bool>;
#[doc = "Field `pu_rbb` writer - "]
pub type PU_RBB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_rmx` reader - "]
pub type PU_RMX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rmx` writer - "]
pub type PU_RMX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_lna` reader - "]
pub type PU_LNA_R = crate::BitReader<bool>;
#[doc = "Field `pu_lna` writer - "]
pub type PU_LNA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pu_pa` reader - "]
pub type PU_PA_R = crate::BitReader<bool>;
#[doc = "Field `pu_pa` writer - "]
pub type PU_PA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `rx_bypass_en` reader - "]
pub type RX_BYPASS_EN_R = crate::BitReader<bool>;
#[doc = "Field `rx_bypass_en` writer - "]
pub type RX_BYPASS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `pa_seri_cap_en` reader - "]
pub type PA_SERI_CAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `pa_seri_cap_en` writer - "]
pub type PA_SERI_CAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `rst_adpll` reader - "]
pub type RST_ADPLL_R = crate::BitReader<bool>;
#[doc = "Field `rst_adpll` writer - "]
pub type RST_ADPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `rst_lotpm_hfp` reader - "]
pub type RST_LOTPM_HFP_R = crate::BitReader<bool>;
#[doc = "Field `rst_lotpm_hfp` writer - "]
pub type RST_LOTPM_HFP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
#[doc = "Field `rst_fbdv` reader - "]
pub type RST_FBDV_R = crate::BitReader<bool>;
#[doc = "Field `rst_fbdv` writer - "]
pub type RST_FBDV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_REG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lodist_tx_en(&self) -> LODIST_TX_EN_R {
        LODIST_TX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_txbuf(&self) -> PU_TXBUF_R {
        PU_TXBUF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_rxbuf(&self) -> PU_RXBUF_R {
        PU_RXBUF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_dtc(&self) -> PU_DTC_R {
        PU_DTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_adpll_sfreg(&self) -> PU_ADPLL_SFREG_R {
        PU_ADPLL_SFREG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_adpll_adc(&self) -> PU_ADPLL_ADC_R {
        PU_ADPLL_ADC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_clk_en(&self) -> ADPLL_CLK_EN_R {
        ADPLL_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lotpm_hfp_bypass(&self) -> LOTPM_HFP_BYPASS_R {
        LOTPM_HFP_BYPASS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lotpm_lfp_bypass(&self) -> LOTPM_LFP_BYPASS_R {
        LOTPM_LFP_BYPASS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lotpm_hfp_clk_en(&self) -> LOTPM_HFP_CLK_EN_R {
        LOTPM_HFP_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_fbdv_buf(&self) -> PU_FBDV_BUF_R {
        PU_FBDV_BUF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_fbdv(&self) -> PU_FBDV_R {
        PU_FBDV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_vco(&self) -> PU_VCO_R {
        PU_VCO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pu_vco_ldo(&self) -> PU_VCO_LDO_R {
        PU_VCO_LDO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_lodist_body_bias(&self) -> PU_LODIST_BODY_BIAS_R {
        PU_LODIST_BODY_BIAS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxadc_clk_en(&self) -> RXADC_CLK_EN_R {
        RXADC_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_rxadc(&self) -> PU_RXADC_R {
        PU_RXADC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_rosdac(&self) -> PU_ROSDAC_R {
        PU_ROSDAC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_rbb_pkdet(&self) -> PU_RBB_PKDET_R {
        PU_RBB_PKDET_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_rbb(&self) -> PU_RBB_R {
        PU_RBB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_rmx(&self) -> PU_RMX_R {
        PU_RMX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_lna(&self) -> PU_LNA_R {
        PU_LNA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pa(&self) -> PU_PA_R {
        PU_PA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_bypass_en(&self) -> RX_BYPASS_EN_R {
        RX_BYPASS_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pa_seri_cap_en(&self) -> PA_SERI_CAP_EN_R {
        PA_SERI_CAP_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rst_adpll(&self) -> RST_ADPLL_R {
        RST_ADPLL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rst_lotpm_hfp(&self) -> RST_LOTPM_HFP_R {
        RST_LOTPM_HFP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rst_fbdv(&self) -> RST_FBDV_R {
        RST_FBDV_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lodist_tx_en(&mut self) -> LODIST_TX_EN_W<0> {
        LODIST_TX_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pu_txbuf(&mut self) -> PU_TXBUF_W<1> {
        PU_TXBUF_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rxbuf(&mut self) -> PU_RXBUF_W<2> {
        PU_RXBUF_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pu_dtc(&mut self) -> PU_DTC_W<3> {
        PU_DTC_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_adpll_sfreg(&mut self) -> PU_ADPLL_SFREG_W<4> {
        PU_ADPLL_SFREG_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_adpll_adc(&mut self) -> PU_ADPLL_ADC_W<5> {
        PU_ADPLL_ADC_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_clk_en(&mut self) -> ADPLL_CLK_EN_W<6> {
        ADPLL_CLK_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn lotpm_hfp_bypass(&mut self) -> LOTPM_HFP_BYPASS_W<7> {
        LOTPM_HFP_BYPASS_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lotpm_lfp_bypass(&mut self) -> LOTPM_LFP_BYPASS_W<8> {
        LOTPM_LFP_BYPASS_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn lotpm_hfp_clk_en(&mut self) -> LOTPM_HFP_CLK_EN_W<9> {
        LOTPM_HFP_CLK_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_fbdv_buf(&mut self) -> PU_FBDV_BUF_W<10> {
        PU_FBDV_BUF_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu_fbdv(&mut self) -> PU_FBDV_W<11> {
        PU_FBDV_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pu_vco(&mut self) -> PU_VCO_W<12> {
        PU_VCO_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pu_vco_ldo(&mut self) -> PU_VCO_LDO_W<13> {
        PU_VCO_LDO_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pu_lodist_body_bias(&mut self) -> PU_LODIST_BODY_BIAS_W<14> {
        PU_LODIST_BODY_BIAS_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rxadc_clk_en(&mut self) -> RXADC_CLK_EN_W<15> {
        RXADC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rxadc(&mut self) -> PU_RXADC_W<16> {
        PU_RXADC_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rosdac(&mut self) -> PU_ROSDAC_W<17> {
        PU_ROSDAC_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rbb_pkdet(&mut self) -> PU_RBB_PKDET_W<18> {
        PU_RBB_PKDET_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rbb(&mut self) -> PU_RBB_W<19> {
        PU_RBB_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rmx(&mut self) -> PU_RMX_W<20> {
        PU_RMX_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pu_lna(&mut self) -> PU_LNA_W<21> {
        PU_LNA_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pu_pa(&mut self) -> PU_PA_W<22> {
        PU_PA_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn rx_bypass_en(&mut self) -> RX_BYPASS_EN_W<23> {
        RX_BYPASS_EN_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pa_seri_cap_en(&mut self) -> PA_SERI_CAP_EN_W<24> {
        PA_SERI_CAP_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn rst_adpll(&mut self) -> RST_ADPLL_W<25> {
        RST_ADPLL_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn rst_lotpm_hfp(&mut self) -> RST_LOTPM_HFP_W<26> {
        RST_LOTPM_HFP_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn rst_fbdv(&mut self) -> RST_FBDV_W<27> {
        RST_FBDV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register control of power up signals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr_reg](index.html) module"]
pub struct PUCR_REG_SPEC;
impl crate::RegisterSpec for PUCR_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr_reg::R](R) reader structure"]
impl crate::Readable for PUCR_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr_reg::W](W) writer structure"]
impl crate::Writable for PUCR_REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pucr_reg to value 0"]
impl crate::Resettable for PUCR_REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
