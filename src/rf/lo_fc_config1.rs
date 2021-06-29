#[doc = "Register `lo_fc_config1` reader"]
pub struct R(crate::R<LO_FC_CONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_FC_CONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_FC_CONFIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_FC_CONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_fc_config1` writer"]
pub struct W(crate::W<LO_FC_CONFIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_FC_CONFIG1_SPEC>;
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
impl From<crate::W<LO_FC_CONFIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_FC_CONFIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_fcw` reader - "]
pub struct LO_FCW_R(crate::FieldReader<u32, u32>);
impl LO_FCW_R {
    pub(crate) fn new(bits: u32) -> Self {
        LO_FCW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_FCW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_fcw` writer - "]
pub struct LO_FCW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FCW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | (value as u32 & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn lo_fcw(&self) -> LO_FCW_R {
        LO_FCW_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn lo_fcw(&mut self) -> LO_FCW_W {
        LO_FCW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_fc_config1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_fc_config1](index.html) module"]
pub struct LO_FC_CONFIG1_SPEC;
impl crate::RegisterSpec for LO_FC_CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_fc_config1::R](R) reader structure"]
impl crate::Readable for LO_FC_CONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_fc_config1::W](W) writer structure"]
impl crate::Writable for LO_FC_CONFIG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_fc_config1 to value 0"]
impl crate::Resettable for LO_FC_CONFIG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
