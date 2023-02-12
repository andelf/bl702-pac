#[doc = "Register `uart_sw_mode` reader"]
pub struct R(crate::R<UART_SW_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SW_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_SW_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_SW_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_sw_mode` writer"]
pub struct W(crate::W<UART_SW_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SW_MODE_SPEC>;
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
impl From<crate::W<UART_SW_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_SW_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_utx_txd_sw_mode` reader - "]
pub type CR_UTX_TXD_SW_MODE_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_txd_sw_mode` writer - "]
pub type CR_UTX_TXD_SW_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_SW_MODE_SPEC, bool, O>;
#[doc = "Field `cr_utx_txd_sw_val` reader - "]
pub type CR_UTX_TXD_SW_VAL_R = crate::BitReader<bool>;
#[doc = "Field `cr_utx_txd_sw_val` writer - "]
pub type CR_UTX_TXD_SW_VAL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_SW_MODE_SPEC, bool, O>;
#[doc = "Field `cr_urx_rts_sw_mode` reader - "]
pub type CR_URX_RTS_SW_MODE_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_rts_sw_mode` writer - "]
pub type CR_URX_RTS_SW_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_SW_MODE_SPEC, bool, O>;
#[doc = "Field `cr_urx_rts_sw_val` reader - "]
pub type CR_URX_RTS_SW_VAL_R = crate::BitReader<bool>;
#[doc = "Field `cr_urx_rts_sw_val` writer - "]
pub type CR_URX_RTS_SW_VAL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_SW_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_txd_sw_mode(&self) -> CR_UTX_TXD_SW_MODE_R {
        CR_UTX_TXD_SW_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_txd_sw_val(&self) -> CR_UTX_TXD_SW_VAL_R {
        CR_UTX_TXD_SW_VAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_mode(&self) -> CR_URX_RTS_SW_MODE_R {
        CR_URX_RTS_SW_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_val(&self) -> CR_URX_RTS_SW_VAL_R {
        CR_URX_RTS_SW_VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_txd_sw_mode(&mut self) -> CR_UTX_TXD_SW_MODE_W<0> {
        CR_UTX_TXD_SW_MODE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_utx_txd_sw_val(&mut self) -> CR_UTX_TXD_SW_VAL_W<1> {
        CR_UTX_TXD_SW_VAL_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_rts_sw_mode(&mut self) -> CR_URX_RTS_SW_MODE_W<2> {
        CR_URX_RTS_SW_MODE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_urx_rts_sw_val(&mut self) -> CR_URX_RTS_SW_VAL_W<3> {
        CR_URX_RTS_SW_VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_sw_mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sw_mode](index.html) module"]
pub struct UART_SW_MODE_SPEC;
impl crate::RegisterSpec for UART_SW_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_sw_mode::R](R) reader structure"]
impl crate::Readable for UART_SW_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_sw_mode::W](W) writer structure"]
impl crate::Writable for UART_SW_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets uart_sw_mode to value 0"]
impl crate::Resettable for UART_SW_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
