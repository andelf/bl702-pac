#[doc = "Register `se_gmac_0_status` reader"]
pub struct R(crate::R<SE_GMAC_0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_GMAC_0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_GMAC_0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_GMAC_0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_gmac_0_status` writer"]
pub struct W(crate::W<SE_GMAC_0_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_GMAC_0_STATUS_SPEC>;
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
impl From<crate::W<SE_GMAC_0_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_GMAC_0_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_gmac_0_status` reader - "]
pub type SE_GMAC_0_STATUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `se_gmac_0_status` writer - "]
pub type SE_GMAC_0_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SE_GMAC_0_STATUS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_gmac_0_status(&self) -> SE_GMAC_0_STATUS_R {
        SE_GMAC_0_STATUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn se_gmac_0_status(&mut self) -> SE_GMAC_0_STATUS_W<0> {
        SE_GMAC_0_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_gmac_0_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_gmac_0_status](index.html) module"]
pub struct SE_GMAC_0_STATUS_SPEC;
impl crate::RegisterSpec for SE_GMAC_0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_gmac_0_status::R](R) reader structure"]
impl crate::Readable for SE_GMAC_0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_gmac_0_status::W](W) writer structure"]
impl crate::Writable for SE_GMAC_0_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets se_gmac_0_status to value 0"]
impl crate::Resettable for SE_GMAC_0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
