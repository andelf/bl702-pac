#[doc = "Register `hit_cnt_msb` reader"]
pub struct R(crate::R<HIT_CNT_MSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIT_CNT_MSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIT_CNT_MSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIT_CNT_MSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hit_cnt_msb` writer"]
pub struct W(crate::W<HIT_CNT_MSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIT_CNT_MSB_SPEC>;
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
impl From<crate::W<HIT_CNT_MSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIT_CNT_MSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hit_cnt_msb` reader - "]
pub type HIT_CNT_MSB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `hit_cnt_msb` writer - "]
pub type HIT_CNT_MSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HIT_CNT_MSB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_msb(&self) -> HIT_CNT_MSB_R {
        HIT_CNT_MSB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_msb(&mut self) -> HIT_CNT_MSB_W<0> {
        HIT_CNT_MSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "hit_cnt_msb.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hit_cnt_msb](index.html) module"]
pub struct HIT_CNT_MSB_SPEC;
impl crate::RegisterSpec for HIT_CNT_MSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hit_cnt_msb::R](R) reader structure"]
impl crate::Readable for HIT_CNT_MSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hit_cnt_msb::W](W) writer structure"]
impl crate::Writable for HIT_CNT_MSB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets hit_cnt_msb to value 0"]
impl crate::Resettable for HIT_CNT_MSB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
