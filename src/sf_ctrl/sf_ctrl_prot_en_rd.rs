#[doc = "Register `sf_ctrl_prot_en_rd` reader"]
pub struct R(crate::R<SF_CTRL_PROT_EN_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_PROT_EN_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CTRL_PROT_EN_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CTRL_PROT_EN_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_prot_en_rd` writer"]
pub struct W(crate::W<SF_CTRL_PROT_EN_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_PROT_EN_RD_SPEC>;
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
impl From<crate::W<SF_CTRL_PROT_EN_RD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_CTRL_PROT_EN_RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_ctrl_prot_en_rd` reader - "]
pub type SF_CTRL_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `sf_ctrl_prot_en_rd` writer - "]
pub type SF_CTRL_PROT_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_CTRL_PROT_EN_RD_SPEC, bool, O>;
#[doc = "Field `sf_ctrl_id0_en_rd` reader - "]
pub type SF_CTRL_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `sf_ctrl_id0_en_rd` writer - "]
pub type SF_CTRL_ID0_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_CTRL_PROT_EN_RD_SPEC, bool, O>;
#[doc = "Field `sf_ctrl_id1_en_rd` reader - "]
pub type SF_CTRL_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `sf_ctrl_id1_en_rd` writer - "]
pub type SF_CTRL_ID1_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_CTRL_PROT_EN_RD_SPEC, bool, O>;
#[doc = "Field `sf_if_0_trig_wr_lock` reader - "]
pub type SF_IF_0_TRIG_WR_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `sf_if_0_trig_wr_lock` writer - "]
pub type SF_IF_0_TRIG_WR_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_CTRL_PROT_EN_RD_SPEC, bool, O>;
#[doc = "Field `sf_dbg_dis` reader - "]
pub type SF_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `sf_dbg_dis` writer - "]
pub type SF_DBG_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SF_CTRL_PROT_EN_RD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en_rd(&self) -> SF_CTRL_PROT_EN_RD_R {
        SF_CTRL_PROT_EN_RD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en_rd(&self) -> SF_CTRL_ID0_EN_RD_R {
        SF_CTRL_ID0_EN_RD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en_rd(&self) -> SF_CTRL_ID1_EN_RD_R {
        SF_CTRL_ID1_EN_RD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_if_0_trig_wr_lock(&self) -> SF_IF_0_TRIG_WR_LOCK_R {
        SF_IF_0_TRIG_WR_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_dbg_dis(&self) -> SF_DBG_DIS_R {
        SF_DBG_DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sf_ctrl_prot_en_rd(&mut self) -> SF_CTRL_PROT_EN_RD_W<0> {
        SF_CTRL_PROT_EN_RD_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sf_ctrl_id0_en_rd(&mut self) -> SF_CTRL_ID0_EN_RD_W<1> {
        SF_CTRL_ID0_EN_RD_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sf_ctrl_id1_en_rd(&mut self) -> SF_CTRL_ID1_EN_RD_W<2> {
        SF_CTRL_ID1_EN_RD_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn sf_if_0_trig_wr_lock(&mut self) -> SF_IF_0_TRIG_WR_LOCK_W<30> {
        SF_IF_0_TRIG_WR_LOCK_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn sf_dbg_dis(&mut self) -> SF_DBG_DIS_W<31> {
        SF_DBG_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_prot_en_rd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_prot_en_rd](index.html) module"]
pub struct SF_CTRL_PROT_EN_RD_SPEC;
impl crate::RegisterSpec for SF_CTRL_PROT_EN_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_prot_en_rd::R](R) reader structure"]
impl crate::Readable for SF_CTRL_PROT_EN_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_prot_en_rd::W](W) writer structure"]
impl crate::Writable for SF_CTRL_PROT_EN_RD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sf_ctrl_prot_en_rd to value 0"]
impl crate::Resettable for SF_CTRL_PROT_EN_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
