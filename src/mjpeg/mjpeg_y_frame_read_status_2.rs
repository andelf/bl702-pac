#[doc = "Register `mjpeg_Y_frame_read_status_2` reader"]
pub struct R(crate::R<MJPEG_Y_FRAME_READ_STATUS_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_Y_FRAME_READ_STATUS_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_Y_FRAME_READ_STATUS_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_Y_FRAME_READ_STATUS_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_Y_frame_read_status_2` writer"]
pub struct W(crate::W<MJPEG_Y_FRAME_READ_STATUS_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_Y_FRAME_READ_STATUS_2_SPEC>;
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
impl From<crate::W<MJPEG_Y_FRAME_READ_STATUS_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_Y_FRAME_READ_STATUS_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `yy_frm_cnt_r` reader - "]
pub struct YY_FRM_CNT_R_R(crate::FieldReader<u8, u8>);
impl YY_FRM_CNT_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        YY_FRM_CNT_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YY_FRM_CNT_R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `yy_frm_cnt_r` writer - "]
pub struct YY_FRM_CNT_R_W<'a> {
    w: &'a mut W,
}
impl<'a> YY_FRM_CNT_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `yy_mem_rnd_r` reader - "]
pub struct YY_MEM_RND_R_R(crate::FieldReader<u8, u8>);
impl YY_MEM_RND_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        YY_MEM_RND_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YY_MEM_RND_R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `yy_mem_rnd_r` writer - "]
pub struct YY_MEM_RND_R_W<'a> {
    w: &'a mut W,
}
impl<'a> YY_MEM_RND_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `yy_wblk_r` reader - "]
pub struct YY_WBLK_R_R(crate::FieldReader<u16, u16>);
impl YY_WBLK_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        YY_WBLK_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for YY_WBLK_R_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `yy_wblk_r` writer - "]
pub struct YY_WBLK_R_W<'a> {
    w: &'a mut W,
}
impl<'a> YY_WBLK_R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u32 & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn yy_frm_cnt_r(&self) -> YY_FRM_CNT_R_R {
        YY_FRM_CNT_R_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn yy_mem_rnd_r(&self) -> YY_MEM_RND_R_R {
        YY_MEM_RND_R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn yy_wblk_r(&self) -> YY_WBLK_R_R {
        YY_WBLK_R_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn yy_frm_cnt_r(&mut self) -> YY_FRM_CNT_R_W {
        YY_FRM_CNT_R_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn yy_mem_rnd_r(&mut self) -> YY_MEM_RND_R_W {
        YY_MEM_RND_R_W { w: self }
    }
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn yy_wblk_r(&mut self) -> YY_WBLK_R_W {
        YY_WBLK_R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_Y_frame_read_status_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_y_frame_read_status_2](index.html) module"]
pub struct MJPEG_Y_FRAME_READ_STATUS_2_SPEC;
impl crate::RegisterSpec for MJPEG_Y_FRAME_READ_STATUS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_y_frame_read_status_2::R](R) reader structure"]
impl crate::Readable for MJPEG_Y_FRAME_READ_STATUS_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_y_frame_read_status_2::W](W) writer structure"]
impl crate::Writable for MJPEG_Y_FRAME_READ_STATUS_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_Y_frame_read_status_2 to value 0"]
impl crate::Resettable for MJPEG_Y_FRAME_READ_STATUS_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
