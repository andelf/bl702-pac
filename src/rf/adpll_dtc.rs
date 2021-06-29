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
#[doc = "Field `adpll_dtc_inv_vth_sel` reader - "]
pub struct ADPLL_DTC_INV_VTH_SEL_R(crate::FieldReader<u8, u8>);
impl ADPLL_DTC_INV_VTH_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_DTC_INV_VTH_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_DTC_INV_VTH_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_dtc_inv_vth_sel` writer - "]
pub struct ADPLL_DTC_INV_VTH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_DTC_INV_VTH_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `adpll_dtc_r_sel` reader - "]
pub struct ADPLL_DTC_R_SEL_R(crate::FieldReader<u8, u8>);
impl ADPLL_DTC_R_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADPLL_DTC_R_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_DTC_R_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_dtc_r_sel` writer - "]
pub struct ADPLL_DTC_R_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_DTC_R_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `adpll_dtc_bypass` reader - "]
pub struct ADPLL_DTC_BYPASS_R(crate::FieldReader<bool, bool>);
impl ADPLL_DTC_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADPLL_DTC_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADPLL_DTC_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adpll_dtc_bypass` writer - "]
pub struct ADPLL_DTC_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADPLL_DTC_BYPASS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_dtc_inv_vth_sel(&self) -> ADPLL_DTC_INV_VTH_SEL_R {
        ADPLL_DTC_INV_VTH_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn adpll_dtc_r_sel(&self) -> ADPLL_DTC_R_SEL_R {
        ADPLL_DTC_R_SEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_dtc_bypass(&self) -> ADPLL_DTC_BYPASS_R {
        ADPLL_DTC_BYPASS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn adpll_dtc_inv_vth_sel(&mut self) -> ADPLL_DTC_INV_VTH_SEL_W {
        ADPLL_DTC_INV_VTH_SEL_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn adpll_dtc_r_sel(&mut self) -> ADPLL_DTC_R_SEL_W {
        ADPLL_DTC_R_SEL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adpll_dtc_bypass(&mut self) -> ADPLL_DTC_BYPASS_W {
        ADPLL_DTC_BYPASS_W { w: self }
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
