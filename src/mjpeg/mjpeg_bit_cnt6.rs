#[doc = "Register `mjpeg_bit_cnt6` reader"]
pub struct R(crate::R<MJPEG_BIT_CNT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_BIT_CNT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_BIT_CNT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_BIT_CNT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_bit_cnt6` writer"]
pub struct W(crate::W<MJPEG_BIT_CNT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_BIT_CNT6_SPEC>;
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
impl From<crate::W<MJPEG_BIT_CNT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_BIT_CNT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frame_bit_cnt_6` reader - "]
pub struct FRAME_BIT_CNT_6_R(crate::FieldReader<u32, u32>);
impl FRAME_BIT_CNT_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FRAME_BIT_CNT_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_BIT_CNT_6_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `frame_bit_cnt_6` writer - "]
pub struct FRAME_BIT_CNT_6_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_BIT_CNT_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn frame_bit_cnt_6(&self) -> FRAME_BIT_CNT_6_R {
        FRAME_BIT_CNT_6_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn frame_bit_cnt_6(&mut self) -> FRAME_BIT_CNT_6_W {
        FRAME_BIT_CNT_6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_bit_cnt6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_bit_cnt6](index.html) module"]
pub struct MJPEG_BIT_CNT6_SPEC;
impl crate::RegisterSpec for MJPEG_BIT_CNT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_bit_cnt6::R](R) reader structure"]
impl crate::Readable for MJPEG_BIT_CNT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_bit_cnt6::W](W) writer structure"]
impl crate::Writable for MJPEG_BIT_CNT6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_bit_cnt6 to value 0"]
impl crate::Resettable for MJPEG_BIT_CNT6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
