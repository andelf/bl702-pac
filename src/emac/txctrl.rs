#[doc = "Register `TXCTRL` reader"]
pub struct R(crate::R<TXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCTRL` writer"]
pub struct W(crate::W<TXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCTRL_SPEC>;
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
impl From<crate::W<TXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPAUSERQ` reader - "]
pub struct TXPAUSERQ_R(crate::FieldReader<bool, bool>);
impl TXPAUSERQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXPAUSERQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPAUSERQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPAUSERQ` writer - "]
pub struct TXPAUSERQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPAUSERQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TXPAUSETV` reader - "]
pub struct TXPAUSETV_R(crate::FieldReader<u16, u16>);
impl TXPAUSETV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TXPAUSETV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPAUSETV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPAUSETV` writer - "]
pub struct TXPAUSETV_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPAUSETV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn txpauserq(&self) -> TXPAUSERQ_R {
        TXPAUSERQ_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn txpausetv(&self) -> TXPAUSETV_R {
        TXPAUSETV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn txpauserq(&mut self) -> TXPAUSERQ_W {
        TXPAUSERQ_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn txpausetv(&mut self) -> TXPAUSETV_W {
        TXPAUSETV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXCTRL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctrl](index.html) module"]
pub struct TXCTRL_SPEC;
impl crate::RegisterSpec for TXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txctrl::R](R) reader structure"]
impl crate::Readable for TXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txctrl::W](W) writer structure"]
impl crate::Writable for TXCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCTRL to value 0"]
impl crate::Resettable for TXCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
