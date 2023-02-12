#[doc = "Register `kcal1` reader"]
pub struct R(crate::R<KCAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KCAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KCAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KCAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `kcal1` writer"]
pub struct W(crate::W<KCAL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KCAL1_SPEC>;
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
impl From<crate::W<KCAL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KCAL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `kcal_div` reader - "]
pub type KCAL_DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `kcal_div` writer - "]
pub type KCAL_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KCAL1_SPEC, u16, u16, 16, O>;
#[doc = "Field `kcal_cnt_start` reader - "]
pub type KCAL_CNT_START_R = crate::BitReader<bool>;
#[doc = "Field `kcal_cnt_start` writer - "]
pub type KCAL_CNT_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, KCAL1_SPEC, bool, O>;
#[doc = "Field `kcal_ratio` reader - "]
pub type KCAL_RATIO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `kcal_ratio` writer - "]
pub type KCAL_RATIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KCAL1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn kcal_div(&self) -> KCAL_DIV_R {
        KCAL_DIV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn kcal_cnt_start(&self) -> KCAL_CNT_START_R {
        KCAL_CNT_START_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn kcal_ratio(&self) -> KCAL_RATIO_R {
        KCAL_RATIO_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn kcal_div(&mut self) -> KCAL_DIV_W<0> {
        KCAL_DIV_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn kcal_cnt_start(&mut self) -> KCAL_CNT_START_W<16> {
        KCAL_CNT_START_W::new(self)
    }
    #[doc = "Bits 20:29"]
    #[inline(always)]
    #[must_use]
    pub fn kcal_ratio(&mut self) -> KCAL_RATIO_W<20> {
        KCAL_RATIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "kcal1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kcal1](index.html) module"]
pub struct KCAL1_SPEC;
impl crate::RegisterSpec for KCAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kcal1::R](R) reader structure"]
impl crate::Readable for KCAL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kcal1::W](W) writer structure"]
impl crate::Writable for KCAL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets kcal1 to value 0"]
impl crate::Resettable for KCAL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
