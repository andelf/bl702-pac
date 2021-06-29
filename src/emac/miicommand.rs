#[doc = "Register `MIICOMMAND` reader"]
pub struct R(crate::R<MIICOMMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIICOMMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIICOMMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIICOMMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIICOMMAND` writer"]
pub struct W(crate::W<MIICOMMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIICOMMAND_SPEC>;
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
impl From<crate::W<MIICOMMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIICOMMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WCTRLDATA` reader - "]
pub struct WCTRLDATA_R(crate::FieldReader<bool, bool>);
impl WCTRLDATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCTRLDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCTRLDATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCTRLDATA` writer - "]
pub struct WCTRLDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WCTRLDATA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RSTAT` reader - "]
pub struct RSTAT_R(crate::FieldReader<bool, bool>);
impl RSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTAT` writer - "]
pub struct RSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTAT_W<'a> {
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
#[doc = "Field `SCANSTAT` reader - "]
pub struct SCANSTAT_R(crate::FieldReader<bool, bool>);
impl SCANSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCANSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCANSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCANSTAT` writer - "]
pub struct SCANSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANSTAT_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wctrldata(&self) -> WCTRLDATA_R {
        WCTRLDATA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstat(&self) -> RSTAT_R {
        RSTAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scanstat(&self) -> SCANSTAT_R {
        SCANSTAT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wctrldata(&mut self) -> WCTRLDATA_W {
        WCTRLDATA_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstat(&mut self) -> RSTAT_W {
        RSTAT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scanstat(&mut self) -> SCANSTAT_W {
        SCANSTAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIICOMMAND.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miicommand](index.html) module"]
pub struct MIICOMMAND_SPEC;
impl crate::RegisterSpec for MIICOMMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miicommand::R](R) reader structure"]
impl crate::Readable for MIICOMMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miicommand::W](W) writer structure"]
impl crate::Writable for MIICOMMAND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIICOMMAND to value 0"]
impl crate::Resettable for MIICOMMAND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
