#[doc = "Register `mjpeg_q_mode_c` reader"]
pub struct R(crate::R<MJPEG_Q_MODE_C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_Q_MODE_C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_Q_MODE_C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_Q_MODE_C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_q_mode_c` writer"]
pub struct W(crate::W<MJPEG_Q_MODE_C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_Q_MODE_C_SPEC>;
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
impl From<crate::W<MJPEG_Q_MODE_C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_Q_MODE_C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frame_q_mode_c` reader - "]
pub struct FRAME_Q_MODE_C_R(crate::FieldReader<u8, u8>);
impl FRAME_Q_MODE_C_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRAME_Q_MODE_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_Q_MODE_C_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `frame_q_mode_c` writer - "]
pub struct FRAME_Q_MODE_C_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_Q_MODE_C_W<'a> {
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
    pub fn frame_q_mode_c(&self) -> FRAME_Q_MODE_C_R {
        FRAME_Q_MODE_C_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn frame_q_mode_c(&mut self) -> FRAME_Q_MODE_C_W {
        FRAME_Q_MODE_C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_q_mode_c.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_q_mode_c](index.html) module"]
pub struct MJPEG_Q_MODE_C_SPEC;
impl crate::RegisterSpec for MJPEG_Q_MODE_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_q_mode_c::R](R) reader structure"]
impl crate::Readable for MJPEG_Q_MODE_C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_q_mode_c::W](W) writer structure"]
impl crate::Writable for MJPEG_Q_MODE_C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_q_mode_c to value 0"]
impl crate::Resettable for MJPEG_Q_MODE_C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
