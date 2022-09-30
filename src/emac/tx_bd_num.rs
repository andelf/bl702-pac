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
#[doc = "Field `TXBDNUM` reader - "]
pub type TXBDNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXBDNUM` writer - "]
pub type TXBDNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_BD_NUM_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXBDPTR` reader - "]
pub type TXBDPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXBDPTR` writer - "]
pub type TXBDPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_BD_NUM_SPEC, u8, u8, 7, O>;
#[doc = "Field `RXBDPTR` reader - "]
pub type RXBDPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXBDPTR` writer - "]
pub type RXBDPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_BD_NUM_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txbdnum(&self) -> TXBDNUM_R {
        TXBDNUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn txbdptr(&self) -> TXBDPTR_R {
        TXBDPTR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn rxbdptr(&self) -> RXBDPTR_R {
        RXBDPTR_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txbdnum(&mut self) -> TXBDNUM_W<0> {
        TXBDNUM_W::new(self)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn txbdptr(&mut self) -> TXBDPTR_W<16> {
        TXBDPTR_W::new(self)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn rxbdptr(&mut self) -> RXBDPTR_W<24> {
        RXBDPTR_W::new(self)
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
