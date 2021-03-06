#[doc = "Register `sf_if_iahb_3` reader"]
pub struct R(crate::R<SF_IF_IAHB_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_IF_IAHB_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_IF_IAHB_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_IF_IAHB_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_if_iahb_3` writer"]
pub struct W(crate::W<SF_IF_IAHB_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_IF_IAHB_3_SPEC>;
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
impl From<crate::W<SF_IF_IAHB_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_IF_IAHB_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_if_2_qpi_mode_en` reader - "]
pub struct SF_IF_2_QPI_MODE_EN_R(crate::FieldReader<bool, bool>);
impl SF_IF_2_QPI_MODE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_2_QPI_MODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_2_QPI_MODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_2_qpi_mode_en` writer - "]
pub struct SF_IF_2_QPI_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_2_QPI_MODE_EN_W<'a> {
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
#[doc = "Field `sf_if_2_spi_mode` reader - "]
pub struct SF_IF_2_SPI_MODE_R(crate::FieldReader<u8, u8>);
impl SF_IF_2_SPI_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IF_2_SPI_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_2_SPI_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_2_spi_mode` writer - "]
pub struct SF_IF_2_SPI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_2_SPI_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `sf_if_2_cmd_en` reader - "]
pub struct SF_IF_2_CMD_EN_R(crate::FieldReader<bool, bool>);
impl SF_IF_2_CMD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_2_CMD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_2_CMD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_2_cmd_en` writer - "]
pub struct SF_IF_2_CMD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_2_CMD_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `sf_if_2_adr_en` reader - "]
pub struct SF_IF_2_ADR_EN_R(crate::FieldReader<bool, bool>);
impl SF_IF_2_ADR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_2_ADR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_2_ADR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_2_adr_en` writer - "]
pub struct SF_IF_2_ADR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_2_ADR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `sf_if_2_dmy_en` reader - "]
pub struct SF_IF_2_DMY_EN_R(crate::FieldReader<bool, bool>);
impl SF_IF_2_DMY_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_2_DMY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_2_DMY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_2_dmy_en` writer - "]
pub struct SF_IF_2_DMY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_2_DMY_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `sf_if_2_dat_en` reader - "]
pub struct SF_IF_2_DAT_EN_R(crate::FieldReader<bool, bool>);
impl SF_IF_2_DAT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_2_DAT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_2_DAT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_2_dat_en` writer - "]
pub struct SF_IF_2_DAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_2_DAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `sf_if_2_dat_rw  ` reader - "]
pub struct SF_IF_2_DAT_RW_R(crate::FieldReader<bool, bool>);
impl SF_IF_2_DAT_RW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_2_DAT_RW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_2_DAT_RW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_2_dat_rw  ` writer - "]
pub struct SF_IF_2_DAT_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_2_DAT_RW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `sf_if_2_cmd_byte` reader - "]
pub struct SF_IF_2_CMD_BYTE_R(crate::FieldReader<u8, u8>);
impl SF_IF_2_CMD_BYTE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IF_2_CMD_BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_2_CMD_BYTE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_2_cmd_byte` writer - "]
pub struct SF_IF_2_CMD_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_2_CMD_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `sf_if_2_adr_byte` reader - "]
pub struct SF_IF_2_ADR_BYTE_R(crate::FieldReader<u8, u8>);
impl SF_IF_2_ADR_BYTE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IF_2_ADR_BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_2_ADR_BYTE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_2_adr_byte` writer - "]
pub struct SF_IF_2_ADR_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_2_ADR_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "Field `sf_if_2_dmy_byte` reader - "]
pub struct SF_IF_2_DMY_BYTE_R(crate::FieldReader<u8, u8>);
impl SF_IF_2_DMY_BYTE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IF_2_DMY_BYTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_2_DMY_BYTE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_2_dmy_byte` writer - "]
pub struct SF_IF_2_DMY_BYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_2_DMY_BYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | ((value as u32 & 0x1f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_2_qpi_mode_en(&self) -> SF_IF_2_QPI_MODE_EN_R {
        SF_IF_2_QPI_MODE_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_2_spi_mode(&self) -> SF_IF_2_SPI_MODE_R {
        SF_IF_2_SPI_MODE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_2_cmd_en(&self) -> SF_IF_2_CMD_EN_R {
        SF_IF_2_CMD_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_2_adr_en(&self) -> SF_IF_2_ADR_EN_R {
        SF_IF_2_ADR_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_2_dmy_en(&self) -> SF_IF_2_DMY_EN_R {
        SF_IF_2_DMY_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_2_dat_en(&self) -> SF_IF_2_DAT_EN_R {
        SF_IF_2_DAT_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_2_dat_rw(&self) -> SF_IF_2_DAT_RW_R {
        SF_IF_2_DAT_RW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_2_cmd_byte(&self) -> SF_IF_2_CMD_BYTE_R {
        SF_IF_2_CMD_BYTE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_2_adr_byte(&self) -> SF_IF_2_ADR_BYTE_R {
        SF_IF_2_ADR_BYTE_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_2_dmy_byte(&self) -> SF_IF_2_DMY_BYTE_R {
        SF_IF_2_DMY_BYTE_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_if_2_qpi_mode_en(&mut self) -> SF_IF_2_QPI_MODE_EN_W {
        SF_IF_2_QPI_MODE_EN_W { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn sf_if_2_spi_mode(&mut self) -> SF_IF_2_SPI_MODE_W {
        SF_IF_2_SPI_MODE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_if_2_cmd_en(&mut self) -> SF_IF_2_CMD_EN_W {
        SF_IF_2_CMD_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_if_2_adr_en(&mut self) -> SF_IF_2_ADR_EN_W {
        SF_IF_2_ADR_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_2_dmy_en(&mut self) -> SF_IF_2_DMY_EN_W {
        SF_IF_2_DMY_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_2_dat_en(&mut self) -> SF_IF_2_DAT_EN_W {
        SF_IF_2_DAT_EN_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_if_2_dat_rw(&mut self) -> SF_IF_2_DAT_RW_W {
        SF_IF_2_DAT_RW_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_2_cmd_byte(&mut self) -> SF_IF_2_CMD_BYTE_W {
        SF_IF_2_CMD_BYTE_W { w: self }
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn sf_if_2_adr_byte(&mut self) -> SF_IF_2_ADR_BYTE_W {
        SF_IF_2_ADR_BYTE_W { w: self }
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn sf_if_2_dmy_byte(&mut self) -> SF_IF_2_DMY_BYTE_W {
        SF_IF_2_DMY_BYTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_if_iahb_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_iahb_3](index.html) module"]
pub struct SF_IF_IAHB_3_SPEC;
impl crate::RegisterSpec for SF_IF_IAHB_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_if_iahb_3::R](R) reader structure"]
impl crate::Readable for SF_IF_IAHB_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_if_iahb_3::W](W) writer structure"]
impl crate::Writable for SF_IF_IAHB_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_if_iahb_3 to value 0"]
impl crate::Resettable for SF_IF_IAHB_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
