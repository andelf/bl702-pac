#[doc = "Register `xcvr_if_config` reader"]
pub struct R(crate::R<XCVR_IF_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XCVR_IF_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XCVR_IF_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XCVR_IF_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `xcvr_if_config` writer"]
pub struct W(crate::W<XCVR_IF_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XCVR_IF_CONFIG_SPEC>;
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
impl From<crate::W<XCVR_IF_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XCVR_IF_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sts_vbus_det` reader - "]
pub struct STS_VBUS_DET_R(crate::FieldReader<bool, bool>);
impl STS_VBUS_DET_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_VBUS_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_VBUS_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_vbus_det` writer - "]
pub struct STS_VBUS_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_VBUS_DET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `cr_xcvr_om_rx_dn` reader - "]
pub struct CR_XCVR_OM_RX_DN_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_OM_RX_DN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_OM_RX_DN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_OM_RX_DN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_om_rx_dn` writer - "]
pub struct CR_XCVR_OM_RX_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_OM_RX_DN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `cr_xcvr_om_rx_dp` reader - "]
pub struct CR_XCVR_OM_RX_DP_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_OM_RX_DP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_OM_RX_DP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_OM_RX_DP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_om_rx_dp` writer - "]
pub struct CR_XCVR_OM_RX_DP_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_OM_RX_DP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `cr_xcvr_om_rx_d` reader - "]
pub struct CR_XCVR_OM_RX_D_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_OM_RX_D_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_OM_RX_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_OM_RX_D_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_om_rx_d` writer - "]
pub struct CR_XCVR_OM_RX_D_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_OM_RX_D_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `cr_xcvr_om_rx_sel` reader - "]
pub struct CR_XCVR_OM_RX_SEL_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_OM_RX_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_OM_RX_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_OM_RX_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_om_rx_sel` writer - "]
pub struct CR_XCVR_OM_RX_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_OM_RX_SEL_W<'a> {
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
#[doc = "Field `cr_xcvr_force_rx_dn` reader - "]
pub struct CR_XCVR_FORCE_RX_DN_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_FORCE_RX_DN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_FORCE_RX_DN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_FORCE_RX_DN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_force_rx_dn` writer - "]
pub struct CR_XCVR_FORCE_RX_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_FORCE_RX_DN_W<'a> {
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
#[doc = "Field `cr_xcvr_force_rx_dp` reader - "]
pub struct CR_XCVR_FORCE_RX_DP_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_FORCE_RX_DP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_FORCE_RX_DP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_FORCE_RX_DP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_force_rx_dp` writer - "]
pub struct CR_XCVR_FORCE_RX_DP_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_FORCE_RX_DP_W<'a> {
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
#[doc = "Field `cr_xcvr_force_rx_d` reader - "]
pub struct CR_XCVR_FORCE_RX_D_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_FORCE_RX_D_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_FORCE_RX_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_FORCE_RX_D_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_force_rx_d` writer - "]
pub struct CR_XCVR_FORCE_RX_D_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_FORCE_RX_D_W<'a> {
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
#[doc = "Field `cr_xcvr_force_rx_en` reader - "]
pub struct CR_XCVR_FORCE_RX_EN_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_FORCE_RX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_FORCE_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_FORCE_RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_force_rx_en` writer - "]
pub struct CR_XCVR_FORCE_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_FORCE_RX_EN_W<'a> {
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
#[doc = "Field `cr_xcvr_force_tx_dn` reader - "]
pub struct CR_XCVR_FORCE_TX_DN_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_FORCE_TX_DN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_FORCE_TX_DN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_FORCE_TX_DN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_force_tx_dn` writer - "]
pub struct CR_XCVR_FORCE_TX_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_FORCE_TX_DN_W<'a> {
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
#[doc = "Field `cr_xcvr_force_tx_dp` reader - "]
pub struct CR_XCVR_FORCE_TX_DP_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_FORCE_TX_DP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_FORCE_TX_DP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_FORCE_TX_DP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_force_tx_dp` writer - "]
pub struct CR_XCVR_FORCE_TX_DP_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_FORCE_TX_DP_W<'a> {
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
#[doc = "Field `cr_xcvr_force_tx_oe` reader - "]
pub struct CR_XCVR_FORCE_TX_OE_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_FORCE_TX_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_FORCE_TX_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_FORCE_TX_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_force_tx_oe` writer - "]
pub struct CR_XCVR_FORCE_TX_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_FORCE_TX_OE_W<'a> {
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
#[doc = "Field `cr_xcvr_force_tx_en` reader - "]
pub struct CR_XCVR_FORCE_TX_EN_R(crate::FieldReader<bool, bool>);
impl CR_XCVR_FORCE_TX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_XCVR_FORCE_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_XCVR_FORCE_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_xcvr_force_tx_en` writer - "]
pub struct CR_XCVR_FORCE_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_XCVR_FORCE_TX_EN_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sts_vbus_det(&self) -> STS_VBUS_DET_R {
        STS_VBUS_DET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_xcvr_om_rx_dn(&self) -> CR_XCVR_OM_RX_DN_R {
        CR_XCVR_OM_RX_DN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_xcvr_om_rx_dp(&self) -> CR_XCVR_OM_RX_DP_R {
        CR_XCVR_OM_RX_DP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_xcvr_om_rx_d(&self) -> CR_XCVR_OM_RX_D_R {
        CR_XCVR_OM_RX_D_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_xcvr_om_rx_sel(&self) -> CR_XCVR_OM_RX_SEL_R {
        CR_XCVR_OM_RX_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_xcvr_force_rx_dn(&self) -> CR_XCVR_FORCE_RX_DN_R {
        CR_XCVR_FORCE_RX_DN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_xcvr_force_rx_dp(&self) -> CR_XCVR_FORCE_RX_DP_R {
        CR_XCVR_FORCE_RX_DP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_xcvr_force_rx_d(&self) -> CR_XCVR_FORCE_RX_D_R {
        CR_XCVR_FORCE_RX_D_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_xcvr_force_rx_en(&self) -> CR_XCVR_FORCE_RX_EN_R {
        CR_XCVR_FORCE_RX_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_xcvr_force_tx_dn(&self) -> CR_XCVR_FORCE_TX_DN_R {
        CR_XCVR_FORCE_TX_DN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xcvr_force_tx_dp(&self) -> CR_XCVR_FORCE_TX_DP_R {
        CR_XCVR_FORCE_TX_DP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_xcvr_force_tx_oe(&self) -> CR_XCVR_FORCE_TX_OE_R {
        CR_XCVR_FORCE_TX_OE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_xcvr_force_tx_en(&self) -> CR_XCVR_FORCE_TX_EN_R {
        CR_XCVR_FORCE_TX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sts_vbus_det(&mut self) -> STS_VBUS_DET_W {
        STS_VBUS_DET_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_xcvr_om_rx_dn(&mut self) -> CR_XCVR_OM_RX_DN_W {
        CR_XCVR_OM_RX_DN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_xcvr_om_rx_dp(&mut self) -> CR_XCVR_OM_RX_DP_W {
        CR_XCVR_OM_RX_DP_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_xcvr_om_rx_d(&mut self) -> CR_XCVR_OM_RX_D_W {
        CR_XCVR_OM_RX_D_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_xcvr_om_rx_sel(&mut self) -> CR_XCVR_OM_RX_SEL_W {
        CR_XCVR_OM_RX_SEL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_xcvr_force_rx_dn(&mut self) -> CR_XCVR_FORCE_RX_DN_W {
        CR_XCVR_FORCE_RX_DN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_xcvr_force_rx_dp(&mut self) -> CR_XCVR_FORCE_RX_DP_W {
        CR_XCVR_FORCE_RX_DP_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_xcvr_force_rx_d(&mut self) -> CR_XCVR_FORCE_RX_D_W {
        CR_XCVR_FORCE_RX_D_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_xcvr_force_rx_en(&mut self) -> CR_XCVR_FORCE_RX_EN_W {
        CR_XCVR_FORCE_RX_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_xcvr_force_tx_dn(&mut self) -> CR_XCVR_FORCE_TX_DN_W {
        CR_XCVR_FORCE_TX_DN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_xcvr_force_tx_dp(&mut self) -> CR_XCVR_FORCE_TX_DP_W {
        CR_XCVR_FORCE_TX_DP_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_xcvr_force_tx_oe(&mut self) -> CR_XCVR_FORCE_TX_OE_W {
        CR_XCVR_FORCE_TX_OE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_xcvr_force_tx_en(&mut self) -> CR_XCVR_FORCE_TX_EN_W {
        CR_XCVR_FORCE_TX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xcvr_if_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xcvr_if_config](index.html) module"]
pub struct XCVR_IF_CONFIG_SPEC;
impl crate::RegisterSpec for XCVR_IF_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xcvr_if_config::R](R) reader structure"]
impl crate::Readable for XCVR_IF_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xcvr_if_config::W](W) writer structure"]
impl crate::Writable for XCVR_IF_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets xcvr_if_config to value 0"]
impl crate::Resettable for XCVR_IF_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
