#[doc = "Register `ep2_config` reader"]
pub struct R(crate::R<EP2_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP2_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP2_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP2_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep2_config` writer"]
pub struct W(crate::W<EP2_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP2_CONFIG_SPEC>;
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
impl From<crate::W<EP2_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP2_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_ep2_size` reader - "]
pub type CR_EP2_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_ep2_size` writer - "]
pub type CR_EP2_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP2_CONFIG_SPEC, u16, u16, 11, O>;
#[doc = "Field `cr_ep2_dir` reader - "]
pub type CR_EP2_DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_ep2_dir` writer - "]
pub type CR_EP2_DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP2_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_ep2_type` reader - "]
pub type CR_EP2_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_ep2_type` writer - "]
pub type CR_EP2_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP2_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `cr_ep2_stall` reader - "]
pub type CR_EP2_STALL_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep2_stall` writer - "]
pub type CR_EP2_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP2_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_ep2_nack` reader - "]
pub type CR_EP2_NACK_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep2_nack` writer - "]
pub type CR_EP2_NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP2_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_ep2_rdy` reader - "]
pub type CR_EP2_RDY_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep2_rdy` writer - "]
pub type CR_EP2_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP2_CONFIG_SPEC, bool, O>;
#[doc = "Field `sts_ep2_rdy` reader - "]
pub type STS_EP2_RDY_R = crate::BitReader<bool>;
#[doc = "Field `sts_ep2_rdy` writer - "]
pub type STS_EP2_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP2_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn cr_ep2_size(&self) -> CR_EP2_SIZE_R {
        CR_EP2_SIZE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn cr_ep2_dir(&self) -> CR_EP2_DIR_R {
        CR_EP2_DIR_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn cr_ep2_type(&self) -> CR_EP2_TYPE_R {
        CR_EP2_TYPE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_ep2_stall(&self) -> CR_EP2_STALL_R {
        CR_EP2_STALL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_ep2_nack(&self) -> CR_EP2_NACK_R {
        CR_EP2_NACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_ep2_rdy(&self) -> CR_EP2_RDY_R {
        CR_EP2_RDY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sts_ep2_rdy(&self) -> STS_EP2_RDY_R {
        STS_EP2_RDY_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep2_size(&mut self) -> CR_EP2_SIZE_W<0> {
        CR_EP2_SIZE_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep2_dir(&mut self) -> CR_EP2_DIR_W<11> {
        CR_EP2_DIR_W::new(self)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep2_type(&mut self) -> CR_EP2_TYPE_W<13> {
        CR_EP2_TYPE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep2_stall(&mut self) -> CR_EP2_STALL_W<16> {
        CR_EP2_STALL_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep2_nack(&mut self) -> CR_EP2_NACK_W<17> {
        CR_EP2_NACK_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cr_ep2_rdy(&mut self) -> CR_EP2_RDY_W<18> {
        CR_EP2_RDY_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn sts_ep2_rdy(&mut self) -> STS_EP2_RDY_W<19> {
        STS_EP2_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep2_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep2_config](index.html) module"]
pub struct EP2_CONFIG_SPEC;
impl crate::RegisterSpec for EP2_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep2_config::R](R) reader structure"]
impl crate::Readable for EP2_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep2_config::W](W) writer structure"]
impl crate::Writable for EP2_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ep2_config to value 0"]
impl crate::Resettable for EP2_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
