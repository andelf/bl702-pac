#[doc = "Register `mjpeg_frame_size` reader"]
pub struct R(crate::R<MJPEG_FRAME_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_FRAME_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_FRAME_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_FRAME_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_frame_size` writer"]
pub struct W(crate::W<MJPEG_FRAME_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_FRAME_SIZE_SPEC>;
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
impl From<crate::W<MJPEG_FRAME_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_FRAME_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_frame_wblk` reader - "]
pub type REG_FRAME_WBLK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_frame_wblk` writer - "]
pub type REG_FRAME_WBLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_FRAME_SIZE_SPEC, u16, u16, 12, O>;
#[doc = "Field `reg_frame_hblk` reader - "]
pub type REG_FRAME_HBLK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_frame_hblk` writer - "]
pub type REG_FRAME_HBLK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_FRAME_SIZE_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_frame_wblk(&self) -> REG_FRAME_WBLK_R {
        REG_FRAME_WBLK_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn reg_frame_hblk(&self) -> REG_FRAME_HBLK_R {
        REG_FRAME_HBLK_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_frame_wblk(&mut self) -> REG_FRAME_WBLK_W<0> {
        REG_FRAME_WBLK_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn reg_frame_hblk(&mut self) -> REG_FRAME_HBLK_W<16> {
        REG_FRAME_HBLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_frame_size.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_frame_size](index.html) module"]
pub struct MJPEG_FRAME_SIZE_SPEC;
impl crate::RegisterSpec for MJPEG_FRAME_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_frame_size::R](R) reader structure"]
impl crate::Readable for MJPEG_FRAME_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_frame_size::W](W) writer structure"]
impl crate::Writable for MJPEG_FRAME_SIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mjpeg_frame_size to value 0"]
impl crate::Resettable for MJPEG_FRAME_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
