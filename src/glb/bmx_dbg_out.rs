#[doc = "Register `bmx_dbg_out` reader"]
pub struct R(crate::R<BMX_DBG_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_DBG_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_DBG_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_DBG_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_dbg_out` writer"]
pub struct W(crate::W<BMX_DBG_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_DBG_OUT_SPEC>;
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
impl From<crate::W<BMX_DBG_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMX_DBG_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bmx_dbg_out` reader - "]
pub type BMX_DBG_OUT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `bmx_dbg_out` writer - "]
pub type BMX_DBG_OUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BMX_DBG_OUT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_dbg_out(&self) -> BMX_DBG_OUT_R {
        BMX_DBG_OUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_dbg_out(&mut self) -> BMX_DBG_OUT_W<0> {
        BMX_DBG_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_dbg_out.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_dbg_out](index.html) module"]
pub struct BMX_DBG_OUT_SPEC;
impl crate::RegisterSpec for BMX_DBG_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_dbg_out::R](R) reader structure"]
impl crate::Readable for BMX_DBG_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_dbg_out::W](W) writer structure"]
impl crate::Writable for BMX_DBG_OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_dbg_out to value 0"]
impl crate::Resettable for BMX_DBG_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
