#[doc = "Register `dvp2axi_configue` reader"]
pub struct R(crate::R<DVP2AXI_CONFIGUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP2AXI_CONFIGUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP2AXI_CONFIGUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP2AXI_CONFIGUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dvp2axi_configue` writer"]
pub struct W(crate::W<DVP2AXI_CONFIGUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP2AXI_CONFIGUE_SPEC>;
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
impl From<crate::W<DVP2AXI_CONFIGUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP2AXI_CONFIGUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_dvp_enable` reader - "]
pub type REG_DVP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `reg_dvp_enable` writer - "]
pub type REG_DVP_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_sw_mode` reader - "]
pub type REG_SW_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_sw_mode` writer - "]
pub type REG_SW_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_fram_vld_pol` reader - "]
pub type REG_FRAM_VLD_POL_R = crate::BitReader<bool>;
#[doc = "Field `reg_fram_vld_pol` writer - "]
pub type REG_FRAM_VLD_POL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_line_vld_pol` reader - "]
pub type REG_LINE_VLD_POL_R = crate::BitReader<bool>;
#[doc = "Field `reg_line_vld_pol` writer - "]
pub type REG_LINE_VLD_POL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_hburst` reader - "]
pub type REG_HBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_hburst` writer - "]
pub type REG_HBURST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_dvp_mode` reader - "]
pub type REG_DVP_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_dvp_mode` writer - "]
pub type REG_DVP_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, u8, u8, 3, O>;
#[doc = "Field `reg_hw_mode_fwrap` reader - "]
pub type REG_HW_MODE_FWRAP_R = crate::BitReader<bool>;
#[doc = "Field `reg_hw_mode_fwrap` writer - "]
pub type REG_HW_MODE_FWRAP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_drop_en` reader - "]
pub type REG_DROP_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_drop_en` writer - "]
pub type REG_DROP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_drop_even` reader - "]
pub type REG_DROP_EVEN_R = crate::BitReader<bool>;
#[doc = "Field `reg_drop_even` writer - "]
pub type REG_DROP_EVEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_subsample_en` reader - "]
pub type REG_SUBSAMPLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_subsample_en` writer - "]
pub type REG_SUBSAMPLE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_subsample_even` reader - "]
pub type REG_SUBSAMPLE_EVEN_R = crate::BitReader<bool>;
#[doc = "Field `reg_subsample_even` writer - "]
pub type REG_SUBSAMPLE_EVEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_interlv_mode` reader - "]
pub type REG_INTERLV_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_interlv_mode` writer - "]
pub type REG_INTERLV_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_dvp_pix_clk_cg` reader - "]
pub type REG_DVP_PIX_CLK_CG_R = crate::BitReader<bool>;
#[doc = "Field `reg_dvp_pix_clk_cg` writer - "]
pub type REG_DVP_PIX_CLK_CG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, bool, O>;
#[doc = "Field `reg_dvp_wait_cycle` reader - "]
pub type REG_DVP_WAIT_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_dvp_wait_cycle` writer - "]
pub type REG_DVP_WAIT_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DVP2AXI_CONFIGUE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dvp_enable(&self) -> REG_DVP_ENABLE_R {
        REG_DVP_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_sw_mode(&self) -> REG_SW_MODE_R {
        REG_SW_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_fram_vld_pol(&self) -> REG_FRAM_VLD_POL_R {
        REG_FRAM_VLD_POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_line_vld_pol(&self) -> REG_LINE_VLD_POL_R {
        REG_LINE_VLD_POL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_hburst(&self) -> REG_HBURST_R {
        REG_HBURST_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn reg_dvp_mode(&self) -> REG_DVP_MODE_R {
        REG_DVP_MODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_hw_mode_fwrap(&self) -> REG_HW_MODE_FWRAP_R {
        REG_HW_MODE_FWRAP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_drop_en(&self) -> REG_DROP_EN_R {
        REG_DROP_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_drop_even(&self) -> REG_DROP_EVEN_R {
        REG_DROP_EVEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_subsample_en(&self) -> REG_SUBSAMPLE_EN_R {
        REG_SUBSAMPLE_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_subsample_even(&self) -> REG_SUBSAMPLE_EVEN_R {
        REG_SUBSAMPLE_EVEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_interlv_mode(&self) -> REG_INTERLV_MODE_R {
        REG_INTERLV_MODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_dvp_pix_clk_cg(&self) -> REG_DVP_PIX_CLK_CG_R {
        REG_DVP_PIX_CLK_CG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_dvp_wait_cycle(&self) -> REG_DVP_WAIT_CYCLE_R {
        REG_DVP_WAIT_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dvp_enable(&mut self) -> REG_DVP_ENABLE_W<0> {
        REG_DVP_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_sw_mode(&mut self) -> REG_SW_MODE_W<1> {
        REG_SW_MODE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_fram_vld_pol(&mut self) -> REG_FRAM_VLD_POL_W<2> {
        REG_FRAM_VLD_POL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_line_vld_pol(&mut self) -> REG_LINE_VLD_POL_W<3> {
        REG_LINE_VLD_POL_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn reg_hburst(&mut self) -> REG_HBURST_W<4> {
        REG_HBURST_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn reg_dvp_mode(&mut self) -> REG_DVP_MODE_W<8> {
        REG_DVP_MODE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_hw_mode_fwrap(&mut self) -> REG_HW_MODE_FWRAP_W<11> {
        REG_HW_MODE_FWRAP_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_drop_en(&mut self) -> REG_DROP_EN_W<12> {
        REG_DROP_EN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_drop_even(&mut self) -> REG_DROP_EVEN_W<13> {
        REG_DROP_EVEN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_subsample_en(&mut self) -> REG_SUBSAMPLE_EN_W<14> {
        REG_SUBSAMPLE_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_subsample_even(&mut self) -> REG_SUBSAMPLE_EVEN_W<15> {
        REG_SUBSAMPLE_EVEN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_interlv_mode(&mut self) -> REG_INTERLV_MODE_W<16> {
        REG_INTERLV_MODE_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_dvp_pix_clk_cg(&mut self) -> REG_DVP_PIX_CLK_CG_W<20> {
        REG_DVP_PIX_CLK_CG_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reg_dvp_wait_cycle(&mut self) -> REG_DVP_WAIT_CYCLE_W<24> {
        REG_DVP_WAIT_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dvp2axi_configue.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp2axi_configue](index.html) module"]
pub struct DVP2AXI_CONFIGUE_SPEC;
impl crate::RegisterSpec for DVP2AXI_CONFIGUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp2axi_configue::R](R) reader structure"]
impl crate::Readable for DVP2AXI_CONFIGUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp2axi_configue::W](W) writer structure"]
impl crate::Writable for DVP2AXI_CONFIGUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dvp2axi_configue to value 0"]
impl crate::Resettable for DVP2AXI_CONFIGUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
