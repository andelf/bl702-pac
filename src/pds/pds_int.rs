#[doc = "Register `PDS_INT` reader"]
pub struct R(crate::R<PDS_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_INT` writer"]
pub struct W(crate::W<PDS_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_INT_SPEC>;
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
impl From<crate::W<PDS_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ro_pds_wakeup_event` reader - "]
pub struct RO_PDS_WAKEUP_EVENT_R(crate::FieldReader<u8, u8>);
impl RO_PDS_WAKEUP_EVENT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RO_PDS_WAKEUP_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_WAKEUP_EVENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_pds_wakeup_event` writer - "]
pub struct RO_PDS_WAKEUP_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_WAKEUP_EVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `cr_pds_wakeup_src_en` reader - "]
pub struct CR_PDS_WAKEUP_SRC_EN_R(crate::FieldReader<u8, u8>);
impl CR_PDS_WAKEUP_SRC_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_PDS_WAKEUP_SRC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_WAKEUP_SRC_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_wakeup_src_en` writer - "]
pub struct CR_PDS_WAKEUP_SRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WAKEUP_SRC_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `cr_pds_int_clr` reader - "]
pub struct CR_PDS_INT_CLR_R(crate::FieldReader<bool, bool>);
impl CR_PDS_INT_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_int_clr` writer - "]
pub struct CR_PDS_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_INT_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `cr_pds_pll_done_int_mask` reader - "]
pub struct CR_PDS_PLL_DONE_INT_MASK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_PLL_DONE_INT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_PLL_DONE_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_PLL_DONE_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_pll_done_int_mask` writer - "]
pub struct CR_PDS_PLL_DONE_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_PLL_DONE_INT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `cr_pds_rf_done_int_mask` reader - "]
pub struct CR_PDS_RF_DONE_INT_MASK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_RF_DONE_INT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_RF_DONE_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_RF_DONE_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_rf_done_int_mask` writer - "]
pub struct CR_PDS_RF_DONE_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_RF_DONE_INT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `cr_pds_wake_int_mask` reader - "]
pub struct CR_PDS_WAKE_INT_MASK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_WAKE_INT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_WAKE_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_WAKE_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_wake_int_mask` writer - "]
pub struct CR_PDS_WAKE_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WAKE_INT_MASK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `pds_clr_reset_event` reader - "]
pub struct PDS_CLR_RESET_EVENT_R(crate::FieldReader<bool, bool>);
impl PDS_CLR_RESET_EVENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDS_CLR_RESET_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDS_CLR_RESET_EVENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pds_clr_reset_event` writer - "]
pub struct PDS_CLR_RESET_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> PDS_CLR_RESET_EVENT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `pds_reset_event` reader - "]
pub struct PDS_RESET_EVENT_R(crate::FieldReader<u8, u8>);
impl PDS_RESET_EVENT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDS_RESET_EVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDS_RESET_EVENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pds_reset_event` writer - "]
pub struct PDS_RESET_EVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> PDS_RESET_EVENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `ro_pds_pll_done_int` reader - "]
pub struct RO_PDS_PLL_DONE_INT_R(crate::FieldReader<bool, bool>);
impl RO_PDS_PLL_DONE_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RO_PDS_PLL_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_PLL_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_pds_pll_done_int` writer - "]
pub struct RO_PDS_PLL_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_PLL_DONE_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ro_pds_rf_done_int` reader - "]
pub struct RO_PDS_RF_DONE_INT_R(crate::FieldReader<bool, bool>);
impl RO_PDS_RF_DONE_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RO_PDS_RF_DONE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_RF_DONE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_pds_rf_done_int` writer - "]
pub struct RO_PDS_RF_DONE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_RF_DONE_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ro_pds_wake_int` reader - "]
pub struct RO_PDS_WAKE_INT_R(crate::FieldReader<bool, bool>);
impl RO_PDS_WAKE_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RO_PDS_WAKE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_PDS_WAKE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_pds_wake_int` writer - "]
pub struct RO_PDS_WAKE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_PDS_WAKE_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ro_pds_wakeup_event(&self) -> RO_PDS_WAKEUP_EVENT_R {
        RO_PDS_WAKEUP_EVENT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_pds_wakeup_src_en(&self) -> CR_PDS_WAKEUP_SRC_EN_R {
        CR_PDS_WAKEUP_SRC_EN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&self) -> CR_PDS_INT_CLR_R {
        CR_PDS_INT_CLR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_pll_done_int_mask(&self) -> CR_PDS_PLL_DONE_INT_MASK_R {
        CR_PDS_PLL_DONE_INT_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&self) -> CR_PDS_RF_DONE_INT_MASK_R {
        CR_PDS_RF_DONE_INT_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&self) -> CR_PDS_WAKE_INT_MASK_R {
        CR_PDS_WAKE_INT_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pds_clr_reset_event(&self) -> PDS_CLR_RESET_EVENT_R {
        PDS_CLR_RESET_EVENT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pds_reset_event(&self) -> PDS_RESET_EVENT_R {
        PDS_RESET_EVENT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_pll_done_int(&self) -> RO_PDS_PLL_DONE_INT_R {
        RO_PDS_PLL_DONE_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&self) -> RO_PDS_RF_DONE_INT_R {
        RO_PDS_RF_DONE_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&self) -> RO_PDS_WAKE_INT_R {
        RO_PDS_WAKE_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ro_pds_wakeup_event(&mut self) -> RO_PDS_WAKEUP_EVENT_W {
        RO_PDS_WAKEUP_EVENT_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_pds_wakeup_src_en(&mut self) -> CR_PDS_WAKEUP_SRC_EN_W {
        CR_PDS_WAKEUP_SRC_EN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_int_clr(&mut self) -> CR_PDS_INT_CLR_W {
        CR_PDS_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_pll_done_int_mask(&mut self) -> CR_PDS_PLL_DONE_INT_MASK_W {
        CR_PDS_PLL_DONE_INT_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_rf_done_int_mask(&mut self) -> CR_PDS_RF_DONE_INT_MASK_W {
        CR_PDS_RF_DONE_INT_MASK_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_wake_int_mask(&mut self) -> CR_PDS_WAKE_INT_MASK_W {
        CR_PDS_WAKE_INT_MASK_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pds_clr_reset_event(&mut self) -> PDS_CLR_RESET_EVENT_W {
        PDS_CLR_RESET_EVENT_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pds_reset_event(&mut self) -> PDS_RESET_EVENT_W {
        PDS_RESET_EVENT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ro_pds_pll_done_int(&mut self) -> RO_PDS_PLL_DONE_INT_W {
        RO_PDS_PLL_DONE_INT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ro_pds_rf_done_int(&mut self) -> RO_PDS_RF_DONE_INT_W {
        RO_PDS_RF_DONE_INT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ro_pds_wake_int(&mut self) -> RO_PDS_WAKE_INT_W {
        RO_PDS_WAKE_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_INT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_int](index.html) module"]
pub struct PDS_INT_SPEC;
impl crate::RegisterSpec for PDS_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_int::R](R) reader structure"]
impl crate::Readable for PDS_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_int::W](W) writer structure"]
impl crate::Writable for PDS_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDS_INT to value 0"]
impl crate::Resettable for PDS_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
