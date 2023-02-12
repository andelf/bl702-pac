#[doc = "Register `rbb_cap_3` reader"]
pub struct R(crate::R<RBB_CAP_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_CAP_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB_CAP_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB_CAP_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb_cap_3` writer"]
pub struct W(crate::W<RBB_CAP_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_CAP_3_SPEC>;
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
impl From<crate::W<RBB_CAP_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB_CAP_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rbb_cap2_fc_q_bw0` reader - "]
pub type RBB_CAP2_FC_Q_BW0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_cap2_fc_q_bw0` writer - "]
pub type RBB_CAP2_FC_Q_BW0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_CAP_3_SPEC, u8, u8, 6, O>;
#[doc = "Field `rbb_cap2_fc_i_bw0` reader - "]
pub type RBB_CAP2_FC_I_BW0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_cap2_fc_i_bw0` writer - "]
pub type RBB_CAP2_FC_I_BW0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_CAP_3_SPEC, u8, u8, 6, O>;
#[doc = "Field `rbb_cap1_fc_q_bw0` reader - "]
pub type RBB_CAP1_FC_Q_BW0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_cap1_fc_q_bw0` writer - "]
pub type RBB_CAP1_FC_Q_BW0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_CAP_3_SPEC, u8, u8, 6, O>;
#[doc = "Field `rbb_cap1_fc_i_bw0` reader - "]
pub type RBB_CAP1_FC_I_BW0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rbb_cap1_fc_i_bw0` writer - "]
pub type RBB_CAP1_FC_I_BW0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RBB_CAP_3_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rbb_cap2_fc_q_bw0(&self) -> RBB_CAP2_FC_Q_BW0_R {
        RBB_CAP2_FC_Q_BW0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rbb_cap2_fc_i_bw0(&self) -> RBB_CAP2_FC_I_BW0_R {
        RBB_CAP2_FC_I_BW0_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rbb_cap1_fc_q_bw0(&self) -> RBB_CAP1_FC_Q_BW0_R {
        RBB_CAP1_FC_Q_BW0_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rbb_cap1_fc_i_bw0(&self) -> RBB_CAP1_FC_I_BW0_R {
        RBB_CAP1_FC_I_BW0_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_cap2_fc_q_bw0(&mut self) -> RBB_CAP2_FC_Q_BW0_W<0> {
        RBB_CAP2_FC_Q_BW0_W::new(self)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_cap2_fc_i_bw0(&mut self) -> RBB_CAP2_FC_I_BW0_W<8> {
        RBB_CAP2_FC_I_BW0_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_cap1_fc_q_bw0(&mut self) -> RBB_CAP1_FC_Q_BW0_W<16> {
        RBB_CAP1_FC_Q_BW0_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_cap1_fc_i_bw0(&mut self) -> RBB_CAP1_FC_I_BW0_W<24> {
        RBB_CAP1_FC_I_BW0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb_cap_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_cap_3](index.html) module"]
pub struct RBB_CAP_3_SPEC;
impl crate::RegisterSpec for RBB_CAP_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb_cap_3::R](R) reader structure"]
impl crate::Readable for RBB_CAP_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb_cap_3::W](W) writer structure"]
impl crate::Writable for RBB_CAP_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rbb_cap_3 to value 0"]
impl crate::Resettable for RBB_CAP_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
