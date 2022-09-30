#[doc = "Register `INT_SOURCE` reader"]
pub struct R(crate::R<INT_SOURCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_SOURCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_SOURCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_SOURCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_SOURCE` writer"]
pub struct W(crate::W<INT_SOURCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_SOURCE_SPEC>;
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
impl From<crate::W<INT_SOURCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_SOURCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXB` reader - "]
pub type TXB_R = crate::BitReader<bool>;
#[doc = "Field `TXB` writer - "]
pub type TXB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SOURCE_SPEC, bool, O>;
#[doc = "Field `TXE` reader - "]
pub type TXE_R = crate::BitReader<bool>;
#[doc = "Field `TXE` writer - "]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SOURCE_SPEC, bool, O>;
#[doc = "Field `RXB` reader - "]
pub type RXB_R = crate::BitReader<bool>;
#[doc = "Field `RXB` writer - "]
pub type RXB_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SOURCE_SPEC, bool, O>;
#[doc = "Field `RXE` reader - "]
pub type RXE_R = crate::BitReader<bool>;
#[doc = "Field `RXE` writer - "]
pub type RXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SOURCE_SPEC, bool, O>;
#[doc = "Field `BUSY` reader - "]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` writer - "]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SOURCE_SPEC, bool, O>;
#[doc = "Field `TXC` reader - "]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - "]
pub type TXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SOURCE_SPEC, bool, O>;
#[doc = "Field `RXC` reader - "]
pub type RXC_R = crate::BitReader<bool>;
#[doc = "Field `RXC` writer - "]
pub type RXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_SOURCE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txb(&self) -> TXB_R {
        TXB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxb(&self) -> RXB_R {
        RXB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxc(&self) -> RXC_R {
        RXC_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txb(&mut self) -> TXB_W<0> {
        TXB_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W<1> {
        TXE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxb(&mut self) -> RXB_W<2> {
        RXB_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxe(&mut self) -> RXE_W<3> {
        RXE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W<4> {
        BUSY_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W<5> {
        TXC_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxc(&mut self) -> RXC_W<6> {
        RXC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INT_SOURCE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_source](index.html) module"]
pub struct INT_SOURCE_SPEC;
impl crate::RegisterSpec for INT_SOURCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_source::R](R) reader structure"]
impl crate::Readable for INT_SOURCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_source::W](W) writer structure"]
impl crate::Writable for INT_SOURCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_SOURCE to value 0"]
impl crate::Resettable for INT_SOURCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
