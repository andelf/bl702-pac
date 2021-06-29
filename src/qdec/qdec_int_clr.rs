#[doc = "Register `qdec_int_clr` reader"]
pub struct R(crate::R<QDEC_INT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDEC_INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDEC_INT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDEC_INT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `qdec_int_clr` writer"]
pub struct W(crate::W<QDEC_INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDEC_INT_CLR_SPEC>;
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
impl From<crate::W<QDEC_INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDEC_INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `overflow_clr` reader - "]
pub struct OVERFLOW_CLR_R(crate::FieldReader<bool, bool>);
impl OVERFLOW_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFLOW_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `overflow_clr` writer - "]
pub struct OVERFLOW_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `dbl_rdy_clr` reader - "]
pub struct DBL_RDY_CLR_R(crate::FieldReader<bool, bool>);
impl DBL_RDY_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBL_RDY_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBL_RDY_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbl_rdy_clr` writer - "]
pub struct DBL_RDY_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DBL_RDY_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `spl_rdy_clr` reader - "]
pub struct SPL_RDY_CLR_R(crate::FieldReader<bool, bool>);
impl SPL_RDY_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPL_RDY_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPL_RDY_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spl_rdy_clr` writer - "]
pub struct SPL_RDY_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SPL_RDY_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `rpt_rdy_clr` reader - "]
pub struct RPT_RDY_CLR_R(crate::FieldReader<bool, bool>);
impl RPT_RDY_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPT_RDY_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT_RDY_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rpt_rdy_clr` writer - "]
pub struct RPT_RDY_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RPT_RDY_CLR_W<'a> {
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
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn overflow_clr(&self) -> OVERFLOW_CLR_R {
        OVERFLOW_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dbl_rdy_clr(&self) -> DBL_RDY_CLR_R {
        DBL_RDY_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spl_rdy_clr(&self) -> SPL_RDY_CLR_R {
        SPL_RDY_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rpt_rdy_clr(&self) -> RPT_RDY_CLR_R {
        RPT_RDY_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn overflow_clr(&mut self) -> OVERFLOW_CLR_W {
        OVERFLOW_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dbl_rdy_clr(&mut self) -> DBL_RDY_CLR_W {
        DBL_RDY_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spl_rdy_clr(&mut self) -> SPL_RDY_CLR_W {
        SPL_RDY_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rpt_rdy_clr(&mut self) -> RPT_RDY_CLR_W {
        RPT_RDY_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "qdec_int_clr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdec_int_clr](index.html) module"]
pub struct QDEC_INT_CLR_SPEC;
impl crate::RegisterSpec for QDEC_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdec_int_clr::R](R) reader structure"]
impl crate::Readable for QDEC_INT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdec_int_clr::W](W) writer structure"]
impl crate::Writable for QDEC_INT_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets qdec_int_clr to value 0"]
impl crate::Resettable for QDEC_INT_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
