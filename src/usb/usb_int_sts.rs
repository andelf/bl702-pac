#[doc = "Register `usb_int_sts` reader"]
pub struct R(crate::R<USB_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_INT_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_int_sts` writer"]
pub struct W(crate::W<USB_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_INT_STS_SPEC>;
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
impl From<crate::W<USB_INT_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sof_int` reader - "]
pub type SOF_INT_R = crate::BitReader<bool>;
#[doc = "Field `sof_int` writer - "]
pub type SOF_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `usb_reset_int` reader - "]
pub type USB_RESET_INT_R = crate::BitReader<bool>;
#[doc = "Field `usb_reset_int` writer - "]
pub type USB_RESET_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `vbus_tgl_int` reader - "]
pub type VBUS_TGL_INT_R = crate::BitReader<bool>;
#[doc = "Field `vbus_tgl_int` writer - "]
pub type VBUS_TGL_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `get_dct_cmd_int` reader - "]
pub type GET_DCT_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `get_dct_cmd_int` writer - "]
pub type GET_DCT_CMD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep0_setup_cmd_int` reader - "]
pub type EP0_SETUP_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep0_setup_cmd_int` writer - "]
pub type EP0_SETUP_CMD_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep0_setup_done_int` reader - "]
pub type EP0_SETUP_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep0_setup_done_int` writer - "]
pub type EP0_SETUP_DONE_INT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep0_in_cmd_int` reader - "]
pub type EP0_IN_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep0_in_cmd_int` writer - "]
pub type EP0_IN_CMD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep0_in_done_int` reader - "]
pub type EP0_IN_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep0_in_done_int` writer - "]
pub type EP0_IN_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep0_out_cmd_int` reader - "]
pub type EP0_OUT_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep0_out_cmd_int` writer - "]
pub type EP0_OUT_CMD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep0_out_done_int` reader - "]
pub type EP0_OUT_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep0_out_done_int` writer - "]
pub type EP0_OUT_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep1_cmd_int` reader - "]
pub type EP1_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep1_cmd_int` writer - "]
pub type EP1_CMD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep1_done_int` reader - "]
pub type EP1_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep1_done_int` writer - "]
pub type EP1_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep2_cmd_int` reader - "]
pub type EP2_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep2_cmd_int` writer - "]
pub type EP2_CMD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep2_done_int` reader - "]
pub type EP2_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep2_done_int` writer - "]
pub type EP2_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep3_cmd_int` reader - "]
pub type EP3_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep3_cmd_int` writer - "]
pub type EP3_CMD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep3_done_int` reader - "]
pub type EP3_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep3_done_int` writer - "]
pub type EP3_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep4_cmd_int` reader - "]
pub type EP4_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep4_cmd_int` writer - "]
pub type EP4_CMD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep4_done_int` reader - "]
pub type EP4_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep4_done_int` writer - "]
pub type EP4_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep5_cmd_int` reader - "]
pub type EP5_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep5_cmd_int` writer - "]
pub type EP5_CMD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep5_done_int` reader - "]
pub type EP5_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep5_done_int` writer - "]
pub type EP5_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep6_cmd_int` reader - "]
pub type EP6_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep6_cmd_int` writer - "]
pub type EP6_CMD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep6_done_int` reader - "]
pub type EP6_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep6_done_int` writer - "]
pub type EP6_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep7_cmd_int` reader - "]
pub type EP7_CMD_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep7_cmd_int` writer - "]
pub type EP7_CMD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `ep7_done_int` reader - "]
pub type EP7_DONE_INT_R = crate::BitReader<bool>;
#[doc = "Field `ep7_done_int` writer - "]
pub type EP7_DONE_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `rsvd_27_24` reader - "]
pub type RSVD_27_24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rsvd_27_24` writer - "]
pub type RSVD_27_24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USB_INT_STS_SPEC, u8, u8, 4, O>;
#[doc = "Field `lpm_wkup_int` reader - "]
pub type LPM_WKUP_INT_R = crate::BitReader<bool>;
#[doc = "Field `lpm_wkup_int` writer - "]
pub type LPM_WKUP_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `lpm_pkt_int` reader - "]
pub type LPM_PKT_INT_R = crate::BitReader<bool>;
#[doc = "Field `lpm_pkt_int` writer - "]
pub type LPM_PKT_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `sof_3ms_int` reader - "]
pub type SOF_3MS_INT_R = crate::BitReader<bool>;
#[doc = "Field `sof_3ms_int` writer - "]
pub type SOF_3MS_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
#[doc = "Field `usb_err_int` reader - "]
pub type USB_ERR_INT_R = crate::BitReader<bool>;
#[doc = "Field `usb_err_int` writer - "]
pub type USB_ERR_INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_STS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sof_int(&self) -> SOF_INT_R {
        SOF_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn usb_reset_int(&self) -> USB_RESET_INT_R {
        USB_RESET_INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vbus_tgl_int(&self) -> VBUS_TGL_INT_R {
        VBUS_TGL_INT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn get_dct_cmd_int(&self) -> GET_DCT_CMD_INT_R {
        GET_DCT_CMD_INT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ep0_setup_cmd_int(&self) -> EP0_SETUP_CMD_INT_R {
        EP0_SETUP_CMD_INT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ep0_setup_done_int(&self) -> EP0_SETUP_DONE_INT_R {
        EP0_SETUP_DONE_INT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ep0_in_cmd_int(&self) -> EP0_IN_CMD_INT_R {
        EP0_IN_CMD_INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ep0_in_done_int(&self) -> EP0_IN_DONE_INT_R {
        EP0_IN_DONE_INT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ep0_out_cmd_int(&self) -> EP0_OUT_CMD_INT_R {
        EP0_OUT_CMD_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ep0_out_done_int(&self) -> EP0_OUT_DONE_INT_R {
        EP0_OUT_DONE_INT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ep1_cmd_int(&self) -> EP1_CMD_INT_R {
        EP1_CMD_INT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ep1_done_int(&self) -> EP1_DONE_INT_R {
        EP1_DONE_INT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ep2_cmd_int(&self) -> EP2_CMD_INT_R {
        EP2_CMD_INT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ep2_done_int(&self) -> EP2_DONE_INT_R {
        EP2_DONE_INT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ep3_cmd_int(&self) -> EP3_CMD_INT_R {
        EP3_CMD_INT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ep3_done_int(&self) -> EP3_DONE_INT_R {
        EP3_DONE_INT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ep4_cmd_int(&self) -> EP4_CMD_INT_R {
        EP4_CMD_INT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ep4_done_int(&self) -> EP4_DONE_INT_R {
        EP4_DONE_INT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ep5_cmd_int(&self) -> EP5_CMD_INT_R {
        EP5_CMD_INT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ep5_done_int(&self) -> EP5_DONE_INT_R {
        EP5_DONE_INT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ep6_cmd_int(&self) -> EP6_CMD_INT_R {
        EP6_CMD_INT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ep6_done_int(&self) -> EP6_DONE_INT_R {
        EP6_DONE_INT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ep7_cmd_int(&self) -> EP7_CMD_INT_R {
        EP7_CMD_INT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ep7_done_int(&self) -> EP7_DONE_INT_R {
        EP7_DONE_INT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rsvd_27_24(&self) -> RSVD_27_24_R {
        RSVD_27_24_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lpm_wkup_int(&self) -> LPM_WKUP_INT_R {
        LPM_WKUP_INT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lpm_pkt_int(&self) -> LPM_PKT_INT_R {
        LPM_PKT_INT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sof_3ms_int(&self) -> SOF_3MS_INT_R {
        SOF_3MS_INT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn usb_err_int(&self) -> USB_ERR_INT_R {
        USB_ERR_INT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sof_int(&mut self) -> SOF_INT_W<0> {
        SOF_INT_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn usb_reset_int(&mut self) -> USB_RESET_INT_W<1> {
        USB_RESET_INT_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_tgl_int(&mut self) -> VBUS_TGL_INT_W<2> {
        VBUS_TGL_INT_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn get_dct_cmd_int(&mut self) -> GET_DCT_CMD_INT_W<3> {
        GET_DCT_CMD_INT_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_setup_cmd_int(&mut self) -> EP0_SETUP_CMD_INT_W<4> {
        EP0_SETUP_CMD_INT_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_setup_done_int(&mut self) -> EP0_SETUP_DONE_INT_W<5> {
        EP0_SETUP_DONE_INT_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_in_cmd_int(&mut self) -> EP0_IN_CMD_INT_W<6> {
        EP0_IN_CMD_INT_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_in_done_int(&mut self) -> EP0_IN_DONE_INT_W<7> {
        EP0_IN_DONE_INT_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_out_cmd_int(&mut self) -> EP0_OUT_CMD_INT_W<8> {
        EP0_OUT_CMD_INT_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_out_done_int(&mut self) -> EP0_OUT_DONE_INT_W<9> {
        EP0_OUT_DONE_INT_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_cmd_int(&mut self) -> EP1_CMD_INT_W<10> {
        EP1_CMD_INT_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_done_int(&mut self) -> EP1_DONE_INT_W<11> {
        EP1_DONE_INT_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_cmd_int(&mut self) -> EP2_CMD_INT_W<12> {
        EP2_CMD_INT_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_done_int(&mut self) -> EP2_DONE_INT_W<13> {
        EP2_DONE_INT_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_cmd_int(&mut self) -> EP3_CMD_INT_W<14> {
        EP3_CMD_INT_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_done_int(&mut self) -> EP3_DONE_INT_W<15> {
        EP3_DONE_INT_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_cmd_int(&mut self) -> EP4_CMD_INT_W<16> {
        EP4_CMD_INT_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_done_int(&mut self) -> EP4_DONE_INT_W<17> {
        EP4_DONE_INT_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_cmd_int(&mut self) -> EP5_CMD_INT_W<18> {
        EP5_CMD_INT_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_done_int(&mut self) -> EP5_DONE_INT_W<19> {
        EP5_DONE_INT_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_cmd_int(&mut self) -> EP6_CMD_INT_W<20> {
        EP6_CMD_INT_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_done_int(&mut self) -> EP6_DONE_INT_W<21> {
        EP6_DONE_INT_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_cmd_int(&mut self) -> EP7_CMD_INT_W<22> {
        EP7_CMD_INT_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_done_int(&mut self) -> EP7_DONE_INT_W<23> {
        EP7_DONE_INT_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_27_24(&mut self) -> RSVD_27_24_W<24> {
        RSVD_27_24_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_wkup_int(&mut self) -> LPM_WKUP_INT_W<28> {
        LPM_WKUP_INT_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_pkt_int(&mut self) -> LPM_PKT_INT_W<29> {
        LPM_PKT_INT_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn sof_3ms_int(&mut self) -> SOF_3MS_INT_W<30> {
        SOF_3MS_INT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn usb_err_int(&mut self) -> USB_ERR_INT_W<31> {
        USB_ERR_INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_int_sts](index.html) module"]
pub struct USB_INT_STS_SPEC;
impl crate::RegisterSpec for USB_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_int_sts::R](R) reader structure"]
impl crate::Readable for USB_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_int_sts::W](W) writer structure"]
impl crate::Writable for USB_INT_STS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets usb_int_sts to value 0"]
impl crate::Resettable for USB_INT_STS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
