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
#[doc = "Field `cr_i2s_txd_inv` reader - "]
pub type CR_I2S_TXD_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2s_txd_inv` writer - "]
pub type CR_I2S_TXD_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_IO_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2s_rxd_inv` reader - "]
pub type CR_I2S_RXD_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2s_rxd_inv` writer - "]
pub type CR_I2S_RXD_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_IO_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2s_fs_inv` reader - "]
pub type CR_I2S_FS_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2s_fs_inv` writer - "]
pub type CR_I2S_FS_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_IO_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_i2s_bclk_inv` reader - "]
pub type CR_I2S_BCLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `cr_i2s_bclk_inv` writer - "]
pub type CR_I2S_BCLK_INV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2S_IO_CONFIG_SPEC, bool, O>;
#[doc = "Field `cr_deg_cnt` reader - "]
pub type CR_DEG_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_deg_cnt` writer - "]
pub type CR_DEG_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_IO_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `cr_deg_en` reader - "]
pub type CR_DEG_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_deg_en` writer - "]
pub type CR_DEG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2S_IO_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2s_txd_inv(&self) -> CR_I2S_TXD_INV_R {
        CR_I2S_TXD_INV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2s_rxd_inv(&self) -> CR_I2S_RXD_INV_R {
        CR_I2S_RXD_INV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2s_fs_inv(&self) -> CR_I2S_FS_INV_R {
        CR_I2S_FS_INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2s_bclk_inv(&self) -> CR_I2S_BCLK_INV_R {
        CR_I2S_BCLK_INV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn cr_deg_cnt(&self) -> CR_DEG_CNT_R {
        CR_DEG_CNT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_deg_en(&self) -> CR_DEG_EN_R {
        CR_DEG_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2s_txd_inv(&mut self) -> CR_I2S_TXD_INV_W<0> {
        CR_I2S_TXD_INV_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2s_rxd_inv(&mut self) -> CR_I2S_RXD_INV_W<1> {
        CR_I2S_RXD_INV_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2s_fs_inv(&mut self) -> CR_I2S_FS_INV_W<2> {
        CR_I2S_FS_INV_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_i2s_bclk_inv(&mut self) -> CR_I2S_BCLK_INV_W<3> {
        CR_I2S_BCLK_INV_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn cr_deg_cnt(&mut self) -> CR_DEG_CNT_W<4> {
        CR_DEG_CNT_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_deg_en(&mut self) -> CR_DEG_EN_W<7> {
        CR_DEG_EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_io_config to value 0"]
impl crate::Resettable for I2S_IO_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
