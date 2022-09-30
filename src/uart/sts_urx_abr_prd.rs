#[doc = "Register `sts_urx_abr_prd` reader"]
pub struct R(crate::R<STS_URX_ABR_PRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS_URX_ABR_PRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS_URX_ABR_PRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS_URX_ABR_PRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sts_urx_abr_prd` writer"]
pub struct W(crate::W<STS_URX_ABR_PRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STS_URX_ABR_PRD_SPEC>;
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
impl From<crate::W<STS_URX_ABR_PRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STS_URX_ABR_PRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_urx_abr_prd_start` reader - "]
pub type STS_URX_ABR_PRD_START_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sts_urx_abr_prd_start` writer - "]
pub type STS_URX_ABR_PRD_START_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STS_URX_ABR_PRD_SPEC, u16, u16, 16, O>;
#[doc = "Field `sts_urx_abr_prd_0x55` reader - "]
pub type STS_URX_ABR_PRD_0X55_R = crate::FieldReader<u16, u16>;
#[doc = "Field `sts_urx_abr_prd_0x55` writer - "]
pub type STS_URX_ABR_PRD_0X55_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STS_URX_ABR_PRD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_start(&self) -> STS_URX_ABR_PRD_START_R {
        STS_URX_ABR_PRD_START_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_0x55(&self) -> STS_URX_ABR_PRD_0X55_R {
        STS_URX_ABR_PRD_0X55_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_start(&mut self) -> STS_URX_ABR_PRD_START_W<0> {
        STS_URX_ABR_PRD_START_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn sts_urx_abr_prd_0x55(&mut self) -> STS_URX_ABR_PRD_0X55_W<16> {
        STS_URX_ABR_PRD_0X55_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sts_urx_abr_prd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts_urx_abr_prd](index.html) module"]
pub struct STS_URX_ABR_PRD_SPEC;
impl crate::RegisterSpec for STS_URX_ABR_PRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts_urx_abr_prd::R](R) reader structure"]
impl crate::Readable for STS_URX_ABR_PRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sts_urx_abr_prd::W](W) writer structure"]
impl crate::Writable for STS_URX_ABR_PRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sts_urx_abr_prd to value 0"]
impl crate::Resettable for STS_URX_ABR_PRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
