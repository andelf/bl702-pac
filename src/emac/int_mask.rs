#[doc = "Register `INT_MASK` reader"]
pub struct R(crate::R<INT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_MASK` writer"]
pub struct W(crate::W<INT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_MASK_SPEC>;
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
impl From<crate::W<INT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXC_M` reader - "]
pub struct RXC_M_R(crate::FieldReader<bool, bool>);
impl RXC_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXC_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXC_M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXC_M` writer - "]
pub struct RXC_M_W<'a> {
    w: &'a mut W,
}
impl<'a> RXC_M_W<'a> {
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
#[doc = "Field `TXC_M` reader - "]
pub struct TXC_M_R(crate::FieldReader<bool, bool>);
impl TXC_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXC_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXC_M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXC_M` writer - "]
pub struct TXC_M_W<'a> {
    w: &'a mut W,
}
impl<'a> TXC_M_W<'a> {
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
#[doc = "Field `BUSY_M` reader - "]
pub struct BUSY_M_R(crate::FieldReader<bool, bool>);
impl BUSY_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY_M` writer - "]
pub struct BUSY_M_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_M_W<'a> {
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
#[doc = "Field `RXE_M` reader - "]
pub struct RXE_M_R(crate::FieldReader<bool, bool>);
impl RXE_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXE_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXE_M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXE_M` writer - "]
pub struct RXE_M_W<'a> {
    w: &'a mut W,
}
impl<'a> RXE_M_W<'a> {
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
#[doc = "Field `RXB_M` reader - "]
pub struct RXB_M_R(crate::FieldReader<bool, bool>);
impl RXB_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXB_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXB_M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXB_M` writer - "]
pub struct RXB_M_W<'a> {
    w: &'a mut W,
}
impl<'a> RXB_M_W<'a> {
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
#[doc = "Field `TXE_M` reader - "]
pub struct TXE_M_R(crate::FieldReader<bool, bool>);
impl TXE_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXE_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXE_M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXE_M` writer - "]
pub struct TXE_M_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_M_W<'a> {
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
#[doc = "Field `TXB_M` reader - "]
pub struct TXB_M_R(crate::FieldReader<bool, bool>);
impl TXB_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXB_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXB_M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXB_M` writer - "]
pub struct TXB_M_W<'a> {
    w: &'a mut W,
}
impl<'a> TXB_M_W<'a> {
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
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxc_m(&self) -> RXC_M_R {
        RXC_M_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txc_m(&self) -> TXC_M_R {
        TXC_M_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busy_m(&self) -> BUSY_M_R {
        BUSY_M_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxe_m(&self) -> RXE_M_R {
        RXE_M_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxb_m(&self) -> RXB_M_R {
        RXB_M_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txe_m(&self) -> TXE_M_R {
        TXE_M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txb_m(&self) -> TXB_M_R {
        TXB_M_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxc_m(&mut self) -> RXC_M_W {
        RXC_M_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txc_m(&mut self) -> TXC_M_W {
        TXC_M_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busy_m(&mut self) -> BUSY_M_W {
        BUSY_M_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxe_m(&mut self) -> RXE_M_W {
        RXE_M_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxb_m(&mut self) -> RXB_M_W {
        RXB_M_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txe_m(&mut self) -> TXE_M_W {
        TXE_M_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn txb_m(&mut self) -> TXB_M_W {
        TXB_M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "INT_MASK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mask](index.html) module"]
pub struct INT_MASK_SPEC;
impl crate::RegisterSpec for INT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_mask::R](R) reader structure"]
impl crate::Readable for INT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_mask::W](W) writer structure"]
impl crate::Writable for INT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_MASK to value 0"]
impl crate::Resettable for INT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
