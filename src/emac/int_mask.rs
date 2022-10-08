#[doc = "Register `INT_MASK` reader"]
pub struct R(crate::R<INT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_MASK` writer"]
pub struct W(crate::W<INT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_MASK_SPEC>;
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
impl From<crate::W<INT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXB_M` reader - "]
pub type TXB_M_R = crate::BitReader<bool>;
#[doc = "Field `TXB_M` writer - "]
pub type TXB_M_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_MASK_SPEC, bool, O>;
#[doc = "Field `TXE_M` reader - "]
pub type TXE_M_R = crate::BitReader<bool>;
#[doc = "Field `TXE_M` writer - "]
pub type TXE_M_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_MASK_SPEC, bool, O>;
#[doc = "Field `RXB_M` reader - "]
pub type RXB_M_R = crate::BitReader<bool>;
#[doc = "Field `RXB_M` writer - "]
pub type RXB_M_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_MASK_SPEC, bool, O>;
#[doc = "Field `RXE_M` reader - "]
pub type RXE_M_R = crate::BitReader<bool>;
#[doc = "Field `RXE_M` writer - "]
pub type RXE_M_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_MASK_SPEC, bool, O>;
#[doc = "Field `BUSY_M` reader - "]
pub type BUSY_M_R = crate::BitReader<bool>;
#[doc = "Field `BUSY_M` writer - "]
pub type BUSY_M_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_MASK_SPEC, bool, O>;
#[doc = "Field `TXC_M` reader - "]
pub type TXC_M_R = crate::BitReader<bool>;
#[doc = "Field `TXC_M` writer - "]
pub type TXC_M_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_MASK_SPEC, bool, O>;
#[doc = "Field `RXC_M` reader - "]
pub type RXC_M_R = crate::BitReader<bool>;
#[doc = "Field `RXC_M` writer - "]
pub type RXC_M_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txb_m(&self) -> TXB_M_R {
        TXB_M_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txe_m(&self) -> TXE_M_R {
        TXE_M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxb_m(&self) -> RXB_M_R {
        RXB_M_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxe_m(&self) -> RXE_M_R {
        RXE_M_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busy_m(&self) -> BUSY_M_R {
        BUSY_M_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txc_m(&self) -> TXC_M_R {
        TXC_M_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxc_m(&self) -> RXC_M_R {
        RXC_M_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txb_m(&mut self) -> TXB_M_W<0> {
        TXB_M_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txe_m(&mut self) -> TXE_M_W<1> {
        TXE_M_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxb_m(&mut self) -> RXB_M_W<2> {
        RXB_M_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxe_m(&mut self) -> RXE_M_W<3> {
        RXE_M_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busy_m(&mut self) -> BUSY_M_W<4> {
        BUSY_M_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txc_m(&mut self) -> TXC_M_W<5> {
        TXC_M_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxc_m(&mut self) -> RXC_M_W<6> {
        RXC_M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INT_MASK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mask](index.html) module"]
pub struct INT_MASK_SPEC;
impl crate::RegisterSpec for INT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_mask::R](R) reader structure"]
impl crate::Readable for INT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_mask::W](W) writer structure"]
impl crate::Writable for INT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_MASK to value 0"]
impl crate::Resettable for INT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
