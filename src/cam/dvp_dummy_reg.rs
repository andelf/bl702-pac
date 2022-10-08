#[doc = "Register `dvp_dummy_reg` reader"]
pub struct R(crate::R<DVP_DUMMY_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_DUMMY_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_DUMMY_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_DUMMY_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dvp_dummy_reg` writer"]
pub struct W(crate::W<DVP_DUMMY_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP_DUMMY_REG_SPEC>;
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
impl From<crate::W<DVP_DUMMY_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP_DUMMY_REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED_31_0` reader - "]
pub type RESERVED_31_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED_31_0` writer - "]
pub type RESERVED_31_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DVP_DUMMY_REG_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reserved_31_0(&self) -> RESERVED_31_0_R {
        RESERVED_31_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reserved_31_0(&mut self) -> RESERVED_31_0_W<0> {
        RESERVED_31_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dvp_dummy_reg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_dummy_reg](index.html) module"]
pub struct DVP_DUMMY_REG_SPEC;
impl crate::RegisterSpec for DVP_DUMMY_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp_dummy_reg::R](R) reader structure"]
impl crate::Readable for DVP_DUMMY_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp_dummy_reg::W](W) writer structure"]
impl crate::Writable for DVP_DUMMY_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dvp_dummy_reg to value 0"]
impl crate::Resettable for DVP_DUMMY_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
