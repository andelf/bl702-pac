#[doc = "Register `ep5_config` reader"]
pub struct R(crate::R<EP5_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP5_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP5_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP5_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ep5_config` writer"]
pub struct W(crate::W<EP5_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP5_CONFIG_SPEC>;
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
impl From<crate::W<EP5_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP5_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_ep5_size` reader - "]
pub type CR_EP5_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_ep5_size` writer - "]
pub type CR_EP5_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP5_CONFIG_SPEC, u16, u16, 11, O>;
#[doc = "Field `cr_ep5_dir` reader - "]
pub type CR_EP5_DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_ep5_dir` writer - "]
pub type CR_EP5_DIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP5_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_ep5_type` reader - "]
pub type CR_EP5_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_ep5_type` writer - "]
pub type CR_EP5_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EP5_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `cr_ep5_stall` reader - "]
pub type CR_EP5_STALL_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep5_stall` writer - "]
pub type CR_EP5_STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_ep5_nack` reader - "]
pub type CR_EP5_NACK_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep5_nack` writer - "]
pub type CR_EP5_NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_ep5_rdy` reader - "]
pub type CR_EP5_RDY_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep5_rdy` writer - "]
pub type CR_EP5_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5_CONFIG_SPEC, bool, O>;
#[doc = "Field `sts_ep5_rdy` reader - "]
pub type STS_EP5_RDY_R = crate::BitReader<bool>;
#[doc = "Field `sts_ep5_rdy` writer - "]
pub type STS_EP5_RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP5_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn cr_ep5_size(&self) -> CR_EP5_SIZE_R {
        CR_EP5_SIZE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn cr_ep5_dir(&self) -> CR_EP5_DIR_R {
        CR_EP5_DIR_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn cr_ep5_type(&self) -> CR_EP5_TYPE_R {
        CR_EP5_TYPE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_ep5_stall(&self) -> CR_EP5_STALL_R {
        CR_EP5_STALL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_ep5_nack(&self) -> CR_EP5_NACK_R {
        CR_EP5_NACK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_ep5_rdy(&self) -> CR_EP5_RDY_R {
        CR_EP5_RDY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sts_ep5_rdy(&self) -> STS_EP5_RDY_R {
        STS_EP5_RDY_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn cr_ep5_size(&mut self) -> CR_EP5_SIZE_W<0> {
        CR_EP5_SIZE_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn cr_ep5_dir(&mut self) -> CR_EP5_DIR_W<11> {
        CR_EP5_DIR_W::new(self)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn cr_ep5_type(&mut self) -> CR_EP5_TYPE_W<13> {
        CR_EP5_TYPE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_ep5_stall(&mut self) -> CR_EP5_STALL_W<16> {
        CR_EP5_STALL_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_ep5_nack(&mut self) -> CR_EP5_NACK_W<17> {
        CR_EP5_NACK_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_ep5_rdy(&mut self) -> CR_EP5_RDY_W<18> {
        CR_EP5_RDY_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sts_ep5_rdy(&mut self) -> STS_EP5_RDY_W<19> {
        STS_EP5_RDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ep5_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5_config](index.html) module"]
pub struct EP5_CONFIG_SPEC;
impl crate::RegisterSpec for EP5_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep5_config::R](R) reader structure"]
impl crate::Readable for EP5_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep5_config::W](W) writer structure"]
impl crate::Writable for EP5_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ep5_config to value 0"]
impl crate::Resettable for EP5_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
