#[doc = "Register `vco` reader"]
pub struct R(crate::R<VCO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vco` writer"]
pub struct W(crate::W<VCO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCO_SPEC>;
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
impl From<crate::W<VCO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vco_acal_ud` reader - "]
pub struct VCO_ACAL_UD_R(crate::FieldReader<bool, bool>);
impl VCO_ACAL_UD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCO_ACAL_UD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_ACAL_UD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_acal_ud` writer - "]
pub struct VCO_ACAL_UD_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_ACAL_UD_W<'a> {
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
#[doc = "Field `vco_idac_hw` reader - "]
pub struct VCO_IDAC_HW_R(crate::FieldReader<u8, u8>);
impl VCO_IDAC_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_IDAC_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_IDAC_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_idac_hw` writer - "]
pub struct VCO_IDAC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_IDAC_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `vco_idac` reader - "]
pub struct VCO_IDAC_R(crate::FieldReader<u8, u8>);
impl VCO_IDAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_IDAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_IDAC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_idac` writer - "]
pub struct VCO_IDAC_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_IDAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `vco_ldo_bypass` reader - "]
pub struct VCO_LDO_BYPASS_R(crate::FieldReader<bool, bool>);
impl VCO_LDO_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCO_LDO_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_LDO_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_ldo_bypass` writer - "]
pub struct VCO_LDO_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_LDO_BYPASS_W<'a> {
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
#[doc = "Field `vco_ldo_vsel` reader - "]
pub struct VCO_LDO_VSEL_R(crate::FieldReader<u8, u8>);
impl VCO_LDO_VSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_LDO_VSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_LDO_VSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_ldo_vsel` writer - "]
pub struct VCO_LDO_VSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_LDO_VSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `vco_idac_boost` reader - "]
pub struct VCO_IDAC_BOOST_R(crate::FieldReader<bool, bool>);
impl VCO_IDAC_BOOST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCO_IDAC_BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_IDAC_BOOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_idac_boost` writer - "]
pub struct VCO_IDAC_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_IDAC_BOOST_W<'a> {
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
#[doc = "Field `vco_vbias` reader - "]
pub struct VCO_VBIAS_R(crate::FieldReader<u8, u8>);
impl VCO_VBIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_VBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_VBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_vbias` writer - "]
pub struct VCO_VBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_VBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `vco_acal_vref` reader - "]
pub struct VCO_ACAL_VREF_R(crate::FieldReader<u8, u8>);
impl VCO_ACAL_VREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_ACAL_VREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_ACAL_VREF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_acal_vref` writer - "]
pub struct VCO_ACAL_VREF_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_ACAL_VREF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `vco_modcap_sel` reader - "]
pub struct VCO_MODCAP_SEL_R(crate::FieldReader<u8, u8>);
impl VCO_MODCAP_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        VCO_MODCAP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_MODCAP_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_modcap_sel` writer - "]
pub struct VCO_MODCAP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_MODCAP_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `vco_short_idac_filter` reader - "]
pub struct VCO_SHORT_IDAC_FILTER_R(crate::FieldReader<bool, bool>);
impl VCO_SHORT_IDAC_FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCO_SHORT_IDAC_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_SHORT_IDAC_FILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_short_idac_filter` writer - "]
pub struct VCO_SHORT_IDAC_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_SHORT_IDAC_FILTER_W<'a> {
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
#[doc = "Field `vco_short_vbias_filter` reader - "]
pub struct VCO_SHORT_VBIAS_FILTER_R(crate::FieldReader<bool, bool>);
impl VCO_SHORT_VBIAS_FILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCO_SHORT_VBIAS_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCO_SHORT_VBIAS_FILTER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vco_short_vbias_filter` writer - "]
pub struct VCO_SHORT_VBIAS_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> VCO_SHORT_VBIAS_FILTER_W<'a> {
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
    pub fn vco_acal_ud(&self) -> VCO_ACAL_UD_R {
        VCO_ACAL_UD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn vco_idac_hw(&self) -> VCO_IDAC_HW_R {
        VCO_IDAC_HW_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn vco_idac(&self) -> VCO_IDAC_R {
        VCO_IDAC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn vco_ldo_bypass(&self) -> VCO_LDO_BYPASS_R {
        VCO_LDO_BYPASS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn vco_ldo_vsel(&self) -> VCO_LDO_VSEL_R {
        VCO_LDO_VSEL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn vco_idac_boost(&self) -> VCO_IDAC_BOOST_R {
        VCO_IDAC_BOOST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn vco_vbias(&self) -> VCO_VBIAS_R {
        VCO_VBIAS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn vco_acal_vref(&self) -> VCO_ACAL_VREF_R {
        VCO_ACAL_VREF_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vco_modcap_sel(&self) -> VCO_MODCAP_SEL_R {
        VCO_MODCAP_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vco_short_idac_filter(&self) -> VCO_SHORT_IDAC_FILTER_R {
        VCO_SHORT_IDAC_FILTER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vco_short_vbias_filter(&self) -> VCO_SHORT_VBIAS_FILTER_R {
        VCO_SHORT_VBIAS_FILTER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn vco_acal_ud(&mut self) -> VCO_ACAL_UD_W {
        VCO_ACAL_UD_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn vco_idac_hw(&mut self) -> VCO_IDAC_HW_W {
        VCO_IDAC_HW_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn vco_idac(&mut self) -> VCO_IDAC_W {
        VCO_IDAC_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn vco_ldo_bypass(&mut self) -> VCO_LDO_BYPASS_W {
        VCO_LDO_BYPASS_W { w: self }
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn vco_ldo_vsel(&mut self) -> VCO_LDO_VSEL_W {
        VCO_LDO_VSEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn vco_idac_boost(&mut self) -> VCO_IDAC_BOOST_W {
        VCO_IDAC_BOOST_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn vco_vbias(&mut self) -> VCO_VBIAS_W {
        VCO_VBIAS_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn vco_acal_vref(&mut self) -> VCO_ACAL_VREF_W {
        VCO_ACAL_VREF_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vco_modcap_sel(&mut self) -> VCO_MODCAP_SEL_W {
        VCO_MODCAP_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn vco_short_idac_filter(&mut self) -> VCO_SHORT_IDAC_FILTER_W {
        VCO_SHORT_IDAC_FILTER_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn vco_short_vbias_filter(&mut self) -> VCO_SHORT_VBIAS_FILTER_W {
        VCO_SHORT_VBIAS_FILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "vco.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vco](index.html) module"]
pub struct VCO_SPEC;
impl crate::RegisterSpec for VCO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vco::R](R) reader structure"]
impl crate::Readable for VCO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vco::W](W) writer structure"]
impl crate::Writable for VCO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets vco to value 0"]
impl crate::Resettable for VCO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
