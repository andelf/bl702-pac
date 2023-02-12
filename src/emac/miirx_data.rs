#[doc = "Register `MIIRX_DATA` reader"]
pub struct R(crate::R<MIIRX_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIIRX_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIIRX_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIIRX_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIIRX_DATA` writer"]
pub struct W(crate::W<MIIRX_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIIRX_DATA_SPEC>;
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
impl From<crate::W<MIIRX_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIIRX_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRSD` reader - "]
pub type PRSD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRSD` writer - "]
pub type PRSD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIIRX_DATA_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn prsd(&self) -> PRSD_R {
        PRSD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn prsd(&mut self) -> PRSD_W<0> {
        PRSD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIIRX_DATA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miirx_data](index.html) module"]
pub struct MIIRX_DATA_SPEC;
impl crate::RegisterSpec for MIIRX_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miirx_data::R](R) reader structure"]
impl crate::Readable for MIIRX_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miirx_data::W](W) writer structure"]
impl crate::Writable for MIIRX_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIIRX_DATA to value 0"]
impl crate::Resettable for MIIRX_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
