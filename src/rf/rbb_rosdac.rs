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
#[doc = "Field `rosdac_i` reader - "]
pub struct ROSDAC_I_R(crate::FieldReader<u8, u8>);
impl ROSDAC_I_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROSDAC_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_i` writer - "]
pub struct ROSDAC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `rosdac_q` reader - "]
pub struct ROSDAC_Q_R(crate::FieldReader<u8, u8>);
impl ROSDAC_Q_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROSDAC_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_Q_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_q` writer - "]
pub struct ROSDAC_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_Q_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `rosdac_i_hw` reader - "]
pub struct ROSDAC_I_HW_R(crate::FieldReader<u8, u8>);
impl ROSDAC_I_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROSDAC_I_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_I_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_i_hw` writer - "]
pub struct ROSDAC_I_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_I_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `rosdac_q_hw` reader - "]
pub struct ROSDAC_Q_HW_R(crate::FieldReader<u8, u8>);
impl ROSDAC_Q_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ROSDAC_Q_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSDAC_Q_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rosdac_q_hw` writer - "]
pub struct ROSDAC_Q_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSDAC_Q_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_i(&self) -> ROSDAC_I_R {
        ROSDAC_I_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_q(&self) -> ROSDAC_Q_R {
        ROSDAC_Q_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_i_hw(&self) -> ROSDAC_I_HW_R {
        ROSDAC_I_HW_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_q_hw(&self) -> ROSDAC_Q_HW_R {
        ROSDAC_Q_HW_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn rosdac_i(&mut self) -> ROSDAC_I_W {
        ROSDAC_I_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn rosdac_q(&mut self) -> ROSDAC_Q_W {
        ROSDAC_Q_W { w: self }
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rosdac_i_hw(&mut self) -> ROSDAC_I_HW_W {
        ROSDAC_I_HW_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rosdac_q_hw(&mut self) -> ROSDAC_Q_HW_W {
        ROSDAC_Q_HW_W { w: self }
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
}
#[doc = "`reset()` method sets rbb_rosdac to value 0"]
impl crate::Resettable for RBB_ROSDAC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
