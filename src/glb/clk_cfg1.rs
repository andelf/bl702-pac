#[doc = "Register `clk_cfg1` reader"]
pub struct R(crate::R<CLK_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg1` writer"]
pub struct W(crate::W<CLK_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG1_SPEC>;
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
impl From<crate::W<CLK_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `qdec_clk_div` reader - "]
pub type QDEC_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `qdec_clk_div` writer - "]
pub type QDEC_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG1_SPEC, u8, u8, 5, O>;
#[doc = "Field `qdec_clk_sel` reader - "]
pub type QDEC_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `qdec_clk_sel` writer - "]
pub type QDEC_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `usb_clk_en` reader - "]
pub type USB_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `usb_clk_en` writer - "]
pub type USB_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `dll_48m_div_en` reader - "]
pub type DLL_48M_DIV_EN_R = crate::BitReader<bool>;
#[doc = "Field `dll_48m_div_en` writer - "]
pub type DLL_48M_DIV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_i2s_clk_sel` reader - "]
pub type REG_I2S_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_i2s_clk_sel` writer - "]
pub type REG_I2S_CLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_i2s0_clk_en` reader - "]
pub type REG_I2S0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_i2s0_clk_en` writer - "]
pub type REG_I2S0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_i2s_0_ref_clk_oe` reader - "]
pub type REG_I2S_0_REF_CLK_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_i2s_0_ref_clk_oe` writer - "]
pub type REG_I2S_0_REF_CLK_OE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `ble_clk_sel` reader - "]
pub type BLE_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ble_clk_sel` writer - "]
pub type BLE_CLK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_CFG1_SPEC, u8, u8, 6, O>;
#[doc = "Field `ble_en` reader - "]
pub type BLE_EN_R = crate::BitReader<bool>;
#[doc = "Field `ble_en` writer - "]
pub type BLE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `m154_zbEn` reader - "]
pub type M154_ZB_EN_R = crate::BitReader<bool>;
#[doc = "Field `m154_zbEn` writer - "]
pub type M154_ZB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_cam_ref_clk_en` reader - "]
pub type REG_CAM_REF_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_cam_ref_clk_en` writer - "]
pub type REG_CAM_REF_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_cam_ref_clk_src_sel` reader - "]
pub type REG_CAM_REF_CLK_SRC_SEL_R = crate::BitReader<bool>;
#[doc = "Field `reg_cam_ref_clk_src_sel` writer - "]
pub type REG_CAM_REF_CLK_SRC_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_cam_ref_clk_div` reader - "]
pub type REG_CAM_REF_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_cam_ref_clk_div` writer - "]
pub type REG_CAM_REF_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CLK_CFG1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn qdec_clk_div(&self) -> QDEC_CLK_DIV_R {
        QDEC_CLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn qdec_clk_sel(&self) -> QDEC_CLK_SEL_R {
        QDEC_CLK_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn usb_clk_en(&self) -> USB_CLK_EN_R {
        USB_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dll_48m_div_en(&self) -> DLL_48M_DIV_EN_R {
        DLL_48M_DIV_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_i2s_clk_sel(&self) -> REG_I2S_CLK_SEL_R {
        REG_I2S_CLK_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_i2s0_clk_en(&self) -> REG_I2S0_CLK_EN_R {
        REG_I2S0_CLK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_i2s_0_ref_clk_oe(&self) -> REG_I2S_0_REF_CLK_OE_R {
        REG_I2S_0_REF_CLK_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn ble_clk_sel(&self) -> BLE_CLK_SEL_R {
        BLE_CLK_SEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ble_en(&self) -> BLE_EN_R {
        BLE_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn m154_zb_en(&self) -> M154_ZB_EN_R {
        M154_ZB_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_en(&self) -> REG_CAM_REF_CLK_EN_R {
        REG_CAM_REF_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_src_sel(&self) -> REG_CAM_REF_CLK_SRC_SEL_R {
        REG_CAM_REF_CLK_SRC_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn reg_cam_ref_clk_div(&self) -> REG_CAM_REF_CLK_DIV_R {
        REG_CAM_REF_CLK_DIV_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn qdec_clk_div(&mut self) -> QDEC_CLK_DIV_W<0> {
        QDEC_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn qdec_clk_sel(&mut self) -> QDEC_CLK_SEL_W<7> {
        QDEC_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn usb_clk_en(&mut self) -> USB_CLK_EN_W<8> {
        USB_CLK_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn dll_48m_div_en(&mut self) -> DLL_48M_DIV_EN_W<9> {
        DLL_48M_DIV_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s_clk_sel(&mut self) -> REG_I2S_CLK_SEL_W<12> {
        REG_I2S_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s0_clk_en(&mut self) -> REG_I2S0_CLK_EN_W<13> {
        REG_I2S0_CLK_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2s_0_ref_clk_oe(&mut self) -> REG_I2S_0_REF_CLK_OE_W<14> {
        REG_I2S_0_REF_CLK_OE_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn ble_clk_sel(&mut self) -> BLE_CLK_SEL_W<16> {
        BLE_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn ble_en(&mut self) -> BLE_EN_W<24> {
        BLE_EN_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn m154_zb_en(&mut self) -> M154_ZB_EN_W<25> {
        M154_ZB_EN_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cam_ref_clk_en(&mut self) -> REG_CAM_REF_CLK_EN_W<28> {
        REG_CAM_REF_CLK_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cam_ref_clk_src_sel(&mut self) -> REG_CAM_REF_CLK_SRC_SEL_W<29> {
        REG_CAM_REF_CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cam_ref_clk_div(&mut self) -> REG_CAM_REF_CLK_DIV_W<30> {
        REG_CAM_REF_CLK_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clk_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg1](index.html) module"]
pub struct CLK_CFG1_SPEC;
impl crate::RegisterSpec for CLK_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg1::R](R) reader structure"]
impl crate::Readable for CLK_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg1::W](W) writer structure"]
impl crate::Writable for CLK_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets clk_cfg1 to value 0"]
impl crate::Resettable for CLK_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
