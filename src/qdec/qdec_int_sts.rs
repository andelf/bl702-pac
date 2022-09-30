#[doc = "Register `qdec_int_sts` reader"]
pub struct R(crate::R<QDEC_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDEC_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDEC_INT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDEC_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `qdec_int_sts` writer"]
pub struct W(crate::W<QDEC_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDEC_INT_STS_SPEC>;
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
impl From<crate::W<QDEC_INT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDEC_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rpt_rdy_sts` reader - "]
pub type RPT_RDY_STS_R = crate::BitReader<bool>;
#[doc = "Field `rpt_rdy_sts` writer - "]
pub type RPT_RDY_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_INT_STS_SPEC, bool, O>;
#[doc = "Field `spl_rdy_sts` reader - "]
pub type SPL_RDY_STS_R = crate::BitReader<bool>;
#[doc = "Field `spl_rdy_sts` writer - "]
pub type SPL_RDY_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_INT_STS_SPEC, bool, O>;
#[doc = "Field `dbl_rdy_sts` reader - "]
pub type DBL_RDY_STS_R = crate::BitReader<bool>;
#[doc = "Field `dbl_rdy_sts` writer - "]
pub type DBL_RDY_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_INT_STS_SPEC, bool, O>;
#[doc = "Field `overflow_sts` reader - "]
pub type OVERFLOW_STS_R = crate::BitReader<bool>;
#[doc = "Field `overflow_sts` writer - "]
pub type OVERFLOW_STS_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_INT_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rpt_rdy_sts(&self) -> RPT_RDY_STS_R {
        RPT_RDY_STS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spl_rdy_sts(&self) -> SPL_RDY_STS_R {
        SPL_RDY_STS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dbl_rdy_sts(&self) -> DBL_RDY_STS_R {
        DBL_RDY_STS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn overflow_sts(&self) -> OVERFLOW_STS_R {
        OVERFLOW_STS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rpt_rdy_sts(&mut self) -> RPT_RDY_STS_W<0> {
        RPT_RDY_STS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spl_rdy_sts(&mut self) -> SPL_RDY_STS_W<1> {
        SPL_RDY_STS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dbl_rdy_sts(&mut self) -> DBL_RDY_STS_W<2> {
        DBL_RDY_STS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn overflow_sts(&mut self) -> OVERFLOW_STS_W<3> {
        OVERFLOW_STS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "qdec_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdec_int_sts](index.html) module"]
pub struct QDEC_INT_STS_SPEC;
impl crate::RegisterSpec for QDEC_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdec_int_sts::R](R) reader structure"]
impl crate::Readable for QDEC_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdec_int_sts::W](W) writer structure"]
impl crate::Writable for QDEC_INT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets qdec_int_sts to value 0"]
impl crate::Resettable for QDEC_INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
