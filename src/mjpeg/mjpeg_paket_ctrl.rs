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
#[doc = "Field `reg_pket_en` reader - "]
pub type REG_PKET_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_pket_en` writer - "]
pub type REG_PKET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MJPEG_PAKET_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_jend_to_pend` reader - "]
pub type REG_JEND_TO_PEND_R = crate::BitReader<bool>;
#[doc = "Field `reg_jend_to_pend` writer - "]
pub type REG_JEND_TO_PEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_PAKET_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_pket_body_byte` reader - "]
pub type REG_PKET_BODY_BYTE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_pket_body_byte` writer - "]
pub type REG_PKET_BODY_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MJPEG_PAKET_CTRL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_pket_en(&self) -> REG_PKET_EN_R {
        REG_PKET_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_jend_to_pend(&self) -> REG_JEND_TO_PEND_R {
        REG_JEND_TO_PEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_pket_body_byte(&self) -> REG_PKET_BODY_BYTE_R {
        REG_PKET_BODY_BYTE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pket_en(&mut self) -> REG_PKET_EN_W<0> {
        REG_PKET_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_jend_to_pend(&mut self) -> REG_JEND_TO_PEND_W<1> {
        REG_JEND_TO_PEND_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pket_body_byte(&mut self) -> REG_PKET_BODY_BYTE_W<16> {
        REG_PKET_BODY_BYTE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mjpeg_paket_ctrl to value 0"]
impl crate::Resettable for MJPEG_PAKET_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
