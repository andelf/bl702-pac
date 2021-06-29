#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rsvd_23_18` reader - "]
pub struct RSVD_23_18_R(crate::FieldReader<u8, u8>);
impl RSVD_23_18_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSVD_23_18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_23_18_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_23_18` writer - "]
pub struct RSVD_23_18_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_23_18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | ((value as u32 & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `RMII_EN` reader - "]
pub struct RMII_EN_R(crate::FieldReader<bool, bool>);
impl RMII_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMII_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMII_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMII_EN` writer - "]
pub struct RMII_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMII_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RECSMALL` reader - "]
pub struct RECSMALL_R(crate::FieldReader<bool, bool>);
impl RECSMALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RECSMALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RECSMALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RECSMALL` writer - "]
pub struct RECSMALL_W<'a> {
    w: &'a mut W,
}
impl<'a> RECSMALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PAD` reader - "]
pub struct PAD_R(crate::FieldReader<bool, bool>);
impl PAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD` writer - "]
pub struct PAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_W<'a> {
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
#[doc = "Field `HUGEN` reader - "]
pub struct HUGEN_R(crate::FieldReader<bool, bool>);
impl HUGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HUGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HUGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HUGEN` writer - "]
pub struct HUGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HUGEN_W<'a> {
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
#[doc = "Field `CRCEN` reader - "]
pub struct CRCEN_R(crate::FieldReader<bool, bool>);
impl CRCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCEN` writer - "]
pub struct CRCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEN_W<'a> {
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
#[doc = "Field `rsvd_12_11` reader - "]
pub struct RSVD_12_11_R(crate::FieldReader<u8, u8>);
impl RSVD_12_11_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSVD_12_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_12_11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_12_11` writer - "]
pub struct RSVD_12_11_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_12_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `FULLD` reader - "]
pub struct FULLD_R(crate::FieldReader<bool, bool>);
impl FULLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FULLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULLD` writer - "]
pub struct FULLD_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLD_W<'a> {
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
#[doc = "Field `rsvd_9_7` reader - "]
pub struct RSVD_9_7_R(crate::FieldReader<u8, u8>);
impl RSVD_9_7_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSVD_9_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_9_7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_9_7` writer - "]
pub struct RSVD_9_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_9_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
#[doc = "Field `IFG` reader - "]
pub struct IFG_R(crate::FieldReader<bool, bool>);
impl IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFG` writer - "]
pub struct IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> IFG_W<'a> {
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
#[doc = "Field `PRO` reader - "]
pub struct PRO_R(crate::FieldReader<bool, bool>);
impl PRO_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRO` writer - "]
pub struct PRO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_W<'a> {
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
#[doc = "Field `rsvd_4` reader - "]
pub struct RSVD_4_R(crate::FieldReader<bool, bool>);
impl RSVD_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSVD_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_4` writer - "]
pub struct RSVD_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_4_W<'a> {
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
#[doc = "Field `BRO` reader - "]
pub struct BRO_R(crate::FieldReader<bool, bool>);
impl BRO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRO` writer - "]
pub struct BRO_W<'a> {
    w: &'a mut W,
}
impl<'a> BRO_W<'a> {
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
#[doc = "Field `NOPRE` reader - "]
pub struct NOPRE_R(crate::FieldReader<bool, bool>);
impl NOPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOPRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOPRE` writer - "]
pub struct NOPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> NOPRE_W<'a> {
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
#[doc = "Field `TXEN` reader - "]
pub struct TXEN_R(crate::FieldReader<bool, bool>);
impl TXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEN` writer - "]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
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
#[doc = "Field `RXEN` reader - "]
pub struct RXEN_R(crate::FieldReader<bool, bool>);
impl RXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXEN` writer - "]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
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
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rsvd_23_18(&self) -> RSVD_23_18_R {
        RSVD_23_18_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rmii_en(&self) -> RMII_EN_R {
        RMII_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn recsmall(&self) -> RECSMALL_R {
        RECSMALL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pad(&self) -> PAD_R {
        PAD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hugen(&self) -> HUGEN_R {
        HUGEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn rsvd_12_11(&self) -> RSVD_12_11_R {
        RSVD_12_11_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fulld(&self) -> FULLD_R {
        FULLD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn rsvd_9_7(&self) -> RSVD_9_7_R {
        RSVD_9_7_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pro(&self) -> PRO_R {
        PRO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rsvd_4(&self) -> RSVD_4_R {
        RSVD_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bro(&self) -> BRO_R {
        BRO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn nopre(&self) -> NOPRE_R {
        NOPRE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rsvd_23_18(&mut self) -> RSVD_23_18_W {
        RSVD_23_18_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rmii_en(&mut self) -> RMII_EN_W {
        RMII_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn recsmall(&mut self) -> RECSMALL_W {
        RECSMALL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pad(&mut self) -> PAD_W {
        PAD_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hugen(&mut self) -> HUGEN_W {
        HUGEN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W {
        CRCEN_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn rsvd_12_11(&mut self) -> RSVD_12_11_W {
        RSVD_12_11_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fulld(&mut self) -> FULLD_W {
        FULLD_W { w: self }
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn rsvd_9_7(&mut self) -> RSVD_9_7_W {
        RSVD_9_7_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ifg(&mut self) -> IFG_W {
        IFG_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pro(&mut self) -> PRO_W {
        PRO_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rsvd_4(&mut self) -> RSVD_4_W {
        RSVD_4_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bro(&mut self) -> BRO_W {
        BRO_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn nopre(&mut self) -> NOPRE_W {
        NOPRE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
