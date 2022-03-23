#[doc = "Register `dll` reader"]
pub struct R(crate::R<DLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dll` writer"]
pub struct W(crate::W<DLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLL_SPEC>;
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
impl From<crate::W<DLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ppu_dll` reader - "]
pub struct PPU_DLL_R(crate::FieldReader<bool, bool>);
impl PPU_DLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PPU_DLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPU_DLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ppu_dll` writer - "]
pub struct PPU_DLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PPU_DLL_W<'a> {
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
#[doc = "Field `pu_dll` reader - "]
pub struct PU_DLL_R(crate::FieldReader<bool, bool>);
impl PU_DLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PU_DLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_DLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_dll` writer - "]
pub struct PU_DLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `dll_reset` reader - "]
pub struct DLL_RESET_R(crate::FieldReader<bool, bool>);
impl DLL_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_reset` writer - "]
pub struct DLL_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `dll_refclk_sel` reader - "]
pub struct DLL_REFCLK_SEL_R(crate::FieldReader<bool, bool>);
impl DLL_REFCLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_REFCLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_REFCLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_refclk_sel` writer - "]
pub struct DLL_REFCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_REFCLK_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `dll_cp_hiz` reader - "]
pub struct DLL_CP_HIZ_R(crate::FieldReader<bool, bool>);
impl DLL_CP_HIZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_CP_HIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_CP_HIZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_cp_hiz` writer - "]
pub struct DLL_CP_HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CP_HIZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `dll_cp_op_en` reader - "]
pub struct DLL_CP_OP_EN_R(crate::FieldReader<bool, bool>);
impl DLL_CP_OP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_CP_OP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_CP_OP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_cp_op_en` writer - "]
pub struct DLL_CP_OP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CP_OP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `dll_delay_sel` reader - "]
pub struct DLL_DELAY_SEL_R(crate::FieldReader<u8, u8>);
impl DLL_DELAY_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLL_DELAY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_DELAY_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_delay_sel` writer - "]
pub struct DLL_DELAY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_DELAY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `dll_post_div` reader - "]
pub struct DLL_POST_DIV_R(crate::FieldReader<u8, u8>);
impl DLL_POST_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLL_POST_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_POST_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_post_div` writer - "]
pub struct DLL_POST_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_POST_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `dll_vctrl_force_en` reader - "]
pub struct DLL_VCTRL_FORCE_EN_R(crate::FieldReader<bool, bool>);
impl DLL_VCTRL_FORCE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_VCTRL_FORCE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_VCTRL_FORCE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_vctrl_force_en` writer - "]
pub struct DLL_VCTRL_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_VCTRL_FORCE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `dll_prechg_en` reader - "]
pub struct DLL_PRECHG_EN_R(crate::FieldReader<bool, bool>);
impl DLL_PRECHG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_PRECHG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_PRECHG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_prechg_en` writer - "]
pub struct DLL_PRECHG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_PRECHG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `dll_prechg_reg` reader - "]
pub struct DLL_PRECHG_REG_R(crate::FieldReader<bool, bool>);
impl DLL_PRECHG_REG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_PRECHG_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_PRECHG_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_prechg_reg` writer - "]
pub struct DLL_PRECHG_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_PRECHG_REG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `dll_prechg_sel` reader - "]
pub struct DLL_PRECHG_SEL_R(crate::FieldReader<bool, bool>);
impl DLL_PRECHG_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_PRECHG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_PRECHG_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_prechg_sel` writer - "]
pub struct DLL_PRECHG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_PRECHG_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `dll_vctrl_sel` reader - "]
pub struct DLL_VCTRL_SEL_R(crate::FieldReader<u8, u8>);
impl DLL_VCTRL_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DLL_VCTRL_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_VCTRL_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_vctrl_sel` writer - "]
pub struct DLL_VCTRL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_VCTRL_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `dll_clk_57p6M_en` reader - "]
pub struct DLL_CLK_57P6M_EN_R(crate::FieldReader<bool, bool>);
impl DLL_CLK_57P6M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_CLK_57P6M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_CLK_57P6M_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_clk_57p6M_en` writer - "]
pub struct DLL_CLK_57P6M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CLK_57P6M_EN_W<'a> {
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
#[doc = "Field `dll_clk_96M_en` reader - "]
pub struct DLL_CLK_96M_EN_R(crate::FieldReader<bool, bool>);
impl DLL_CLK_96M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_CLK_96M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_CLK_96M_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_clk_96M_en` writer - "]
pub struct DLL_CLK_96M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CLK_96M_EN_W<'a> {
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
#[doc = "Field `dll_clk_144M_en` reader - "]
pub struct DLL_CLK_144M_EN_R(crate::FieldReader<bool, bool>);
impl DLL_CLK_144M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_CLK_144M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_CLK_144M_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_clk_144M_en` writer - "]
pub struct DLL_CLK_144M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CLK_144M_EN_W<'a> {
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
#[doc = "Field `dll_clk_288M_en` reader - "]
pub struct DLL_CLK_288M_EN_R(crate::FieldReader<bool, bool>);
impl DLL_CLK_288M_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_CLK_288M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_CLK_288M_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_clk_288M_en` writer - "]
pub struct DLL_CLK_288M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CLK_288M_EN_W<'a> {
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
#[doc = "Field `dll_clk_mmdiv_en` reader - "]
pub struct DLL_CLK_MMDIV_EN_R(crate::FieldReader<bool, bool>);
impl DLL_CLK_MMDIV_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DLL_CLK_MMDIV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLL_CLK_MMDIV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dll_clk_mmdiv_en` writer - "]
pub struct DLL_CLK_MMDIV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_CLK_MMDIV_EN_W<'a> {
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
#[doc = "Field `ten_dll` reader - "]
pub struct TEN_DLL_R(crate::FieldReader<bool, bool>);
impl TEN_DLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_DLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_DLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_dll` writer - "]
pub struct TEN_DLL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DLL_W<'a> {
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
#[doc = "Field `dtest_en_dll_outclk` reader - "]
pub struct DTEST_EN_DLL_OUTCLK_R(crate::FieldReader<bool, bool>);
impl DTEST_EN_DLL_OUTCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTEST_EN_DLL_OUTCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEST_EN_DLL_OUTCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dtest_en_dll_outclk` writer - "]
pub struct DTEST_EN_DLL_OUTCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEST_EN_DLL_OUTCLK_W<'a> {
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
#[doc = "Field `dtest_en_dll_refclk` reader - "]
pub struct DTEST_EN_DLL_REFCLK_R(crate::FieldReader<bool, bool>);
impl DTEST_EN_DLL_REFCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DTEST_EN_DLL_REFCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEST_EN_DLL_REFCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dtest_en_dll_refclk` writer - "]
pub struct DTEST_EN_DLL_REFCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEST_EN_DLL_REFCLK_W<'a> {
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
    pub fn ppu_dll(&self) -> PPU_DLL_R {
        PPU_DLL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_dll(&self) -> PU_DLL_R {
        PU_DLL_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dll_reset(&self) -> DLL_RESET_R {
        DLL_RESET_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dll_refclk_sel(&self) -> DLL_REFCLK_SEL_R {
        DLL_REFCLK_SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dll_cp_hiz(&self) -> DLL_CP_HIZ_R {
        DLL_CP_HIZ_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dll_cp_op_en(&self) -> DLL_CP_OP_EN_R {
        DLL_CP_OP_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dll_delay_sel(&self) -> DLL_DELAY_SEL_R {
        DLL_DELAY_SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dll_post_div(&self) -> DLL_POST_DIV_R {
        DLL_POST_DIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dll_vctrl_force_en(&self) -> DLL_VCTRL_FORCE_EN_R {
        DLL_VCTRL_FORCE_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dll_prechg_en(&self) -> DLL_PRECHG_EN_R {
        DLL_PRECHG_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dll_prechg_reg(&self) -> DLL_PRECHG_REG_R {
        DLL_PRECHG_REG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dll_prechg_sel(&self) -> DLL_PRECHG_SEL_R {
        DLL_PRECHG_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dll_vctrl_sel(&self) -> DLL_VCTRL_SEL_R {
        DLL_VCTRL_SEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dll_clk_57p6m_en(&self) -> DLL_CLK_57P6M_EN_R {
        DLL_CLK_57P6M_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dll_clk_96m_en(&self) -> DLL_CLK_96M_EN_R {
        DLL_CLK_96M_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dll_clk_144m_en(&self) -> DLL_CLK_144M_EN_R {
        DLL_CLK_144M_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dll_clk_288m_en(&self) -> DLL_CLK_288M_EN_R {
        DLL_CLK_288M_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dll_clk_mmdiv_en(&self) -> DLL_CLK_MMDIV_EN_R {
        DLL_CLK_MMDIV_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_dll(&self) -> TEN_DLL_R {
        TEN_DLL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dtest_en_dll_outclk(&self) -> DTEST_EN_DLL_OUTCLK_R {
        DTEST_EN_DLL_OUTCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dtest_en_dll_refclk(&self) -> DTEST_EN_DLL_REFCLK_R {
        DTEST_EN_DLL_REFCLK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ppu_dll(&mut self) -> PPU_DLL_W {
        PPU_DLL_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_dll(&mut self) -> PU_DLL_W {
        PU_DLL_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn dll_reset(&mut self) -> DLL_RESET_W {
        DLL_RESET_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn dll_refclk_sel(&mut self) -> DLL_REFCLK_SEL_W {
        DLL_REFCLK_SEL_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn dll_cp_hiz(&mut self) -> DLL_CP_HIZ_W {
        DLL_CP_HIZ_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn dll_cp_op_en(&mut self) -> DLL_CP_OP_EN_W {
        DLL_CP_OP_EN_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dll_delay_sel(&mut self) -> DLL_DELAY_SEL_W {
        DLL_DELAY_SEL_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dll_post_div(&mut self) -> DLL_POST_DIV_W {
        DLL_POST_DIV_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dll_vctrl_force_en(&mut self) -> DLL_VCTRL_FORCE_EN_W {
        DLL_VCTRL_FORCE_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dll_prechg_en(&mut self) -> DLL_PRECHG_EN_W {
        DLL_PRECHG_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dll_prechg_reg(&mut self) -> DLL_PRECHG_REG_W {
        DLL_PRECHG_REG_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dll_prechg_sel(&mut self) -> DLL_PRECHG_SEL_W {
        DLL_PRECHG_SEL_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dll_vctrl_sel(&mut self) -> DLL_VCTRL_SEL_W {
        DLL_VCTRL_SEL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dll_clk_57p6m_en(&mut self) -> DLL_CLK_57P6M_EN_W {
        DLL_CLK_57P6M_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dll_clk_96m_en(&mut self) -> DLL_CLK_96M_EN_W {
        DLL_CLK_96M_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dll_clk_144m_en(&mut self) -> DLL_CLK_144M_EN_W {
        DLL_CLK_144M_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dll_clk_288m_en(&mut self) -> DLL_CLK_288M_EN_W {
        DLL_CLK_288M_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dll_clk_mmdiv_en(&mut self) -> DLL_CLK_MMDIV_EN_W {
        DLL_CLK_MMDIV_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ten_dll(&mut self) -> TEN_DLL_W {
        TEN_DLL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dtest_en_dll_outclk(&mut self) -> DTEST_EN_DLL_OUTCLK_W {
        DTEST_EN_DLL_OUTCLK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dtest_en_dll_refclk(&mut self) -> DTEST_EN_DLL_REFCLK_W {
        DTEST_EN_DLL_REFCLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dll.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dll](index.html) module"]
pub struct DLL_SPEC;
impl crate::RegisterSpec for DLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dll::R](R) reader structure"]
impl crate::Readable for DLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dll::W](W) writer structure"]
impl crate::Writable for DLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dll to value 0"]
impl crate::Resettable for DLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
