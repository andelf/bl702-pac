#[doc = "Register `MIISTATUS` reader"]
pub struct R(crate::R<MIISTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIISTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIISTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIISTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIISTATUS` writer"]
pub struct W(crate::W<MIISTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIISTATUS_SPEC>;
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
impl From<crate::W<MIISTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIISTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIIM_LINKFAIL` reader - "]
pub type MIIM_LINKFAIL_R = crate::BitReader<bool>;
#[doc = "Field `MIIM_LINKFAIL` writer - "]
pub type MIIM_LINKFAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIISTATUS_SPEC, bool, O>;
#[doc = "Field `MIIM_BUSY` reader - "]
pub type MIIM_BUSY_R = crate::BitReader<bool>;
#[doc = "Field `MIIM_BUSY` writer - "]
pub type MIIM_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIISTATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn miim_linkfail(&self) -> MIIM_LINKFAIL_R {
        MIIM_LINKFAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn miim_busy(&self) -> MIIM_BUSY_R {
        MIIM_BUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn miim_linkfail(&mut self) -> MIIM_LINKFAIL_W<0> {
        MIIM_LINKFAIL_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn miim_busy(&mut self) -> MIIM_BUSY_W<1> {
        MIIM_BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIISTATUS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miistatus](index.html) module"]
pub struct MIISTATUS_SPEC;
impl crate::RegisterSpec for MIISTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miistatus::R](R) reader structure"]
impl crate::Readable for MIISTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miistatus::W](W) writer structure"]
impl crate::Writable for MIISTATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIISTATUS to value 0"]
impl crate::Resettable for MIISTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
