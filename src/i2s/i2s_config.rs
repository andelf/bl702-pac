#[doc = "Register `i2s_config` reader"]
pub struct R(crate::R<I2S_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_config` writer"]
pub struct W(crate::W<I2S_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_CONFIG_SPEC>;
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
impl From<crate::W<I2S_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_ofs_en` reader - "]
pub struct CR_OFS_EN_R(crate::FieldReader<bool, bool>);
impl CR_OFS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_OFS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_OFS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ofs_en` writer - "]
pub struct CR_OFS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_OFS_EN_W<'a> {
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
#[doc = "Field `cr_ofs_cnt` reader - "]
pub struct CR_OFS_CNT_R(crate::FieldReader<u8, u8>);
impl CR_OFS_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CR_OFS_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_OFS_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_ofs_cnt` writer - "]
pub struct CR_OFS_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_OFS_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `cr_mono_rx_ch` reader - "]
pub struct CR_MONO_RX_CH_R(crate::FieldReader<bool, bool>);
impl CR_MONO_RX_CH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_MONO_RX_CH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_MONO_RX_CH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_mono_rx_ch` writer - "]
pub struct CR_MONO_RX_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_MONO_RX_CH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `cr_endian` reader - "]
pub struct CR_ENDIAN_R(crate::FieldReader<bool, bool>);
impl CR_ENDIAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_endian` writer - "]
pub struct CR_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_ENDIAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `cr_i2s_mode` reader - "]
pub struct CR_I2S_MODE_R(crate::FieldReader<u8, u8>);
impl CR_I2S_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CR_I2S_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_mode` writer - "]
pub struct CR_I2S_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `cr_data_size` reader - "]
pub struct CR_DATA_SIZE_R(crate::FieldReader<u8, u8>);
impl CR_DATA_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CR_DATA_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_DATA_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_data_size` writer - "]
pub struct CR_DATA_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_DATA_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `cr_frame_size` reader - "]
pub struct CR_FRAME_SIZE_R(crate::FieldReader<u8, u8>);
impl CR_FRAME_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CR_FRAME_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_FRAME_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_frame_size` writer - "]
pub struct CR_FRAME_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_FRAME_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `cr_fs_3ch_mode` reader - "]
pub struct CR_FS_3CH_MODE_R(crate::FieldReader<bool, bool>);
impl CR_FS_3CH_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_FS_3CH_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_FS_3CH_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_fs_3ch_mode` writer - "]
pub struct CR_FS_3CH_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_FS_3CH_MODE_W<'a> {
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
#[doc = "Field `cr_fs_4ch_mode` reader - "]
pub struct CR_FS_4CH_MODE_R(crate::FieldReader<bool, bool>);
impl CR_FS_4CH_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_FS_4CH_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_FS_4CH_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_fs_4ch_mode` writer - "]
pub struct CR_FS_4CH_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_FS_4CH_MODE_W<'a> {
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
#[doc = "Field `cr_fs_1t_mode` reader - "]
pub struct CR_FS_1T_MODE_R(crate::FieldReader<bool, bool>);
impl CR_FS_1T_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_FS_1T_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_FS_1T_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_fs_1t_mode` writer - "]
pub struct CR_FS_1T_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_FS_1T_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `cr_mute_mode` reader - "]
pub struct CR_MUTE_MODE_R(crate::FieldReader<bool, bool>);
impl CR_MUTE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_MUTE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_MUTE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_mute_mode` writer - "]
pub struct CR_MUTE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_MUTE_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `cr_mono_mode` reader - "]
pub struct CR_MONO_MODE_R(crate::FieldReader<bool, bool>);
impl CR_MONO_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_MONO_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_MONO_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_mono_mode` writer - "]
pub struct CR_MONO_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_MONO_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `cr_i2s_rxd_en` reader - "]
pub struct CR_I2S_RXD_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2S_RXD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_RXD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_RXD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_rxd_en` writer - "]
pub struct CR_I2S_RXD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_RXD_EN_W<'a> {
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
#[doc = "Field `cr_i2s_txd_en` reader - "]
pub struct CR_I2S_TXD_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2S_TXD_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_TXD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_TXD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_txd_en` writer - "]
pub struct CR_I2S_TXD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_TXD_EN_W<'a> {
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
#[doc = "Field `cr_i2s_s_en` reader - "]
pub struct CR_I2S_S_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2S_S_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_s_en` writer - "]
pub struct CR_I2S_S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_S_EN_W<'a> {
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
#[doc = "Field `cr_i2s_m_en` reader - "]
pub struct CR_I2S_M_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2S_M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2S_M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2S_M_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2s_m_en` writer - "]
pub struct CR_I2S_M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2S_M_EN_W<'a> {
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
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_ofs_en(&self) -> CR_OFS_EN_R {
        CR_OFS_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn cr_ofs_cnt(&self) -> CR_OFS_CNT_R {
        CR_OFS_CNT_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_mono_rx_ch(&self) -> CR_MONO_RX_CH_R {
        CR_MONO_RX_CH_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_endian(&self) -> CR_ENDIAN_R {
        CR_ENDIAN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cr_i2s_mode(&self) -> CR_I2S_MODE_R {
        CR_I2S_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cr_data_size(&self) -> CR_DATA_SIZE_R {
        CR_DATA_SIZE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_frame_size(&self) -> CR_FRAME_SIZE_R {
        CR_FRAME_SIZE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_fs_3ch_mode(&self) -> CR_FS_3CH_MODE_R {
        CR_FS_3CH_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_fs_4ch_mode(&self) -> CR_FS_4CH_MODE_R {
        CR_FS_4CH_MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_fs_1t_mode(&self) -> CR_FS_1T_MODE_R {
        CR_FS_1T_MODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_mute_mode(&self) -> CR_MUTE_MODE_R {
        CR_MUTE_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_mono_mode(&self) -> CR_MONO_MODE_R {
        CR_MONO_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2s_rxd_en(&self) -> CR_I2S_RXD_EN_R {
        CR_I2S_RXD_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2s_txd_en(&self) -> CR_I2S_TXD_EN_R {
        CR_I2S_TXD_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2s_s_en(&self) -> CR_I2S_S_EN_R {
        CR_I2S_S_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2s_m_en(&self) -> CR_I2S_M_EN_R {
        CR_I2S_M_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_ofs_en(&mut self) -> CR_OFS_EN_W {
        CR_OFS_EN_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn cr_ofs_cnt(&mut self) -> CR_OFS_CNT_W {
        CR_OFS_CNT_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_mono_rx_ch(&mut self) -> CR_MONO_RX_CH_W {
        CR_MONO_RX_CH_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_endian(&mut self) -> CR_ENDIAN_W {
        CR_ENDIAN_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cr_i2s_mode(&mut self) -> CR_I2S_MODE_W {
        CR_I2S_MODE_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cr_data_size(&mut self) -> CR_DATA_SIZE_W {
        CR_DATA_SIZE_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_frame_size(&mut self) -> CR_FRAME_SIZE_W {
        CR_FRAME_SIZE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_fs_3ch_mode(&mut self) -> CR_FS_3CH_MODE_W {
        CR_FS_3CH_MODE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_fs_4ch_mode(&mut self) -> CR_FS_4CH_MODE_W {
        CR_FS_4CH_MODE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_fs_1t_mode(&mut self) -> CR_FS_1T_MODE_W {
        CR_FS_1T_MODE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_mute_mode(&mut self) -> CR_MUTE_MODE_W {
        CR_MUTE_MODE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_mono_mode(&mut self) -> CR_MONO_MODE_W {
        CR_MONO_MODE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2s_rxd_en(&mut self) -> CR_I2S_RXD_EN_W {
        CR_I2S_RXD_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2s_txd_en(&mut self) -> CR_I2S_TXD_EN_W {
        CR_I2S_TXD_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2s_s_en(&mut self) -> CR_I2S_S_EN_W {
        CR_I2S_S_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2s_m_en(&mut self) -> CR_I2S_M_EN_W {
        CR_I2S_M_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2s_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_config](index.html) module"]
pub struct I2S_CONFIG_SPEC;
impl crate::RegisterSpec for I2S_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_config::R](R) reader structure"]
impl crate::Readable for I2S_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_config::W](W) writer structure"]
impl crate::Writable for I2S_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2s_config to value 0"]
impl crate::Resettable for I2S_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
