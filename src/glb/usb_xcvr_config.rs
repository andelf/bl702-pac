#[doc = "Register `usb_xcvr_config` reader"]
pub struct R(crate::R<USB_XCVR_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_XCVR_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_XCVR_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_XCVR_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_xcvr_config` writer"]
pub struct W(crate::W<USB_XCVR_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_XCVR_CONFIG_SPEC>;
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
impl From<crate::W<USB_XCVR_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_XCVR_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `usb_v_hys_m` reader - "]
pub type USB_V_HYS_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_v_hys_m` writer - "]
pub type USB_V_HYS_M_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `usb_v_hys_p` reader - "]
pub type USB_V_HYS_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_v_hys_p` writer - "]
pub type USB_V_HYS_P_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `usb_bd_vth` reader - "]
pub type USB_BD_VTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_bd_vth` writer - "]
pub type USB_BD_VTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `reg_usb_use_xcvr` reader - "]
pub type REG_USB_USE_XCVR_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_use_xcvr` writer - "]
pub type REG_USB_USE_XCVR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_XCVR_CONFIG_SPEC, bool, O>;
#[doc = "Field `usb_str_drv` reader - "]
pub type USB_STR_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_str_drv` writer - "]
pub type USB_STR_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `reg_usb_use_ctrl` reader - "]
pub type REG_USB_USE_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `reg_usb_use_ctrl` writer - "]
pub type REG_USB_USE_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_XCVR_CONFIG_SPEC, bool, O>;
#[doc = "Field `usb_res_pullup_tune` reader - "]
pub type USB_RES_PULLUP_TUNE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_res_pullup_tune` writer - "]
pub type USB_RES_PULLUP_TUNE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `usb_slewrate_m_fall` reader - "]
pub type USB_SLEWRATE_M_FALL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_slewrate_m_fall` writer - "]
pub type USB_SLEWRATE_M_FALL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `usb_slewrate_m_rise` reader - "]
pub type USB_SLEWRATE_M_RISE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_slewrate_m_rise` writer - "]
pub type USB_SLEWRATE_M_RISE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `usb_slewrate_p_fall` reader - "]
pub type USB_SLEWRATE_P_FALL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_slewrate_p_fall` writer - "]
pub type USB_SLEWRATE_P_FALL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_CONFIG_SPEC, u8, u8, 3, O>;
#[doc = "Field `usb_slewrate_p_rise` reader - "]
pub type USB_SLEWRATE_P_RISE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_slewrate_p_rise` writer - "]
pub type USB_SLEWRATE_P_RISE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_CONFIG_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn usb_v_hys_m(&self) -> USB_V_HYS_M_R {
        USB_V_HYS_M_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn usb_v_hys_p(&self) -> USB_V_HYS_P_R {
        USB_V_HYS_P_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn usb_bd_vth(&self) -> USB_BD_VTH_R {
        USB_BD_VTH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_usb_use_xcvr(&self) -> REG_USB_USE_XCVR_R {
        REG_USB_USE_XCVR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn usb_str_drv(&self) -> USB_STR_DRV_R {
        USB_STR_DRV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_usb_use_ctrl(&self) -> REG_USB_USE_CTRL_R {
        REG_USB_USE_CTRL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn usb_res_pullup_tune(&self) -> USB_RES_PULLUP_TUNE_R {
        USB_RES_PULLUP_TUNE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn usb_slewrate_m_fall(&self) -> USB_SLEWRATE_M_FALL_R {
        USB_SLEWRATE_M_FALL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn usb_slewrate_m_rise(&self) -> USB_SLEWRATE_M_RISE_R {
        USB_SLEWRATE_M_RISE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn usb_slewrate_p_fall(&self) -> USB_SLEWRATE_P_FALL_R {
        USB_SLEWRATE_P_FALL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn usb_slewrate_p_rise(&self) -> USB_SLEWRATE_P_RISE_R {
        USB_SLEWRATE_P_RISE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn usb_v_hys_m(&mut self) -> USB_V_HYS_M_W<0> {
        USB_V_HYS_M_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn usb_v_hys_p(&mut self) -> USB_V_HYS_P_W<2> {
        USB_V_HYS_P_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn usb_bd_vth(&mut self) -> USB_BD_VTH_W<4> {
        USB_BD_VTH_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_usb_use_xcvr(&mut self) -> REG_USB_USE_XCVR_W<7> {
        REG_USB_USE_XCVR_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn usb_str_drv(&mut self) -> USB_STR_DRV_W<8> {
        USB_STR_DRV_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_usb_use_ctrl(&mut self) -> REG_USB_USE_CTRL_W<11> {
        REG_USB_USE_CTRL_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn usb_res_pullup_tune(&mut self) -> USB_RES_PULLUP_TUNE_W<12> {
        USB_RES_PULLUP_TUNE_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn usb_slewrate_m_fall(&mut self) -> USB_SLEWRATE_M_FALL_W<16> {
        USB_SLEWRATE_M_FALL_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn usb_slewrate_m_rise(&mut self) -> USB_SLEWRATE_M_RISE_W<20> {
        USB_SLEWRATE_M_RISE_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn usb_slewrate_p_fall(&mut self) -> USB_SLEWRATE_P_FALL_W<24> {
        USB_SLEWRATE_P_FALL_W::new(self)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn usb_slewrate_p_rise(&mut self) -> USB_SLEWRATE_P_RISE_W<28> {
        USB_SLEWRATE_P_RISE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_xcvr_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_xcvr_config](index.html) module"]
pub struct USB_XCVR_CONFIG_SPEC;
impl crate::RegisterSpec for USB_XCVR_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_xcvr_config::R](R) reader structure"]
impl crate::Readable for USB_XCVR_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_xcvr_config::W](W) writer structure"]
impl crate::Writable for USB_XCVR_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_xcvr_config to value 0"]
impl crate::Resettable for USB_XCVR_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
