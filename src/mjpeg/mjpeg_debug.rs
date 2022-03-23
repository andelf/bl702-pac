#[doc = "Register `mjpeg_debug` reader"]
pub struct R(crate::R<MJPEG_DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_DEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_debug` writer"]
pub struct W(crate::W<MJPEG_DEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_DEBUG_SPEC>;
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
impl From<crate::W<MJPEG_DEBUG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_DEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mjpeg_dbg_sel` reader - "]
pub struct REG_MJPEG_DBG_SEL_R(crate::FieldReader<u8, u8>);
impl REG_MJPEG_DBG_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REG_MJPEG_DBG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MJPEG_DBG_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mjpeg_dbg_sel` writer - "]
pub struct REG_MJPEG_DBG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MJPEG_DBG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `reg_mjpeg_dbg_en` reader - "]
pub struct REG_MJPEG_DBG_EN_R(crate::FieldReader<bool, bool>);
impl REG_MJPEG_DBG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REG_MJPEG_DBG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MJPEG_DBG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mjpeg_dbg_en` writer - "]
pub struct REG_MJPEG_DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MJPEG_DBG_EN_W<'a> {
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
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_mjpeg_dbg_sel(&self) -> REG_MJPEG_DBG_SEL_R {
        REG_MJPEG_DBG_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mjpeg_dbg_en(&self) -> REG_MJPEG_DBG_EN_R {
        REG_MJPEG_DBG_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_mjpeg_dbg_sel(&mut self) -> REG_MJPEG_DBG_SEL_W {
        REG_MJPEG_DBG_SEL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mjpeg_dbg_en(&mut self) -> REG_MJPEG_DBG_EN_W {
        REG_MJPEG_DBG_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_debug.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_debug](index.html) module"]
pub struct MJPEG_DEBUG_SPEC;
impl crate::RegisterSpec for MJPEG_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_debug::R](R) reader structure"]
impl crate::Readable for MJPEG_DEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_debug::W](W) writer structure"]
impl crate::Writable for MJPEG_DEBUG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets mjpeg_debug to value 0"]
impl crate::Resettable for MJPEG_DEBUG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
