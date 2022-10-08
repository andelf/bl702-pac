#[doc = "Register `adpll_dtc` reader"]
pub struct R(crate::R<ADPLL_DTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_DTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_DTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_DTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_dtc` writer"]
pub struct W(crate::W<ADPLL_DTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_DTC_SPEC>;
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
impl From<crate::W<ADPLL_DTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_DTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_dtc_bypass` reader - "]
pub type ADPLL_DTC_BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `adpll_dtc_bypass` writer - "]
pub type ADPLL_DTC_BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_DTC_SPEC, bool, O>;
#[doc = "Field `adpll_dtc_r_sel` reader - "]
pub type ADPLL_DTC_R_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_dtc_r_sel` writer - "]
pub type ADPLL_DTC_R_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_DTC_SPEC, u8, u8, 3, O>;
#[doc = "Field `adpll_dtc_inv_vth_sel` reader - "]
pub type ADPLL_DTC_INV_VTH_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_dtc_inv_vth_sel` writer - "]
pub type ADPLL_DTC_INV_VTH_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_DTC_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_dtc_bypass(&self) -> ADPLL_DTC_BYPASS_R {
        ADPLL_DTC_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn adpll_dtc_r_sel(&self) -> ADPLL_DTC_R_SEL_R {
        ADPLL_DTC_R_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_dtc_inv_vth_sel(&self) -> ADPLL_DTC_INV_VTH_SEL_R {
        ADPLL_DTC_INV_VTH_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_dtc_bypass(&mut self) -> ADPLL_DTC_BYPASS_W<0> {
        ADPLL_DTC_BYPASS_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn adpll_dtc_r_sel(&mut self) -> ADPLL_DTC_R_SEL_W<4> {
        ADPLL_DTC_R_SEL_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_dtc_inv_vth_sel(&mut self) -> ADPLL_DTC_INV_VTH_SEL_W<8> {
        ADPLL_DTC_INV_VTH_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_dtc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_dtc](index.html) module"]
pub struct ADPLL_DTC_SPEC;
impl crate::RegisterSpec for ADPLL_DTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_dtc::R](R) reader structure"]
impl crate::Readable for ADPLL_DTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_dtc::W](W) writer structure"]
impl crate::Writable for ADPLL_DTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets adpll_dtc to value 0"]
impl crate::Resettable for ADPLL_DTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
