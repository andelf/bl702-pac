#[doc = "Register `usb_xcvr` reader"]
pub struct R(crate::R<USB_XCVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_XCVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_XCVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_XCVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_xcvr` writer"]
pub struct W(crate::W<USB_XCVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_XCVR_SPEC>;
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
impl From<crate::W<USB_XCVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_XCVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `usb_ldo_vfb` reader - "]
pub type USB_LDO_VFB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_ldo_vfb` writer - "]
pub type USB_LDO_VFB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USB_XCVR_SPEC, u8, u8, 3, O>;
#[doc = "Field `pu_usb_ldo` reader - "]
pub type PU_USB_LDO_R = crate::BitReader<bool>;
#[doc = "Field `pu_usb_ldo` writer - "]
pub type PU_USB_LDO_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_rout_nmos` reader - "]
pub type USB_ROUT_NMOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_rout_nmos` writer - "]
pub type USB_ROUT_NMOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_SPEC, u8, u8, 3, O>;
#[doc = "Field `usb_rout_pmos` reader - "]
pub type USB_ROUT_PMOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `usb_rout_pmos` writer - "]
pub type USB_ROUT_PMOS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_XCVR_SPEC, u8, u8, 3, O>;
#[doc = "Field `usb_oeb_sel` reader - "]
pub type USB_OEB_SEL_R = crate::BitReader<bool>;
#[doc = "Field `usb_oeb_sel` writer - "]
pub type USB_OEB_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_oeb_reg` reader - "]
pub type USB_OEB_REG_R = crate::BitReader<bool>;
#[doc = "Field `usb_oeb_reg` writer - "]
pub type USB_OEB_REG_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_oeb` reader - "]
pub type USB_OEB_R = crate::BitReader<bool>;
#[doc = "Field `usb_oeb` writer - "]
pub type USB_OEB_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_data_convert` reader - "]
pub type USB_DATA_CONVERT_R = crate::BitReader<bool>;
#[doc = "Field `usb_data_convert` writer - "]
pub type USB_DATA_CONVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_enum` reader - "]
pub type USB_ENUM_R = crate::BitReader<bool>;
#[doc = "Field `usb_enum` writer - "]
pub type USB_ENUM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_spd` reader - "]
pub type USB_SPD_R = crate::BitReader<bool>;
#[doc = "Field `usb_spd` writer - "]
pub type USB_SPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_sus` reader - "]
pub type USB_SUS_R = crate::BitReader<bool>;
#[doc = "Field `usb_sus` writer - "]
pub type USB_SUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `pu_usb` reader - "]
pub type PU_USB_R = crate::BitReader<bool>;
#[doc = "Field `pu_usb` writer - "]
pub type PU_USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_bd` reader - "]
pub type USB_BD_R = crate::BitReader<bool>;
#[doc = "Field `usb_bd` writer - "]
pub type USB_BD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_vim` reader - "]
pub type USB_VIM_R = crate::BitReader<bool>;
#[doc = "Field `usb_vim` writer - "]
pub type USB_VIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_vip` reader - "]
pub type USB_VIP_R = crate::BitReader<bool>;
#[doc = "Field `usb_vip` writer - "]
pub type USB_VIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
#[doc = "Field `usb_rcv` reader - "]
pub type USB_RCV_R = crate::BitReader<bool>;
#[doc = "Field `usb_rcv` writer - "]
pub type USB_RCV_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_XCVR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn usb_ldo_vfb(&self) -> USB_LDO_VFB_R {
        USB_LDO_VFB_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pu_usb_ldo(&self) -> PU_USB_LDO_R {
        PU_USB_LDO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn usb_rout_nmos(&self) -> USB_ROUT_NMOS_R {
        USB_ROUT_NMOS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn usb_rout_pmos(&self) -> USB_ROUT_PMOS_R {
        USB_ROUT_PMOS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn usb_oeb_sel(&self) -> USB_OEB_SEL_R {
        USB_OEB_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn usb_oeb_reg(&self) -> USB_OEB_REG_R {
        USB_OEB_REG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn usb_oeb(&self) -> USB_OEB_R {
        USB_OEB_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn usb_data_convert(&self) -> USB_DATA_CONVERT_R {
        USB_DATA_CONVERT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn usb_enum(&self) -> USB_ENUM_R {
        USB_ENUM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn usb_spd(&self) -> USB_SPD_R {
        USB_SPD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn usb_sus(&self) -> USB_SUS_R {
        USB_SUS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_usb(&self) -> PU_USB_R {
        PU_USB_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn usb_bd(&self) -> USB_BD_R {
        USB_BD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn usb_vim(&self) -> USB_VIM_R {
        USB_VIM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn usb_vip(&self) -> USB_VIP_R {
        USB_VIP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn usb_rcv(&self) -> USB_RCV_R {
        USB_RCV_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn usb_ldo_vfb(&mut self) -> USB_LDO_VFB_W<0> {
        USB_LDO_VFB_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pu_usb_ldo(&mut self) -> PU_USB_LDO_W<3> {
        PU_USB_LDO_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rout_nmos(&mut self) -> USB_ROUT_NMOS_W<4> {
        USB_ROUT_NMOS_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rout_pmos(&mut self) -> USB_ROUT_PMOS_W<8> {
        USB_ROUT_PMOS_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn usb_oeb_sel(&mut self) -> USB_OEB_SEL_W<12> {
        USB_OEB_SEL_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn usb_oeb_reg(&mut self) -> USB_OEB_REG_W<13> {
        USB_OEB_REG_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn usb_oeb(&mut self) -> USB_OEB_W<14> {
        USB_OEB_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn usb_data_convert(&mut self) -> USB_DATA_CONVERT_W<16> {
        USB_DATA_CONVERT_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn usb_enum(&mut self) -> USB_ENUM_W<20> {
        USB_ENUM_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn usb_spd(&mut self) -> USB_SPD_W<21> {
        USB_SPD_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn usb_sus(&mut self) -> USB_SUS_W<22> {
        USB_SUS_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn pu_usb(&mut self) -> PU_USB_W<23> {
        PU_USB_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn usb_bd(&mut self) -> USB_BD_W<24> {
        USB_BD_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vim(&mut self) -> USB_VIM_W<25> {
        USB_VIM_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn usb_vip(&mut self) -> USB_VIP_W<26> {
        USB_VIP_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn usb_rcv(&mut self) -> USB_RCV_W<27> {
        USB_RCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "usb_xcvr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_xcvr](index.html) module"]
pub struct USB_XCVR_SPEC;
impl crate::RegisterSpec for USB_XCVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_xcvr::R](R) reader structure"]
impl crate::Readable for USB_XCVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_xcvr::W](W) writer structure"]
impl crate::Writable for USB_XCVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usb_xcvr to value 0"]
impl crate::Resettable for USB_XCVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
