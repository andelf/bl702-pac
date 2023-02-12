#[doc = "Register `TXCTRL` reader"]
pub struct R(crate::R<TXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCTRL` writer"]
pub struct W(crate::W<TXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCTRL_SPEC>;
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
impl From<crate::W<TXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPAUSETV` reader - "]
pub type TXPAUSETV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXPAUSETV` writer - "]
pub type TXPAUSETV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXCTRL_SPEC, u16, u16, 16, O>;
#[doc = "Field `TXPAUSERQ` reader - "]
pub type TXPAUSERQ_R = crate::BitReader<bool>;
#[doc = "Field `TXPAUSERQ` writer - "]
pub type TXPAUSERQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn txpausetv(&self) -> TXPAUSETV_R {
        TXPAUSETV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn txpauserq(&self) -> TXPAUSERQ_R {
        TXPAUSERQ_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn txpausetv(&mut self) -> TXPAUSETV_W<0> {
        TXPAUSETV_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn txpauserq(&mut self) -> TXPAUSERQ_W<16> {
        TXPAUSERQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TXCTRL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctrl](index.html) module"]
pub struct TXCTRL_SPEC;
impl crate::RegisterSpec for TXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txctrl::R](R) reader structure"]
impl crate::Readable for TXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txctrl::W](W) writer structure"]
impl crate::Writable for TXCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCTRL to value 0"]
impl crate::Resettable for TXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
