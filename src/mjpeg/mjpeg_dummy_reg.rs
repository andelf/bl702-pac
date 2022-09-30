#[doc = "Register `mjpeg_dummy_reg` reader"]
pub struct R(crate::R<MJPEG_DUMMY_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_DUMMY_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_DUMMY_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_DUMMY_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_dummy_reg` writer"]
pub struct W(crate::W<MJPEG_DUMMY_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_DUMMY_REG_SPEC>;
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
impl From<crate::W<MJPEG_DUMMY_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_DUMMY_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mjpeg_dummy_reg` reader - "]
pub type MJPEG_DUMMY_REG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `mjpeg_dummy_reg` writer - "]
pub type MJPEG_DUMMY_REG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_DUMMY_REG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mjpeg_dummy_reg(&self) -> MJPEG_DUMMY_REG_R {
        MJPEG_DUMMY_REG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mjpeg_dummy_reg(&mut self) -> MJPEG_DUMMY_REG_W<0> {
        MJPEG_DUMMY_REG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_dummy_reg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_dummy_reg](index.html) module"]
pub struct MJPEG_DUMMY_REG_SPEC;
impl crate::RegisterSpec for MJPEG_DUMMY_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_dummy_reg::R](R) reader structure"]
impl crate::Readable for MJPEG_DUMMY_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_dummy_reg::W](W) writer structure"]
impl crate::Writable for MJPEG_DUMMY_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_dummy_reg to value 0"]
impl crate::Resettable for MJPEG_DUMMY_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
