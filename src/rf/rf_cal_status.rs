#[doc = "Register `rf_cal_status` reader"]
pub struct R(crate::R<RF_CAL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_CAL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_CAL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_CAL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_cal_status` writer"]
pub struct W(crate::W<RF_CAL_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_CAL_STATUS_SPEC>;
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
impl From<crate::W<RF_CAL_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_CAL_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dl_rfcal_table_status` reader - "]
pub struct DL_RFCAL_TABLE_STATUS_R(crate::FieldReader<u8, u8>);
impl DL_RFCAL_TABLE_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DL_RFCAL_TABLE_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DL_RFCAL_TABLE_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dl_rfcal_table_status` writer - "]
pub struct DL_RFCAL_TABLE_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DL_RFCAL_TABLE_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `rccal_status` reader - "]
pub struct RCCAL_STATUS_R(crate::FieldReader<u8, u8>);
impl RCCAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RCCAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCCAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rccal_status` writer - "]
pub struct RCCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `roscal_status` reader - "]
pub struct ROSCAL_STATUS_R(crate::FieldReader<u8, u8>);
impl ROSCAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ROSCAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROSCAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `roscal_status` writer - "]
pub struct ROSCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ROSCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `kcal_status` reader - "]
pub struct KCAL_STATUS_R(crate::FieldReader<u8, u8>);
impl KCAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        KCAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KCAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `kcal_status` writer - "]
pub struct KCAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> KCAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `acal_status` reader - "]
pub struct ACAL_STATUS_R(crate::FieldReader<u8, u8>);
impl ACAL_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACAL_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACAL_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acal_status` writer - "]
pub struct ACAL_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACAL_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dl_rfcal_table_status(&self) -> DL_RFCAL_TABLE_STATUS_R {
        DL_RFCAL_TABLE_STATUS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rccal_status(&self) -> RCCAL_STATUS_R {
        RCCAL_STATUS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn roscal_status(&self) -> ROSCAL_STATUS_R {
        ROSCAL_STATUS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn kcal_status(&self) -> KCAL_STATUS_R {
        KCAL_STATUS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn acal_status(&self) -> ACAL_STATUS_R {
        ACAL_STATUS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dl_rfcal_table_status(&mut self) -> DL_RFCAL_TABLE_STATUS_W {
        DL_RFCAL_TABLE_STATUS_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rccal_status(&mut self) -> RCCAL_STATUS_W {
        RCCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn roscal_status(&mut self) -> ROSCAL_STATUS_W {
        ROSCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn kcal_status(&mut self) -> KCAL_STATUS_W {
        KCAL_STATUS_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn acal_status(&mut self) -> ACAL_STATUS_W {
        ACAL_STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_cal_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_cal_status](index.html) module"]
pub struct RF_CAL_STATUS_SPEC;
impl crate::RegisterSpec for RF_CAL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_cal_status::R](R) reader structure"]
impl crate::Readable for RF_CAL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_cal_status::W](W) writer structure"]
impl crate::Writable for RF_CAL_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_cal_status to value 0"]
impl crate::Resettable for RF_CAL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
