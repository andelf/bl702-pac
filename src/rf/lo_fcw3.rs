#[doc = "Register `lo_fcw3` reader"]
pub struct R(crate::R<LO_FCW3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_FCW3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_FCW3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_FCW3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_fcw3` writer"]
pub struct W(crate::W<LO_FCW3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_FCW3_SPEC>;
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
impl From<crate::W<LO_FCW3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_FCW3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_freq_mod_hp` reader - "]
pub struct TX_FREQ_MOD_HP_R(crate::FieldReader<u16, u16>);
impl TX_FREQ_MOD_HP_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_FREQ_MOD_HP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FREQ_MOD_HP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_freq_mod_hp` writer - "]
pub struct TX_FREQ_MOD_HP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FREQ_MOD_HP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `tx_freq_mod_lp` reader - "]
pub struct TX_FREQ_MOD_LP_R(crate::FieldReader<u16, u16>);
impl TX_FREQ_MOD_LP_R {
    pub(crate) fn new(bits: u16) -> Self {
        TX_FREQ_MOD_LP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FREQ_MOD_LP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_freq_mod_lp` writer - "]
pub struct TX_FREQ_MOD_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FREQ_MOD_LP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tx_freq_mod_hp(&self) -> TX_FREQ_MOD_HP_R {
        TX_FREQ_MOD_HP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tx_freq_mod_lp(&self) -> TX_FREQ_MOD_LP_R {
        TX_FREQ_MOD_LP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn tx_freq_mod_hp(&mut self) -> TX_FREQ_MOD_HP_W {
        TX_FREQ_MOD_HP_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn tx_freq_mod_lp(&mut self) -> TX_FREQ_MOD_LP_W {
        TX_FREQ_MOD_LP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_fcw3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_fcw3](index.html) module"]
pub struct LO_FCW3_SPEC;
impl crate::RegisterSpec for LO_FCW3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_fcw3::R](R) reader structure"]
impl crate::Readable for LO_FCW3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_fcw3::W](W) writer structure"]
impl crate::Writable for LO_FCW3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_fcw3 to value 0"]
impl crate::Resettable for LO_FCW3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
