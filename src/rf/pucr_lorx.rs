#[doc = "Register `pucr_lorx` reader"]
pub struct R(crate::R<PUCR_LORX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR_LORX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCR_LORX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCR_LORX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pucr_lorx` writer"]
pub struct W(crate::W<PUCR_LORX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR_LORX_SPEC>;
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
impl From<crate::W<PUCR_LORX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCR_LORX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lodist_tx_en_lorx` reader - "]
pub type LODIST_TX_EN_LORX_R = crate::BitReader<bool>;
#[doc = "Field `lodist_tx_en_lorx` writer - "]
pub type LODIST_TX_EN_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_txbuf_lorx` reader - "]
pub type PU_TXBUF_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_txbuf_lorx` writer - "]
pub type PU_TXBUF_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_rxbuf_lorx` reader - "]
pub type PU_RXBUF_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rxbuf_lorx` writer - "]
pub type PU_RXBUF_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_dtc_lorx` reader - "]
pub type PU_DTC_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_dtc_lorx` writer - "]
pub type PU_DTC_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_adpll_sfreg_lorx` reader - "]
pub type PU_ADPLL_SFREG_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_adpll_sfreg_lorx` writer - "]
pub type PU_ADPLL_SFREG_LORX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_adpll_adc_lorx` reader - "]
pub type PU_ADPLL_ADC_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_adpll_adc_lorx` writer - "]
pub type PU_ADPLL_ADC_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `adpll_clk_en_lorx` reader - "]
pub type ADPLL_CLK_EN_LORX_R = crate::BitReader<bool>;
#[doc = "Field `adpll_clk_en_lorx` writer - "]
pub type ADPLL_CLK_EN_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `lotpm_hfp_bypass_lorx` reader - "]
pub type LOTPM_HFP_BYPASS_LORX_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_hfp_bypass_lorx` writer - "]
pub type LOTPM_HFP_BYPASS_LORX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `lotpm_lfp_bypass_lorx` reader - "]
pub type LOTPM_LFP_BYPASS_LORX_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_lfp_bypass_lorx` writer - "]
pub type LOTPM_LFP_BYPASS_LORX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `lotpm_hfp_clk_en_lorx` reader - "]
pub type LOTPM_HFP_CLK_EN_LORX_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_hfp_clk_en_lorx` writer - "]
pub type LOTPM_HFP_CLK_EN_LORX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_fbdv_buf_lorx` reader - "]
pub type PU_FBDV_BUF_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_fbdv_buf_lorx` writer - "]
pub type PU_FBDV_BUF_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_fbdv_lorx` reader - "]
pub type PU_FBDV_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_fbdv_lorx` writer - "]
pub type PU_FBDV_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_vco_lorx` reader - "]
pub type PU_VCO_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_vco_lorx` writer - "]
pub type PU_VCO_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_vco_ldo_lorx` reader - "]
pub type PU_VCO_LDO_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_vco_ldo_lorx` writer - "]
pub type PU_VCO_LDO_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_lodist_body_bias_lorx` reader - "]
pub type PU_LODIST_BODY_BIAS_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_lodist_body_bias_lorx` writer - "]
pub type PU_LODIST_BODY_BIAS_LORX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `rxadc_clk_en_lorx` reader - "]
pub type RXADC_CLK_EN_LORX_R = crate::BitReader<bool>;
#[doc = "Field `rxadc_clk_en_lorx` writer - "]
pub type RXADC_CLK_EN_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_rxadc_lorx` reader - "]
pub type PU_RXADC_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rxadc_lorx` writer - "]
pub type PU_RXADC_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_rosdac_lorx` reader - "]
pub type PU_ROSDAC_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rosdac_lorx` writer - "]
pub type PU_ROSDAC_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_rbb_pkdet_lorx` reader - "]
pub type PU_RBB_PKDET_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rbb_pkdet_lorx` writer - "]
pub type PU_RBB_PKDET_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_rbb_lorx` reader - "]
pub type PU_RBB_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rbb_lorx` writer - "]
pub type PU_RBB_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_rmx_lorx` reader - "]
pub type PU_RMX_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rmx_lorx` writer - "]
pub type PU_RMX_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_lna_lorx` reader - "]
pub type PU_LNA_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_lna_lorx` writer - "]
pub type PU_LNA_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pu_pa_lorx` reader - "]
pub type PU_PA_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pu_pa_lorx` writer - "]
pub type PU_PA_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `rx_bypass_en_lorx` reader - "]
pub type RX_BYPASS_EN_LORX_R = crate::BitReader<bool>;
#[doc = "Field `rx_bypass_en_lorx` writer - "]
pub type RX_BYPASS_EN_LORX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
#[doc = "Field `pa_seri_cap_en_lorx` reader - "]
pub type PA_SERI_CAP_EN_LORX_R = crate::BitReader<bool>;
#[doc = "Field `pa_seri_cap_en_lorx` writer - "]
pub type PA_SERI_CAP_EN_LORX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PUCR_LORX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lodist_tx_en_lorx(&self) -> LODIST_TX_EN_LORX_R {
        LODIST_TX_EN_LORX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_txbuf_lorx(&self) -> PU_TXBUF_LORX_R {
        PU_TXBUF_LORX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_rxbuf_lorx(&self) -> PU_RXBUF_LORX_R {
        PU_RXBUF_LORX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_dtc_lorx(&self) -> PU_DTC_LORX_R {
        PU_DTC_LORX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_adpll_sfreg_lorx(&self) -> PU_ADPLL_SFREG_LORX_R {
        PU_ADPLL_SFREG_LORX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_adpll_adc_lorx(&self) -> PU_ADPLL_ADC_LORX_R {
        PU_ADPLL_ADC_LORX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_clk_en_lorx(&self) -> ADPLL_CLK_EN_LORX_R {
        ADPLL_CLK_EN_LORX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lotpm_hfp_bypass_lorx(&self) -> LOTPM_HFP_BYPASS_LORX_R {
        LOTPM_HFP_BYPASS_LORX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lotpm_lfp_bypass_lorx(&self) -> LOTPM_LFP_BYPASS_LORX_R {
        LOTPM_LFP_BYPASS_LORX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lotpm_hfp_clk_en_lorx(&self) -> LOTPM_HFP_CLK_EN_LORX_R {
        LOTPM_HFP_CLK_EN_LORX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_fbdv_buf_lorx(&self) -> PU_FBDV_BUF_LORX_R {
        PU_FBDV_BUF_LORX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_fbdv_lorx(&self) -> PU_FBDV_LORX_R {
        PU_FBDV_LORX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_vco_lorx(&self) -> PU_VCO_LORX_R {
        PU_VCO_LORX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pu_vco_ldo_lorx(&self) -> PU_VCO_LDO_LORX_R {
        PU_VCO_LDO_LORX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_lodist_body_bias_lorx(&self) -> PU_LODIST_BODY_BIAS_LORX_R {
        PU_LODIST_BODY_BIAS_LORX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxadc_clk_en_lorx(&self) -> RXADC_CLK_EN_LORX_R {
        RXADC_CLK_EN_LORX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_rxadc_lorx(&self) -> PU_RXADC_LORX_R {
        PU_RXADC_LORX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_rosdac_lorx(&self) -> PU_ROSDAC_LORX_R {
        PU_ROSDAC_LORX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_rbb_pkdet_lorx(&self) -> PU_RBB_PKDET_LORX_R {
        PU_RBB_PKDET_LORX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_rbb_lorx(&self) -> PU_RBB_LORX_R {
        PU_RBB_LORX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_rmx_lorx(&self) -> PU_RMX_LORX_R {
        PU_RMX_LORX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_lna_lorx(&self) -> PU_LNA_LORX_R {
        PU_LNA_LORX_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pa_lorx(&self) -> PU_PA_LORX_R {
        PU_PA_LORX_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_bypass_en_lorx(&self) -> RX_BYPASS_EN_LORX_R {
        RX_BYPASS_EN_LORX_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pa_seri_cap_en_lorx(&self) -> PA_SERI_CAP_EN_LORX_R {
        PA_SERI_CAP_EN_LORX_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lodist_tx_en_lorx(&mut self) -> LODIST_TX_EN_LORX_W<0> {
        LODIST_TX_EN_LORX_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_txbuf_lorx(&mut self) -> PU_TXBUF_LORX_W<1> {
        PU_TXBUF_LORX_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_rxbuf_lorx(&mut self) -> PU_RXBUF_LORX_W<2> {
        PU_RXBUF_LORX_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_dtc_lorx(&mut self) -> PU_DTC_LORX_W<3> {
        PU_DTC_LORX_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_adpll_sfreg_lorx(&mut self) -> PU_ADPLL_SFREG_LORX_W<4> {
        PU_ADPLL_SFREG_LORX_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_adpll_adc_lorx(&mut self) -> PU_ADPLL_ADC_LORX_W<5> {
        PU_ADPLL_ADC_LORX_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_clk_en_lorx(&mut self) -> ADPLL_CLK_EN_LORX_W<6> {
        ADPLL_CLK_EN_LORX_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lotpm_hfp_bypass_lorx(&mut self) -> LOTPM_HFP_BYPASS_LORX_W<7> {
        LOTPM_HFP_BYPASS_LORX_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lotpm_lfp_bypass_lorx(&mut self) -> LOTPM_LFP_BYPASS_LORX_W<8> {
        LOTPM_LFP_BYPASS_LORX_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lotpm_hfp_clk_en_lorx(&mut self) -> LOTPM_HFP_CLK_EN_LORX_W<9> {
        LOTPM_HFP_CLK_EN_LORX_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_fbdv_buf_lorx(&mut self) -> PU_FBDV_BUF_LORX_W<10> {
        PU_FBDV_BUF_LORX_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_fbdv_lorx(&mut self) -> PU_FBDV_LORX_W<11> {
        PU_FBDV_LORX_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_vco_lorx(&mut self) -> PU_VCO_LORX_W<12> {
        PU_VCO_LORX_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pu_vco_ldo_lorx(&mut self) -> PU_VCO_LDO_LORX_W<13> {
        PU_VCO_LDO_LORX_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_lodist_body_bias_lorx(&mut self) -> PU_LODIST_BODY_BIAS_LORX_W<14> {
        PU_LODIST_BODY_BIAS_LORX_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxadc_clk_en_lorx(&mut self) -> RXADC_CLK_EN_LORX_W<15> {
        RXADC_CLK_EN_LORX_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_rxadc_lorx(&mut self) -> PU_RXADC_LORX_W<16> {
        PU_RXADC_LORX_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_rosdac_lorx(&mut self) -> PU_ROSDAC_LORX_W<17> {
        PU_ROSDAC_LORX_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_rbb_pkdet_lorx(&mut self) -> PU_RBB_PKDET_LORX_W<18> {
        PU_RBB_PKDET_LORX_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_rbb_lorx(&mut self) -> PU_RBB_LORX_W<19> {
        PU_RBB_LORX_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_rmx_lorx(&mut self) -> PU_RMX_LORX_W<20> {
        PU_RMX_LORX_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_lna_lorx(&mut self) -> PU_LNA_LORX_W<21> {
        PU_LNA_LORX_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pa_lorx(&mut self) -> PU_PA_LORX_W<22> {
        PU_PA_LORX_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_bypass_en_lorx(&mut self) -> RX_BYPASS_EN_LORX_W<23> {
        RX_BYPASS_EN_LORX_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pa_seri_cap_en_lorx(&mut self) -> PA_SERI_CAP_EN_LORX_W<24> {
        PA_SERI_CAP_EN_LORX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power up in LORX state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr_lorx](index.html) module"]
pub struct PUCR_LORX_SPEC;
impl crate::RegisterSpec for PUCR_LORX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr_lorx::R](R) reader structure"]
impl crate::Readable for PUCR_LORX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr_lorx::W](W) writer structure"]
impl crate::Writable for PUCR_LORX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pucr_lorx to value 0"]
impl crate::Resettable for PUCR_LORX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
