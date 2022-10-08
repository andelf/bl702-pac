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
#[doc = "Field `cr_i2s_m_en` reader - "]
pub type CR_I2S_M_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2s_m_en` writer - "]
pub type CR_I2S_M_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2s_s_en` reader - "]
pub type CR_I2S_S_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2s_s_en` writer - "]
pub type CR_I2S_S_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2s_txd_en` reader - "]
pub type CR_I2S_TXD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2s_txd_en` writer - "]
pub type CR_I2S_TXD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2s_rxd_en` reader - "]
pub type CR_I2S_RXD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2s_rxd_en` writer - "]
pub type CR_I2S_RXD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_mono_mode` reader - "]
pub type CR_MONO_MODE_R = crate::BitReader<bool>;
#[doc = "Field `cr_mono_mode` writer - "]
pub type CR_MONO_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_mute_mode` reader - "]
pub type CR_MUTE_MODE_R = crate::BitReader<bool>;
#[doc = "Field `cr_mute_mode` writer - "]
pub type CR_MUTE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_fs_1t_mode` reader - "]
pub type CR_FS_1T_MODE_R = crate::BitReader<bool>;
#[doc = "Field `cr_fs_1t_mode` writer - "]
pub type CR_FS_1T_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_fs_4ch_mode` reader - "]
pub type CR_FS_4CH_MODE_R = crate::BitReader<bool>;
#[doc = "Field `cr_fs_4ch_mode` writer - "]
pub type CR_FS_4CH_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_fs_3ch_mode` reader - "]
pub type CR_FS_3CH_MODE_R = crate::BitReader<bool>;
#[doc = "Field `cr_fs_3ch_mode` writer - "]
pub type CR_FS_3CH_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_frame_size` reader - "]
pub type CR_FRAME_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_frame_size` writer - "]
pub type CR_FRAME_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_data_size` reader - "]
pub type CR_DATA_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_data_size` writer - "]
pub type CR_DATA_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_i2s_mode` reader - "]
pub type CR_I2S_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_i2s_mode` writer - "]
pub type CR_I2S_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `cr_endian` reader - "]
pub type CR_ENDIAN_R = crate::BitReader<bool>;
#[doc = "Field `cr_endian` writer - "]
pub type CR_ENDIAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_mono_rx_ch` reader - "]
pub type CR_MONO_RX_CH_R = crate::BitReader<bool>;
#[doc = "Field `cr_mono_rx_ch` writer - "]
pub type CR_MONO_RX_CH_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_ofs_cnt` reader - "]
pub type CR_OFS_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_ofs_cnt` writer - "]
pub type CR_OFS_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2S_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `cr_ofs_en` reader - "]
pub type CR_OFS_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ofs_en` writer - "]
pub type CR_OFS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2s_m_en(&self) -> CR_I2S_M_EN_R {
        CR_I2S_M_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2s_s_en(&self) -> CR_I2S_S_EN_R {
        CR_I2S_S_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2s_txd_en(&self) -> CR_I2S_TXD_EN_R {
        CR_I2S_TXD_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2s_rxd_en(&self) -> CR_I2S_RXD_EN_R {
        CR_I2S_RXD_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_mono_mode(&self) -> CR_MONO_MODE_R {
        CR_MONO_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_mute_mode(&self) -> CR_MUTE_MODE_R {
        CR_MUTE_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_fs_1t_mode(&self) -> CR_FS_1T_MODE_R {
        CR_FS_1T_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_fs_4ch_mode(&self) -> CR_FS_4CH_MODE_R {
        CR_FS_4CH_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_fs_3ch_mode(&self) -> CR_FS_3CH_MODE_R {
        CR_FS_3CH_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_frame_size(&self) -> CR_FRAME_SIZE_R {
        CR_FRAME_SIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cr_data_size(&self) -> CR_DATA_SIZE_R {
        CR_DATA_SIZE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cr_i2s_mode(&self) -> CR_I2S_MODE_R {
        CR_I2S_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_endian(&self) -> CR_ENDIAN_R {
        CR_ENDIAN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_mono_rx_ch(&self) -> CR_MONO_RX_CH_R {
        CR_MONO_RX_CH_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn cr_ofs_cnt(&self) -> CR_OFS_CNT_R {
        CR_OFS_CNT_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_ofs_en(&self) -> CR_OFS_EN_R {
        CR_OFS_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2s_m_en(&mut self) -> CR_I2S_M_EN_W<0> {
        CR_I2S_M_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2s_s_en(&mut self) -> CR_I2S_S_EN_W<1> {
        CR_I2S_S_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2s_txd_en(&mut self) -> CR_I2S_TXD_EN_W<2> {
        CR_I2S_TXD_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2s_rxd_en(&mut self) -> CR_I2S_RXD_EN_W<3> {
        CR_I2S_RXD_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_mono_mode(&mut self) -> CR_MONO_MODE_W<4> {
        CR_MONO_MODE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_mute_mode(&mut self) -> CR_MUTE_MODE_W<5> {
        CR_MUTE_MODE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_fs_1t_mode(&mut self) -> CR_FS_1T_MODE_W<6> {
        CR_FS_1T_MODE_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_fs_4ch_mode(&mut self) -> CR_FS_4CH_MODE_W<7> {
        CR_FS_4CH_MODE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_fs_3ch_mode(&mut self) -> CR_FS_3CH_MODE_W<8> {
        CR_FS_3CH_MODE_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_frame_size(&mut self) -> CR_FRAME_SIZE_W<12> {
        CR_FRAME_SIZE_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn cr_data_size(&mut self) -> CR_DATA_SIZE_W<14> {
        CR_DATA_SIZE_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn cr_i2s_mode(&mut self) -> CR_I2S_MODE_W<16> {
        CR_I2S_MODE_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_endian(&mut self) -> CR_ENDIAN_W<18> {
        CR_ENDIAN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_mono_rx_ch(&mut self) -> CR_MONO_RX_CH_W<19> {
        CR_MONO_RX_CH_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn cr_ofs_cnt(&mut self) -> CR_OFS_CNT_W<20> {
        CR_OFS_CNT_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_ofs_en(&mut self) -> CR_OFS_EN_W<25> {
        CR_OFS_EN_W::new(self)
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
