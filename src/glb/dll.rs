#[doc = "Register `dll` reader"]
pub struct R(crate::R<DLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dll` writer"]
pub struct W(crate::W<DLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLL_SPEC>;
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
impl From<crate::W<DLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dtest_en_dll_refclk` reader - "]
pub type DTEST_EN_DLL_REFCLK_R = crate::BitReader<bool>;
#[doc = "Field `dtest_en_dll_refclk` writer - "]
pub type DTEST_EN_DLL_REFCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dtest_en_dll_outclk` reader - "]
pub type DTEST_EN_DLL_OUTCLK_R = crate::BitReader<bool>;
#[doc = "Field `dtest_en_dll_outclk` writer - "]
pub type DTEST_EN_DLL_OUTCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `ten_dll` reader - "]
pub type TEN_DLL_R = crate::BitReader<bool>;
#[doc = "Field `ten_dll` writer - "]
pub type TEN_DLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_clk_mmdiv_en` reader - "]
pub type DLL_CLK_MMDIV_EN_R = crate::BitReader<bool>;
#[doc = "Field `dll_clk_mmdiv_en` writer - "]
pub type DLL_CLK_MMDIV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_clk_288M_en` reader - "]
pub type DLL_CLK_288M_EN_R = crate::BitReader<bool>;
#[doc = "Field `dll_clk_288M_en` writer - "]
pub type DLL_CLK_288M_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_clk_144M_en` reader - "]
pub type DLL_CLK_144M_EN_R = crate::BitReader<bool>;
#[doc = "Field `dll_clk_144M_en` writer - "]
pub type DLL_CLK_144M_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_clk_96M_en` reader - "]
pub type DLL_CLK_96M_EN_R = crate::BitReader<bool>;
#[doc = "Field `dll_clk_96M_en` writer - "]
pub type DLL_CLK_96M_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_clk_57p6M_en` reader - "]
pub type DLL_CLK_57P6M_EN_R = crate::BitReader<bool>;
#[doc = "Field `dll_clk_57p6M_en` writer - "]
pub type DLL_CLK_57P6M_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_vctrl_sel` reader - "]
pub type DLL_VCTRL_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dll_vctrl_sel` writer - "]
pub type DLL_VCTRL_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLL_SPEC, u8, u8, 3, O>;
#[doc = "Field `dll_prechg_sel` reader - "]
pub type DLL_PRECHG_SEL_R = crate::BitReader<bool>;
#[doc = "Field `dll_prechg_sel` writer - "]
pub type DLL_PRECHG_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_prechg_reg` reader - "]
pub type DLL_PRECHG_REG_R = crate::BitReader<bool>;
#[doc = "Field `dll_prechg_reg` writer - "]
pub type DLL_PRECHG_REG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_prechg_en` reader - "]
pub type DLL_PRECHG_EN_R = crate::BitReader<bool>;
#[doc = "Field `dll_prechg_en` writer - "]
pub type DLL_PRECHG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_vctrl_force_en` reader - "]
pub type DLL_VCTRL_FORCE_EN_R = crate::BitReader<bool>;
#[doc = "Field `dll_vctrl_force_en` writer - "]
pub type DLL_VCTRL_FORCE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_post_div` reader - "]
pub type DLL_POST_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dll_post_div` writer - "]
pub type DLL_POST_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLL_SPEC, u8, u8, 4, O>;
#[doc = "Field `dll_delay_sel` reader - "]
pub type DLL_DELAY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dll_delay_sel` writer - "]
pub type DLL_DELAY_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLL_SPEC, u8, u8, 2, O>;
#[doc = "Field `dll_cp_op_en` reader - "]
pub type DLL_CP_OP_EN_R = crate::BitReader<bool>;
#[doc = "Field `dll_cp_op_en` writer - "]
pub type DLL_CP_OP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_cp_hiz` reader - "]
pub type DLL_CP_HIZ_R = crate::BitReader<bool>;
#[doc = "Field `dll_cp_hiz` writer - "]
pub type DLL_CP_HIZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_refclk_sel` reader - "]
pub type DLL_REFCLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `dll_refclk_sel` writer - "]
pub type DLL_REFCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `dll_reset` reader - "]
pub type DLL_RESET_R = crate::BitReader<bool>;
#[doc = "Field `dll_reset` writer - "]
pub type DLL_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `pu_dll` reader - "]
pub type PU_DLL_R = crate::BitReader<bool>;
#[doc = "Field `pu_dll` writer - "]
pub type PU_DLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
#[doc = "Field `ppu_dll` reader - "]
pub type PPU_DLL_R = crate::BitReader<bool>;
#[doc = "Field `ppu_dll` writer - "]
pub type PPU_DLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DLL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dtest_en_dll_refclk(&self) -> DTEST_EN_DLL_REFCLK_R {
        DTEST_EN_DLL_REFCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dtest_en_dll_outclk(&self) -> DTEST_EN_DLL_OUTCLK_R {
        DTEST_EN_DLL_OUTCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_dll(&self) -> TEN_DLL_R {
        TEN_DLL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dll_clk_mmdiv_en(&self) -> DLL_CLK_MMDIV_EN_R {
        DLL_CLK_MMDIV_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dll_clk_288m_en(&self) -> DLL_CLK_288M_EN_R {
        DLL_CLK_288M_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dll_clk_144m_en(&self) -> DLL_CLK_144M_EN_R {
        DLL_CLK_144M_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dll_clk_96m_en(&self) -> DLL_CLK_96M_EN_R {
        DLL_CLK_96M_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dll_clk_57p6m_en(&self) -> DLL_CLK_57P6M_EN_R {
        DLL_CLK_57P6M_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dll_vctrl_sel(&self) -> DLL_VCTRL_SEL_R {
        DLL_VCTRL_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dll_prechg_sel(&self) -> DLL_PRECHG_SEL_R {
        DLL_PRECHG_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dll_prechg_reg(&self) -> DLL_PRECHG_REG_R {
        DLL_PRECHG_REG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dll_prechg_en(&self) -> DLL_PRECHG_EN_R {
        DLL_PRECHG_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dll_vctrl_force_en(&self) -> DLL_VCTRL_FORCE_EN_R {
        DLL_VCTRL_FORCE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dll_post_div(&self) -> DLL_POST_DIV_R {
        DLL_POST_DIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dll_delay_sel(&self) -> DLL_DELAY_SEL_R {
        DLL_DELAY_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dll_cp_op_en(&self) -> DLL_CP_OP_EN_R {
        DLL_CP_OP_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dll_cp_hiz(&self) -> DLL_CP_HIZ_R {
        DLL_CP_HIZ_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dll_refclk_sel(&self) -> DLL_REFCLK_SEL_R {
        DLL_REFCLK_SEL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dll_reset(&self) -> DLL_RESET_R {
        DLL_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_dll(&self) -> PU_DLL_R {
        PU_DLL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ppu_dll(&self) -> PPU_DLL_R {
        PPU_DLL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dtest_en_dll_refclk(&mut self) -> DTEST_EN_DLL_REFCLK_W<0> {
        DTEST_EN_DLL_REFCLK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dtest_en_dll_outclk(&mut self) -> DTEST_EN_DLL_OUTCLK_W<1> {
        DTEST_EN_DLL_OUTCLK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ten_dll(&mut self) -> TEN_DLL_W<2> {
        TEN_DLL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dll_clk_mmdiv_en(&mut self) -> DLL_CLK_MMDIV_EN_W<3> {
        DLL_CLK_MMDIV_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dll_clk_288m_en(&mut self) -> DLL_CLK_288M_EN_W<4> {
        DLL_CLK_288M_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dll_clk_144m_en(&mut self) -> DLL_CLK_144M_EN_W<5> {
        DLL_CLK_144M_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dll_clk_96m_en(&mut self) -> DLL_CLK_96M_EN_W<6> {
        DLL_CLK_96M_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn dll_clk_57p6m_en(&mut self) -> DLL_CLK_57P6M_EN_W<7> {
        DLL_CLK_57P6M_EN_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn dll_vctrl_sel(&mut self) -> DLL_VCTRL_SEL_W<8> {
        DLL_VCTRL_SEL_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dll_prechg_sel(&mut self) -> DLL_PRECHG_SEL_W<12> {
        DLL_PRECHG_SEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn dll_prechg_reg(&mut self) -> DLL_PRECHG_REG_W<13> {
        DLL_PRECHG_REG_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn dll_prechg_en(&mut self) -> DLL_PRECHG_EN_W<14> {
        DLL_PRECHG_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn dll_vctrl_force_en(&mut self) -> DLL_VCTRL_FORCE_EN_W<15> {
        DLL_VCTRL_FORCE_EN_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn dll_post_div(&mut self) -> DLL_POST_DIV_W<16> {
        DLL_POST_DIV_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn dll_delay_sel(&mut self) -> DLL_DELAY_SEL_W<20> {
        DLL_DELAY_SEL_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn dll_cp_op_en(&mut self) -> DLL_CP_OP_EN_W<22> {
        DLL_CP_OP_EN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn dll_cp_hiz(&mut self) -> DLL_CP_HIZ_W<23> {
        DLL_CP_HIZ_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn dll_refclk_sel(&mut self) -> DLL_REFCLK_SEL_W<28> {
        DLL_REFCLK_SEL_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn dll_reset(&mut self) -> DLL_RESET_W<29> {
        DLL_RESET_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn pu_dll(&mut self) -> PU_DLL_W<30> {
        PU_DLL_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn ppu_dll(&mut self) -> PPU_DLL_W<31> {
        PPU_DLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dll.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dll](index.html) module"]
pub struct DLL_SPEC;
impl crate::RegisterSpec for DLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dll::R](R) reader structure"]
impl crate::Readable for DLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dll::W](W) writer structure"]
impl crate::Writable for DLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets dll to value 0"]
impl crate::Resettable for DLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
