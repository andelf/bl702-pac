#[doc = "Register `dvp_status_and_error` reader"]
pub struct R(crate::R<DVP_STATUS_AND_ERROR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_STATUS_AND_ERROR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_STATUS_AND_ERROR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_STATUS_AND_ERROR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dvp_status_and_error` writer"]
pub struct W(crate::W<DVP_STATUS_AND_ERROR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP_STATUS_AND_ERROR_SPEC>;
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
impl From<crate::W<DVP_STATUS_AND_ERROR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP_STATUS_AND_ERROR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_normal_int_0` reader - "]
pub type STS_NORMAL_INT_0_R = crate::BitReader<bool>;
#[doc = "Field `sts_normal_int_0` writer - "]
pub type STS_NORMAL_INT_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `sts_normal_int_1` reader - "]
pub type STS_NORMAL_INT_1_R = crate::BitReader<bool>;
#[doc = "Field `sts_normal_int_1` writer - "]
pub type STS_NORMAL_INT_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `sts_mem_int_0` reader - "]
pub type STS_MEM_INT_0_R = crate::BitReader<bool>;
#[doc = "Field `sts_mem_int_0` writer - "]
pub type STS_MEM_INT_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `sts_mem_int_1` reader - "]
pub type STS_MEM_INT_1_R = crate::BitReader<bool>;
#[doc = "Field `sts_mem_int_1` writer - "]
pub type STS_MEM_INT_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `sts_frame_int_0` reader - "]
pub type STS_FRAME_INT_0_R = crate::BitReader<bool>;
#[doc = "Field `sts_frame_int_0` writer - "]
pub type STS_FRAME_INT_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `sts_frame_int_1` reader - "]
pub type STS_FRAME_INT_1_R = crate::BitReader<bool>;
#[doc = "Field `sts_frame_int_1` writer - "]
pub type STS_FRAME_INT_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `sts_fifo_int_0` reader - "]
pub type STS_FIFO_INT_0_R = crate::BitReader<bool>;
#[doc = "Field `sts_fifo_int_0` writer - "]
pub type STS_FIFO_INT_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `sts_fifo_int_1` reader - "]
pub type STS_FIFO_INT_1_R = crate::BitReader<bool>;
#[doc = "Field `sts_fifo_int_1` writer - "]
pub type STS_FIFO_INT_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `sts_hcnt_int` reader - "]
pub type STS_HCNT_INT_R = crate::BitReader<bool>;
#[doc = "Field `sts_hcnt_int` writer - "]
pub type STS_HCNT_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `sts_vcnt_int` reader - "]
pub type STS_VCNT_INT_R = crate::BitReader<bool>;
#[doc = "Field `sts_vcnt_int` writer - "]
pub type STS_VCNT_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `ahb_idle_0` reader - "]
pub type AHB_IDLE_0_R = crate::BitReader<bool>;
#[doc = "Field `ahb_idle_0` writer - "]
pub type AHB_IDLE_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `ahb_idle_1` reader - "]
pub type AHB_IDLE_1_R = crate::BitReader<bool>;
#[doc = "Field `ahb_idle_1` writer - "]
pub type AHB_IDLE_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `st_dvp_idle` reader - "]
pub type ST_DVP_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `st_dvp_idle` writer - "]
pub type ST_DVP_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `frame_valid_cnt_0` reader - "]
pub type FRAME_VALID_CNT_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `frame_valid_cnt_0` writer - "]
pub type FRAME_VALID_CNT_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, u8, u8, 4, O>;
#[doc = "Field `frame_valid_cnt_1` reader - "]
pub type FRAME_VALID_CNT_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `frame_valid_cnt_1` writer - "]
pub type FRAME_VALID_CNT_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, u8, u8, 4, O>;
#[doc = "Field `st_bus_idle` reader - "]
pub type ST_BUS_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `st_bus_idle` writer - "]
pub type ST_BUS_IDLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `st_bus_func` reader - "]
pub type ST_BUS_FUNC_R = crate::BitReader<bool>;
#[doc = "Field `st_bus_func` writer - "]
pub type ST_BUS_FUNC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `st_bus_wait` reader - "]
pub type ST_BUS_WAIT_R = crate::BitReader<bool>;
#[doc = "Field `st_bus_wait` writer - "]
pub type ST_BUS_WAIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
#[doc = "Field `st_bus_flsh` reader - "]
pub type ST_BUS_FLSH_R = crate::BitReader<bool>;
#[doc = "Field `st_bus_flsh` writer - "]
pub type ST_BUS_FLSH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DVP_STATUS_AND_ERROR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_normal_int_0(&self) -> STS_NORMAL_INT_0_R {
        STS_NORMAL_INT_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_normal_int_1(&self) -> STS_NORMAL_INT_1_R {
        STS_NORMAL_INT_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sts_mem_int_0(&self) -> STS_MEM_INT_0_R {
        STS_MEM_INT_0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sts_mem_int_1(&self) -> STS_MEM_INT_1_R {
        STS_MEM_INT_1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sts_frame_int_0(&self) -> STS_FRAME_INT_0_R {
        STS_FRAME_INT_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sts_frame_int_1(&self) -> STS_FRAME_INT_1_R {
        STS_FRAME_INT_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sts_fifo_int_0(&self) -> STS_FIFO_INT_0_R {
        STS_FIFO_INT_0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sts_fifo_int_1(&self) -> STS_FIFO_INT_1_R {
        STS_FIFO_INT_1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sts_hcnt_int(&self) -> STS_HCNT_INT_R {
        STS_HCNT_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sts_vcnt_int(&self) -> STS_VCNT_INT_R {
        STS_VCNT_INT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ahb_idle_0(&self) -> AHB_IDLE_0_R {
        AHB_IDLE_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ahb_idle_1(&self) -> AHB_IDLE_1_R {
        AHB_IDLE_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn st_dvp_idle(&self) -> ST_DVP_IDLE_R {
        ST_DVP_IDLE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn frame_valid_cnt_0(&self) -> FRAME_VALID_CNT_0_R {
        FRAME_VALID_CNT_0_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn frame_valid_cnt_1(&self) -> FRAME_VALID_CNT_1_R {
        FRAME_VALID_CNT_1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn st_bus_idle(&self) -> ST_BUS_IDLE_R {
        ST_BUS_IDLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn st_bus_func(&self) -> ST_BUS_FUNC_R {
        ST_BUS_FUNC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn st_bus_wait(&self) -> ST_BUS_WAIT_R {
        ST_BUS_WAIT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn st_bus_flsh(&self) -> ST_BUS_FLSH_R {
        ST_BUS_FLSH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_normal_int_0(&mut self) -> STS_NORMAL_INT_0_W<0> {
        STS_NORMAL_INT_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_normal_int_1(&mut self) -> STS_NORMAL_INT_1_W<1> {
        STS_NORMAL_INT_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sts_mem_int_0(&mut self) -> STS_MEM_INT_0_W<2> {
        STS_MEM_INT_0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sts_mem_int_1(&mut self) -> STS_MEM_INT_1_W<3> {
        STS_MEM_INT_1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sts_frame_int_0(&mut self) -> STS_FRAME_INT_0_W<4> {
        STS_FRAME_INT_0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sts_frame_int_1(&mut self) -> STS_FRAME_INT_1_W<5> {
        STS_FRAME_INT_1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sts_fifo_int_0(&mut self) -> STS_FIFO_INT_0_W<6> {
        STS_FIFO_INT_0_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sts_fifo_int_1(&mut self) -> STS_FIFO_INT_1_W<7> {
        STS_FIFO_INT_1_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sts_hcnt_int(&mut self) -> STS_HCNT_INT_W<8> {
        STS_HCNT_INT_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sts_vcnt_int(&mut self) -> STS_VCNT_INT_W<9> {
        STS_VCNT_INT_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ahb_idle_0(&mut self) -> AHB_IDLE_0_W<16> {
        AHB_IDLE_0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ahb_idle_1(&mut self) -> AHB_IDLE_1_W<17> {
        AHB_IDLE_1_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn st_dvp_idle(&mut self) -> ST_DVP_IDLE_W<19> {
        ST_DVP_IDLE_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn frame_valid_cnt_0(&mut self) -> FRAME_VALID_CNT_0_W<20> {
        FRAME_VALID_CNT_0_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn frame_valid_cnt_1(&mut self) -> FRAME_VALID_CNT_1_W<24> {
        FRAME_VALID_CNT_1_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn st_bus_idle(&mut self) -> ST_BUS_IDLE_W<28> {
        ST_BUS_IDLE_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn st_bus_func(&mut self) -> ST_BUS_FUNC_W<29> {
        ST_BUS_FUNC_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn st_bus_wait(&mut self) -> ST_BUS_WAIT_W<30> {
        ST_BUS_WAIT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn st_bus_flsh(&mut self) -> ST_BUS_FLSH_W<31> {
        ST_BUS_FLSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dvp_status_and_error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_status_and_error](index.html) module"]
pub struct DVP_STATUS_AND_ERROR_SPEC;
impl crate::RegisterSpec for DVP_STATUS_AND_ERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp_status_and_error::R](R) reader structure"]
impl crate::Readable for DVP_STATUS_AND_ERROR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp_status_and_error::W](W) writer structure"]
impl crate::Writable for DVP_STATUS_AND_ERROR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dvp_status_and_error to value 0"]
impl crate::Resettable for DVP_STATUS_AND_ERROR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
