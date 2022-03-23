#[doc = "Register `rbb_gain_ctrl12` reader"]
pub struct R(crate::R<RBB_GAIN_CTRL12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RBB_GAIN_CTRL12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RBB_GAIN_CTRL12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RBB_GAIN_CTRL12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rbb_gain_ctrl12` writer"]
pub struct W(crate::W<RBB_GAIN_CTRL12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RBB_GAIN_CTRL12_SPEC>;
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
impl From<crate::W<RBB_GAIN_CTRL12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RBB_GAIN_CTRL12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gain_ctrl12_g_rbb1` reader - "]
pub struct GAIN_CTRL12_G_RBB1_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL12_G_RBB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL12_G_RBB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL12_G_RBB1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl12_g_rbb1` writer - "]
pub struct GAIN_CTRL12_G_RBB1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL12_G_RBB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `gain_ctrl12_g_rbb2` reader - "]
pub struct GAIN_CTRL12_G_RBB2_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL12_G_RBB2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL12_G_RBB2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL12_G_RBB2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl12_g_rbb2` writer - "]
pub struct GAIN_CTRL12_G_RBB2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL12_G_RBB2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `gain_ctrl12_rosdac_i_bw1` reader - "]
pub struct GAIN_CTRL12_ROSDAC_I_BW1_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL12_ROSDAC_I_BW1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL12_ROSDAC_I_BW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL12_ROSDAC_I_BW1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl12_rosdac_i_bw1` writer - "]
pub struct GAIN_CTRL12_ROSDAC_I_BW1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL12_ROSDAC_I_BW1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `gain_ctrl12_rosdac_q_bw1` reader - "]
pub struct GAIN_CTRL12_ROSDAC_Q_BW1_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL12_ROSDAC_Q_BW1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL12_ROSDAC_Q_BW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL12_ROSDAC_Q_BW1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl12_rosdac_q_bw1` writer - "]
pub struct GAIN_CTRL12_ROSDAC_Q_BW1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL12_ROSDAC_Q_BW1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | ((value as u32 & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `gain_ctrl12_rosdac_i_bw0` reader - "]
pub struct GAIN_CTRL12_ROSDAC_I_BW0_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL12_ROSDAC_I_BW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL12_ROSDAC_I_BW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL12_ROSDAC_I_BW0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl12_rosdac_i_bw0` writer - "]
pub struct GAIN_CTRL12_ROSDAC_I_BW0_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL12_ROSDAC_I_BW0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | ((value as u32 & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `gain_ctrl12_rosdac_q_bw0` reader - "]
pub struct GAIN_CTRL12_ROSDAC_Q_BW0_R(crate::FieldReader<u8, u8>);
impl GAIN_CTRL12_ROSDAC_Q_BW0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GAIN_CTRL12_ROSDAC_Q_BW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN_CTRL12_ROSDAC_Q_BW0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gain_ctrl12_rosdac_q_bw0` writer - "]
pub struct GAIN_CTRL12_ROSDAC_Q_BW0_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN_CTRL12_ROSDAC_Q_BW0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl12_g_rbb1(&self) -> GAIN_CTRL12_G_RBB1_R {
        GAIN_CTRL12_G_RBB1_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl12_g_rbb2(&self) -> GAIN_CTRL12_G_RBB2_R {
        GAIN_CTRL12_G_RBB2_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn gain_ctrl12_rosdac_i_bw1(&self) -> GAIN_CTRL12_ROSDAC_I_BW1_R {
        GAIN_CTRL12_ROSDAC_I_BW1_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn gain_ctrl12_rosdac_q_bw1(&self) -> GAIN_CTRL12_ROSDAC_Q_BW1_R {
        GAIN_CTRL12_ROSDAC_Q_BW1_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn gain_ctrl12_rosdac_i_bw0(&self) -> GAIN_CTRL12_ROSDAC_I_BW0_R {
        GAIN_CTRL12_ROSDAC_I_BW0_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gain_ctrl12_rosdac_q_bw0(&self) -> GAIN_CTRL12_ROSDAC_Q_BW0_R {
        GAIN_CTRL12_ROSDAC_Q_BW0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gain_ctrl12_g_rbb1(&mut self) -> GAIN_CTRL12_G_RBB1_W {
        GAIN_CTRL12_G_RBB1_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn gain_ctrl12_g_rbb2(&mut self) -> GAIN_CTRL12_G_RBB2_W {
        GAIN_CTRL12_G_RBB2_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn gain_ctrl12_rosdac_i_bw1(&mut self) -> GAIN_CTRL12_ROSDAC_I_BW1_W {
        GAIN_CTRL12_ROSDAC_I_BW1_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn gain_ctrl12_rosdac_q_bw1(&mut self) -> GAIN_CTRL12_ROSDAC_Q_BW1_W {
        GAIN_CTRL12_ROSDAC_Q_BW1_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn gain_ctrl12_rosdac_i_bw0(&mut self) -> GAIN_CTRL12_ROSDAC_I_BW0_W {
        GAIN_CTRL12_ROSDAC_I_BW0_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn gain_ctrl12_rosdac_q_bw0(&mut self) -> GAIN_CTRL12_ROSDAC_Q_BW0_W {
        GAIN_CTRL12_ROSDAC_Q_BW0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rbb_gain_ctrl12.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbb_gain_ctrl12](index.html) module"]
pub struct RBB_GAIN_CTRL12_SPEC;
impl crate::RegisterSpec for RBB_GAIN_CTRL12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rbb_gain_ctrl12::R](R) reader structure"]
impl crate::Readable for RBB_GAIN_CTRL12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rbb_gain_ctrl12::W](W) writer structure"]
impl crate::Writable for RBB_GAIN_CTRL12_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rbb_gain_ctrl12 to value 0"]
impl crate::Resettable for RBB_GAIN_CTRL12_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
