#[doc = "Register `keycode_value` reader"]
pub struct R(crate::R<KEYCODE_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYCODE_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYCODE_VALUE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYCODE_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `keycode_value` writer"]
pub struct W(crate::W<KEYCODE_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYCODE_VALUE_SPEC>;
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
impl From<crate::W<KEYCODE_VALUE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYCODE_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `keycode0` reader - "]
pub type KEYCODE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `keycode0` writer - "]
pub type KEYCODE0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEYCODE_VALUE_SPEC, u8, u8, 8, O>;
#[doc = "Field `keycode1` reader - "]
pub type KEYCODE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `keycode1` writer - "]
pub type KEYCODE1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEYCODE_VALUE_SPEC, u8, u8, 8, O>;
#[doc = "Field `keycode2` reader - "]
pub type KEYCODE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `keycode2` writer - "]
pub type KEYCODE2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEYCODE_VALUE_SPEC, u8, u8, 8, O>;
#[doc = "Field `keycode3` reader - "]
pub type KEYCODE3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `keycode3` writer - "]
pub type KEYCODE3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEYCODE_VALUE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn keycode0(&self) -> KEYCODE0_R {
        KEYCODE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn keycode1(&self) -> KEYCODE1_R {
        KEYCODE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn keycode2(&self) -> KEYCODE2_R {
        KEYCODE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn keycode3(&self) -> KEYCODE3_R {
        KEYCODE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn keycode0(&mut self) -> KEYCODE0_W<0> {
        KEYCODE0_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn keycode1(&mut self) -> KEYCODE1_W<8> {
        KEYCODE1_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn keycode2(&mut self) -> KEYCODE2_W<16> {
        KEYCODE2_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn keycode3(&mut self) -> KEYCODE3_W<24> {
        KEYCODE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "keycode_value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keycode_value](index.html) module"]
pub struct KEYCODE_VALUE_SPEC;
impl crate::RegisterSpec for KEYCODE_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keycode_value::R](R) reader structure"]
impl crate::Readable for KEYCODE_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keycode_value::W](W) writer structure"]
impl crate::Writable for KEYCODE_VALUE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets keycode_value to value 0"]
impl crate::Resettable for KEYCODE_VALUE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
