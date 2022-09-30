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
#[doc = "Field `acc1_val` reader - "]
pub type ACC1_VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `acc1_val` writer - "]
pub type ACC1_VAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QDEC_VALUE_SPEC, u16, u16, 11, O>;
#[doc = "Field `acc2_val` reader - "]
pub type ACC2_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acc2_val` writer - "]
pub type ACC2_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QDEC_VALUE_SPEC, u8, u8, 4, O>;
#[doc = "Field `spl_val` reader - "]
pub type SPL_VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spl_val` writer - "]
pub type SPL_VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QDEC_VALUE_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn acc1_val(&self) -> ACC1_VAL_R {
        ACC1_VAL_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn acc2_val(&self) -> ACC2_VAL_R {
        ACC2_VAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn spl_val(&self) -> SPL_VAL_R {
        SPL_VAL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn acc1_val(&mut self) -> ACC1_VAL_W<0> {
        ACC1_VAL_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn acc2_val(&mut self) -> ACC2_VAL_W<16> {
        ACC2_VAL_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn spl_val(&mut self) -> SPL_VAL_W<28> {
        SPL_VAL_W::new(self)
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
