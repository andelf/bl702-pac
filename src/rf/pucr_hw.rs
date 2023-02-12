#[doc = "Register `pucr_hw` reader"]
pub struct R(crate::R<PUCR_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCR_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCR_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pucr_hw` writer"]
pub struct W(crate::W<PUCR_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR_HW_SPEC>;
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
impl From<crate::W<PUCR_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCR_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lodist_tx_en_hw` reader - "]
pub type LODIST_TX_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `lodist_tx_en_hw` writer - "]
pub type LODIST_TX_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_txbuf_hw` reader - "]
pub type PU_TXBUF_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_txbuf_hw` writer - "]
pub type PU_TXBUF_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_rxbuf_hw` reader - "]
pub type PU_RXBUF_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rxbuf_hw` writer - "]
pub type PU_RXBUF_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_dtc_hw` reader - "]
pub type PU_DTC_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_dtc_hw` writer - "]
pub type PU_DTC_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_adpll_sfreg_hw` reader - "]
pub type PU_ADPLL_SFREG_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_adpll_sfreg_hw` writer - "]
pub type PU_ADPLL_SFREG_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_adpll_adc_hw` reader - "]
pub type PU_ADPLL_ADC_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_adpll_adc_hw` writer - "]
pub type PU_ADPLL_ADC_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `adpll_clk_en_hw` reader - "]
pub type ADPLL_CLK_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `adpll_clk_en_hw` writer - "]
pub type ADPLL_CLK_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `lotpm_hfp_bypass_hw` reader - "]
pub type LOTPM_HFP_BYPASS_HW_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_hfp_bypass_hw` writer - "]
pub type LOTPM_HFP_BYPASS_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `lotpm_lfp_bypass_hw` reader - "]
pub type LOTPM_LFP_BYPASS_HW_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_lfp_bypass_hw` writer - "]
pub type LOTPM_LFP_BYPASS_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `lotpm_hfp_clk_en_hw` reader - "]
pub type LOTPM_HFP_CLK_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `lotpm_hfp_clk_en_hw` writer - "]
pub type LOTPM_HFP_CLK_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_fbdv_buf_hw` reader - "]
pub type PU_FBDV_BUF_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_fbdv_buf_hw` writer - "]
pub type PU_FBDV_BUF_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_fbdv_hw` reader - "]
pub type PU_FBDV_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_fbdv_hw` writer - "]
pub type PU_FBDV_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_vco_hw` reader - "]
pub type PU_VCO_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_vco_hw` writer - "]
pub type PU_VCO_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_vco_ldo_hw` reader - "]
pub type PU_VCO_LDO_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_vco_ldo_hw` writer - "]
pub type PU_VCO_LDO_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_lodist_body_bias_hw` reader - "]
pub type PU_LODIST_BODY_BIAS_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_lodist_body_bias_hw` writer - "]
pub type PU_LODIST_BODY_BIAS_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `rxadc_clk_en_hw` reader - "]
pub type RXADC_CLK_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `rxadc_clk_en_hw` writer - "]
pub type RXADC_CLK_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_rxadc_hw` reader - "]
pub type PU_RXADC_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rxadc_hw` writer - "]
pub type PU_RXADC_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_rosdac_hw` reader - "]
pub type PU_ROSDAC_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rosdac_hw` writer - "]
pub type PU_ROSDAC_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_rbb_pkdet_hw` reader - "]
pub type PU_RBB_PKDET_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rbb_pkdet_hw` writer - "]
pub type PU_RBB_PKDET_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_rbb_hw` reader - "]
pub type PU_RBB_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rbb_hw` writer - "]
pub type PU_RBB_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_rmx_hw` reader - "]
pub type PU_RMX_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_rmx_hw` writer - "]
pub type PU_RMX_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_lna_hw` reader - "]
pub type PU_LNA_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_lna_hw` writer - "]
pub type PU_LNA_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pu_pa_hw` reader - "]
pub type PU_PA_HW_R = crate::BitReader<bool>;
#[doc = "Field `pu_pa_hw` writer - "]
pub type PU_PA_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `rx_bypass_en_hw` reader - "]
pub type RX_BYPASS_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `rx_bypass_en_hw` writer - "]
pub type RX_BYPASS_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `pa_seri_cap_en_hw` reader - "]
pub type PA_SERI_CAP_EN_HW_R = crate::BitReader<bool>;
#[doc = "Field `pa_seri_cap_en_hw` writer - "]
pub type PA_SERI_CAP_EN_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `rst_adpll_hw` reader - "]
pub type RST_ADPLL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rst_adpll_hw` writer - "]
pub type RST_ADPLL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `rst_lotpm_hfp_hw` reader - "]
pub type RST_LOTPM_HFP_HW_R = crate::BitReader<bool>;
#[doc = "Field `rst_lotpm_hfp_hw` writer - "]
pub type RST_LOTPM_HFP_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
#[doc = "Field `rst_fbdv_hw` reader - "]
pub type RST_FBDV_HW_R = crate::BitReader<bool>;
#[doc = "Field `rst_fbdv_hw` writer - "]
pub type RST_FBDV_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCR_HW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lodist_tx_en_hw(&self) -> LODIST_TX_EN_HW_R {
        LODIST_TX_EN_HW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_txbuf_hw(&self) -> PU_TXBUF_HW_R {
        PU_TXBUF_HW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_rxbuf_hw(&self) -> PU_RXBUF_HW_R {
        PU_RXBUF_HW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_dtc_hw(&self) -> PU_DTC_HW_R {
        PU_DTC_HW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_adpll_sfreg_hw(&self) -> PU_ADPLL_SFREG_HW_R {
        PU_ADPLL_SFREG_HW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_adpll_adc_hw(&self) -> PU_ADPLL_ADC_HW_R {
        PU_ADPLL_ADC_HW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn adpll_clk_en_hw(&self) -> ADPLL_CLK_EN_HW_R {
        ADPLL_CLK_EN_HW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lotpm_hfp_bypass_hw(&self) -> LOTPM_HFP_BYPASS_HW_R {
        LOTPM_HFP_BYPASS_HW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lotpm_lfp_bypass_hw(&self) -> LOTPM_LFP_BYPASS_HW_R {
        LOTPM_LFP_BYPASS_HW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lotpm_hfp_clk_en_hw(&self) -> LOTPM_HFP_CLK_EN_HW_R {
        LOTPM_HFP_CLK_EN_HW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_fbdv_buf_hw(&self) -> PU_FBDV_BUF_HW_R {
        PU_FBDV_BUF_HW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_fbdv_hw(&self) -> PU_FBDV_HW_R {
        PU_FBDV_HW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_vco_hw(&self) -> PU_VCO_HW_R {
        PU_VCO_HW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pu_vco_ldo_hw(&self) -> PU_VCO_LDO_HW_R {
        PU_VCO_LDO_HW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_lodist_body_bias_hw(&self) -> PU_LODIST_BODY_BIAS_HW_R {
        PU_LODIST_BODY_BIAS_HW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxadc_clk_en_hw(&self) -> RXADC_CLK_EN_HW_R {
        RXADC_CLK_EN_HW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_rxadc_hw(&self) -> PU_RXADC_HW_R {
        PU_RXADC_HW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_rosdac_hw(&self) -> PU_ROSDAC_HW_R {
        PU_ROSDAC_HW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_rbb_pkdet_hw(&self) -> PU_RBB_PKDET_HW_R {
        PU_RBB_PKDET_HW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_rbb_hw(&self) -> PU_RBB_HW_R {
        PU_RBB_HW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_rmx_hw(&self) -> PU_RMX_HW_R {
        PU_RMX_HW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_lna_hw(&self) -> PU_LNA_HW_R {
        PU_LNA_HW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pa_hw(&self) -> PU_PA_HW_R {
        PU_PA_HW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rx_bypass_en_hw(&self) -> RX_BYPASS_EN_HW_R {
        RX_BYPASS_EN_HW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pa_seri_cap_en_hw(&self) -> PA_SERI_CAP_EN_HW_R {
        PA_SERI_CAP_EN_HW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rst_adpll_hw(&self) -> RST_ADPLL_HW_R {
        RST_ADPLL_HW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rst_lotpm_hfp_hw(&self) -> RST_LOTPM_HFP_HW_R {
        RST_LOTPM_HFP_HW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rst_fbdv_hw(&self) -> RST_FBDV_HW_R {
        RST_FBDV_HW_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn lodist_tx_en_hw(&mut self) -> LODIST_TX_EN_HW_W<0> {
        LODIST_TX_EN_HW_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pu_txbuf_hw(&mut self) -> PU_TXBUF_HW_W<1> {
        PU_TXBUF_HW_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rxbuf_hw(&mut self) -> PU_RXBUF_HW_W<2> {
        PU_RXBUF_HW_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pu_dtc_hw(&mut self) -> PU_DTC_HW_W<3> {
        PU_DTC_HW_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn pu_adpll_sfreg_hw(&mut self) -> PU_ADPLL_SFREG_HW_W<4> {
        PU_ADPLL_SFREG_HW_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pu_adpll_adc_hw(&mut self) -> PU_ADPLL_ADC_HW_W<5> {
        PU_ADPLL_ADC_HW_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_clk_en_hw(&mut self) -> ADPLL_CLK_EN_HW_W<6> {
        ADPLL_CLK_EN_HW_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn lotpm_hfp_bypass_hw(&mut self) -> LOTPM_HFP_BYPASS_HW_W<7> {
        LOTPM_HFP_BYPASS_HW_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn lotpm_lfp_bypass_hw(&mut self) -> LOTPM_LFP_BYPASS_HW_W<8> {
        LOTPM_LFP_BYPASS_HW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn lotpm_hfp_clk_en_hw(&mut self) -> LOTPM_HFP_CLK_EN_HW_W<9> {
        LOTPM_HFP_CLK_EN_HW_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pu_fbdv_buf_hw(&mut self) -> PU_FBDV_BUF_HW_W<10> {
        PU_FBDV_BUF_HW_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pu_fbdv_hw(&mut self) -> PU_FBDV_HW_W<11> {
        PU_FBDV_HW_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn pu_vco_hw(&mut self) -> PU_VCO_HW_W<12> {
        PU_VCO_HW_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn pu_vco_ldo_hw(&mut self) -> PU_VCO_LDO_HW_W<13> {
        PU_VCO_LDO_HW_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn pu_lodist_body_bias_hw(&mut self) -> PU_LODIST_BODY_BIAS_HW_W<14> {
        PU_LODIST_BODY_BIAS_HW_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn rxadc_clk_en_hw(&mut self) -> RXADC_CLK_EN_HW_W<15> {
        RXADC_CLK_EN_HW_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rxadc_hw(&mut self) -> PU_RXADC_HW_W<16> {
        PU_RXADC_HW_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rosdac_hw(&mut self) -> PU_ROSDAC_HW_W<17> {
        PU_ROSDAC_HW_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rbb_pkdet_hw(&mut self) -> PU_RBB_PKDET_HW_W<18> {
        PU_RBB_PKDET_HW_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rbb_hw(&mut self) -> PU_RBB_HW_W<19> {
        PU_RBB_HW_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn pu_rmx_hw(&mut self) -> PU_RMX_HW_W<20> {
        PU_RMX_HW_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn pu_lna_hw(&mut self) -> PU_LNA_HW_W<21> {
        PU_LNA_HW_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn pu_pa_hw(&mut self) -> PU_PA_HW_W<22> {
        PU_PA_HW_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn rx_bypass_en_hw(&mut self) -> RX_BYPASS_EN_HW_W<23> {
        RX_BYPASS_EN_HW_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn pa_seri_cap_en_hw(&mut self) -> PA_SERI_CAP_EN_HW_W<24> {
        PA_SERI_CAP_EN_HW_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn rst_adpll_hw(&mut self) -> RST_ADPLL_HW_W<25> {
        RST_ADPLL_HW_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn rst_lotpm_hfp_hw(&mut self) -> RST_LOTPM_HFP_HW_W<26> {
        RST_LOTPM_HFP_HW_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn rst_fbdv_hw(&mut self) -> RST_FBDV_HW_W<27> {
        RST_FBDV_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hardware read back of power up signals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr_hw](index.html) module"]
pub struct PUCR_HW_SPEC;
impl crate::RegisterSpec for PUCR_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr_hw::R](R) reader structure"]
impl crate::Readable for PUCR_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr_hw::W](W) writer structure"]
impl crate::Writable for PUCR_HW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pucr_hw to value 0"]
impl crate::Resettable for PUCR_HW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
