#[doc = "Register `TX_BD_NUM` reader"]
pub struct R(crate::R<TX_BD_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_BD_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_BD_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_BD_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_BD_NUM` writer"]
pub struct W(crate::W<TX_BD_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_BD_NUM_SPEC>;
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
impl From<crate::W<TX_BD_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_BD_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXBDPTR` reader - "]
pub struct RXBDPTR_R(crate::FieldReader<u8, u8>);
impl RXBDPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXBDPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBDPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBDPTR` writer - "]
pub struct RXBDPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBDPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
#[doc = "Field `TXBDPTR` reader - "]
pub struct TXBDPTR_R(crate::FieldReader<u8, u8>);
impl TXBDPTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXBDPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBDPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBDPTR` writer - "]
pub struct TXBDPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBDPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `TXBDNUM` reader - "]
pub struct TXBDNUM_R(crate::FieldReader<u8, u8>);
impl TXBDNUM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXBDNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBDNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBDNUM` writer - "]
pub struct TXBDNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBDNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn rxbdptr(&self) -> RXBDPTR_R {
        RXBDPTR_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn txbdptr(&self) -> TXBDPTR_R {
        TXBDPTR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txbdnum(&self) -> TXBDNUM_R {
        TXBDNUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn rxbdptr(&mut self) -> RXBDPTR_W {
        RXBDPTR_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn txbdptr(&mut self) -> TXBDPTR_W {
        TXBDPTR_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txbdnum(&mut self) -> TXBDNUM_W {
        TXBDNUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX_BD_NUM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_bd_num](index.html) module"]
pub struct TX_BD_NUM_SPEC;
impl crate::RegisterSpec for TX_BD_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_bd_num::R](R) reader structure"]
impl crate::Readable for TX_BD_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_bd_num::W](W) writer structure"]
impl crate::Writable for TX_BD_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX_BD_NUM to value 0"]
impl crate::Resettable for TX_BD_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
