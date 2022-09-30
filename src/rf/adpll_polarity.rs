#[doc = "Register `adpll_polarity` reader"]
pub struct R(crate::R<ADPLL_POLARITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_POLARITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_POLARITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_POLARITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_polarity` writer"]
pub struct W(crate::W<ADPLL_POLARITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_POLARITY_SPEC>;
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
impl From<crate::W<ADPLL_POLARITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_POLARITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_lp_mom_polarity` reader - "]
pub type ADPLL_LP_MOM_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lp_mom_polarity` writer - "]
pub type ADPLL_LP_MOM_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_POLARITY_SPEC, bool, O>;
#[doc = "Field `adpll_lms_polarity` reader - "]
pub type ADPLL_LMS_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lms_polarity` writer - "]
pub type ADPLL_LMS_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_POLARITY_SPEC, bool, O>;
#[doc = "Field `adpll_fcal_polarity` reader - "]
pub type ADPLL_FCAL_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `adpll_fcal_polarity` writer - "]
pub type ADPLL_FCAL_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_POLARITY_SPEC, bool, O>;
#[doc = "Field `adpll_lp_polarity` reader - "]
pub type ADPLL_LP_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `adpll_lp_polarity` writer - "]
pub type ADPLL_LP_POLARITY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ADPLL_POLARITY_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adpll_lp_mom_polarity(&self) -> ADPLL_LP_MOM_POLARITY_R {
        ADPLL_LP_MOM_POLARITY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_lms_polarity(&self) -> ADPLL_LMS_POLARITY_R {
        ADPLL_LMS_POLARITY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adpll_fcal_polarity(&self) -> ADPLL_FCAL_POLARITY_R {
        ADPLL_FCAL_POLARITY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adpll_lp_polarity(&self) -> ADPLL_LP_POLARITY_R {
        ADPLL_LP_POLARITY_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn adpll_lp_mom_polarity(&mut self) -> ADPLL_LP_MOM_POLARITY_W<8> {
        ADPLL_LP_MOM_POLARITY_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adpll_lms_polarity(&mut self) -> ADPLL_LMS_POLARITY_W<12> {
        ADPLL_LMS_POLARITY_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn adpll_fcal_polarity(&mut self) -> ADPLL_FCAL_POLARITY_W<16> {
        ADPLL_FCAL_POLARITY_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adpll_lp_polarity(&mut self) -> ADPLL_LP_POLARITY_W<20> {
        ADPLL_LP_POLARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_polarity.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_polarity](index.html) module"]
pub struct ADPLL_POLARITY_SPEC;
impl crate::RegisterSpec for ADPLL_POLARITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_polarity::R](R) reader structure"]
impl crate::Readable for ADPLL_POLARITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_polarity::W](W) writer structure"]
impl crate::Writable for ADPLL_POLARITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_polarity to value 0"]
impl crate::Resettable for ADPLL_POLARITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
