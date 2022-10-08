#[doc = "Register `adpll_lf_reg` reader"]
pub struct R(crate::R<ADPLL_LF_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_LF_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_LF_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_LF_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_lf_reg` writer"]
pub struct W(crate::W<ADPLL_LF_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_LF_REG_SPEC>;
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
impl From<crate::W<ADPLL_LF_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_LF_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_lf_vctrl_range_ext` reader - "]
pub type ADPLL_LF_VCTRL_RANGE_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_vctrl_range_ext` writer - "]
pub type ADPLL_LF_VCTRL_RANGE_EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_REG_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_lf_lsb_ext` reader - "]
pub type ADPLL_LF_LSB_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_lsb_ext` writer - "]
pub type ADPLL_LF_LSB_EXT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_REG_SPEC, u8, u8, 7, O>;
#[doc = "Field `adpll_lf_avg_en` reader - "]
pub type ADPLL_LF_AVG_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lf_avg_en` writer - "]
pub type ADPLL_LF_AVG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_LF_REG_SPEC, bool, O>;
#[doc = "Field `adpll_lf_f_p3` reader - "]
pub type ADPLL_LF_F_P3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_f_p3` writer - "]
pub type ADPLL_LF_F_P3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_REG_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_lf_beta_fast` reader - "]
pub type ADPLL_LF_BETA_FAST_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lf_beta_fast` writer - "]
pub type ADPLL_LF_BETA_FAST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LF_REG_SPEC, bool, O>;
#[doc = "Field `adpll_lf_beta_exp` reader - "]
pub type ADPLL_LF_BETA_EXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_beta_exp` writer - "]
pub type ADPLL_LF_BETA_EXP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_REG_SPEC, u8, u8, 3, O>;
#[doc = "Field `adpll_lf_beta_base` reader - "]
pub type ADPLL_LF_BETA_BASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_beta_base` writer - "]
pub type ADPLL_LF_BETA_BASE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_REG_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_lf_alpha_fast` reader - "]
pub type ADPLL_LF_ALPHA_FAST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_alpha_fast` writer - "]
pub type ADPLL_LF_ALPHA_FAST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_REG_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_lf_alpha_exp` reader - "]
pub type ADPLL_LF_ALPHA_EXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_alpha_exp` writer - "]
pub type ADPLL_LF_ALPHA_EXP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_REG_SPEC, u8, u8, 3, O>;
#[doc = "Field `adpll_lf_alpha_base` reader - "]
pub type ADPLL_LF_ALPHA_BASE_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lf_alpha_base` writer - "]
pub type ADPLL_LF_ALPHA_BASE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LF_REG_SPEC, bool, O>;
#[doc = "Field `adpll_lf_ctrl_hw` reader - "]
pub type ADPLL_LF_CTRL_HW_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lf_ctrl_hw` writer - "]
pub type ADPLL_LF_CTRL_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LF_REG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adpll_lf_vctrl_range_ext(&self) -> ADPLL_LF_VCTRL_RANGE_EXT_R {
        ADPLL_LF_VCTRL_RANGE_EXT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:8"]
    #[inline(always)]
    pub fn adpll_lf_lsb_ext(&self) -> ADPLL_LF_LSB_EXT_R {
        ADPLL_LF_LSB_EXT_R::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adpll_lf_avg_en(&self) -> ADPLL_LF_AVG_EN_R {
        ADPLL_LF_AVG_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adpll_lf_f_p3(&self) -> ADPLL_LF_F_P3_R {
        ADPLL_LF_F_P3_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_lf_beta_fast(&self) -> ADPLL_LF_BETA_FAST_R {
        ADPLL_LF_BETA_FAST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn adpll_lf_beta_exp(&self) -> ADPLL_LF_BETA_EXP_R {
        ADPLL_LF_BETA_EXP_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn adpll_lf_beta_base(&self) -> ADPLL_LF_BETA_BASE_R {
        ADPLL_LF_BETA_BASE_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_lf_alpha_fast(&self) -> ADPLL_LF_ALPHA_FAST_R {
        ADPLL_LF_ALPHA_FAST_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adpll_lf_alpha_exp(&self) -> ADPLL_LF_ALPHA_EXP_R {
        ADPLL_LF_ALPHA_EXP_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_lf_alpha_base(&self) -> ADPLL_LF_ALPHA_BASE_R {
        ADPLL_LF_ALPHA_BASE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adpll_lf_ctrl_hw(&self) -> ADPLL_LF_CTRL_HW_R {
        ADPLL_LF_CTRL_HW_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adpll_lf_vctrl_range_ext(&mut self) -> ADPLL_LF_VCTRL_RANGE_EXT_W<0> {
        ADPLL_LF_VCTRL_RANGE_EXT_W::new(self)
    }
    #[doc = "Bits 2:8"]
    #[inline(always)]
    pub fn adpll_lf_lsb_ext(&mut self) -> ADPLL_LF_LSB_EXT_W<2> {
        ADPLL_LF_LSB_EXT_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn adpll_lf_avg_en(&mut self) -> ADPLL_LF_AVG_EN_W<9> {
        ADPLL_LF_AVG_EN_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adpll_lf_f_p3(&mut self) -> ADPLL_LF_F_P3_W<10> {
        ADPLL_LF_F_P3_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_lf_beta_fast(&mut self) -> ADPLL_LF_BETA_FAST_W<13> {
        ADPLL_LF_BETA_FAST_W::new(self)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn adpll_lf_beta_exp(&mut self) -> ADPLL_LF_BETA_EXP_W<14> {
        ADPLL_LF_BETA_EXP_W::new(self)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn adpll_lf_beta_base(&mut self) -> ADPLL_LF_BETA_BASE_W<17> {
        ADPLL_LF_BETA_BASE_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_lf_alpha_fast(&mut self) -> ADPLL_LF_ALPHA_FAST_W<20> {
        ADPLL_LF_ALPHA_FAST_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adpll_lf_alpha_exp(&mut self) -> ADPLL_LF_ALPHA_EXP_W<24> {
        ADPLL_LF_ALPHA_EXP_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_lf_alpha_base(&mut self) -> ADPLL_LF_ALPHA_BASE_W<27> {
        ADPLL_LF_ALPHA_BASE_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adpll_lf_ctrl_hw(&mut self) -> ADPLL_LF_CTRL_HW_W<28> {
        ADPLL_LF_CTRL_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_lf_reg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_lf_reg](index.html) module"]
pub struct ADPLL_LF_REG_SPEC;
impl crate::RegisterSpec for ADPLL_LF_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_lf_reg::R](R) reader structure"]
impl crate::Readable for ADPLL_LF_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_lf_reg::W](W) writer structure"]
impl crate::Writable for ADPLL_LF_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_lf_reg to value 0"]
impl crate::Resettable for ADPLL_LF_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
