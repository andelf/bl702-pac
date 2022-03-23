#[doc = "Register `lodist` reader"]
pub struct R(crate::R<LODIST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LODIST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LODIST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LODIST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lodist` writer"]
pub struct W(crate::W<LODIST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LODIST_SPEC>;
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
impl From<crate::W<LODIST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LODIST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lodist_75dc_sel` reader - "]
pub struct LODIST_75DC_SEL_R(crate::FieldReader<bool, bool>);
impl LODIST_75DC_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LODIST_75DC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIST_75DC_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lodist_75dc_sel` writer - "]
pub struct LODIST_75DC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIST_75DC_SEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `lodist_nwell_bias` reader - "]
pub struct LODIST_NWELL_BIAS_R(crate::FieldReader<u8, u8>);
impl LODIST_NWELL_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LODIST_NWELL_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIST_NWELL_BIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lodist_nwell_bias` writer - "]
pub struct LODIST_NWELL_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIST_NWELL_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `lodist_rwell_bias` reader - "]
pub struct LODIST_RWELL_BIAS_R(crate::FieldReader<u8, u8>);
impl LODIST_RWELL_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LODIST_RWELL_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIST_RWELL_BIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lodist_rwell_bias` writer - "]
pub struct LODIST_RWELL_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIST_RWELL_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `lodist_rxbuf_supply_boost` reader - "]
pub struct LODIST_RXBUF_SUPPLY_BOOST_R(crate::FieldReader<bool, bool>);
impl LODIST_RXBUF_SUPPLY_BOOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LODIST_RXBUF_SUPPLY_BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIST_RXBUF_SUPPLY_BOOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lodist_rxbuf_supply_boost` writer - "]
pub struct LODIST_RXBUF_SUPPLY_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIST_RXBUF_SUPPLY_BOOST_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `lodist_rxbuf_supply_mode` reader - "]
pub struct LODIST_RXBUF_SUPPLY_MODE_R(crate::FieldReader<u8, u8>);
impl LODIST_RXBUF_SUPPLY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LODIST_RXBUF_SUPPLY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIST_RXBUF_SUPPLY_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lodist_rxbuf_supply_mode` writer - "]
pub struct LODIST_RXBUF_SUPPLY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIST_RXBUF_SUPPLY_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `lodist_txbuf_supply_mode` reader - "]
pub struct LODIST_TXBUF_SUPPLY_MODE_R(crate::FieldReader<u8, u8>);
impl LODIST_TXBUF_SUPPLY_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LODIST_TXBUF_SUPPLY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIST_TXBUF_SUPPLY_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lodist_txbuf_supply_mode` writer - "]
pub struct LODIST_TXBUF_SUPPLY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIST_TXBUF_SUPPLY_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lodist_75dc_sel(&self) -> LODIST_75DC_SEL_R {
        LODIST_75DC_SEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lodist_nwell_bias(&self) -> LODIST_NWELL_BIAS_R {
        LODIST_NWELL_BIAS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lodist_rwell_bias(&self) -> LODIST_RWELL_BIAS_R {
        LODIST_RWELL_BIAS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lodist_rxbuf_supply_boost(&self) -> LODIST_RXBUF_SUPPLY_BOOST_R {
        LODIST_RXBUF_SUPPLY_BOOST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lodist_rxbuf_supply_mode(&self) -> LODIST_RXBUF_SUPPLY_MODE_R {
        LODIST_RXBUF_SUPPLY_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lodist_txbuf_supply_mode(&self) -> LODIST_TXBUF_SUPPLY_MODE_R {
        LODIST_TXBUF_SUPPLY_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lodist_75dc_sel(&mut self) -> LODIST_75DC_SEL_W {
        LODIST_75DC_SEL_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lodist_nwell_bias(&mut self) -> LODIST_NWELL_BIAS_W {
        LODIST_NWELL_BIAS_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lodist_rwell_bias(&mut self) -> LODIST_RWELL_BIAS_W {
        LODIST_RWELL_BIAS_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lodist_rxbuf_supply_boost(&mut self) -> LODIST_RXBUF_SUPPLY_BOOST_W {
        LODIST_RXBUF_SUPPLY_BOOST_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lodist_rxbuf_supply_mode(&mut self) -> LODIST_RXBUF_SUPPLY_MODE_W {
        LODIST_RXBUF_SUPPLY_MODE_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lodist_txbuf_supply_mode(&mut self) -> LODIST_TXBUF_SUPPLY_MODE_W {
        LODIST_TXBUF_SUPPLY_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lodist.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lodist](index.html) module"]
pub struct LODIST_SPEC;
impl crate::RegisterSpec for LODIST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lodist::R](R) reader structure"]
impl crate::Readable for LODIST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lodist::W](W) writer structure"]
impl crate::Writable for LODIST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lodist to value 0"]
impl crate::Resettable for LODIST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
