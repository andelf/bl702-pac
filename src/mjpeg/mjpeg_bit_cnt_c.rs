#[doc = "Register `mjpeg_bit_cnt_c` reader"]
pub struct R(crate::R<MJPEG_BIT_CNT_C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_BIT_CNT_C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_BIT_CNT_C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_BIT_CNT_C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_bit_cnt_c` writer"]
pub struct W(crate::W<MJPEG_BIT_CNT_C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_BIT_CNT_C_SPEC>;
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
impl From<crate::W<MJPEG_BIT_CNT_C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_BIT_CNT_C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frame_bit_cnt_c` reader - "]
pub type FRAME_BIT_CNT_C_R = crate::FieldReader<u32, u32>;
#[doc = "Field `frame_bit_cnt_c` writer - "]
pub type FRAME_BIT_CNT_C_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_BIT_CNT_C_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn frame_bit_cnt_c(&self) -> FRAME_BIT_CNT_C_R {
        FRAME_BIT_CNT_C_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn frame_bit_cnt_c(&mut self) -> FRAME_BIT_CNT_C_W<0> {
        FRAME_BIT_CNT_C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_bit_cnt_c.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_bit_cnt_c](index.html) module"]
pub struct MJPEG_BIT_CNT_C_SPEC;
impl crate::RegisterSpec for MJPEG_BIT_CNT_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_bit_cnt_c::R](R) reader structure"]
impl crate::Readable for MJPEG_BIT_CNT_C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_bit_cnt_c::W](W) writer structure"]
impl crate::Writable for MJPEG_BIT_CNT_C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mjpeg_bit_cnt_c to value 0"]
impl crate::Resettable for MJPEG_BIT_CNT_C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
