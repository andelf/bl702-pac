#[doc = "Register `MIISTATUS` reader"]
pub struct R(crate::R<MIISTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIISTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIISTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIISTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIISTATUS` writer"]
pub struct W(crate::W<MIISTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIISTATUS_SPEC>;
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
impl From<crate::W<MIISTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIISTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIIM_BUSY` reader - "]
pub struct MIIM_BUSY_R(crate::FieldReader<bool, bool>);
impl MIIM_BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MIIM_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIIM_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIIM_BUSY` writer - "]
pub struct MIIM_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> MIIM_BUSY_W<'a> {
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
#[doc = "Field `MIIM_LINKFAIL` reader - "]
pub struct MIIM_LINKFAIL_R(crate::FieldReader<bool, bool>);
impl MIIM_LINKFAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MIIM_LINKFAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIIM_LINKFAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIIM_LINKFAIL` writer - "]
pub struct MIIM_LINKFAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> MIIM_LINKFAIL_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn miim_busy(&self) -> MIIM_BUSY_R {
        MIIM_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn miim_linkfail(&self) -> MIIM_LINKFAIL_R {
        MIIM_LINKFAIL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn miim_busy(&mut self) -> MIIM_BUSY_W {
        MIIM_BUSY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn miim_linkfail(&mut self) -> MIIM_LINKFAIL_W {
        MIIM_LINKFAIL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIISTATUS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miistatus](index.html) module"]
pub struct MIISTATUS_SPEC;
impl crate::RegisterSpec for MIISTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miistatus::R](R) reader structure"]
impl crate::Readable for MIISTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miistatus::W](W) writer structure"]
impl crate::Writable for MIISTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIISTATUS to value 0"]
impl crate::Resettable for MIISTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
