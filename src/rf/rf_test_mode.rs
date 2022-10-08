#[doc = "Register `rf_test_mode` reader"]
pub struct R(crate::R<RF_TEST_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_TEST_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_TEST_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_TEST_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_test_mode` writer"]
pub struct W(crate::W<RF_TEST_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_TEST_MODE_SPEC>;
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
impl From<crate::W<RF_TEST_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_TEST_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dacout_hw` reader - "]
pub type DACOUT_HW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dacout_hw` writer - "]
pub type DACOUT_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_TEST_MODE_SPEC, u16, u16, 12, O>;
#[doc = "Field `dacout_4s` reader - "]
pub type DACOUT_4S_R = crate::FieldReader<u16, u16>;
#[doc = "Field `dacout_4s` writer - "]
pub type DACOUT_4S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_TEST_MODE_SPEC, u16, u16, 12, O>;
#[doc = "Field `dacout_4s_en` reader - "]
pub type DACOUT_4S_EN_R = crate::BitReader<bool>;
#[doc = "Field `dacout_4s_en` writer - "]
pub type DACOUT_4S_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TEST_MODE_SPEC, bool, O>;
#[doc = "Field `dacout_4s_sram_en` reader - "]
pub type DACOUT_4S_SRAM_EN_R = crate::BitReader<bool>;
#[doc = "Field `dacout_4s_sram_en` writer - "]
pub type DACOUT_4S_SRAM_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RF_TEST_MODE_SPEC, bool, O>;
#[doc = "Field `rf_test_mode_en` reader - "]
pub type RF_TEST_MODE_EN_R = crate::BitReader<bool>;
#[doc = "Field `rf_test_mode_en` writer - "]
pub type RF_TEST_MODE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RF_TEST_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dacout_hw(&self) -> DACOUT_HW_R {
        DACOUT_HW_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn dacout_4s(&self) -> DACOUT_4S_R {
        DACOUT_4S_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dacout_4s_en(&self) -> DACOUT_4S_EN_R {
        DACOUT_4S_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dacout_4s_sram_en(&self) -> DACOUT_4S_SRAM_EN_R {
        DACOUT_4S_SRAM_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_test_mode_en(&self) -> RF_TEST_MODE_EN_R {
        RF_TEST_MODE_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dacout_hw(&mut self) -> DACOUT_HW_W<0> {
        DACOUT_HW_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn dacout_4s(&mut self) -> DACOUT_4S_W<16> {
        DACOUT_4S_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dacout_4s_en(&mut self) -> DACOUT_4S_EN_W<28> {
        DACOUT_4S_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dacout_4s_sram_en(&mut self) -> DACOUT_4S_SRAM_EN_W<29> {
        DACOUT_4S_SRAM_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rf_test_mode_en(&mut self) -> RF_TEST_MODE_EN_W<30> {
        RF_TEST_MODE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_test_mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_test_mode](index.html) module"]
pub struct RF_TEST_MODE_SPEC;
impl crate::RegisterSpec for RF_TEST_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_test_mode::R](R) reader structure"]
impl crate::Readable for RF_TEST_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_test_mode::W](W) writer structure"]
impl crate::Writable for RF_TEST_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_test_mode to value 0"]
impl crate::Resettable for RF_TEST_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
