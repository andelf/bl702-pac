#[doc = "Register `mjpeg_yy_frame_addr` reader"]
pub struct R(crate::R<MJPEG_YY_FRAME_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_YY_FRAME_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_YY_FRAME_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_YY_FRAME_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_yy_frame_addr` writer"]
pub struct W(crate::W<MJPEG_YY_FRAME_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_YY_FRAME_ADDR_SPEC>;
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
impl From<crate::W<MJPEG_YY_FRAME_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_YY_FRAME_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_yy_addr_start` reader - "]
pub struct REG_YY_ADDR_START_R(crate::FieldReader<u32, u32>);
impl REG_YY_ADDR_START_R {
    pub(crate) fn new(bits: u32) -> Self {
        REG_YY_ADDR_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_YY_ADDR_START_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_yy_addr_start` writer - "]
pub struct REG_YY_ADDR_START_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_YY_ADDR_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_yy_addr_start(&self) -> REG_YY_ADDR_START_R {
        REG_YY_ADDR_START_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_yy_addr_start(&mut self) -> REG_YY_ADDR_START_W {
        REG_YY_ADDR_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_yy_frame_addr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_yy_frame_addr](index.html) module"]
pub struct MJPEG_YY_FRAME_ADDR_SPEC;
impl crate::RegisterSpec for MJPEG_YY_FRAME_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_yy_frame_addr::R](R) reader structure"]
impl crate::Readable for MJPEG_YY_FRAME_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_yy_frame_addr::W](W) writer structure"]
impl crate::Writable for MJPEG_YY_FRAME_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_yy_frame_addr to value 0"]
impl crate::Resettable for MJPEG_YY_FRAME_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
