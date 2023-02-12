#[doc = "Register `l1c_misc` reader"]
pub struct R(crate::R<L1C_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1C_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1C_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1C_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `l1c_misc` writer"]
pub struct W(crate::W<L1C_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1C_MISC_SPEC>;
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
impl From<crate::W<L1C_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1C_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `l1c_fsm` reader - "]
pub type L1C_FSM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `l1c_fsm` writer - "]
pub type L1C_FSM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L1C_MISC_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn l1c_fsm(&self) -> L1C_FSM_R {
        L1C_FSM_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    #[must_use]
    pub fn l1c_fsm(&mut self) -> L1C_FSM_W<28> {
        L1C_FSM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "l1c_misc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_misc](index.html) module"]
pub struct L1C_MISC_SPEC;
impl crate::RegisterSpec for L1C_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1c_misc::R](R) reader structure"]
impl crate::Readable for L1C_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1c_misc::W](W) writer structure"]
impl crate::Writable for L1C_MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets l1c_misc to value 0"]
impl crate::Resettable for L1C_MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
