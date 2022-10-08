#[doc = "Register `dvp2ahb_addr_start_1` reader"]
pub struct R(crate::R<DVP2AHB_ADDR_START_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP2AHB_ADDR_START_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP2AHB_ADDR_START_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP2AHB_ADDR_START_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dvp2ahb_addr_start_1` writer"]
pub struct W(crate::W<DVP2AHB_ADDR_START_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP2AHB_ADDR_START_1_SPEC>;
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
impl From<crate::W<DVP2AHB_ADDR_START_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP2AHB_ADDR_START_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_addr_start_1` reader - "]
pub type REG_ADDR_START_1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `reg_addr_start_1` writer - "]
pub type REG_ADDR_START_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DVP2AHB_ADDR_START_1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_addr_start_1(&self) -> REG_ADDR_START_1_R {
        REG_ADDR_START_1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_addr_start_1(&mut self) -> REG_ADDR_START_1_W<0> {
        REG_ADDR_START_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dvp2ahb_addr_start_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp2ahb_addr_start_1](index.html) module"]
pub struct DVP2AHB_ADDR_START_1_SPEC;
impl crate::RegisterSpec for DVP2AHB_ADDR_START_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp2ahb_addr_start_1::R](R) reader structure"]
impl crate::Readable for DVP2AHB_ADDR_START_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp2ahb_addr_start_1::W](W) writer structure"]
impl crate::Writable for DVP2AHB_ADDR_START_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dvp2ahb_addr_start_1 to value 0"]
impl crate::Resettable for DVP2AHB_ADDR_START_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
