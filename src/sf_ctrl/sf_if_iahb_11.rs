#[doc = "Register `sf_if_iahb_11` reader"]
pub struct R(crate::R<SF_IF_IAHB_11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_IF_IAHB_11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_IF_IAHB_11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_IF_IAHB_11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_if_iahb_11` writer"]
pub struct W(crate::W<SF_IF_IAHB_11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_IF_IAHB_11_SPEC>;
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
impl From<crate::W<SF_IF_IAHB_11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_IF_IAHB_11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_if_4_cmd_buf_1` reader - "]
pub type SF_IF_4_CMD_BUF_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `sf_if_4_cmd_buf_1` writer - "]
pub type SF_IF_4_CMD_BUF_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF_IF_IAHB_11_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_4_cmd_buf_1(&self) -> SF_IF_4_CMD_BUF_1_R {
        SF_IF_4_CMD_BUF_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_4_cmd_buf_1(&mut self) -> SF_IF_4_CMD_BUF_1_W<0> {
        SF_IF_4_CMD_BUF_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_if_iahb_11.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_11](index.html) module"]
pub struct SF_IF_IAHB_11_SPEC;
impl crate::RegisterSpec for SF_IF_IAHB_11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_if_iahb_11::R](R) reader structure"]
impl crate::Readable for SF_IF_IAHB_11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_11::W](W) writer structure"]
impl crate::Writable for SF_IF_IAHB_11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_if_iahb_11 to value 0"]
impl crate::Resettable for SF_IF_IAHB_11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
