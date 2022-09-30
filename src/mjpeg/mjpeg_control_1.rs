#[doc = "Register `mjpeg_control_1` reader"]
pub struct R(crate::R<MJPEG_CONTROL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_CONTROL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_CONTROL_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_CONTROL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_control_1` writer"]
pub struct W(crate::W<MJPEG_CONTROL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_CONTROL_1_SPEC>;
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
impl From<crate::W<MJPEG_CONTROL_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_CONTROL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mjpeg_enable` reader - "]
pub type REG_MJPEG_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `reg_mjpeg_enable` writer - "]
pub type REG_MJPEG_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_1_SPEC, bool, O>;
#[doc = "Field `reg_mjpeg_bit_order` reader - "]
pub type REG_MJPEG_BIT_ORDER_R = crate::BitReader<bool>;
#[doc = "Field `reg_mjpeg_bit_order` writer - "]
pub type REG_MJPEG_BIT_ORDER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_1_SPEC, bool, O>;
#[doc = "Field `reg_order_u_even` reader - "]
pub type REG_ORDER_U_EVEN_R = crate::BitReader<bool>;
#[doc = "Field `reg_order_u_even` writer - "]
pub type REG_ORDER_U_EVEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_1_SPEC, bool, O>;
#[doc = "Field `reg_wr_over_stop` reader - "]
pub type REG_WR_OVER_STOP_R = crate::BitReader<bool>;
#[doc = "Field `reg_wr_over_stop` writer - "]
pub type REG_WR_OVER_STOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_1_SPEC, bool, O>;
#[doc = "Field `reg_last_hf_wblk_dmy` reader - "]
pub type REG_LAST_HF_WBLK_DMY_R = crate::BitReader<bool>;
#[doc = "Field `reg_last_hf_wblk_dmy` writer - "]
pub type REG_LAST_HF_WBLK_DMY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_1_SPEC, bool, O>;
#[doc = "Field `reg_last_hf_hblk_dmy` reader - "]
pub type REG_LAST_HF_HBLK_DMY_R = crate::BitReader<bool>;
#[doc = "Field `reg_last_hf_hblk_dmy` writer - "]
pub type REG_LAST_HF_HBLK_DMY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_1_SPEC, bool, O>;
#[doc = "Field `reg_reflect_dmy` reader - "]
pub type REG_REFLECT_DMY_R = crate::BitReader<bool>;
#[doc = "Field `reg_reflect_dmy` writer - "]
pub type REG_REFLECT_DMY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_1_SPEC, bool, O>;
#[doc = "Field `reg_h_bust` reader - "]
pub type REG_H_BUST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_h_bust` writer - "]
pub type REG_H_BUST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_yuv_mode` reader - "]
pub type REG_YUV_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_yuv_mode` writer - "]
pub type REG_YUV_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_q_mode` reader - "]
pub type REG_Q_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_q_mode` writer - "]
pub type REG_Q_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_1_SPEC, u8, u8, 7, O>;
#[doc = "Field `reg_y0_order` reader - "]
pub type REG_Y0_ORDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_y0_order` writer - "]
pub type REG_Y0_ORDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_u0_order` reader - "]
pub type REG_U0_ORDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_u0_order` writer - "]
pub type REG_U0_ORDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_y1_order` reader - "]
pub type REG_Y1_ORDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_y1_order` writer - "]
pub type REG_Y1_ORDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_1_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_v0_order` reader - "]
pub type REG_V0_ORDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_v0_order` writer - "]
pub type REG_V0_ORDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mjpeg_enable(&self) -> REG_MJPEG_ENABLE_R {
        REG_MJPEG_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mjpeg_bit_order(&self) -> REG_MJPEG_BIT_ORDER_R {
        REG_MJPEG_BIT_ORDER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_order_u_even(&self) -> REG_ORDER_U_EVEN_R {
        REG_ORDER_U_EVEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_wr_over_stop(&self) -> REG_WR_OVER_STOP_R {
        REG_WR_OVER_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_last_hf_wblk_dmy(&self) -> REG_LAST_HF_WBLK_DMY_R {
        REG_LAST_HF_WBLK_DMY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_last_hf_hblk_dmy(&self) -> REG_LAST_HF_HBLK_DMY_R {
        REG_LAST_HF_HBLK_DMY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_reflect_dmy(&self) -> REG_REFLECT_DMY_R {
        REG_REFLECT_DMY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reg_h_bust(&self) -> REG_H_BUST_R {
        REG_H_BUST_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn reg_yuv_mode(&self) -> REG_YUV_MODE_R {
        REG_YUV_MODE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn reg_q_mode(&self) -> REG_Q_MODE_R {
        REG_Q_MODE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn reg_y0_order(&self) -> REG_Y0_ORDER_R {
        REG_Y0_ORDER_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reg_u0_order(&self) -> REG_U0_ORDER_R {
        REG_U0_ORDER_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_y1_order(&self) -> REG_Y1_ORDER_R {
        REG_Y1_ORDER_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_v0_order(&self) -> REG_V0_ORDER_R {
        REG_V0_ORDER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mjpeg_enable(&mut self) -> REG_MJPEG_ENABLE_W<0> {
        REG_MJPEG_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mjpeg_bit_order(&mut self) -> REG_MJPEG_BIT_ORDER_W<1> {
        REG_MJPEG_BIT_ORDER_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_order_u_even(&mut self) -> REG_ORDER_U_EVEN_W<2> {
        REG_ORDER_U_EVEN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_wr_over_stop(&mut self) -> REG_WR_OVER_STOP_W<3> {
        REG_WR_OVER_STOP_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_last_hf_wblk_dmy(&mut self) -> REG_LAST_HF_WBLK_DMY_W<4> {
        REG_LAST_HF_WBLK_DMY_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_last_hf_hblk_dmy(&mut self) -> REG_LAST_HF_HBLK_DMY_W<5> {
        REG_LAST_HF_HBLK_DMY_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_reflect_dmy(&mut self) -> REG_REFLECT_DMY_W<6> {
        REG_REFLECT_DMY_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn reg_h_bust(&mut self) -> REG_H_BUST_W<8> {
        REG_H_BUST_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn reg_yuv_mode(&mut self) -> REG_YUV_MODE_W<12> {
        REG_YUV_MODE_W::new(self)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn reg_q_mode(&mut self) -> REG_Q_MODE_W<16> {
        REG_Q_MODE_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn reg_y0_order(&mut self) -> REG_Y0_ORDER_W<24> {
        REG_Y0_ORDER_W::new(self)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn reg_u0_order(&mut self) -> REG_U0_ORDER_W<26> {
        REG_U0_ORDER_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn reg_y1_order(&mut self) -> REG_Y1_ORDER_W<28> {
        REG_Y1_ORDER_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_v0_order(&mut self) -> REG_V0_ORDER_W<30> {
        REG_V0_ORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_control_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_control_1](index.html) module"]
pub struct MJPEG_CONTROL_1_SPEC;
impl crate::RegisterSpec for MJPEG_CONTROL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_control_1::R](R) reader structure"]
impl crate::Readable for MJPEG_CONTROL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_control_1::W](W) writer structure"]
impl crate::Writable for MJPEG_CONTROL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_control_1 to value 0"]
impl crate::Resettable for MJPEG_CONTROL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
