#[doc = "Register `kcal2` reader"]
pub struct R(crate::R<KCAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KCAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KCAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KCAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `kcal2` writer"]
pub struct W(crate::W<KCAL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KCAL2_SPEC>;
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
impl From<crate::W<KCAL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KCAL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `kcal_ratio_hw` reader - "]
pub struct KCAL_RATIO_HW_R(crate::FieldReader<u16, u16>);
impl KCAL_RATIO_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        KCAL_RATIO_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_RATIO_HW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_ratio_hw` writer - "]
pub struct KCAL_RATIO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_RATIO_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Field `kcal_cnt_rdy` reader - "]
pub struct KCAL_CNT_RDY_R(crate::FieldReader<bool, bool>);
impl KCAL_CNT_RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        KCAL_CNT_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_CNT_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_cnt_rdy` writer - "]
pub struct KCAL_CNT_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_CNT_RDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `kcal_cnt_op` reader - "]
pub struct KCAL_CNT_OP_R(crate::FieldReader<u16, u16>);
impl KCAL_CNT_OP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        KCAL_CNT_OP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_CNT_OP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_cnt_op` writer - "]
pub struct KCAL_CNT_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_CNT_OP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn kcal_ratio_hw(&self) -> KCAL_RATIO_HW_R {
        KCAL_RATIO_HW_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn kcal_cnt_rdy(&self) -> KCAL_CNT_RDY_R {
        KCAL_CNT_RDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn kcal_cnt_op(&self) -> KCAL_CNT_OP_R {
        KCAL_CNT_OP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn kcal_ratio_hw(&mut self) -> KCAL_RATIO_HW_W {
        KCAL_RATIO_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn kcal_cnt_rdy(&mut self) -> KCAL_CNT_RDY_W {
        KCAL_CNT_RDY_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn kcal_cnt_op(&mut self) -> KCAL_CNT_OP_W {
        KCAL_CNT_OP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "kcal2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kcal2](index.html) module"]
pub struct KCAL2_SPEC;
impl crate::RegisterSpec for KCAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kcal2::R](R) reader structure"]
impl crate::Readable for KCAL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kcal2::W](W) writer structure"]
impl crate::Writable for KCAL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets kcal2 to value 0"]
impl crate::Resettable for KCAL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
