#[doc = "Register `pucr_rx` reader"]
pub struct R(crate::R<PUCR_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCR_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCR_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pucr_rx` writer"]
pub struct W(crate::W<PUCR_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR_RX_SPEC>;
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
impl From<crate::W<PUCR_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCR_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lodist_tx_en_rx` reader - "]
pub type LODIST_TX_EN_RX_R = crate::BitReader<bool>;
#[doc = "Field `lodist_tx_en_rx` writer - "]
pub type LODIST_TX_EN_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_txbuf_rx` reader - "]
pub type PU_TXBUF_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_txbuf_rx` writer - "]
pub type PU_TXBUF_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_rxbuf_rx` reader - "]
pub type PU_RXBUF_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rxbuf_rx` writer - "]
pub type PU_RXBUF_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_dtc_rx` reader - "]
pub type PU_DTC_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_dtc_rx` writer - "]
pub type PU_DTC_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_adpll_sfreg_rx` reader - "]
pub type PU_ADPLL_SFREG_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_adpll_sfreg_rx` writer - "]
pub type PU_ADPLL_SFREG_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_adpll_adc_rx` reader - "]
pub type PU_ADPLL_ADC_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_adpll_adc_rx` writer - "]
pub type PU_ADPLL_ADC_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `adpll_clk_en_rx` reader - "]
pub type ADPLL_CLK_EN_RX_R = crate::BitReader<bool>;
#[doc = "Field `adpll_clk_en_rx` writer - "]
pub type ADPLL_CLK_EN_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `lotpm_hfp_bypass_rx` reader - "]
pub type LOTPM_HFP_BYPASS_RX_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_hfp_bypass_rx` writer - "]
pub type LOTPM_HFP_BYPASS_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `lotpm_lfp_bypass_rx` reader - "]
pub type LOTPM_LFP_BYPASS_RX_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_lfp_bypass_rx` writer - "]
pub type LOTPM_LFP_BYPASS_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `lotpm_hfp_clk_en_rx` reader - "]
pub type LOTPM_HFP_CLK_EN_RX_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_hfp_clk_en_rx` writer - "]
pub type LOTPM_HFP_CLK_EN_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_fbdv_buf_rx` reader - "]
pub type PU_FBDV_BUF_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_fbdv_buf_rx` writer - "]
pub type PU_FBDV_BUF_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_fbdv_rx` reader - "]
pub type PU_FBDV_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_fbdv_rx` writer - "]
pub type PU_FBDV_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_vco_rx` reader - "]
pub type PU_VCO_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_vco_rx` writer - "]
pub type PU_VCO_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_vco_ldo_rx` reader - "]
pub type PU_VCO_LDO_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_vco_ldo_rx` writer - "]
pub type PU_VCO_LDO_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_lodist_body_bias_rx` reader - "]
pub type PU_LODIST_BODY_BIAS_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_lodist_body_bias_rx` writer - "]
pub type PU_LODIST_BODY_BIAS_RX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `rxadc_clk_en_rx` reader - "]
pub type RXADC_CLK_EN_RX_R = crate::BitReader<bool>;
#[doc = "Field `rxadc_clk_en_rx` writer - "]
pub type RXADC_CLK_EN_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_rxadc_rx` reader - "]
pub type PU_RXADC_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rxadc_rx` writer - "]
pub type PU_RXADC_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_rosdac_rx` reader - "]
pub type PU_ROSDAC_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rosdac_rx` writer - "]
pub type PU_ROSDAC_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_rbb_pkdet_rx` reader - "]
pub type PU_RBB_PKDET_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rbb_pkdet_rx` writer - "]
pub type PU_RBB_PKDET_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_rbb_rx` reader - "]
pub type PU_RBB_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rbb_rx` writer - "]
pub type PU_RBB_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_rmx_rx` reader - "]
pub type PU_RMX_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_rmx_rx` writer - "]
pub type PU_RMX_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_lna_rx` reader - "]
pub type PU_LNA_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_lna_rx` writer - "]
pub type PU_LNA_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pu_pa_rx` reader - "]
pub type PU_PA_RX_R = crate::BitReader<bool>;
#[doc = "Field `pu_pa_rx` writer - "]
pub type PU_PA_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `rx_bypass_en_rx` reader - "]
pub type RX_BYPASS_EN_RX_R = crate::BitReader<bool>;
#[doc = "Field `rx_bypass_en_rx` writer - "]
pub type RX_BYPASS_EN_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
#[doc = "Field `pa_seri_cap_en_rx` reader - "]
pub type PA_SERI_CAP_EN_RX_R = crate::BitReader<bool>;
#[doc = "Field `pa_seri_cap_en_rx` writer - "]
pub type PA_SERI_CAP_EN_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_RX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lodist_tx_en_rx(&self) -> LODIST_TX_EN_RX_R {
        LODIST_TX_EN_RX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_txbuf_rx(&self) -> PU_TXBUF_RX_R {
        PU_TXBUF_RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_rxbuf_rx(&self) -> PU_RXBUF_RX_R {
        PU_RXBUF_RX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_dtc_rx(&self) -> PU_DTC_RX_R {
        PU_DTC_RX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_adpll_sfreg_rx(&self) -> PU_ADPLL_SFREG_RX_R {
        PU_ADPLL_SFREG_RX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_adpll_adc_rx(&self) -> PU_ADPLL_ADC_RX_R {
        PU_ADPLL_ADC_RX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_clk_en_rx(&self) -> ADPLL_CLK_EN_RX_R {
        ADPLL_CLK_EN_RX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lotpm_hfp_bypass_rx(&self) -> LOTPM_HFP_BYPASS_RX_R {
        LOTPM_HFP_BYPASS_RX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lotpm_lfp_bypass_rx(&self) -> LOTPM_LFP_BYPASS_RX_R {
        LOTPM_LFP_BYPASS_RX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lotpm_hfp_clk_en_rx(&self) -> LOTPM_HFP_CLK_EN_RX_R {
        LOTPM_HFP_CLK_EN_RX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_fbdv_buf_rx(&self) -> PU_FBDV_BUF_RX_R {
        PU_FBDV_BUF_RX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_fbdv_rx(&self) -> PU_FBDV_RX_R {
        PU_FBDV_RX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_vco_rx(&self) -> PU_VCO_RX_R {
        PU_VCO_RX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pu_vco_ldo_rx(&self) -> PU_VCO_LDO_RX_R {
        PU_VCO_LDO_RX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_lodist_body_bias_rx(&self) -> PU_LODIST_BODY_BIAS_RX_R {
        PU_LODIST_BODY_BIAS_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxadc_clk_en_rx(&self) -> RXADC_CLK_EN_RX_R {
        RXADC_CLK_EN_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_rxadc_rx(&self) -> PU_RXADC_RX_R {
        PU_RXADC_RX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_rosdac_rx(&self) -> PU_ROSDAC_RX_R {
        PU_ROSDAC_RX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_rbb_pkdet_rx(&self) -> PU_RBB_PKDET_RX_R {
        PU_RBB_PKDET_RX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_rbb_rx(&self) -> PU_RBB_RX_R {
        PU_RBB_RX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_rmx_rx(&self) -> PU_RMX_RX_R {
        PU_RMX_RX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_lna_rx(&self) -> PU_LNA_RX_R {
        PU_LNA_RX_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pa_rx(&self) -> PU_PA_RX_R {
        PU_PA_RX_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_bypass_en_rx(&self) -> RX_BYPASS_EN_RX_R {
        RX_BYPASS_EN_RX_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pa_seri_cap_en_rx(&self) -> PA_SERI_CAP_EN_RX_R {
        PA_SERI_CAP_EN_RX_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lodist_tx_en_rx(&mut self) -> LODIST_TX_EN_RX_W<0> {
        LODIST_TX_EN_RX_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pu_txbuf_rx(&mut self) -> PU_TXBUF_RX_W<1> {
        PU_TXBUF_RX_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rxbuf_rx(&mut self) -> PU_RXBUF_RX_W<2> {
        PU_RXBUF_RX_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pu_dtc_rx(&mut self) -> PU_DTC_RX_W<3> {
        PU_DTC_RX_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_adpll_sfreg_rx(&mut self) -> PU_ADPLL_SFREG_RX_W<4> {
        PU_ADPLL_SFREG_RX_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_adpll_adc_rx(&mut self) -> PU_ADPLL_ADC_RX_W<5> {
        PU_ADPLL_ADC_RX_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_clk_en_rx(&mut self) -> ADPLL_CLK_EN_RX_W<6> {
        ADPLL_CLK_EN_RX_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn lotpm_hfp_bypass_rx(&mut self) -> LOTPM_HFP_BYPASS_RX_W<7> {
        LOTPM_HFP_BYPASS_RX_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lotpm_lfp_bypass_rx(&mut self) -> LOTPM_LFP_BYPASS_RX_W<8> {
        LOTPM_LFP_BYPASS_RX_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn lotpm_hfp_clk_en_rx(&mut self) -> LOTPM_HFP_CLK_EN_RX_W<9> {
        LOTPM_HFP_CLK_EN_RX_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_fbdv_buf_rx(&mut self) -> PU_FBDV_BUF_RX_W<10> {
        PU_FBDV_BUF_RX_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu_fbdv_rx(&mut self) -> PU_FBDV_RX_W<11> {
        PU_FBDV_RX_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pu_vco_rx(&mut self) -> PU_VCO_RX_W<12> {
        PU_VCO_RX_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pu_vco_ldo_rx(&mut self) -> PU_VCO_LDO_RX_W<13> {
        PU_VCO_LDO_RX_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pu_lodist_body_bias_rx(&mut self) -> PU_LODIST_BODY_BIAS_RX_W<14> {
        PU_LODIST_BODY_BIAS_RX_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rxadc_clk_en_rx(&mut self) -> RXADC_CLK_EN_RX_W<15> {
        RXADC_CLK_EN_RX_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rxadc_rx(&mut self) -> PU_RXADC_RX_W<16> {
        PU_RXADC_RX_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rosdac_rx(&mut self) -> PU_ROSDAC_RX_W<17> {
        PU_ROSDAC_RX_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rbb_pkdet_rx(&mut self) -> PU_RBB_PKDET_RX_W<18> {
        PU_RBB_PKDET_RX_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rbb_rx(&mut self) -> PU_RBB_RX_W<19> {
        PU_RBB_RX_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rmx_rx(&mut self) -> PU_RMX_RX_W<20> {
        PU_RMX_RX_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pu_lna_rx(&mut self) -> PU_LNA_RX_W<21> {
        PU_LNA_RX_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pu_pa_rx(&mut self) -> PU_PA_RX_W<22> {
        PU_PA_RX_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn rx_bypass_en_rx(&mut self) -> RX_BYPASS_EN_RX_W<23> {
        RX_BYPASS_EN_RX_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pa_seri_cap_en_rx(&mut self) -> PA_SERI_CAP_EN_RX_W<24> {
        PA_SERI_CAP_EN_RX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power up in RX state\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr_rx](index.html) module"]
pub struct PUCR_RX_SPEC;
impl crate::RegisterSpec for PUCR_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr_rx::R](R) reader structure"]
impl crate::Readable for PUCR_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr_rx::W](W) writer structure"]
impl crate::Writable for PUCR_RX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pucr_rx to value 0"]
impl crate::Resettable for PUCR_RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
