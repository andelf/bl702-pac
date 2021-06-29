#[doc = "Register `qdec_value` reader"]
pub struct R(crate::R<QDEC_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDEC_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDEC_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDEC_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `qdec_value` writer"]
pub struct W(crate::W<QDEC_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDEC_VALUE_SPEC>;
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
impl From<crate::W<QDEC_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDEC_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spl_val` reader - "]
pub struct SPL_VAL_R(crate::FieldReader<u8, u8>);
impl SPL_VAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPL_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPL_VAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spl_val` writer - "]
pub struct SPL_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPL_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `acc2_val` reader - "]
pub struct ACC2_VAL_R(crate::FieldReader<u8, u8>);
impl ACC2_VAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACC2_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACC2_VAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acc2_val` writer - "]
pub struct ACC2_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC2_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `acc1_val` reader - "]
pub struct ACC1_VAL_R(crate::FieldReader<u16, u16>);
impl ACC1_VAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        ACC1_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACC1_VAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acc1_val` writer - "]
pub struct ACC1_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACC1_VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn spl_val(&self) -> SPL_VAL_R {
        SPL_VAL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn acc2_val(&self) -> ACC2_VAL_R {
        ACC2_VAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn acc1_val(&self) -> ACC1_VAL_R {
        ACC1_VAL_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn spl_val(&mut self) -> SPL_VAL_W {
        SPL_VAL_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn acc2_val(&mut self) -> ACC2_VAL_W {
        ACC2_VAL_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn acc1_val(&mut self) -> ACC1_VAL_W {
        ACC1_VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "qdec_value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdec_value](index.html) module"]
pub struct QDEC_VALUE_SPEC;
impl crate::RegisterSpec for QDEC_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdec_value::R](R) reader structure"]
impl crate::Readable for QDEC_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdec_value::W](W) writer structure"]
impl crate::Writable for QDEC_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets qdec_value to value 0"]
impl crate::Resettable for QDEC_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
