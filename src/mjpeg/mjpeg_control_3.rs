#[doc = "Register `mjpeg_control_3` reader"]
pub struct R(crate::R<MJPEG_CONTROL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_CONTROL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_CONTROL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_CONTROL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_control_3` writer"]
pub struct W(crate::W<MJPEG_CONTROL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_CONTROL_3_SPEC>;
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
impl From<crate::W<MJPEG_CONTROL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_CONTROL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_int_normal_en` reader - "]
pub type REG_INT_NORMAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_normal_en` writer - "]
pub type REG_INT_NORMAL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `reg_int_cam_en` reader - "]
pub type REG_INT_CAM_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_cam_en` writer - "]
pub type REG_INT_CAM_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `reg_int_mem_en` reader - "]
pub type REG_INT_MEM_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_mem_en` writer - "]
pub type REG_INT_MEM_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `reg_int_frame_en` reader - "]
pub type REG_INT_FRAME_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_frame_en` writer - "]
pub type REG_INT_FRAME_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `sts_normal_int` reader - "]
pub type STS_NORMAL_INT_R = crate::BitReader<bool>;
#[doc = "Field `sts_normal_int` writer - "]
pub type STS_NORMAL_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `sts_cam_int` reader - "]
pub type STS_CAM_INT_R = crate::BitReader<bool>;
#[doc = "Field `sts_cam_int` writer - "]
pub type STS_CAM_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `sts_mem_int` reader - "]
pub type STS_MEM_INT_R = crate::BitReader<bool>;
#[doc = "Field `sts_mem_int` writer - "]
pub type STS_MEM_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `sts_frame_int` reader - "]
pub type STS_FRAME_INT_R = crate::BitReader<bool>;
#[doc = "Field `sts_frame_int` writer - "]
pub type STS_FRAME_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `mjpeg_idle` reader - "]
pub type MJPEG_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `mjpeg_idle` writer - "]
pub type MJPEG_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `mjpeg_func` reader - "]
pub type MJPEG_FUNC_R = crate::BitReader<bool>;
#[doc = "Field `mjpeg_func` writer - "]
pub type MJPEG_FUNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `mjpeg_wait` reader - "]
pub type MJPEG_WAIT_R = crate::BitReader<bool>;
#[doc = "Field `mjpeg_wait` writer - "]
pub type MJPEG_WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `mjpeg_flsh` reader - "]
pub type MJPEG_FLSH_R = crate::BitReader<bool>;
#[doc = "Field `mjpeg_flsh` writer - "]
pub type MJPEG_FLSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `mjpeg_mans` reader - "]
pub type MJPEG_MANS_R = crate::BitReader<bool>;
#[doc = "Field `mjpeg_mans` writer - "]
pub type MJPEG_MANS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `mjpeg_manf` reader - "]
pub type MJPEG_MANF_R = crate::BitReader<bool>;
#[doc = "Field `mjpeg_manf` writer - "]
pub type MJPEG_MANF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `ahb_idle` reader - "]
pub type AHB_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `ahb_idle` writer - "]
pub type AHB_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `reg_frame_cnt_trgr_int` reader - "]
pub type REG_FRAME_CNT_TRGR_INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_frame_cnt_trgr_int` writer - "]
pub type REG_FRAME_CNT_TRGR_INT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_3_SPEC, u8, u8, 5, O>;
#[doc = "Field `reg_int_idle_en` reader - "]
pub type REG_INT_IDLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_idle_en` writer - "]
pub type REG_INT_IDLE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `sts_idle_int` reader - "]
pub type STS_IDLE_INT_R = crate::BitReader<bool>;
#[doc = "Field `sts_idle_int` writer - "]
pub type STS_IDLE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `frame_valid_cnt` reader - "]
pub type FRAME_VALID_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `frame_valid_cnt` writer - "]
pub type FRAME_VALID_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_CONTROL_3_SPEC, u8, u8, 5, O>;
#[doc = "Field `reg_int_swap_en` reader - "]
pub type REG_INT_SWAP_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_int_swap_en` writer - "]
pub type REG_INT_SWAP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
#[doc = "Field `sts_swap_int` reader - "]
pub type STS_SWAP_INT_R = crate::BitReader<bool>;
#[doc = "Field `sts_swap_int` writer - "]
pub type STS_SWAP_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_CONTROL_3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_int_normal_en(&self) -> REG_INT_NORMAL_EN_R {
        REG_INT_NORMAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_int_cam_en(&self) -> REG_INT_CAM_EN_R {
        REG_INT_CAM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_int_mem_en(&self) -> REG_INT_MEM_EN_R {
        REG_INT_MEM_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_int_frame_en(&self) -> REG_INT_FRAME_EN_R {
        REG_INT_FRAME_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sts_normal_int(&self) -> STS_NORMAL_INT_R {
        STS_NORMAL_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sts_cam_int(&self) -> STS_CAM_INT_R {
        STS_CAM_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sts_mem_int(&self) -> STS_MEM_INT_R {
        STS_MEM_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sts_frame_int(&self) -> STS_FRAME_INT_R {
        STS_FRAME_INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mjpeg_idle(&self) -> MJPEG_IDLE_R {
        MJPEG_IDLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn mjpeg_func(&self) -> MJPEG_FUNC_R {
        MJPEG_FUNC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mjpeg_wait(&self) -> MJPEG_WAIT_R {
        MJPEG_WAIT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mjpeg_flsh(&self) -> MJPEG_FLSH_R {
        MJPEG_FLSH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mjpeg_mans(&self) -> MJPEG_MANS_R {
        MJPEG_MANS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn mjpeg_manf(&self) -> MJPEG_MANF_R {
        MJPEG_MANF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ahb_idle(&self) -> AHB_IDLE_R {
        AHB_IDLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&self) -> REG_FRAME_CNT_TRGR_INT_R {
        REG_FRAME_CNT_TRGR_INT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_int_idle_en(&self) -> REG_INT_IDLE_EN_R {
        REG_INT_IDLE_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sts_idle_int(&self) -> STS_IDLE_INT_R {
        STS_IDLE_INT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn frame_valid_cnt(&self) -> FRAME_VALID_CNT_R {
        FRAME_VALID_CNT_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_int_swap_en(&self) -> REG_INT_SWAP_EN_R {
        REG_INT_SWAP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sts_swap_int(&self) -> STS_SWAP_INT_R {
        STS_SWAP_INT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_int_normal_en(&mut self) -> REG_INT_NORMAL_EN_W<0> {
        REG_INT_NORMAL_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_int_cam_en(&mut self) -> REG_INT_CAM_EN_W<1> {
        REG_INT_CAM_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_int_mem_en(&mut self) -> REG_INT_MEM_EN_W<2> {
        REG_INT_MEM_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_int_frame_en(&mut self) -> REG_INT_FRAME_EN_W<3> {
        REG_INT_FRAME_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sts_normal_int(&mut self) -> STS_NORMAL_INT_W<4> {
        STS_NORMAL_INT_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sts_cam_int(&mut self) -> STS_CAM_INT_W<5> {
        STS_CAM_INT_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sts_mem_int(&mut self) -> STS_MEM_INT_W<6> {
        STS_MEM_INT_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sts_frame_int(&mut self) -> STS_FRAME_INT_W<7> {
        STS_FRAME_INT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn mjpeg_idle(&mut self) -> MJPEG_IDLE_W<8> {
        MJPEG_IDLE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn mjpeg_func(&mut self) -> MJPEG_FUNC_W<9> {
        MJPEG_FUNC_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn mjpeg_wait(&mut self) -> MJPEG_WAIT_W<10> {
        MJPEG_WAIT_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn mjpeg_flsh(&mut self) -> MJPEG_FLSH_W<11> {
        MJPEG_FLSH_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mjpeg_mans(&mut self) -> MJPEG_MANS_W<12> {
        MJPEG_MANS_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn mjpeg_manf(&mut self) -> MJPEG_MANF_W<13> {
        MJPEG_MANF_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ahb_idle(&mut self) -> AHB_IDLE_W<14> {
        AHB_IDLE_W::new(self)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn reg_frame_cnt_trgr_int(&mut self) -> REG_FRAME_CNT_TRGR_INT_W<16> {
        REG_FRAME_CNT_TRGR_INT_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_int_idle_en(&mut self) -> REG_INT_IDLE_EN_W<21> {
        REG_INT_IDLE_EN_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sts_idle_int(&mut self) -> STS_IDLE_INT_W<22> {
        STS_IDLE_INT_W::new(self)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn frame_valid_cnt(&mut self) -> FRAME_VALID_CNT_W<24> {
        FRAME_VALID_CNT_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_int_swap_en(&mut self) -> REG_INT_SWAP_EN_W<29> {
        REG_INT_SWAP_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sts_swap_int(&mut self) -> STS_SWAP_INT_W<30> {
        STS_SWAP_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_control_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_control_3](index.html) module"]
pub struct MJPEG_CONTROL_3_SPEC;
impl crate::RegisterSpec for MJPEG_CONTROL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_control_3::R](R) reader structure"]
impl crate::Readable for MJPEG_CONTROL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_control_3::W](W) writer structure"]
impl crate::Writable for MJPEG_CONTROL_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_control_3 to value 0"]
impl crate::Resettable for MJPEG_CONTROL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
