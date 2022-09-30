#[doc = "Register `usb_int_en` reader"]
pub struct R(crate::R<USB_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `usb_int_en` writer"]
pub struct W(crate::W<USB_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB_INT_EN_SPEC>;
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
impl From<crate::W<USB_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_sof_en` reader - "]
pub type CR_SOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_sof_en` writer - "]
pub type CR_SOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_usb_reset_en` reader - "]
pub type CR_USB_RESET_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_usb_reset_en` writer - "]
pub type CR_USB_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_vbus_tgl_en` reader - "]
pub type CR_VBUS_TGL_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_vbus_tgl_en` writer - "]
pub type CR_VBUS_TGL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_get_dct_cmd_en` reader - "]
pub type CR_GET_DCT_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_get_dct_cmd_en` writer - "]
pub type CR_GET_DCT_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep0_setup_cmd_en` reader - "]
pub type CR_EP0_SETUP_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep0_setup_cmd_en` writer - "]
pub type CR_EP0_SETUP_CMD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep0_setup_done_en` reader - "]
pub type CR_EP0_SETUP_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep0_setup_done_en` writer - "]
pub type CR_EP0_SETUP_DONE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep0_in_cmd_en` reader - "]
pub type CR_EP0_IN_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep0_in_cmd_en` writer - "]
pub type CR_EP0_IN_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep0_in_done_en` reader - "]
pub type CR_EP0_IN_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep0_in_done_en` writer - "]
pub type CR_EP0_IN_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep0_out_cmd_en` reader - "]
pub type CR_EP0_OUT_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep0_out_cmd_en` writer - "]
pub type CR_EP0_OUT_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep0_out_done_en` reader - "]
pub type CR_EP0_OUT_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep0_out_done_en` writer - "]
pub type CR_EP0_OUT_DONE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep1_cmd_en` reader - "]
pub type CR_EP1_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep1_cmd_en` writer - "]
pub type CR_EP1_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep1_done_en` reader - "]
pub type CR_EP1_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep1_done_en` writer - "]
pub type CR_EP1_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep2_cmd_en` reader - "]
pub type CR_EP2_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep2_cmd_en` writer - "]
pub type CR_EP2_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep2_done_en` reader - "]
pub type CR_EP2_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep2_done_en` writer - "]
pub type CR_EP2_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep3_cmd_en` reader - "]
pub type CR_EP3_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep3_cmd_en` writer - "]
pub type CR_EP3_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep3_done_en` reader - "]
pub type CR_EP3_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep3_done_en` writer - "]
pub type CR_EP3_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep4_cmd_en` reader - "]
pub type CR_EP4_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep4_cmd_en` writer - "]
pub type CR_EP4_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep4_done_en` reader - "]
pub type CR_EP4_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep4_done_en` writer - "]
pub type CR_EP4_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep5_cmd_en` reader - "]
pub type CR_EP5_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep5_cmd_en` writer - "]
pub type CR_EP5_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep5_done_en` reader - "]
pub type CR_EP5_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep5_done_en` writer - "]
pub type CR_EP5_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep6_cmd_en` reader - "]
pub type CR_EP6_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep6_cmd_en` writer - "]
pub type CR_EP6_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep6_done_en` reader - "]
pub type CR_EP6_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep6_done_en` writer - "]
pub type CR_EP6_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep7_cmd_en` reader - "]
pub type CR_EP7_CMD_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep7_cmd_en` writer - "]
pub type CR_EP7_CMD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_ep7_done_en` reader - "]
pub type CR_EP7_DONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_ep7_done_en` writer - "]
pub type CR_EP7_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `rsvd_27_24` reader - "]
pub type RSVD_27_24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rsvd_27_24` writer - "]
pub type RSVD_27_24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USB_INT_EN_SPEC, u8, u8, 4, O>;
#[doc = "Field `cr_lpm_wkup_en` reader - "]
pub type CR_LPM_WKUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_lpm_wkup_en` writer - "]
pub type CR_LPM_WKUP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_lpm_pkt_en` reader - "]
pub type CR_LPM_PKT_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_lpm_pkt_en` writer - "]
pub type CR_LPM_PKT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_sof_3ms_en` reader - "]
pub type CR_SOF_3MS_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_sof_3ms_en` writer - "]
pub type CR_SOF_3MS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
#[doc = "Field `cr_usb_err_en` reader - "]
pub type CR_USB_ERR_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_usb_err_en` writer - "]
pub type CR_USB_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_sof_en(&self) -> CR_SOF_EN_R {
        CR_SOF_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_usb_reset_en(&self) -> CR_USB_RESET_EN_R {
        CR_USB_RESET_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_vbus_tgl_en(&self) -> CR_VBUS_TGL_EN_R {
        CR_VBUS_TGL_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_get_dct_cmd_en(&self) -> CR_GET_DCT_CMD_EN_R {
        CR_GET_DCT_CMD_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_ep0_setup_cmd_en(&self) -> CR_EP0_SETUP_CMD_EN_R {
        CR_EP0_SETUP_CMD_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_ep0_setup_done_en(&self) -> CR_EP0_SETUP_DONE_EN_R {
        CR_EP0_SETUP_DONE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_ep0_in_cmd_en(&self) -> CR_EP0_IN_CMD_EN_R {
        CR_EP0_IN_CMD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_ep0_in_done_en(&self) -> CR_EP0_IN_DONE_EN_R {
        CR_EP0_IN_DONE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_ep0_out_cmd_en(&self) -> CR_EP0_OUT_CMD_EN_R {
        CR_EP0_OUT_CMD_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_ep0_out_done_en(&self) -> CR_EP0_OUT_DONE_EN_R {
        CR_EP0_OUT_DONE_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_ep1_cmd_en(&self) -> CR_EP1_CMD_EN_R {
        CR_EP1_CMD_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_ep1_done_en(&self) -> CR_EP1_DONE_EN_R {
        CR_EP1_DONE_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_ep2_cmd_en(&self) -> CR_EP2_CMD_EN_R {
        CR_EP2_CMD_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_ep2_done_en(&self) -> CR_EP2_DONE_EN_R {
        CR_EP2_DONE_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_ep3_cmd_en(&self) -> CR_EP3_CMD_EN_R {
        CR_EP3_CMD_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_ep3_done_en(&self) -> CR_EP3_DONE_EN_R {
        CR_EP3_DONE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_ep4_cmd_en(&self) -> CR_EP4_CMD_EN_R {
        CR_EP4_CMD_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_ep4_done_en(&self) -> CR_EP4_DONE_EN_R {
        CR_EP4_DONE_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_ep5_cmd_en(&self) -> CR_EP5_CMD_EN_R {
        CR_EP5_CMD_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_ep5_done_en(&self) -> CR_EP5_DONE_EN_R {
        CR_EP5_DONE_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_ep6_cmd_en(&self) -> CR_EP6_CMD_EN_R {
        CR_EP6_CMD_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_ep6_done_en(&self) -> CR_EP6_DONE_EN_R {
        CR_EP6_DONE_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_ep7_cmd_en(&self) -> CR_EP7_CMD_EN_R {
        CR_EP7_CMD_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_ep7_done_en(&self) -> CR_EP7_DONE_EN_R {
        CR_EP7_DONE_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rsvd_27_24(&self) -> RSVD_27_24_R {
        RSVD_27_24_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_lpm_wkup_en(&self) -> CR_LPM_WKUP_EN_R {
        CR_LPM_WKUP_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_lpm_pkt_en(&self) -> CR_LPM_PKT_EN_R {
        CR_LPM_PKT_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_sof_3ms_en(&self) -> CR_SOF_3MS_EN_R {
        CR_SOF_3MS_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_usb_err_en(&self) -> CR_USB_ERR_EN_R {
        CR_USB_ERR_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_sof_en(&mut self) -> CR_SOF_EN_W<0> {
        CR_SOF_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_usb_reset_en(&mut self) -> CR_USB_RESET_EN_W<1> {
        CR_USB_RESET_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_vbus_tgl_en(&mut self) -> CR_VBUS_TGL_EN_W<2> {
        CR_VBUS_TGL_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_get_dct_cmd_en(&mut self) -> CR_GET_DCT_CMD_EN_W<3> {
        CR_GET_DCT_CMD_EN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_ep0_setup_cmd_en(&mut self) -> CR_EP0_SETUP_CMD_EN_W<4> {
        CR_EP0_SETUP_CMD_EN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_ep0_setup_done_en(&mut self) -> CR_EP0_SETUP_DONE_EN_W<5> {
        CR_EP0_SETUP_DONE_EN_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_ep0_in_cmd_en(&mut self) -> CR_EP0_IN_CMD_EN_W<6> {
        CR_EP0_IN_CMD_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_ep0_in_done_en(&mut self) -> CR_EP0_IN_DONE_EN_W<7> {
        CR_EP0_IN_DONE_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_ep0_out_cmd_en(&mut self) -> CR_EP0_OUT_CMD_EN_W<8> {
        CR_EP0_OUT_CMD_EN_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_ep0_out_done_en(&mut self) -> CR_EP0_OUT_DONE_EN_W<9> {
        CR_EP0_OUT_DONE_EN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_ep1_cmd_en(&mut self) -> CR_EP1_CMD_EN_W<10> {
        CR_EP1_CMD_EN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_ep1_done_en(&mut self) -> CR_EP1_DONE_EN_W<11> {
        CR_EP1_DONE_EN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_ep2_cmd_en(&mut self) -> CR_EP2_CMD_EN_W<12> {
        CR_EP2_CMD_EN_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_ep2_done_en(&mut self) -> CR_EP2_DONE_EN_W<13> {
        CR_EP2_DONE_EN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_ep3_cmd_en(&mut self) -> CR_EP3_CMD_EN_W<14> {
        CR_EP3_CMD_EN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_ep3_done_en(&mut self) -> CR_EP3_DONE_EN_W<15> {
        CR_EP3_DONE_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_ep4_cmd_en(&mut self) -> CR_EP4_CMD_EN_W<16> {
        CR_EP4_CMD_EN_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_ep4_done_en(&mut self) -> CR_EP4_DONE_EN_W<17> {
        CR_EP4_DONE_EN_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_ep5_cmd_en(&mut self) -> CR_EP5_CMD_EN_W<18> {
        CR_EP5_CMD_EN_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_ep5_done_en(&mut self) -> CR_EP5_DONE_EN_W<19> {
        CR_EP5_DONE_EN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_ep6_cmd_en(&mut self) -> CR_EP6_CMD_EN_W<20> {
        CR_EP6_CMD_EN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn cr_ep6_done_en(&mut self) -> CR_EP6_DONE_EN_W<21> {
        CR_EP6_DONE_EN_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn cr_ep7_cmd_en(&mut self) -> CR_EP7_CMD_EN_W<22> {
        CR_EP7_CMD_EN_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cr_ep7_done_en(&mut self) -> CR_EP7_DONE_EN_W<23> {
        CR_EP7_DONE_EN_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn rsvd_27_24(&mut self) -> RSVD_27_24_W<24> {
        RSVD_27_24_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_lpm_wkup_en(&mut self) -> CR_LPM_WKUP_EN_W<28> {
        CR_LPM_WKUP_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_lpm_pkt_en(&mut self) -> CR_LPM_PKT_EN_W<29> {
        CR_LPM_PKT_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cr_sof_3ms_en(&mut self) -> CR_SOF_3MS_EN_W<30> {
        CR_SOF_3MS_EN_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cr_usb_err_en(&mut self) -> CR_USB_ERR_EN_W<31> {
        CR_USB_ERR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_int_en](index.html) module"]
pub struct USB_INT_EN_SPEC;
impl crate::RegisterSpec for USB_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_int_en::R](R) reader structure"]
impl crate::Readable for USB_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_int_en::W](W) writer structure"]
impl crate::Writable for USB_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets usb_int_en to value 0"]
impl crate::Resettable for USB_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
