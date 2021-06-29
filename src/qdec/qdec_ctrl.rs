#[doc = "Register `qdec_ctrl` reader"]
pub struct R(crate::R<QDEC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDEC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDEC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDEC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `qdec_ctrl` writer"]
pub struct W(crate::W<QDEC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDEC_CTRL_SPEC>;
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
impl From<crate::W<QDEC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDEC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `input_swap` reader - "]
pub struct INPUT_SWAP_R(crate::FieldReader<bool, bool>);
impl INPUT_SWAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        INPUT_SWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_SWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `input_swap` writer - "]
pub struct INPUT_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_SWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `rpt_mode` reader - "]
pub struct RPT_MODE_R(crate::FieldReader<bool, bool>);
impl RPT_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RPT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rpt_mode` writer - "]
pub struct RPT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RPT_MODE_W<'a> {
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
#[doc = "Field `spl_mode` reader - "]
pub struct SPL_MODE_R(crate::FieldReader<bool, bool>);
impl SPL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPL_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPL_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spl_mode` writer - "]
pub struct SPL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPL_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `led_period` reader - "]
pub struct LED_PERIOD_R(crate::FieldReader<u16, u16>);
impl LED_PERIOD_R {
    pub(crate) fn new(bits: u16) -> Self {
        LED_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LED_PERIOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `led_period` writer - "]
pub struct LED_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 20)) | ((value as u32 & 0x01ff) << 20);
        self.w
    }
}
#[doc = "Field `rpt_period` reader - "]
pub struct RPT_PERIOD_R(crate::FieldReader<u8, u8>);
impl RPT_PERIOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPT_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT_PERIOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rpt_period` writer - "]
pub struct RPT_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RPT_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | ((value as u32 & 0xff) << 12);
        self.w
    }
}
#[doc = "Field `spl_period` reader - "]
pub struct SPL_PERIOD_R(crate::FieldReader<u8, u8>);
impl SPL_PERIOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPL_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPL_PERIOD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spl_period` writer - "]
pub struct SPL_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPL_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `deg_cnt` reader - "]
pub struct DEG_CNT_R(crate::FieldReader<u8, u8>);
impl DEG_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEG_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEG_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `deg_cnt` writer - "]
pub struct DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `deg_en` reader - "]
pub struct DEG_EN_R(crate::FieldReader<bool, bool>);
impl DEG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `deg_en` writer - "]
pub struct DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEG_EN_W<'a> {
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
#[doc = "Field `led_pol` reader - "]
pub struct LED_POL_R(crate::FieldReader<bool, bool>);
impl LED_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LED_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LED_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `led_pol` writer - "]
pub struct LED_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_POL_W<'a> {
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
#[doc = "Field `led_en` reader - "]
pub struct LED_EN_R(crate::FieldReader<bool, bool>);
impl LED_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LED_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LED_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `led_en` writer - "]
pub struct LED_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `qdec_en` reader - "]
pub struct QDEC_EN_R(crate::FieldReader<bool, bool>);
impl QDEC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        QDEC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QDEC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `qdec_en` writer - "]
pub struct QDEC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEC_EN_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn input_swap(&self) -> INPUT_SWAP_R {
        INPUT_SWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rpt_mode(&self) -> RPT_MODE_R {
        RPT_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spl_mode(&self) -> SPL_MODE_R {
        SPL_MODE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn led_period(&self) -> LED_PERIOD_R {
        LED_PERIOD_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn rpt_period(&self) -> RPT_PERIOD_R {
        RPT_PERIOD_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn spl_period(&self) -> SPL_PERIOD_R {
        SPL_PERIOD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn deg_cnt(&self) -> DEG_CNT_R {
        DEG_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn deg_en(&self) -> DEG_EN_R {
        DEG_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_pol(&self) -> LED_POL_R {
        LED_POL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn led_en(&self) -> LED_EN_R {
        LED_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn qdec_en(&self) -> QDEC_EN_R {
        QDEC_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn input_swap(&mut self) -> INPUT_SWAP_W {
        INPUT_SWAP_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rpt_mode(&mut self) -> RPT_MODE_W {
        RPT_MODE_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spl_mode(&mut self) -> SPL_MODE_W {
        SPL_MODE_W { w: self }
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn led_period(&mut self) -> LED_PERIOD_W {
        LED_PERIOD_W { w: self }
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn rpt_period(&mut self) -> RPT_PERIOD_W {
        RPT_PERIOD_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn spl_period(&mut self) -> SPL_PERIOD_W {
        SPL_PERIOD_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn deg_cnt(&mut self) -> DEG_CNT_W {
        DEG_CNT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn deg_en(&mut self) -> DEG_EN_W {
        DEG_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_pol(&mut self) -> LED_POL_W {
        LED_POL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn led_en(&mut self) -> LED_EN_W {
        LED_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn qdec_en(&mut self) -> QDEC_EN_W {
        QDEC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "qdec_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdec_ctrl](index.html) module"]
pub struct QDEC_CTRL_SPEC;
impl crate::RegisterSpec for QDEC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdec_ctrl::R](R) reader structure"]
impl crate::Readable for QDEC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdec_ctrl::W](W) writer structure"]
impl crate::Writable for QDEC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets qdec_ctrl to value 0"]
impl crate::Resettable for QDEC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
