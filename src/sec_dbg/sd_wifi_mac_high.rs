#[doc = "Register `sd_wifi_mac_high` reader"]
pub struct R(crate::R<SD_WIFI_MAC_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_WIFI_MAC_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_WIFI_MAC_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_WIFI_MAC_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_wifi_mac_high` writer"]
pub struct W(crate::W<SD_WIFI_MAC_HIGH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_WIFI_MAC_HIGH_SPEC>;
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
impl From<crate::W<SD_WIFI_MAC_HIGH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_WIFI_MAC_HIGH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sd_wifi_mac_high` reader - "]
pub struct SD_WIFI_MAC_HIGH_R(crate::FieldReader<u32, u32>);
impl SD_WIFI_MAC_HIGH_R {
    pub(crate) fn new(bits: u32) -> Self {
        SD_WIFI_MAC_HIGH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_WIFI_MAC_HIGH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sd_wifi_mac_high` writer - "]
pub struct SD_WIFI_MAC_HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_WIFI_MAC_HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_wifi_mac_high(&self) -> SD_WIFI_MAC_HIGH_R {
        SD_WIFI_MAC_HIGH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sd_wifi_mac_high(&mut self) -> SD_WIFI_MAC_HIGH_W {
        SD_WIFI_MAC_HIGH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sd_wifi_mac_high.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_wifi_mac_high](index.html) module"]
pub struct SD_WIFI_MAC_HIGH_SPEC;
impl crate::RegisterSpec for SD_WIFI_MAC_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_wifi_mac_high::R](R) reader structure"]
impl crate::Readable for SD_WIFI_MAC_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_wifi_mac_high::W](W) writer structure"]
impl crate::Writable for SD_WIFI_MAC_HIGH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sd_wifi_mac_high to value 0"]
impl crate::Resettable for SD_WIFI_MAC_HIGH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
