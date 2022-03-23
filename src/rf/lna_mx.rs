#[doc = "Register `lna_mx` reader"]
pub struct R(crate::R<LNA_MX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LNA_MX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LNA_MX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LNA_MX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lna_mx` writer"]
pub struct W(crate::W<LNA_MX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LNA_MX_SPEC>;
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
impl From<crate::W<LNA_MX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LNA_MX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lna_bm_hw` reader - "]
pub struct LNA_BM_HW_R(crate::FieldReader<u8, u8>);
impl LNA_BM_HW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_BM_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_BM_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_bm_hw` writer - "]
pub struct LNA_BM_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `lna_bm_hg` reader - "]
pub struct LNA_BM_HG_R(crate::FieldReader<u8, u8>);
impl LNA_BM_HG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_BM_HG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_BM_HG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_bm_hg` writer - "]
pub struct LNA_BM_HG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_HG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `lna_bm_lg` reader - "]
pub struct LNA_BM_LG_R(crate::FieldReader<u8, u8>);
impl LNA_BM_LG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_BM_LG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_BM_LG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_bm_lg` writer - "]
pub struct LNA_BM_LG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_BM_LG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `lna_cap_lg` reader - "]
pub struct LNA_CAP_LG_R(crate::FieldReader<u8, u8>);
impl LNA_CAP_LG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_CAP_LG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_CAP_LG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_cap_lg` writer - "]
pub struct LNA_CAP_LG_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CAP_LG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `lna_cap_match` reader - "]
pub struct LNA_CAP_MATCH_R(crate::FieldReader<u8, u8>);
impl LNA_CAP_MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_CAP_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_CAP_MATCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_cap_match` writer - "]
pub struct LNA_CAP_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CAP_MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `lna_lg_gsel` reader - "]
pub struct LNA_LG_GSEL_R(crate::FieldReader<u8, u8>);
impl LNA_LG_GSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_LG_GSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_LG_GSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_lg_gsel` writer - "]
pub struct LNA_LG_GSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LG_GSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `lna_load_csw` reader - "]
pub struct LNA_LOAD_CSW_R(crate::FieldReader<u8, u8>);
impl LNA_LOAD_CSW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_LOAD_CSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_LOAD_CSW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_load_csw` writer - "]
pub struct LNA_LOAD_CSW_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_LOAD_CSW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `lna_vdd13_sel` reader - "]
pub struct LNA_VDD13_SEL_R(crate::FieldReader<u8, u8>);
impl LNA_VDD13_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_VDD13_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_VDD13_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_vdd13_sel` writer - "]
pub struct LNA_VDD13_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_VDD13_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `lna_rfb_match` reader - "]
pub struct LNA_RFB_MATCH_R(crate::FieldReader<u8, u8>);
impl LNA_RFB_MATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LNA_RFB_MATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_RFB_MATCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_rfb_match` writer - "]
pub struct LNA_RFB_MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_RFB_MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `rmx_bm` reader - "]
pub struct RMX_BM_R(crate::FieldReader<u8, u8>);
impl RMX_BM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RMX_BM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMX_BM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rmx_bm` writer - "]
pub struct RMX_BM_W<'a> {
    w: &'a mut W,
}
impl<'a> RMX_BM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn lna_bm_hw(&self) -> LNA_BM_HW_R {
        LNA_BM_HW_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn lna_bm_hg(&self) -> LNA_BM_HG_R {
        LNA_BM_HG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lna_bm_lg(&self) -> LNA_BM_LG_R {
        LNA_BM_LG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lna_cap_lg(&self) -> LNA_CAP_LG_R {
        LNA_CAP_LG_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn lna_cap_match(&self) -> LNA_CAP_MATCH_R {
        LNA_CAP_MATCH_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn lna_lg_gsel(&self) -> LNA_LG_GSEL_R {
        LNA_LG_GSEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&self) -> LNA_LOAD_CSW_R {
        LNA_LOAD_CSW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lna_vdd13_sel(&self) -> LNA_VDD13_SEL_R {
        LNA_VDD13_SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn lna_rfb_match(&self) -> LNA_RFB_MATCH_R {
        LNA_RFB_MATCH_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&self) -> RMX_BM_R {
        RMX_BM_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn lna_bm_hw(&mut self) -> LNA_BM_HW_W {
        LNA_BM_HW_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn lna_bm_hg(&mut self) -> LNA_BM_HG_W {
        LNA_BM_HG_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lna_bm_lg(&mut self) -> LNA_BM_LG_W {
        LNA_BM_LG_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lna_cap_lg(&mut self) -> LNA_CAP_LG_W {
        LNA_CAP_LG_W { w: self }
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn lna_cap_match(&mut self) -> LNA_CAP_MATCH_W {
        LNA_CAP_MATCH_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn lna_lg_gsel(&mut self) -> LNA_LG_GSEL_W {
        LNA_LG_GSEL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&mut self) -> LNA_LOAD_CSW_W {
        LNA_LOAD_CSW_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lna_vdd13_sel(&mut self) -> LNA_VDD13_SEL_W {
        LNA_VDD13_SEL_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn lna_rfb_match(&mut self) -> LNA_RFB_MATCH_W {
        LNA_RFB_MATCH_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&mut self) -> RMX_BM_W {
        RMX_BM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LNA mixer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lna_mx](index.html) module"]
pub struct LNA_MX_SPEC;
impl crate::RegisterSpec for LNA_MX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lna_mx::R](R) reader structure"]
impl crate::Readable for LNA_MX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lna_mx::W](W) writer structure"]
impl crate::Writable for LNA_MX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lna_mx to value 0"]
impl crate::Resettable for LNA_MX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
