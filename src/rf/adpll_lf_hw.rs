#[doc = "Register `adpll_lf_hw` reader"]
pub struct R(crate::R<ADPLL_LF_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_LF_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_LF_HW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_LF_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_lf_hw` writer"]
pub struct W(crate::W<ADPLL_LF_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_LF_HW_SPEC>;
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
impl From<crate::W<ADPLL_LF_HW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_LF_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_lf_f_p3_hw` reader - "]
pub type ADPLL_LF_F_P3_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_f_p3_hw` writer - "]
pub type ADPLL_LF_F_P3_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_HW_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_lf_beta_fast_hw` reader - "]
pub type ADPLL_LF_BETA_FAST_HW_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lf_beta_fast_hw` writer - "]
pub type ADPLL_LF_BETA_FAST_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LF_HW_SPEC, bool, O>;
#[doc = "Field `adpll_lf_beta_exp_hw` reader - "]
pub type ADPLL_LF_BETA_EXP_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_beta_exp_hw` writer - "]
pub type ADPLL_LF_BETA_EXP_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_HW_SPEC, u8, u8, 3, O>;
#[doc = "Field `adpll_lf_beta_base_hw` reader - "]
pub type ADPLL_LF_BETA_BASE_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_beta_base_hw` writer - "]
pub type ADPLL_LF_BETA_BASE_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_HW_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_lf_alpha_fast_hw` reader - "]
pub type ADPLL_LF_ALPHA_FAST_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_alpha_fast_hw` writer - "]
pub type ADPLL_LF_ALPHA_FAST_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_HW_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_lf_alpha_exp_hw` reader - "]
pub type ADPLL_LF_ALPHA_EXP_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_lf_alpha_exp_hw` writer - "]
pub type ADPLL_LF_ALPHA_EXP_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_LF_HW_SPEC, u8, u8, 3, O>;
#[doc = "Field `adpll_lf_alpha_base_hw` reader - "]
pub type ADPLL_LF_ALPHA_BASE_HW_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lf_alpha_base_hw` writer - "]
pub type ADPLL_LF_ALPHA_BASE_HW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_LF_HW_SPEC, bool, O>;
impl R {
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn adpll_lf_f_p3_hw(&self) -> ADPLL_LF_F_P3_HW_R {
        ADPLL_LF_F_P3_HW_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adpll_lf_beta_fast_hw(&self) -> ADPLL_LF_BETA_FAST_HW_R {
        ADPLL_LF_BETA_FAST_HW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    pub fn adpll_lf_beta_exp_hw(&self) -> ADPLL_LF_BETA_EXP_HW_R {
        ADPLL_LF_BETA_EXP_HW_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn adpll_lf_beta_base_hw(&self) -> ADPLL_LF_BETA_BASE_HW_R {
        ADPLL_LF_BETA_BASE_HW_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_lf_alpha_fast_hw(&self) -> ADPLL_LF_ALPHA_FAST_HW_R {
        ADPLL_LF_ALPHA_FAST_HW_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn adpll_lf_alpha_exp_hw(&self) -> ADPLL_LF_ALPHA_EXP_HW_R {
        ADPLL_LF_ALPHA_EXP_HW_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn adpll_lf_alpha_base_hw(&self) -> ADPLL_LF_ALPHA_BASE_HW_R {
        ADPLL_LF_ALPHA_BASE_HW_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lf_f_p3_hw(&mut self) -> ADPLL_LF_F_P3_HW_W<10> {
        ADPLL_LF_F_P3_HW_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lf_beta_fast_hw(&mut self) -> ADPLL_LF_BETA_FAST_HW_W<13> {
        ADPLL_LF_BETA_FAST_HW_W::new(self)
    }
    #[doc = "Bits 14:16"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lf_beta_exp_hw(&mut self) -> ADPLL_LF_BETA_EXP_HW_W<14> {
        ADPLL_LF_BETA_EXP_HW_W::new(self)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lf_beta_base_hw(&mut self) -> ADPLL_LF_BETA_BASE_HW_W<17> {
        ADPLL_LF_BETA_BASE_HW_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lf_alpha_fast_hw(&mut self) -> ADPLL_LF_ALPHA_FAST_HW_W<20> {
        ADPLL_LF_ALPHA_FAST_HW_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lf_alpha_exp_hw(&mut self) -> ADPLL_LF_ALPHA_EXP_HW_W<24> {
        ADPLL_LF_ALPHA_EXP_HW_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_lf_alpha_base_hw(&mut self) -> ADPLL_LF_ALPHA_BASE_HW_W<27> {
        ADPLL_LF_ALPHA_BASE_HW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_lf_hw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_lf_hw](index.html) module"]
pub struct ADPLL_LF_HW_SPEC;
impl crate::RegisterSpec for ADPLL_LF_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_lf_hw::R](R) reader structure"]
impl crate::Readable for ADPLL_LF_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_lf_hw::W](W) writer structure"]
impl crate::Writable for ADPLL_LF_HW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets adpll_lf_hw to value 0"]
impl crate::Resettable for ADPLL_LF_HW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
