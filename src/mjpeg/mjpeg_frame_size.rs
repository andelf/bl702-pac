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
#[doc = "Field `reg_frame_hblk` reader - "]
pub struct REG_FRAME_HBLK_R(crate::FieldReader<u16, u16>);
impl REG_FRAME_HBLK_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG_FRAME_HBLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_FRAME_HBLK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_frame_hblk` writer - "]
pub struct REG_FRAME_HBLK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_FRAME_HBLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `reg_frame_wblk` reader - "]
pub struct REG_FRAME_WBLK_R(crate::FieldReader<u16, u16>);
impl REG_FRAME_WBLK_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG_FRAME_WBLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_FRAME_WBLK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_frame_wblk` writer - "]
pub struct REG_FRAME_WBLK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_FRAME_WBLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn reg_frame_hblk(&self) -> REG_FRAME_HBLK_R {
        REG_FRAME_HBLK_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_frame_wblk(&self) -> REG_FRAME_WBLK_R {
        REG_FRAME_WBLK_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn reg_frame_hblk(&mut self) -> REG_FRAME_HBLK_W {
        REG_FRAME_HBLK_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_frame_wblk(&mut self) -> REG_FRAME_WBLK_W {
        REG_FRAME_WBLK_W { w: self }
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
}
#[doc = "`reset()` method sets mjpeg_frame_size to value 0"]
impl crate::Resettable for MJPEG_FRAME_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
