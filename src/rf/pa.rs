#[doc = "Register `pa` reader"]
pub struct R(crate::R<PA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pa` writer"]
pub struct W(crate::W<PA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_SPEC>;
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
impl From<crate::W<PA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_force_short_open` reader - "]
pub struct PA_FORCE_SHORT_OPEN_R(crate::FieldReader<bool, bool>);
impl PA_FORCE_SHORT_OPEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PA_FORCE_SHORT_OPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_FORCE_SHORT_OPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_force_short_open` writer - "]
pub struct PA_FORCE_SHORT_OPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_FORCE_SHORT_OPEN_W<'a> {
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
#[doc = "Field `pa_hp_en` reader - "]
pub struct PA_HP_EN_R(crate::FieldReader<bool, bool>);
impl PA_HP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PA_HP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_HP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_hp_en` writer - "]
pub struct PA_HP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_HP_EN_W<'a> {
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
#[doc = "Field `pa_lp_en` reader - "]
pub struct PA_LP_EN_R(crate::FieldReader<bool, bool>);
impl PA_LP_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PA_LP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_LP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_lp_en` writer - "]
pub struct PA_LP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_LP_EN_W<'a> {
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
#[doc = "Field `pa_ldo_bm` reader - "]
pub struct PA_LDO_BM_R(crate::FieldReader<u8, u8>);
impl PA_LDO_BM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_LDO_BM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_LDO_BM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_ldo_bm` writer - "]
pub struct PA_LDO_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_LDO_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `pa_vdd11_sel` reader - "]
pub struct PA_VDD11_SEL_R(crate::FieldReader<u8, u8>);
impl PA_VDD11_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_VDD11_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VDD11_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vdd11_sel` writer - "]
pub struct PA_VDD11_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VDD11_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `pa_para_cs` reader - "]
pub struct PA_PARA_CS_R(crate::FieldReader<u8, u8>);
impl PA_PARA_CS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_PARA_CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_PARA_CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_para_cs` writer - "]
pub struct PA_PARA_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_PARA_CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `pa_seri_cs_hw` reader - "]
pub struct PA_SERI_CS_HW_R(crate::FieldReader<u8, u8>);
impl PA_SERI_CS_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_SERI_CS_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_SERI_CS_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_seri_cs_hw` writer - "]
pub struct PA_SERI_CS_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_SERI_CS_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `pa_seri_cs_rx` reader - "]
pub struct PA_SERI_CS_RX_R(crate::FieldReader<u8, u8>);
impl PA_SERI_CS_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_SERI_CS_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_SERI_CS_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_seri_cs_rx` writer - "]
pub struct PA_SERI_CS_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_SERI_CS_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `pa_seri_cs_tx` reader - "]
pub struct PA_SERI_CS_TX_R(crate::FieldReader<u8, u8>);
impl PA_SERI_CS_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PA_SERI_CS_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_SERI_CS_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_seri_cs_tx` writer - "]
pub struct PA_SERI_CS_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_SERI_CS_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pa_force_short_open(&self) -> PA_FORCE_SHORT_OPEN_R {
        PA_FORCE_SHORT_OPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pa_hp_en(&self) -> PA_HP_EN_R {
        PA_HP_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pa_lp_en(&self) -> PA_LP_EN_R {
        PA_LP_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_ldo_bm(&self) -> PA_LDO_BM_R {
        PA_LDO_BM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vdd11_sel(&self) -> PA_VDD11_SEL_R {
        PA_VDD11_SEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_para_cs(&self) -> PA_PARA_CS_R {
        PA_PARA_CS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_seri_cs_hw(&self) -> PA_SERI_CS_HW_R {
        PA_SERI_CS_HW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_seri_cs_rx(&self) -> PA_SERI_CS_RX_R {
        PA_SERI_CS_RX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_seri_cs_tx(&self) -> PA_SERI_CS_TX_R {
        PA_SERI_CS_TX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pa_force_short_open(&mut self) -> PA_FORCE_SHORT_OPEN_W {
        PA_FORCE_SHORT_OPEN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pa_hp_en(&mut self) -> PA_HP_EN_W {
        PA_HP_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pa_lp_en(&mut self) -> PA_LP_EN_W {
        PA_LP_EN_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_ldo_bm(&mut self) -> PA_LDO_BM_W {
        PA_LDO_BM_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vdd11_sel(&mut self) -> PA_VDD11_SEL_W {
        PA_VDD11_SEL_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_para_cs(&mut self) -> PA_PARA_CS_W {
        PA_PARA_CS_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_seri_cs_hw(&mut self) -> PA_SERI_CS_HW_W {
        PA_SERI_CS_HW_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_seri_cs_rx(&mut self) -> PA_SERI_CS_RX_W {
        PA_SERI_CS_RX_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_seri_cs_tx(&mut self) -> PA_SERI_CS_TX_W {
        PA_SERI_CS_TX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PA register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa](index.html) module"]
pub struct PA_SPEC;
impl crate::RegisterSpec for PA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa::R](R) reader structure"]
impl crate::Readable for PA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa::W](W) writer structure"]
impl crate::Writable for PA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pa to value 0"]
impl crate::Resettable for PA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
