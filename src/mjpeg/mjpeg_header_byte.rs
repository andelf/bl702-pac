#[doc = "Register `mjpeg_header_byte` reader"]
pub struct R(crate::R<MJPEG_HEADER_BYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_HEADER_BYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_HEADER_BYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_HEADER_BYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_header_byte` writer"]
pub struct W(crate::W<MJPEG_HEADER_BYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_HEADER_BYTE_SPEC>;
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
impl From<crate::W<MJPEG_HEADER_BYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_HEADER_BYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_head_byte` reader - "]
pub type REG_HEAD_BYTE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_head_byte` writer - "]
pub type REG_HEAD_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_HEADER_BYTE_SPEC, u16, u16, 12, O>;
#[doc = "Field `reg_tail_exp` reader - "]
pub type REG_TAIL_EXP_R = crate::BitReader<bool>;
#[doc = "Field `reg_tail_exp` writer - "]
pub type REG_TAIL_EXP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_HEADER_BYTE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_head_byte(&self) -> REG_HEAD_BYTE_R {
        REG_HEAD_BYTE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_tail_exp(&self) -> REG_TAIL_EXP_R {
        REG_TAIL_EXP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_head_byte(&mut self) -> REG_HEAD_BYTE_W<0> {
        REG_HEAD_BYTE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_tail_exp(&mut self) -> REG_TAIL_EXP_W<16> {
        REG_TAIL_EXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_header_byte.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_header_byte](index.html) module"]
pub struct MJPEG_HEADER_BYTE_SPEC;
impl crate::RegisterSpec for MJPEG_HEADER_BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_header_byte::R](R) reader structure"]
impl crate::Readable for MJPEG_HEADER_BYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_header_byte::W](W) writer structure"]
impl crate::Writable for MJPEG_HEADER_BYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_header_byte to value 0"]
impl crate::Resettable for MJPEG_HEADER_BYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
