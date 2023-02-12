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
#[doc = "Field `acal_status` reader - "]
pub type ACAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `acal_status` writer - "]
pub type ACAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_CAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `kcal_status` reader - "]
pub type KCAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `kcal_status` writer - "]
pub type KCAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_CAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `roscal_status` reader - "]
pub type ROSCAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `roscal_status` writer - "]
pub type ROSCAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_CAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `rccal_status` reader - "]
pub type RCCAL_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rccal_status` writer - "]
pub type RCCAL_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_CAL_STATUS_SPEC, u8, u8, 2, O>;
#[doc = "Field `dl_rfcal_table_status` reader - "]
pub type DL_RFCAL_TABLE_STATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `dl_rfcal_table_status` writer - "]
pub type DL_RFCAL_TABLE_STATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RF_CAL_STATUS_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn acal_status(&self) -> ACAL_STATUS_R {
        ACAL_STATUS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn kcal_status(&self) -> KCAL_STATUS_R {
        KCAL_STATUS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn roscal_status(&self) -> ROSCAL_STATUS_R {
        ROSCAL_STATUS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rccal_status(&self) -> RCCAL_STATUS_R {
        RCCAL_STATUS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn dl_rfcal_table_status(&self) -> DL_RFCAL_TABLE_STATUS_R {
        DL_RFCAL_TABLE_STATUS_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn acal_status(&mut self) -> ACAL_STATUS_W<2> {
        ACAL_STATUS_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn kcal_status(&mut self) -> KCAL_STATUS_W<4> {
        KCAL_STATUS_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn roscal_status(&mut self) -> ROSCAL_STATUS_W<6> {
        ROSCAL_STATUS_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn rccal_status(&mut self) -> RCCAL_STATUS_W<8> {
        RCCAL_STATUS_W::new(self)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    #[must_use]
    pub fn dl_rfcal_table_status(&mut self) -> DL_RFCAL_TABLE_STATUS_W<30> {
        DL_RFCAL_TABLE_STATUS_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rf_cal_status to value 0"]
impl crate::Resettable for RF_CAL_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
