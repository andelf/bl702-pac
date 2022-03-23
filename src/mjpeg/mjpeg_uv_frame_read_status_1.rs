#[doc = "Register `mjpeg_UV_frame_read_status_1` reader"]
pub struct R(crate::R<MJPEG_UV_FRAME_READ_STATUS_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_UV_FRAME_READ_STATUS_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_UV_FRAME_READ_STATUS_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_UV_FRAME_READ_STATUS_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_UV_frame_read_status_1` writer"]
pub struct W(crate::W<MJPEG_UV_FRAME_READ_STATUS_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_UV_FRAME_READ_STATUS_1_SPEC>;
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
impl From<crate::W<MJPEG_UV_FRAME_READ_STATUS_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_UV_FRAME_READ_STATUS_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uv_frm_hblk_r` reader - "]
pub struct UV_FRM_HBLK_R_R(crate::FieldReader<u16, u16>);
impl UV_FRM_HBLK_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UV_FRM_HBLK_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UV_FRM_HBLK_R_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uv_frm_hblk_r` writer - "]
pub struct UV_FRM_HBLK_R_W<'a> {
    w: &'a mut W,
}
impl<'a> UV_FRM_HBLK_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 16)) | ((value as u32 & 0x1fff) << 16);
        self.w
    }
}
#[doc = "Field `uv_mem_hblk_r` reader - "]
pub struct UV_MEM_HBLK_R_R(crate::FieldReader<u16, u16>);
impl UV_MEM_HBLK_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        UV_MEM_HBLK_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UV_MEM_HBLK_R_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uv_mem_hblk_r` writer - "]
pub struct UV_MEM_HBLK_R_W<'a> {
    w: &'a mut W,
}
impl<'a> UV_MEM_HBLK_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn uv_frm_hblk_r(&self) -> UV_FRM_HBLK_R_R {
        UV_FRM_HBLK_R_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn uv_mem_hblk_r(&self) -> UV_MEM_HBLK_R_R {
        UV_MEM_HBLK_R_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:28"]
    #[inline(always)]
    pub fn uv_frm_hblk_r(&mut self) -> UV_FRM_HBLK_R_W {
        UV_FRM_HBLK_R_W { w: self }
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn uv_mem_hblk_r(&mut self) -> UV_MEM_HBLK_R_W {
        UV_MEM_HBLK_R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_UV_frame_read_status_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_uv_frame_read_status_1](index.html) module"]
pub struct MJPEG_UV_FRAME_READ_STATUS_1_SPEC;
impl crate::RegisterSpec for MJPEG_UV_FRAME_READ_STATUS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_uv_frame_read_status_1::R](R) reader structure"]
impl crate::Readable for MJPEG_UV_FRAME_READ_STATUS_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_uv_frame_read_status_1::W](W) writer structure"]
impl crate::Writable for MJPEG_UV_FRAME_READ_STATUS_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_UV_frame_read_status_1 to value 0"]
impl crate::Resettable for MJPEG_UV_FRAME_READ_STATUS_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
