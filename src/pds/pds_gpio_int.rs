#[doc = "Register `pds_gpio_int` reader"]
pub struct R(crate::R<PDS_GPIO_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_GPIO_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_GPIO_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_GPIO_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_gpio_int` writer"]
pub struct W(crate::W<PDS_GPIO_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_GPIO_INT_SPEC>;
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
impl From<crate::W<PDS_GPIO_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_GPIO_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pds_gpio_int_select` reader - "]
pub struct PDS_GPIO_INT_SELECT_R(crate::FieldReader<u8, u8>);
impl PDS_GPIO_INT_SELECT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDS_GPIO_INT_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDS_GPIO_INT_SELECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pds_gpio_int_select` writer - "]
pub struct PDS_GPIO_INT_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PDS_GPIO_INT_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `pds_gpio_int_mode` reader - "]
pub struct PDS_GPIO_INT_MODE_R(crate::FieldReader<u8, u8>);
impl PDS_GPIO_INT_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDS_GPIO_INT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDS_GPIO_INT_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pds_gpio_int_mode` writer - "]
pub struct PDS_GPIO_INT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDS_GPIO_INT_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `pds_gpio_int_clr` reader - "]
pub struct PDS_GPIO_INT_CLR_R(crate::FieldReader<bool, bool>);
impl PDS_GPIO_INT_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDS_GPIO_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDS_GPIO_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pds_gpio_int_clr` writer - "]
pub struct PDS_GPIO_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PDS_GPIO_INT_CLR_W<'a> {
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
#[doc = "Field `pds_gpio_int_stat` reader - "]
pub struct PDS_GPIO_INT_STAT_R(crate::FieldReader<bool, bool>);
impl PDS_GPIO_INT_STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDS_GPIO_INT_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDS_GPIO_INT_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pds_gpio_int_stat` writer - "]
pub struct PDS_GPIO_INT_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> PDS_GPIO_INT_STAT_W<'a> {
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
#[doc = "Field `pds_gpio_int_mask` reader - "]
pub struct PDS_GPIO_INT_MASK_R(crate::FieldReader<bool, bool>);
impl PDS_GPIO_INT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDS_GPIO_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDS_GPIO_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pds_gpio_int_mask` writer - "]
pub struct PDS_GPIO_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> PDS_GPIO_INT_MASK_W<'a> {
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
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pds_gpio_int_select(&self) -> PDS_GPIO_INT_SELECT_R {
        PDS_GPIO_INT_SELECT_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pds_gpio_int_mode(&self) -> PDS_GPIO_INT_MODE_R {
        PDS_GPIO_INT_MODE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pds_gpio_int_clr(&self) -> PDS_GPIO_INT_CLR_R {
        PDS_GPIO_INT_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pds_gpio_int_stat(&self) -> PDS_GPIO_INT_STAT_R {
        PDS_GPIO_INT_STAT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pds_gpio_int_mask(&self) -> PDS_GPIO_INT_MASK_R {
        PDS_GPIO_INT_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pds_gpio_int_select(&mut self) -> PDS_GPIO_INT_SELECT_W {
        PDS_GPIO_INT_SELECT_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pds_gpio_int_mode(&mut self) -> PDS_GPIO_INT_MODE_W {
        PDS_GPIO_INT_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pds_gpio_int_clr(&mut self) -> PDS_GPIO_INT_CLR_W {
        PDS_GPIO_INT_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pds_gpio_int_stat(&mut self) -> PDS_GPIO_INT_STAT_W {
        PDS_GPIO_INT_STAT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pds_gpio_int_mask(&mut self) -> PDS_GPIO_INT_MASK_W {
        PDS_GPIO_INT_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_gpio_int.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_gpio_int](index.html) module"]
pub struct PDS_GPIO_INT_SPEC;
impl crate::RegisterSpec for PDS_GPIO_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_gpio_int::R](R) reader structure"]
impl crate::Readable for PDS_GPIO_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_gpio_int::W](W) writer structure"]
impl crate::Writable for PDS_GPIO_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pds_gpio_int to value 0"]
impl crate::Resettable for PDS_GPIO_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
