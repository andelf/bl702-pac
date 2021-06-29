#[doc = "Register `MIIMODE` reader"]
pub struct R(crate::R<MIIMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIIMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIIMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIIMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIIMODE` writer"]
pub struct W(crate::W<MIIMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIIMODE_SPEC>;
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
impl From<crate::W<MIIMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIIMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIINOPRE` reader - "]
pub struct MIINOPRE_R(crate::FieldReader<bool, bool>);
impl MIINOPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MIINOPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIINOPRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIINOPRE` writer - "]
pub struct MIINOPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MIINOPRE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CLKDIV` reader - "]
pub struct CLKDIV_R(crate::FieldReader<u8, u8>);
impl CLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - "]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn miinopre(&self) -> MIINOPRE_R {
        MIINOPRE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn miinopre(&mut self) -> MIINOPRE_W {
        MIINOPRE_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIIMODE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miimode](index.html) module"]
pub struct MIIMODE_SPEC;
impl crate::RegisterSpec for MIIMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miimode::R](R) reader structure"]
impl crate::Readable for MIIMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miimode::W](W) writer structure"]
impl crate::Writable for MIIMODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIIMODE to value 0"]
impl crate::Resettable for MIIMODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
