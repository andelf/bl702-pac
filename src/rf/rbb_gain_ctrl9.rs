#[doc = "Register `rbb_gain_ctrl9` reader"]
pub struct R(crate::R<RBB_GAIN_CTRL9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_GAIN_CTRL9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB_GAIN_CTRL9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB_GAIN_CTRL9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb_gain_ctrl9` writer"]
pub struct W(crate::W<RBB_GAIN_CTRL9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_GAIN_CTRL9_SPEC>;
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
impl From<crate::W<RBB_GAIN_CTRL9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB_GAIN_CTRL9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gain_ctrl9_rosdac_q_bw0` reader - "]
pub type GAIN_CTRL9_ROSDAC_Q_BW0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl9_rosdac_q_bw0` writer - "]
pub type GAIN_CTRL9_ROSDAC_Q_BW0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_CTRL9_SPEC, u8, u8, 6, O>;
#[doc = "Field `gain_ctrl9_rosdac_i_bw0` reader - "]
pub type GAIN_CTRL9_ROSDAC_I_BW0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl9_rosdac_i_bw0` writer - "]
pub type GAIN_CTRL9_ROSDAC_I_BW0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_CTRL9_SPEC, u8, u8, 6, O>;
#[doc = "Field `gain_ctrl9_rosdac_q_bw1` reader - "]
pub type GAIN_CTRL9_ROSDAC_Q_BW1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl9_rosdac_q_bw1` writer - "]
pub type GAIN_CTRL9_ROSDAC_Q_BW1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_CTRL9_SPEC, u8, u8, 6, O>;
#[doc = "Field `gain_ctrl9_rosdac_i_bw1` reader - "]
pub type GAIN_CTRL9_ROSDAC_I_BW1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl9_rosdac_i_bw1` writer - "]
pub type GAIN_CTRL9_ROSDAC_I_BW1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_CTRL9_SPEC, u8, u8, 6, O>;
#[doc = "Field `gain_ctrl9_g_rbb2` reader - "]
pub type GAIN_CTRL9_G_RBB2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl9_g_rbb2` writer - "]
pub type GAIN_CTRL9_G_RBB2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_CTRL9_SPEC, u8, u8, 3, O>;
#[doc = "Field `gain_ctrl9_g_rbb1` reader - "]
pub type GAIN_CTRL9_G_RBB1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `gain_ctrl9_g_rbb1` writer - "]
pub type GAIN_CTRL9_G_RBB1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_GAIN_CTRL9_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gain_ctrl9_rosdac_q_bw0(&self) -> GAIN_CTRL9_ROSDAC_Q_BW0_R {
        GAIN_CTRL9_ROSDAC_Q_BW0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn gain_ctrl9_rosdac_i_bw0(&self) -> GAIN_CTRL9_ROSDAC_I_BW0_R {
        GAIN_CTRL9_ROSDAC_I_BW0_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn gain_ctrl9_rosdac_q_bw1(&self) -> GAIN_CTRL9_ROSDAC_Q_BW1_R {
        GAIN_CTRL9_ROSDAC_Q_BW1_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn gain_ctrl9_rosdac_i_bw1(&self) -> GAIN_CTRL9_ROSDAC_I_BW1_R {
        GAIN_CTRL9_ROSDAC_I_BW1_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl9_g_rbb2(&self) -> GAIN_CTRL9_G_RBB2_R {
        GAIN_CTRL9_G_RBB2_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl9_g_rbb1(&self) -> GAIN_CTRL9_G_RBB1_R {
        GAIN_CTRL9_G_RBB1_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl9_rosdac_q_bw0(&mut self) -> GAIN_CTRL9_ROSDAC_Q_BW0_W<0> {
        GAIN_CTRL9_ROSDAC_Q_BW0_W::new(self)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl9_rosdac_i_bw0(&mut self) -> GAIN_CTRL9_ROSDAC_I_BW0_W<6> {
        GAIN_CTRL9_ROSDAC_I_BW0_W::new(self)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl9_rosdac_q_bw1(&mut self) -> GAIN_CTRL9_ROSDAC_Q_BW1_W<12> {
        GAIN_CTRL9_ROSDAC_Q_BW1_W::new(self)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl9_rosdac_i_bw1(&mut self) -> GAIN_CTRL9_ROSDAC_I_BW1_W<18> {
        GAIN_CTRL9_ROSDAC_I_BW1_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl9_g_rbb2(&mut self) -> GAIN_CTRL9_G_RBB2_W<24> {
        GAIN_CTRL9_G_RBB2_W::new(self)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    #[must_use]
    pub fn gain_ctrl9_g_rbb1(&mut self) -> GAIN_CTRL9_G_RBB1_W<28> {
        GAIN_CTRL9_G_RBB1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb_gain_ctrl9.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_gain_ctrl9](index.html) module"]
pub struct RBB_GAIN_CTRL9_SPEC;
impl crate::RegisterSpec for RBB_GAIN_CTRL9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb_gain_ctrl9::R](R) reader structure"]
impl crate::Readable for RBB_GAIN_CTRL9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb_gain_ctrl9::W](W) writer structure"]
impl crate::Writable for RBB_GAIN_CTRL9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rbb_gain_ctrl9 to value 0"]
impl crate::Resettable for RBB_GAIN_CTRL9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
