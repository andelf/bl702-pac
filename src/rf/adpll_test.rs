#[doc = "Register `adpll_test` reader"]
pub struct R(crate::R<ADPLL_TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADPLL_TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADPLL_TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADPLL_TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `adpll_test` writer"]
pub struct W(crate::W<ADPLL_TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADPLL_TEST_SPEC>;
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
impl From<crate::W<ADPLL_TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADPLL_TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adpll_test_out` reader - "]
pub type ADPLL_TEST_OUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `adpll_test_out` writer - "]
pub type ADPLL_TEST_OUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_TEST_SPEC, u16, u16, 16, O>;
#[doc = "Field `adpll_test_data_sel` reader - "]
pub type ADPLL_TEST_DATA_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_test_data_sel` writer - "]
pub type ADPLL_TEST_DATA_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_TEST_SPEC, u8, u8, 3, O>;
#[doc = "Field `adpll_test_start_sel` reader - "]
pub type ADPLL_TEST_START_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_test_start_sel` writer - "]
pub type ADPLL_TEST_START_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADPLL_TEST_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_test_en` reader - "]
pub type ADPLL_TEST_EN_R = crate::BitReader<bool>;
#[doc = "Field `adpll_test_en` writer - "]
pub type ADPLL_TEST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_TEST_SPEC, bool, O>;
#[doc = "Field `adpll_test_start` reader - "]
pub type ADPLL_TEST_START_R = crate::BitReader<bool>;
#[doc = "Field `adpll_test_start` writer - "]
pub type ADPLL_TEST_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADPLL_TEST_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn adpll_test_out(&self) -> ADPLL_TEST_OUT_R {
        ADPLL_TEST_OUT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn adpll_test_data_sel(&self) -> ADPLL_TEST_DATA_SEL_R {
        ADPLL_TEST_DATA_SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_test_start_sel(&self) -> ADPLL_TEST_START_SEL_R {
        ADPLL_TEST_START_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn adpll_test_en(&self) -> ADPLL_TEST_EN_R {
        ADPLL_TEST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn adpll_test_start(&self) -> ADPLL_TEST_START_R {
        ADPLL_TEST_START_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_test_out(&mut self) -> ADPLL_TEST_OUT_W<0> {
        ADPLL_TEST_OUT_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_test_data_sel(&mut self) -> ADPLL_TEST_DATA_SEL_W<16> {
        ADPLL_TEST_DATA_SEL_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_test_start_sel(&mut self) -> ADPLL_TEST_START_SEL_W<20> {
        ADPLL_TEST_START_SEL_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_test_en(&mut self) -> ADPLL_TEST_EN_W<24> {
        ADPLL_TEST_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn adpll_test_start(&mut self) -> ADPLL_TEST_START_W<25> {
        ADPLL_TEST_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adpll_test.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adpll_test](index.html) module"]
pub struct ADPLL_TEST_SPEC;
impl crate::RegisterSpec for ADPLL_TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adpll_test::R](R) reader structure"]
impl crate::Readable for ADPLL_TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adpll_test::W](W) writer structure"]
impl crate::Writable for ADPLL_TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets adpll_test to value 0"]
impl crate::Resettable for ADPLL_TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
