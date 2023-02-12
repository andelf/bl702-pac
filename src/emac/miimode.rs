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
#[doc = "Field `CLKDIV` reader - "]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - "]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIIMODE_SPEC, u8, u8, 8, O>;
#[doc = "Field `MIINOPRE` reader - "]
pub type MIINOPRE_R = crate::BitReader<bool>;
#[doc = "Field `MIINOPRE` writer - "]
pub type MIINOPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIIMODE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn miinopre(&self) -> MIINOPRE_R {
        MIINOPRE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn miinopre(&mut self) -> MIINOPRE_W<8> {
        MIINOPRE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIIMODE to value 0"]
impl crate::Resettable for MIIMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
