#[doc = "Register `MIITX_DATA` reader"]
pub struct R(crate::R<MIITX_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIITX_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIITX_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIITX_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIITX_DATA` writer"]
pub struct W(crate::W<MIITX_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIITX_DATA_SPEC>;
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
impl From<crate::W<MIITX_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIITX_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTRLDATA` reader - "]
pub struct CTRLDATA_R(crate::FieldReader<u16, u16>);
impl CTRLDATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTRLDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLDATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLDATA` writer - "]
pub struct CTRLDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ctrldata(&self) -> CTRLDATA_R {
        CTRLDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ctrldata(&mut self) -> CTRLDATA_W {
        CTRLDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIITX_DATA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miitx_data](index.html) module"]
pub struct MIITX_DATA_SPEC;
impl crate::RegisterSpec for MIITX_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miitx_data::R](R) reader structure"]
impl crate::Readable for MIITX_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miitx_data::W](W) writer structure"]
impl crate::Writable for MIITX_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIITX_DATA to value 0"]
impl crate::Resettable for MIITX_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
