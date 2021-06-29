#[doc = "Register `irrx_data_count` reader"]
pub struct R(crate::R<IRRX_DATA_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_DATA_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRRX_DATA_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRRX_DATA_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irrx_data_count` writer"]
pub struct W(crate::W<IRRX_DATA_COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRRX_DATA_COUNT_SPEC>;
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
impl From<crate::W<IRRX_DATA_COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRRX_DATA_COUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_irrx_data_cnt` reader - "]
pub struct STS_IRRX_DATA_CNT_R(crate::FieldReader<u8, u8>);
impl STS_IRRX_DATA_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        STS_IRRX_DATA_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_IRRX_DATA_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_irrx_data_cnt` writer - "]
pub struct STS_IRRX_DATA_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_IRRX_DATA_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn sts_irrx_data_cnt(&self) -> STS_IRRX_DATA_CNT_R {
        STS_IRRX_DATA_CNT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn sts_irrx_data_cnt(&mut self) -> STS_IRRX_DATA_CNT_W {
        STS_IRRX_DATA_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irrx_data_count.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_data_count](index.html) module"]
pub struct IRRX_DATA_COUNT_SPEC;
impl crate::RegisterSpec for IRRX_DATA_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_data_count::R](R) reader structure"]
impl crate::Readable for IRRX_DATA_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irrx_data_count::W](W) writer structure"]
impl crate::Writable for IRRX_DATA_COUNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irrx_data_count to value 0"]
impl crate::Resettable for IRRX_DATA_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
