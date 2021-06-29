#[doc = "Register `i2s_io_config` reader"]
pub struct R(crate::R<I2S_IO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_IO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_IO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_IO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_io_config` writer"]
pub struct W(crate::W<I2S_IO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_IO_CONFIG_SPEC>;
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
impl From<crate::W<I2S_IO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_IO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_deg_en` reader - "]
pub struct CR_DEG_EN_R(crate::FieldReader<bool, bool>);
impl CR_DEG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_DEG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_DEG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_deg_en` writer - "]
pub struct CR_DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_DEG_EN_W<'a> {
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
#[doc = "Field `cr_deg_cnt` reader - "]
pub struct CR_DEG_CNT_R(crate::FieldReader<u8, u8>);
impl CR_DEG_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_DEG_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_DEG_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_deg_cnt` writer - "]
pub struct CR_DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `cr_i2s_bclk_inv` reader - "]
pub struct CR_I2S_BCLK_INV_R(crate::FieldReader<bool, bool>);
impl CR_I2S_BCLK_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_BCLK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_BCLK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_bclk_inv` writer - "]
pub struct CR_I2S_BCLK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_BCLK_INV_W<'a> {
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
#[doc = "Field `cr_i2s_fs_inv` reader - "]
pub struct CR_I2S_FS_INV_R(crate::FieldReader<bool, bool>);
impl CR_I2S_FS_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_FS_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_FS_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_fs_inv` writer - "]
pub struct CR_I2S_FS_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_FS_INV_W<'a> {
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
#[doc = "Field `cr_i2s_rxd_inv` reader - "]
pub struct CR_I2S_RXD_INV_R(crate::FieldReader<bool, bool>);
impl CR_I2S_RXD_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_RXD_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_RXD_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_rxd_inv` writer - "]
pub struct CR_I2S_RXD_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_RXD_INV_W<'a> {
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
#[doc = "Field `cr_i2s_txd_inv` reader - "]
pub struct CR_I2S_TXD_INV_R(crate::FieldReader<bool, bool>);
impl CR_I2S_TXD_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_TXD_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_TXD_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_txd_inv` writer - "]
pub struct CR_I2S_TXD_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_TXD_INV_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_deg_en(&self) -> CR_DEG_EN_R {
        CR_DEG_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn cr_deg_cnt(&self) -> CR_DEG_CNT_R {
        CR_DEG_CNT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2s_bclk_inv(&self) -> CR_I2S_BCLK_INV_R {
        CR_I2S_BCLK_INV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2s_fs_inv(&self) -> CR_I2S_FS_INV_R {
        CR_I2S_FS_INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2s_rxd_inv(&self) -> CR_I2S_RXD_INV_R {
        CR_I2S_RXD_INV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2s_txd_inv(&self) -> CR_I2S_TXD_INV_R {
        CR_I2S_TXD_INV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_deg_en(&mut self) -> CR_DEG_EN_W {
        CR_DEG_EN_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn cr_deg_cnt(&mut self) -> CR_DEG_CNT_W {
        CR_DEG_CNT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2s_bclk_inv(&mut self) -> CR_I2S_BCLK_INV_W {
        CR_I2S_BCLK_INV_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2s_fs_inv(&mut self) -> CR_I2S_FS_INV_W {
        CR_I2S_FS_INV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2s_rxd_inv(&mut self) -> CR_I2S_RXD_INV_W {
        CR_I2S_RXD_INV_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2s_txd_inv(&mut self) -> CR_I2S_TXD_INV_W {
        CR_I2S_TXD_INV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2s_io_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_io_config](index.html) module"]
pub struct I2S_IO_CONFIG_SPEC;
impl crate::RegisterSpec for I2S_IO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_io_config::R](R) reader structure"]
impl crate::Readable for I2S_IO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_io_config::W](W) writer structure"]
impl crate::Writable for I2S_IO_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2s_io_config to value 0"]
impl crate::Resettable for I2S_IO_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
