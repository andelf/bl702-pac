#[doc = "Register `mjpeg_start_addr_f` reader"]
pub struct R(crate::R<MJPEG_START_ADDR_F_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_START_ADDR_F_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_START_ADDR_F_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_START_ADDR_F_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_start_addr_f` writer"]
pub struct W(crate::W<MJPEG_START_ADDR_F_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_START_ADDR_F_SPEC>;
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
impl From<crate::W<MJPEG_START_ADDR_F_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_START_ADDR_F_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frame_start_addr_f` reader - "]
pub type FRAME_START_ADDR_F_R = crate::FieldReader<u32, u32>;
#[doc = "Field `frame_start_addr_f` writer - "]
pub type FRAME_START_ADDR_F_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_START_ADDR_F_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn frame_start_addr_f(&self) -> FRAME_START_ADDR_F_R {
        FRAME_START_ADDR_F_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn frame_start_addr_f(&mut self) -> FRAME_START_ADDR_F_W<0> {
        FRAME_START_ADDR_F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_start_addr_f.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_start_addr_f](index.html) module"]
pub struct MJPEG_START_ADDR_F_SPEC;
impl crate::RegisterSpec for MJPEG_START_ADDR_F_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_start_addr_f::R](R) reader structure"]
impl crate::Readable for MJPEG_START_ADDR_F_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_start_addr_f::W](W) writer structure"]
impl crate::Writable for MJPEG_START_ADDR_F_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_start_addr_f to value 0"]
impl crate::Resettable for MJPEG_START_ADDR_F_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
