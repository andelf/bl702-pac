#[doc = "Register `dvp_debug` reader"]
pub struct R(crate::R<DVP_DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP_DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP_DEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP_DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dvp_debug` writer"]
pub struct W(crate::W<DVP_DEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP_DEBUG_SPEC>;
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
impl From<crate::W<DVP_DEBUG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP_DEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_dvp_dbg_en` reader - "]
pub type REG_DVP_DBG_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_dvp_dbg_en` writer - "]
pub type REG_DVP_DBG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DVP_DEBUG_SPEC, bool, O>;
#[doc = "Field `reg_dvp_dbg_sel` reader - "]
pub type REG_DVP_DBG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_dvp_dbg_sel` writer - "]
pub type REG_DVP_DBG_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DVP_DEBUG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dvp_dbg_en(&self) -> REG_DVP_DBG_EN_R {
        REG_DVP_DBG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reg_dvp_dbg_sel(&self) -> REG_DVP_DBG_SEL_R {
        REG_DVP_DBG_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dvp_dbg_en(&mut self) -> REG_DVP_DBG_EN_W<0> {
        REG_DVP_DBG_EN_W::new(self)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn reg_dvp_dbg_sel(&mut self) -> REG_DVP_DBG_SEL_W<1> {
        REG_DVP_DBG_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dvp_debug.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp_debug](index.html) module"]
pub struct DVP_DEBUG_SPEC;
impl crate::RegisterSpec for DVP_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp_debug::R](R) reader structure"]
impl crate::Readable for DVP_DEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp_debug::W](W) writer structure"]
impl crate::Writable for DVP_DEBUG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dvp_debug to value 0"]
impl crate::Resettable for DVP_DEBUG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
