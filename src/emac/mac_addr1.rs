#[doc = "Register `MAC_ADDR1` reader"]
pub struct R(crate::R<MAC_ADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_ADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_ADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_ADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_ADDR1` writer"]
pub struct W(crate::W<MAC_ADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_ADDR1_SPEC>;
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
impl From<crate::W<MAC_ADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_ADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAC_B0` reader - "]
pub struct MAC_B0_R(crate::FieldReader<u8, u8>);
impl MAC_B0_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAC_B0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_B0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_B0` writer - "]
pub struct MAC_B0_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_B0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `MAC_B1` reader - "]
pub struct MAC_B1_R(crate::FieldReader<u8, u8>);
impl MAC_B1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MAC_B1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAC_B1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAC_B1` writer - "]
pub struct MAC_B1_W<'a> {
    w: &'a mut W,
}
impl<'a> MAC_B1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mac_b0(&self) -> MAC_B0_R {
        MAC_B0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mac_b1(&self) -> MAC_B1_R {
        MAC_B1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn mac_b0(&mut self) -> MAC_B0_W {
        MAC_B0_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn mac_b1(&mut self) -> MAC_B1_W {
        MAC_B1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC_ADDR1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_addr1](index.html) module"]
pub struct MAC_ADDR1_SPEC;
impl crate::RegisterSpec for MAC_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_addr1::R](R) reader structure"]
impl crate::Readable for MAC_ADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_addr1::W](W) writer structure"]
impl crate::Writable for MAC_ADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_ADDR1 to value 0"]
impl crate::Resettable for MAC_ADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
