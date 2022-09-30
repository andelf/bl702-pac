#[doc = "Register `MIIADDRESS` reader"]
pub struct R(crate::R<MIIADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIIADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIIADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIIADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIIADDRESS` writer"]
pub struct W(crate::W<MIIADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIIADDRESS_SPEC>;
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
impl From<crate::W<MIIADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIIADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIAD` reader - "]
pub type FIAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIAD` writer - "]
pub type FIAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIIADDRESS_SPEC, u8, u8, 5, O>;
#[doc = "Field `RGAD` reader - "]
pub type RGAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RGAD` writer - "]
pub type RGAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIIADDRESS_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn fiad(&self) -> FIAD_R {
        FIAD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rgad(&self) -> RGAD_R {
        RGAD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn fiad(&mut self) -> FIAD_W<0> {
        FIAD_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn rgad(&mut self) -> RGAD_W<8> {
        RGAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIIADDRESS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miiaddress](index.html) module"]
pub struct MIIADDRESS_SPEC;
impl crate::RegisterSpec for MIIADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miiaddress::R](R) reader structure"]
impl crate::Readable for MIIADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miiaddress::W](W) writer structure"]
impl crate::Writable for MIIADDRESS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIIADDRESS to value 0"]
impl crate::Resettable for MIIADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
