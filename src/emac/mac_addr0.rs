#[doc = "Register `MAC_ADDR0` reader"]
pub struct R(crate::R<MAC_ADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR0` writer"]
pub struct W(crate::W<MAC_ADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR0_SPEC>;
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
impl From<crate::W<MAC_ADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_B5` reader - "]
pub type MAC_B5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAC_B5` writer - "]
pub type MAC_B5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_ADDR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `MAC_B4` reader - "]
pub type MAC_B4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAC_B4` writer - "]
pub type MAC_B4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_ADDR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `MAC_B3` reader - "]
pub type MAC_B3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAC_B3` writer - "]
pub type MAC_B3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_ADDR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `MAC_B2` reader - "]
pub type MAC_B2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAC_B2` writer - "]
pub type MAC_B2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAC_ADDR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mac_b5(&self) -> MAC_B5_R {
        MAC_B5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mac_b4(&self) -> MAC_B4_R {
        MAC_B4_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn mac_b3(&self) -> MAC_B3_R {
        MAC_B3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn mac_b2(&self) -> MAC_B2_R {
        MAC_B2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn mac_b5(&mut self) -> MAC_B5_W<0> {
        MAC_B5_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn mac_b4(&mut self) -> MAC_B4_W<8> {
        MAC_B4_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn mac_b3(&mut self) -> MAC_B3_W<16> {
        MAC_B3_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn mac_b2(&mut self) -> MAC_B2_W<24> {
        MAC_B2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC_ADDR0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr0](index.html) module"]
pub struct MAC_ADDR0_SPEC;
impl crate::RegisterSpec for MAC_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr0::R](R) reader structure"]
impl crate::Readable for MAC_ADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr0::W](W) writer structure"]
impl crate::Writable for MAC_ADDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAC_ADDR0 to value 0"]
impl crate::Resettable for MAC_ADDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
