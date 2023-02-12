#[doc = "Register `hit_cnt_lsb` reader"]
pub struct R(crate::R<HIT_CNT_LSB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIT_CNT_LSB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIT_CNT_LSB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIT_CNT_LSB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hit_cnt_lsb` writer"]
pub struct W(crate::W<HIT_CNT_LSB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIT_CNT_LSB_SPEC>;
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
impl From<crate::W<HIT_CNT_LSB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIT_CNT_LSB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hit_cnt_lsb` reader - "]
pub type HIT_CNT_LSB_R = crate::FieldReader<u32, u32>;
#[doc = "Field `hit_cnt_lsb` writer - "]
pub type HIT_CNT_LSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HIT_CNT_LSB_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hit_cnt_lsb(&self) -> HIT_CNT_LSB_R {
        HIT_CNT_LSB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn hit_cnt_lsb(&mut self) -> HIT_CNT_LSB_W<0> {
        HIT_CNT_LSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "hit_cnt_lsb.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hit_cnt_lsb](index.html) module"]
pub struct HIT_CNT_LSB_SPEC;
impl crate::RegisterSpec for HIT_CNT_LSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hit_cnt_lsb::R](R) reader structure"]
impl crate::Readable for HIT_CNT_LSB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hit_cnt_lsb::W](W) writer structure"]
impl crate::Writable for HIT_CNT_LSB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets hit_cnt_lsb to value 0"]
impl crate::Resettable for HIT_CNT_LSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
