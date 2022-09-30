#[doc = "Register `fcal` reader"]
pub struct R(crate::R<FCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `fcal` writer"]
pub struct W(crate::W<FCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCAL_SPEC>;
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
impl From<crate::W<FCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `fcal_div_ratio_adj_en` reader - "]
pub type FCAL_DIV_RATIO_ADJ_EN_R = crate::BitReader<bool>;
#[doc = "Field `fcal_div_ratio_adj_en` writer - "]
pub type FCAL_DIV_RATIO_ADJ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCAL_SPEC, bool, O>;
#[doc = "Field `fcal_coarse_pha_threshold` reader - "]
pub type FCAL_COARSE_PHA_THRESHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fcal_coarse_pha_threshold` writer - "]
pub type FCAL_COARSE_PHA_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCAL_SPEC, u8, u8, 2, O>;
#[doc = "Field `fcal_mom_toggle_cnt` reader - "]
pub type FCAL_MOM_TOGGLE_CNT_R = crate::BitReader<bool>;
#[doc = "Field `fcal_mom_toggle_cnt` writer - "]
pub type FCAL_MOM_TOGGLE_CNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCAL_SPEC, bool, O>;
#[doc = "Field `fcal_clk_period` reader - "]
pub type FCAL_CLK_PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fcal_clk_period` writer - "]
pub type FCAL_CLK_PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCAL_SPEC, u8, u8, 2, O>;
#[doc = "Field `fcal_mode` reader - "]
pub type FCAL_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fcal_mode` writer - "]
pub type FCAL_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCAL_SPEC, u8, u8, 2, O>;
#[doc = "Field `fcal_mom_ini_ext` reader - "]
pub type FCAL_MOM_INI_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `fcal_mom_ini_ext` writer - "]
pub type FCAL_MOM_INI_EXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FCAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fcal_div_ratio_adj_en(&self) -> FCAL_DIV_RATIO_ADJ_EN_R {
        FCAL_DIV_RATIO_ADJ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fcal_coarse_pha_threshold(&self) -> FCAL_COARSE_PHA_THRESHOLD_R {
        FCAL_COARSE_PHA_THRESHOLD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_mom_toggle_cnt(&self) -> FCAL_MOM_TOGGLE_CNT_R {
        FCAL_MOM_TOGGLE_CNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_clk_period(&self) -> FCAL_CLK_PERIOD_R {
        FCAL_CLK_PERIOD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn fcal_mode(&self) -> FCAL_MODE_R {
        FCAL_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn fcal_mom_ini_ext(&self) -> FCAL_MOM_INI_EXT_R {
        FCAL_MOM_INI_EXT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fcal_div_ratio_adj_en(&mut self) -> FCAL_DIV_RATIO_ADJ_EN_W<0> {
        FCAL_DIV_RATIO_ADJ_EN_W::new(self)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn fcal_coarse_pha_threshold(&mut self) -> FCAL_COARSE_PHA_THRESHOLD_W<1> {
        FCAL_COARSE_PHA_THRESHOLD_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fcal_mom_toggle_cnt(&mut self) -> FCAL_MOM_TOGGLE_CNT_W<3> {
        FCAL_MOM_TOGGLE_CNT_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fcal_clk_period(&mut self) -> FCAL_CLK_PERIOD_W<4> {
        FCAL_CLK_PERIOD_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn fcal_mode(&mut self) -> FCAL_MODE_W<6> {
        FCAL_MODE_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn fcal_mom_ini_ext(&mut self) -> FCAL_MOM_INI_EXT_W<16> {
        FCAL_MOM_INI_EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "fcal.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcal](index.html) module"]
pub struct FCAL_SPEC;
impl crate::RegisterSpec for FCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcal::R](R) reader structure"]
impl crate::Readable for FCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcal::W](W) writer structure"]
impl crate::Writable for FCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets fcal to value 0"]
impl crate::Resettable for FCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
