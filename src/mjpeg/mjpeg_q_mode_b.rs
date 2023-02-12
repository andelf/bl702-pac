#[doc = "Register `mjpeg_q_mode_b` reader"]
pub struct R(crate::R<MJPEG_Q_MODE_B_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_Q_MODE_B_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_Q_MODE_B_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_Q_MODE_B_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_q_mode_b` writer"]
pub struct W(crate::W<MJPEG_Q_MODE_B_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_Q_MODE_B_SPEC>;
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
impl From<crate::W<MJPEG_Q_MODE_B_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_Q_MODE_B_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frame_q_mode_b` reader - "]
pub type FRAME_Q_MODE_B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `frame_q_mode_b` writer - "]
pub type FRAME_Q_MODE_B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_Q_MODE_B_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn frame_q_mode_b(&self) -> FRAME_Q_MODE_B_R {
        FRAME_Q_MODE_B_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn frame_q_mode_b(&mut self) -> FRAME_Q_MODE_B_W<0> {
        FRAME_Q_MODE_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_q_mode_b.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_q_mode_b](index.html) module"]
pub struct MJPEG_Q_MODE_B_SPEC;
impl crate::RegisterSpec for MJPEG_Q_MODE_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_q_mode_b::R](R) reader structure"]
impl crate::Readable for MJPEG_Q_MODE_B_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_q_mode_b::W](W) writer structure"]
impl crate::Writable for MJPEG_Q_MODE_B_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mjpeg_q_mode_b to value 0"]
impl crate::Resettable for MJPEG_Q_MODE_B_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
