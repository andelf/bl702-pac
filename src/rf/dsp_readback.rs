#[doc = "Register `dsp_readback` reader"]
pub struct R(crate::R<DSP_READBACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSP_READBACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSP_READBACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSP_READBACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dsp_readback` writer"]
pub struct W(crate::W<DSP_READBACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSP_READBACK_SPEC>;
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
impl From<crate::W<DSP_READBACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSP_READBACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch_ind_hw` reader - "]
pub type CH_IND_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ch_ind_hw` writer - "]
pub type CH_IND_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DSP_READBACK_SPEC, u8, u8, 7, O>;
#[doc = "Field `rbb_ind_hw` reader - "]
pub type RBB_IND_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_ind_hw` writer - "]
pub type RBB_IND_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DSP_READBACK_SPEC, u8, u8, 5, O>;
#[doc = "Field `rbb_bw_ind_hw` reader - "]
pub type RBB_BW_IND_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bw_ind_hw` writer - "]
pub type RBB_BW_IND_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSP_READBACK_SPEC, bool, O>;
#[doc = "Field `ch_ind` reader - "]
pub type CH_IND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ch_ind` writer - "]
pub type CH_IND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSP_READBACK_SPEC, u8, u8, 7, O>;
#[doc = "Field `ch_ind_ctrl_hw` reader - "]
pub type CH_IND_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `ch_ind_ctrl_hw` writer - "]
pub type CH_IND_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSP_READBACK_SPEC, bool, O>;
#[doc = "Field `rbb_ind` reader - "]
pub type RBB_IND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_ind` writer - "]
pub type RBB_IND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSP_READBACK_SPEC, u8, u8, 5, O>;
#[doc = "Field `rbb_ind_ctrl_hw` reader - "]
pub type RBB_IND_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_ind_ctrl_hw` writer - "]
pub type RBB_IND_CTRL_HW_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSP_READBACK_SPEC, bool, O>;
#[doc = "Field `rbb_bw_ind` reader - "]
pub type RBB_BW_IND_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bw_ind` writer - "]
pub type RBB_BW_IND_W<'a, const O: u8> = crate::BitWriter<'a, u32, DSP_READBACK_SPEC, bool, O>;
#[doc = "Field `rbb_bw_ind_ctrl_hw` reader - "]
pub type RBB_BW_IND_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `rbb_bw_ind_ctrl_hw` writer - "]
pub type RBB_BW_IND_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DSP_READBACK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:8"]
    #[inline(always)]
    pub fn ch_ind_hw(&self) -> CH_IND_HW_R {
        CH_IND_HW_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn rbb_ind_hw(&self) -> RBB_IND_HW_R {
        RBB_IND_HW_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rbb_bw_ind_hw(&self) -> RBB_BW_IND_HW_R {
        RBB_BW_IND_HW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn ch_ind(&self) -> CH_IND_R {
        CH_IND_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ch_ind_ctrl_hw(&self) -> CH_IND_CTRL_HW_R {
        CH_IND_CTRL_HW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rbb_ind(&self) -> RBB_IND_R {
        RBB_IND_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rbb_ind_ctrl_hw(&self) -> RBB_IND_CTRL_HW_R {
        RBB_IND_CTRL_HW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rbb_bw_ind(&self) -> RBB_BW_IND_R {
        RBB_BW_IND_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rbb_bw_ind_ctrl_hw(&self) -> RBB_BW_IND_CTRL_HW_R {
        RBB_BW_IND_CTRL_HW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:8"]
    #[inline(always)]
    pub fn ch_ind_hw(&mut self) -> CH_IND_HW_W<2> {
        CH_IND_HW_W::new(self)
    }
    #[doc = "Bits 9:13"]
    #[inline(always)]
    pub fn rbb_ind_hw(&mut self) -> RBB_IND_HW_W<9> {
        RBB_IND_HW_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rbb_bw_ind_hw(&mut self) -> RBB_BW_IND_HW_W<14> {
        RBB_BW_IND_HW_W::new(self)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn ch_ind(&mut self) -> CH_IND_W<16> {
        CH_IND_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ch_ind_ctrl_hw(&mut self) -> CH_IND_CTRL_HW_W<23> {
        CH_IND_CTRL_HW_W::new(self)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rbb_ind(&mut self) -> RBB_IND_W<24> {
        RBB_IND_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rbb_ind_ctrl_hw(&mut self) -> RBB_IND_CTRL_HW_W<29> {
        RBB_IND_CTRL_HW_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rbb_bw_ind(&mut self) -> RBB_BW_IND_W<30> {
        RBB_BW_IND_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rbb_bw_ind_ctrl_hw(&mut self) -> RBB_BW_IND_CTRL_HW_W<31> {
        RBB_BW_IND_CTRL_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dsp_readback.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsp_readback](index.html) module"]
pub struct DSP_READBACK_SPEC;
impl crate::RegisterSpec for DSP_READBACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsp_readback::R](R) reader structure"]
impl crate::Readable for DSP_READBACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsp_readback::W](W) writer structure"]
impl crate::Writable for DSP_READBACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dsp_readback to value 0"]
impl crate::Resettable for DSP_READBACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
