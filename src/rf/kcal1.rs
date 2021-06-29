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
#[doc = "Field `kcal_ratio` reader - "]
pub struct KCAL_RATIO_R(crate::FieldReader<u16, u16>);
impl KCAL_RATIO_R {
    pub(crate) fn new(bits: u16) -> Self {
        KCAL_RATIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_RATIO_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_ratio` writer - "]
pub struct KCAL_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Field `kcal_cnt_start` reader - "]
pub struct KCAL_CNT_START_R(crate::FieldReader<bool, bool>);
impl KCAL_CNT_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        KCAL_CNT_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_CNT_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_cnt_start` writer - "]
pub struct KCAL_CNT_START_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_CNT_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `kcal_div` reader - "]
pub struct KCAL_DIV_R(crate::FieldReader<u16, u16>);
impl KCAL_DIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        KCAL_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_div` writer - "]
pub struct KCAL_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn kcal_ratio(&self) -> KCAL_RATIO_R {
        KCAL_RATIO_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn kcal_cnt_start(&self) -> KCAL_CNT_START_R {
        KCAL_CNT_START_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn kcal_div(&self) -> KCAL_DIV_R {
        KCAL_DIV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn kcal_ratio(&mut self) -> KCAL_RATIO_W {
        KCAL_RATIO_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn kcal_cnt_start(&mut self) -> KCAL_CNT_START_W {
        KCAL_CNT_START_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn kcal_div(&mut self) -> KCAL_DIV_W {
        KCAL_DIV_W { w: self }
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
}
#[doc = "`reset()` method sets kcal1 to value 0"]
impl crate::Resettable for KCAL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
