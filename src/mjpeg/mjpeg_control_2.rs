#[doc = "Register `mjpeg_control_2` reader"]
pub struct R(crate::R<MJPEG_CONTROL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_CONTROL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_CONTROL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_CONTROL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_control_2` writer"]
pub struct W(crate::W<MJPEG_CONTROL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_CONTROL_2_SPEC>;
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
impl From<crate::W<MJPEG_CONTROL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_CONTROL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_sw_frame` reader - "]
pub type REG_SW_FRAME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_sw_frame` writer - "]
pub type REG_SW_FRAME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_2_SPEC, u8, u8, 5, O>;
#[doc = "Field `reg_mjpeg_sw_mode` reader - "]
pub type REG_MJPEG_SW_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_mjpeg_sw_mode` writer - "]
pub type REG_MJPEG_SW_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_2_SPEC, bool, O>;
#[doc = "Field `reg_mjpeg_sw_run` reader - "]
pub type REG_MJPEG_SW_RUN_R = crate::BitReader<bool>;
#[doc = "Field `reg_mjpeg_sw_run` writer - "]
pub type REG_MJPEG_SW_RUN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_2_SPEC, bool, O>;
#[doc = "Field `reg_yy_dvp2ahb_lsel` reader - "]
pub type REG_YY_DVP2AHB_LSEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_yy_dvp2ahb_lsel` writer - "]
pub type REG_YY_DVP2AHB_LSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_2_SPEC, bool, O>;
#[doc = "Field `reg_yy_dvp2ahb_fsel` reader - "]
pub type REG_YY_DVP2AHB_FSEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_yy_dvp2ahb_fsel` writer - "]
pub type REG_YY_DVP2AHB_FSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_2_SPEC, bool, O>;
#[doc = "Field `reg_uv_dvp2ahb_lsel` reader - "]
pub type REG_UV_DVP2AHB_LSEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_uv_dvp2ahb_lsel` writer - "]
pub type REG_UV_DVP2AHB_LSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_2_SPEC, bool, O>;
#[doc = "Field `reg_uv_dvp2ahb_fsel` reader - "]
pub type REG_UV_DVP2AHB_FSEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_uv_dvp2ahb_fsel` writer - "]
pub type REG_UV_DVP2AHB_FSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_2_SPEC, bool, O>;
#[doc = "Field `reg_mjpeg_wait_cycle` reader - "]
pub type REG_MJPEG_WAIT_CYCLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_mjpeg_wait_cycle` writer - "]
pub type REG_MJPEG_WAIT_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_sw_frame(&self) -> REG_SW_FRAME_R {
        REG_SW_FRAME_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mjpeg_sw_mode(&self) -> REG_MJPEG_SW_MODE_R {
        REG_MJPEG_SW_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mjpeg_sw_run(&self) -> REG_MJPEG_SW_RUN_R {
        REG_MJPEG_SW_RUN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_yy_dvp2ahb_lsel(&self) -> REG_YY_DVP2AHB_LSEL_R {
        REG_YY_DVP2AHB_LSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_yy_dvp2ahb_fsel(&self) -> REG_YY_DVP2AHB_FSEL_R {
        REG_YY_DVP2AHB_FSEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_uv_dvp2ahb_lsel(&self) -> REG_UV_DVP2AHB_LSEL_R {
        REG_UV_DVP2AHB_LSEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_uv_dvp2ahb_fsel(&self) -> REG_UV_DVP2AHB_FSEL_R {
        REG_UV_DVP2AHB_FSEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_mjpeg_wait_cycle(&self) -> REG_MJPEG_WAIT_CYCLE_R {
        REG_MJPEG_WAIT_CYCLE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn reg_sw_frame(&mut self) -> REG_SW_FRAME_W<0> {
        REG_SW_FRAME_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_mjpeg_sw_mode(&mut self) -> REG_MJPEG_SW_MODE_W<8> {
        REG_MJPEG_SW_MODE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_mjpeg_sw_run(&mut self) -> REG_MJPEG_SW_RUN_W<9> {
        REG_MJPEG_SW_RUN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_yy_dvp2ahb_lsel(&mut self) -> REG_YY_DVP2AHB_LSEL_W<12> {
        REG_YY_DVP2AHB_LSEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_yy_dvp2ahb_fsel(&mut self) -> REG_YY_DVP2AHB_FSEL_W<13> {
        REG_YY_DVP2AHB_FSEL_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_uv_dvp2ahb_lsel(&mut self) -> REG_UV_DVP2AHB_LSEL_W<14> {
        REG_UV_DVP2AHB_LSEL_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_uv_dvp2ahb_fsel(&mut self) -> REG_UV_DVP2AHB_FSEL_W<15> {
        REG_UV_DVP2AHB_FSEL_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_mjpeg_wait_cycle(&mut self) -> REG_MJPEG_WAIT_CYCLE_W<16> {
        REG_MJPEG_WAIT_CYCLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_control_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_control_2](index.html) module"]
pub struct MJPEG_CONTROL_2_SPEC;
impl crate::RegisterSpec for MJPEG_CONTROL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_control_2::R](R) reader structure"]
impl crate::Readable for MJPEG_CONTROL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_control_2::W](W) writer structure"]
impl crate::Writable for MJPEG_CONTROL_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_control_2 to value 0"]
impl crate::Resettable for MJPEG_CONTROL_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
