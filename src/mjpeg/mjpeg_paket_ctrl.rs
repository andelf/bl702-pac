#[doc = "Register `mjpeg_paket_ctrl` reader"]
pub struct R(crate::R<MJPEG_PAKET_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_PAKET_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_PAKET_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_PAKET_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_paket_ctrl` writer"]
pub struct W(crate::W<MJPEG_PAKET_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_PAKET_CTRL_SPEC>;
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
impl From<crate::W<MJPEG_PAKET_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_PAKET_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_pket_body_byte` reader - "]
pub struct REG_PKET_BODY_BYTE_R(crate::FieldReader<u16, u16>);
impl REG_PKET_BODY_BYTE_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG_PKET_BODY_BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_PKET_BODY_BYTE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_pket_body_byte` writer - "]
pub struct REG_PKET_BODY_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PKET_BODY_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `reg_jend_to_pend` reader - "]
pub struct REG_JEND_TO_PEND_R(crate::FieldReader<bool, bool>);
impl REG_JEND_TO_PEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_JEND_TO_PEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_JEND_TO_PEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_jend_to_pend` writer - "]
pub struct REG_JEND_TO_PEND_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_JEND_TO_PEND_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `reg_pket_en` reader - "]
pub struct REG_PKET_EN_R(crate::FieldReader<bool, bool>);
impl REG_PKET_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_PKET_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_PKET_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_pket_en` writer - "]
pub struct REG_PKET_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PKET_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_pket_body_byte(&self) -> REG_PKET_BODY_BYTE_R {
        REG_PKET_BODY_BYTE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_jend_to_pend(&self) -> REG_JEND_TO_PEND_R {
        REG_JEND_TO_PEND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pket_en(&self) -> REG_PKET_EN_R {
        REG_PKET_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_pket_body_byte(&mut self) -> REG_PKET_BODY_BYTE_W {
        REG_PKET_BODY_BYTE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_jend_to_pend(&mut self) -> REG_JEND_TO_PEND_W {
        REG_JEND_TO_PEND_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pket_en(&mut self) -> REG_PKET_EN_W {
        REG_PKET_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_paket_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_paket_ctrl](index.html) module"]
pub struct MJPEG_PAKET_CTRL_SPEC;
impl crate::RegisterSpec for MJPEG_PAKET_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_paket_ctrl::R](R) reader structure"]
impl crate::Readable for MJPEG_PAKET_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_paket_ctrl::W](W) writer structure"]
impl crate::Writable for MJPEG_PAKET_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_paket_ctrl to value 0"]
impl crate::Resettable for MJPEG_PAKET_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
