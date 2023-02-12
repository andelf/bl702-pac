#[doc = "Register `non_reg_readback` reader"]
pub struct R(crate::R<NON_REG_READBACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NON_REG_READBACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NON_REG_READBACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NON_REG_READBACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `non_reg_readback` writer"]
pub struct W(crate::W<NON_REG_READBACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NON_REG_READBACK_SPEC>;
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
impl From<crate::W<NON_REG_READBACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NON_REG_READBACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ppu_testbuf_hw` reader - "]
pub type PPU_TESTBUF_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_testbuf_hw` writer - "]
pub type PPU_TESTBUF_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
#[doc = "Field `ppu_txbuf_hw` reader - "]
pub type PPU_TXBUF_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_txbuf_hw` writer - "]
pub type PPU_TXBUF_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
#[doc = "Field `ppu_rxbuf_hw` reader - "]
pub type PPU_RXBUF_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_rxbuf_hw` writer - "]
pub type PPU_RXBUF_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
#[doc = "Field `ppu_adpll_sfreg_hw` reader - "]
pub type PPU_ADPLL_SFREG_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_adpll_sfreg_hw` writer - "]
pub type PPU_ADPLL_SFREG_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
#[doc = "Field `ppu_fbdv_hw` reader - "]
pub type PPU_FBDV_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_fbdv_hw` writer - "]
pub type PPU_FBDV_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
#[doc = "Field `pud_vco_hw` reader - "]
pub type PUD_VCO_HW_R = crate::BitReader<bool>;
#[doc = "Field `pud_vco_hw` writer - "]
pub type PUD_VCO_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
#[doc = "Field `ppu_vco_hw` reader - "]
pub type PPU_VCO_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_vco_hw` writer - "]
pub type PPU_VCO_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
#[doc = "Field `ppu_vco_ldo_hw` reader - "]
pub type PPU_VCO_LDO_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_vco_ldo_hw` writer - "]
pub type PPU_VCO_LDO_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
#[doc = "Field `ppu_lodist_body_bias_hw` reader - "]
pub type PPU_LODIST_BODY_BIAS_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_lodist_body_bias_hw` writer - "]
pub type PPU_LODIST_BODY_BIAS_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
#[doc = "Field `ppu_rbb_hw` reader - "]
pub type PPU_RBB_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_rbb_hw` writer - "]
pub type PPU_RBB_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
#[doc = "Field `ppu_lna_hw` reader - "]
pub type PPU_LNA_HW_R = crate::BitReader<bool>;
#[doc = "Field `ppu_lna_hw` writer - "]
pub type PPU_LNA_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, NON_REG_READBACK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ppu_testbuf_hw(&self) -> PPU_TESTBUF_HW_R {
        PPU_TESTBUF_HW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ppu_txbuf_hw(&self) -> PPU_TXBUF_HW_R {
        PPU_TXBUF_HW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ppu_rxbuf_hw(&self) -> PPU_RXBUF_HW_R {
        PPU_RXBUF_HW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ppu_adpll_sfreg_hw(&self) -> PPU_ADPLL_SFREG_HW_R {
        PPU_ADPLL_SFREG_HW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ppu_fbdv_hw(&self) -> PPU_FBDV_HW_R {
        PPU_FBDV_HW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pud_vco_hw(&self) -> PUD_VCO_HW_R {
        PUD_VCO_HW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ppu_vco_hw(&self) -> PPU_VCO_HW_R {
        PPU_VCO_HW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ppu_vco_ldo_hw(&self) -> PPU_VCO_LDO_HW_R {
        PPU_VCO_LDO_HW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ppu_lodist_body_bias_hw(&self) -> PPU_LODIST_BODY_BIAS_HW_R {
        PPU_LODIST_BODY_BIAS_HW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ppu_rbb_hw(&self) -> PPU_RBB_HW_R {
        PPU_RBB_HW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ppu_lna_hw(&self) -> PPU_LNA_HW_R {
        PPU_LNA_HW_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_testbuf_hw(&mut self) -> PPU_TESTBUF_HW_W<5> {
        PPU_TESTBUF_HW_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_txbuf_hw(&mut self) -> PPU_TXBUF_HW_W<6> {
        PPU_TXBUF_HW_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_rxbuf_hw(&mut self) -> PPU_RXBUF_HW_W<7> {
        PPU_RXBUF_HW_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_adpll_sfreg_hw(&mut self) -> PPU_ADPLL_SFREG_HW_W<8> {
        PPU_ADPLL_SFREG_HW_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_fbdv_hw(&mut self) -> PPU_FBDV_HW_W<9> {
        PPU_FBDV_HW_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn pud_vco_hw(&mut self) -> PUD_VCO_HW_W<10> {
        PUD_VCO_HW_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_vco_hw(&mut self) -> PPU_VCO_HW_W<11> {
        PPU_VCO_HW_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_vco_ldo_hw(&mut self) -> PPU_VCO_LDO_HW_W<12> {
        PPU_VCO_LDO_HW_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_lodist_body_bias_hw(&mut self) -> PPU_LODIST_BODY_BIAS_HW_W<13> {
        PPU_LODIST_BODY_BIAS_HW_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_rbb_hw(&mut self) -> PPU_RBB_HW_W<15> {
        PPU_RBB_HW_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_lna_hw(&mut self) -> PPU_LNA_HW_W<16> {
        PPU_LNA_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "non_reg_readback.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [non_reg_readback](index.html) module"]
pub struct NON_REG_READBACK_SPEC;
impl crate::RegisterSpec for NON_REG_READBACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [non_reg_readback::R](R) reader structure"]
impl crate::Readable for NON_REG_READBACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [non_reg_readback::W](W) writer structure"]
impl crate::Writable for NON_REG_READBACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets non_reg_readback to value 0"]
impl crate::Resettable for NON_REG_READBACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
