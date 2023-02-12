#[doc = "Register `ep1_config` reader"]
pub struct R(crate::R<EP1_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP1_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP1_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP1_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep1_config` writer"]
pub struct W(crate::W<EP1_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP1_CONFIG_SPEC>;
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
impl From<crate::W<EP1_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP1_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_ep1_size` reader - "]
pub type CR_EP1_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_ep1_size` writer - "]
pub type CR_EP1_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP1_CONFIG_SPEC, u16, u16, 11, O>;
#[doc = "Field `cr_ep1_dir` reader - "]
pub type CR_EP1_DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_ep1_dir` writer - "]
pub type CR_EP1_DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP1_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_ep1_type` reader - "]
pub type CR_EP1_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_ep1_type` writer - "]
pub type CR_EP1_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP1_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `cr_ep1_stall` reader - "]
pub type CR_EP1_STALL_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep1_stall` writer - "]
pub type CR_EP1_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_ep1_nack` reader - "]
pub type CR_EP1_NACK_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep1_nack` writer - "]
pub type CR_EP1_NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_ep1_rdy` reader - "]
pub type CR_EP1_RDY_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep1_rdy` writer - "]
pub type CR_EP1_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1_CONFIG_SPEC, bool, O>;
#[doc = "Field `sts_ep1_rdy` reader - "]
pub type STS_EP1_RDY_R = crate::BitReader<bool>;
#[doc = "Field `sts_ep1_rdy` writer - "]
pub type STS_EP1_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP1_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn cr_ep1_size(&self) -> CR_EP1_SIZE_R {
        CR_EP1_SIZE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn cr_ep1_dir(&self) -> CR_EP1_DIR_R {
        CR_EP1_DIR_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn cr_ep1_type(&self) -> CR_EP1_TYPE_R {
        CR_EP1_TYPE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_ep1_stall(&self) -> CR_EP1_STALL_R {
        CR_EP1_STALL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_ep1_nack(&self) -> CR_EP1_NACK_R {
        CR_EP1_NACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_ep1_rdy(&self) -> CR_EP1_RDY_R {
        CR_EP1_RDY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sts_ep1_rdy(&self) -> STS_EP1_RDY_R {
        STS_EP1_RDY_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep1_size(&mut self) -> CR_EP1_SIZE_W<0> {
        CR_EP1_SIZE_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep1_dir(&mut self) -> CR_EP1_DIR_W<11> {
        CR_EP1_DIR_W::new(self)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep1_type(&mut self) -> CR_EP1_TYPE_W<13> {
        CR_EP1_TYPE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep1_stall(&mut self) -> CR_EP1_STALL_W<16> {
        CR_EP1_STALL_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep1_nack(&mut self) -> CR_EP1_NACK_W<17> {
        CR_EP1_NACK_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep1_rdy(&mut self) -> CR_EP1_RDY_W<18> {
        CR_EP1_RDY_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn sts_ep1_rdy(&mut self) -> STS_EP1_RDY_W<19> {
        STS_EP1_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep1_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep1_config](index.html) module"]
pub struct EP1_CONFIG_SPEC;
impl crate::RegisterSpec for EP1_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep1_config::R](R) reader structure"]
impl crate::Readable for EP1_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep1_config::W](W) writer structure"]
impl crate::Writable for EP1_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ep1_config to value 0"]
impl crate::Resettable for EP1_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
