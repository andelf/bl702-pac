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
#[doc = "Field `lodist_txbuf_supply_mode` reader - "]
pub type LODIST_TXBUF_SUPPLY_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lodist_txbuf_supply_mode` writer - "]
pub type LODIST_TXBUF_SUPPLY_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LODIST_SPEC, u8, u8, 2, O>;
#[doc = "Field `lodist_rxbuf_supply_mode` reader - "]
pub type LODIST_RXBUF_SUPPLY_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lodist_rxbuf_supply_mode` writer - "]
pub type LODIST_RXBUF_SUPPLY_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LODIST_SPEC, u8, u8, 2, O>;
#[doc = "Field `lodist_rxbuf_supply_boost` reader - "]
pub type LODIST_RXBUF_SUPPLY_BOOST_R = crate::BitReader<bool>;
#[doc = "Field `lodist_rxbuf_supply_boost` writer - "]
pub type LODIST_RXBUF_SUPPLY_BOOST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LODIST_SPEC, bool, O>;
#[doc = "Field `lodist_rwell_bias` reader - "]
pub type LODIST_RWELL_BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lodist_rwell_bias` writer - "]
pub type LODIST_RWELL_BIAS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LODIST_SPEC, u8, u8, 2, O>;
#[doc = "Field `lodist_nwell_bias` reader - "]
pub type LODIST_NWELL_BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lodist_nwell_bias` writer - "]
pub type LODIST_NWELL_BIAS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LODIST_SPEC, u8, u8, 2, O>;
#[doc = "Field `lodist_75dc_sel` reader - "]
pub type LODIST_75DC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `lodist_75dc_sel` writer - "]
pub type LODIST_75DC_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LODIST_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lodist_txbuf_supply_mode(&self) -> LODIST_TXBUF_SUPPLY_MODE_R {
        LODIST_TXBUF_SUPPLY_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lodist_rxbuf_supply_mode(&self) -> LODIST_RXBUF_SUPPLY_MODE_R {
        LODIST_RXBUF_SUPPLY_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lodist_rxbuf_supply_boost(&self) -> LODIST_RXBUF_SUPPLY_BOOST_R {
        LODIST_RXBUF_SUPPLY_BOOST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lodist_rwell_bias(&self) -> LODIST_RWELL_BIAS_R {
        LODIST_RWELL_BIAS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lodist_nwell_bias(&self) -> LODIST_NWELL_BIAS_R {
        LODIST_NWELL_BIAS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lodist_75dc_sel(&self) -> LODIST_75DC_SEL_R {
        LODIST_75DC_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn lodist_txbuf_supply_mode(&mut self) -> LODIST_TXBUF_SUPPLY_MODE_W<0> {
        LODIST_TXBUF_SUPPLY_MODE_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn lodist_rxbuf_supply_mode(&mut self) -> LODIST_RXBUF_SUPPLY_MODE_W<4> {
        LODIST_RXBUF_SUPPLY_MODE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn lodist_rxbuf_supply_boost(&mut self) -> LODIST_RXBUF_SUPPLY_BOOST_W<6> {
        LODIST_RXBUF_SUPPLY_BOOST_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn lodist_rwell_bias(&mut self) -> LODIST_RWELL_BIAS_W<8> {
        LODIST_RWELL_BIAS_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn lodist_nwell_bias(&mut self) -> LODIST_NWELL_BIAS_W<12> {
        LODIST_NWELL_BIAS_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn lodist_75dc_sel(&mut self) -> LODIST_75DC_SEL_W<16> {
        LODIST_75DC_SEL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets lodist to value 0"]
impl crate::Resettable for LODIST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
