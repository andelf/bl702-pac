#[doc = "Register `rbb_rosdac` reader"]
pub struct R(crate::R<RBB_ROSDAC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_ROSDAC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB_ROSDAC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB_ROSDAC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb_rosdac` writer"]
pub struct W(crate::W<RBB_ROSDAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_ROSDAC_SPEC>;
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
impl From<crate::W<RBB_ROSDAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB_ROSDAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rosdac_q_hw` reader - "]
pub type ROSDAC_Q_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_q_hw` writer - "]
pub type ROSDAC_Q_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_ROSDAC_SPEC, u8, u8, 6, O>;
#[doc = "Field `rosdac_i_hw` reader - "]
pub type ROSDAC_I_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_i_hw` writer - "]
pub type ROSDAC_I_HW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_ROSDAC_SPEC, u8, u8, 6, O>;
#[doc = "Field `rosdac_q` reader - "]
pub type ROSDAC_Q_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_q` writer - "]
pub type ROSDAC_Q_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB_ROSDAC_SPEC, u8, u8, 6, O>;
#[doc = "Field `rosdac_i` reader - "]
pub type ROSDAC_I_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rosdac_i` writer - "]
pub type ROSDAC_I_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RBB_ROSDAC_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_q_hw(&self) -> ROSDAC_Q_HW_R {
        ROSDAC_Q_HW_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_i_hw(&self) -> ROSDAC_I_HW_R {
        ROSDAC_I_HW_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_q(&self) -> ROSDAC_Q_R {
        ROSDAC_Q_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_i(&self) -> ROSDAC_I_R {
        ROSDAC_I_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_q_hw(&mut self) -> ROSDAC_Q_HW_W<0> {
        ROSDAC_Q_HW_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_i_hw(&mut self) -> ROSDAC_I_HW_W<8> {
        ROSDAC_I_HW_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_q(&mut self) -> ROSDAC_Q_W<16> {
        ROSDAC_Q_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn rosdac_i(&mut self) -> ROSDAC_I_W<24> {
        ROSDAC_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb_rosdac.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_rosdac](index.html) module"]
pub struct RBB_ROSDAC_SPEC;
impl crate::RegisterSpec for RBB_ROSDAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb_rosdac::R](R) reader structure"]
impl crate::Readable for RBB_ROSDAC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb_rosdac::W](W) writer structure"]
impl crate::Writable for RBB_ROSDAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rbb_rosdac to value 0"]
impl crate::Resettable for RBB_ROSDAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
