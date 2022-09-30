#[doc = "Register `COLLCONFIG` reader"]
pub struct R(crate::R<COLLCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLLCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLLCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLLCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLLCONFIG` writer"]
pub struct W(crate::W<COLLCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLLCONFIG_SPEC>;
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
impl From<crate::W<COLLCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLLCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COLLVALID` reader - "]
pub type COLLVALID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COLLVALID` writer - "]
pub type COLLVALID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLLCONFIG_SPEC, u8, u8, 6, O>;
#[doc = "Field `MAXRET` reader - "]
pub type MAXRET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAXRET` writer - "]
pub type MAXRET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COLLCONFIG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn collvalid(&self) -> COLLVALID_R {
        COLLVALID_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn maxret(&self) -> MAXRET_R {
        MAXRET_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn collvalid(&mut self) -> COLLVALID_W<0> {
        COLLVALID_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn maxret(&mut self) -> MAXRET_W<16> {
        MAXRET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COLLCONFIG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [collconfig](index.html) module"]
pub struct COLLCONFIG_SPEC;
impl crate::RegisterSpec for COLLCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [collconfig::R](R) reader structure"]
impl crate::Readable for COLLCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [collconfig::W](W) writer structure"]
impl crate::Writable for COLLCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COLLCONFIG to value 0"]
impl crate::Resettable for COLLCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
