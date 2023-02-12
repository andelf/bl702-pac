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
#[doc = "Field `reg_mjpeg_dbg_en` reader - "]
pub type REG_MJPEG_DBG_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_mjpeg_dbg_en` writer - "]
pub type REG_MJPEG_DBG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_DEBUG_SPEC, bool, O>;
#[doc = "Field `reg_mjpeg_dbg_sel` reader - "]
pub type REG_MJPEG_DBG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_mjpeg_dbg_sel` writer - "]
pub type REG_MJPEG_DBG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_DEBUG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_mjpeg_dbg_en(&self) -> REG_MJPEG_DBG_EN_R {
        REG_MJPEG_DBG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reg_mjpeg_dbg_sel(&self) -> REG_MJPEG_DBG_SEL_R {
        REG_MJPEG_DBG_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mjpeg_dbg_en(&mut self) -> REG_MJPEG_DBG_EN_W<0> {
        REG_MJPEG_DBG_EN_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mjpeg_dbg_sel(&mut self) -> REG_MJPEG_DBG_SEL_W<4> {
        REG_MJPEG_DBG_SEL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mjpeg_debug to value 0"]
impl crate::Resettable for MJPEG_DEBUG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
