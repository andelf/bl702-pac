#[doc = "Register `rf_ext_pa` reader"]
pub struct R(crate::R<RF_EXT_PA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_EXT_PA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_EXT_PA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_EXT_PA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_ext_pa` writer"]
pub struct W(crate::W<RF_EXT_PA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_EXT_PA_SPEC>;
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
impl From<crate::W<RF_EXT_PA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_EXT_PA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_ext_pa_rx` reader - "]
pub struct RF_EXT_PA_RX_R(crate::FieldReader<u8, u8>);
impl RF_EXT_PA_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_EXT_PA_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_EXT_PA_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ext_pa_rx` writer - "]
pub struct RF_EXT_PA_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_EXT_PA_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `rf_ext_pa_lorx` reader - "]
pub struct RF_EXT_PA_LORX_R(crate::FieldReader<u8, u8>);
impl RF_EXT_PA_LORX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_EXT_PA_LORX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_EXT_PA_LORX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ext_pa_lorx` writer - "]
pub struct RF_EXT_PA_LORX_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_EXT_PA_LORX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `rf_ext_pa_tx` reader - "]
pub struct RF_EXT_PA_TX_R(crate::FieldReader<u8, u8>);
impl RF_EXT_PA_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_EXT_PA_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_EXT_PA_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ext_pa_tx` writer - "]
pub struct RF_EXT_PA_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_EXT_PA_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `rf_ext_pa_lotx` reader - "]
pub struct RF_EXT_PA_LOTX_R(crate::FieldReader<u8, u8>);
impl RF_EXT_PA_LOTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_EXT_PA_LOTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_EXT_PA_LOTX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ext_pa_lotx` writer - "]
pub struct RF_EXT_PA_LOTX_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_EXT_PA_LOTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `rf_ext_pa_sb` reader - "]
pub struct RF_EXT_PA_SB_R(crate::FieldReader<u8, u8>);
impl RF_EXT_PA_SB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RF_EXT_PA_SB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_EXT_PA_SB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ext_pa_sb` writer - "]
pub struct RF_EXT_PA_SB_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_EXT_PA_SB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn rf_ext_pa_rx(&self) -> RF_EXT_PA_RX_R {
        RF_EXT_PA_RX_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn rf_ext_pa_lorx(&self) -> RF_EXT_PA_LORX_R {
        RF_EXT_PA_LORX_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn rf_ext_pa_tx(&self) -> RF_EXT_PA_TX_R {
        RF_EXT_PA_TX_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn rf_ext_pa_lotx(&self) -> RF_EXT_PA_LOTX_R {
        RF_EXT_PA_LOTX_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ext_pa_sb(&self) -> RF_EXT_PA_SB_R {
        RF_EXT_PA_SB_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn rf_ext_pa_rx(&mut self) -> RF_EXT_PA_RX_W {
        RF_EXT_PA_RX_W { w: self }
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn rf_ext_pa_lorx(&mut self) -> RF_EXT_PA_LORX_W {
        RF_EXT_PA_LORX_W { w: self }
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn rf_ext_pa_tx(&mut self) -> RF_EXT_PA_TX_W {
        RF_EXT_PA_TX_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn rf_ext_pa_lotx(&mut self) -> RF_EXT_PA_LOTX_W {
        RF_EXT_PA_LOTX_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_ext_pa_sb(&mut self) -> RF_EXT_PA_SB_W {
        RF_EXT_PA_SB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_ext_pa.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ext_pa](index.html) module"]
pub struct RF_EXT_PA_SPEC;
impl crate::RegisterSpec for RF_EXT_PA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ext_pa::R](R) reader structure"]
impl crate::Readable for RF_EXT_PA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ext_pa::W](W) writer structure"]
impl crate::Writable for RF_EXT_PA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_ext_pa to value 0"]
impl crate::Resettable for RF_EXT_PA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
