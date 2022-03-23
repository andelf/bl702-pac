#[doc = "Register `dg_ppud_0` reader"]
pub struct R(crate::R<DG_PPUD_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DG_PPUD_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DG_PPUD_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DG_PPUD_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dg_ppud_0` writer"]
pub struct W(crate::W<DG_PPUD_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DG_PPUD_0_SPEC>;
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
impl From<crate::W<DG_PPUD_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DG_PPUD_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ppud_manaual_en` reader - "]
pub struct PPUD_MANAUAL_EN_R(crate::FieldReader<bool, bool>);
impl PPUD_MANAUAL_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PPUD_MANAUAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPUD_MANAUAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppud_manaual_en` writer - "]
pub struct PPUD_MANAUAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PPUD_MANAUAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ppud_cnt1` reader - "]
pub struct PPUD_CNT1_R(crate::FieldReader<u8, u8>);
impl PPUD_CNT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PPUD_CNT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPUD_CNT1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppud_cnt1` writer - "]
pub struct PPUD_CNT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PPUD_CNT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
#[doc = "Field `ppud_cnt2` reader - "]
pub struct PPUD_CNT2_R(crate::FieldReader<u16, u16>);
impl PPUD_CNT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PPUD_CNT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPUD_CNT2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppud_cnt2` writer - "]
pub struct PPUD_CNT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PPUD_CNT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | ((value as u32 & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ppud_manaual_en(&self) -> PPUD_MANAUAL_EN_R {
        PPUD_MANAUAL_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn ppud_cnt1(&self) -> PPUD_CNT1_R {
        PPUD_CNT1_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn ppud_cnt2(&self) -> PPUD_CNT2_R {
        PPUD_CNT2_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ppud_manaual_en(&mut self) -> PPUD_MANAUAL_EN_W {
        PPUD_MANAUAL_EN_W { w: self }
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn ppud_cnt1(&mut self) -> PPUD_CNT1_W {
        PPUD_CNT1_W { w: self }
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn ppud_cnt2(&mut self) -> PPUD_CNT2_W {
        PPUD_CNT2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dg_ppud_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dg_ppud_0](index.html) module"]
pub struct DG_PPUD_0_SPEC;
impl crate::RegisterSpec for DG_PPUD_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dg_ppud_0::R](R) reader structure"]
impl crate::Readable for DG_PPUD_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dg_ppud_0::W](W) writer structure"]
impl crate::Writable for DG_PPUD_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dg_ppud_0 to value 0"]
impl crate::Resettable for DG_PPUD_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
