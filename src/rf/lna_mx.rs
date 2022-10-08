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
#[doc = "Field `rmx_bm` reader - "]
pub type RMX_BM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rmx_bm` writer - "]
pub type RMX_BM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_MX_SPEC, u8, u8, 3, O>;
#[doc = "Field `lna_rfb_match` reader - "]
pub type LNA_RFB_MATCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_rfb_match` writer - "]
pub type LNA_RFB_MATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_MX_SPEC, u8, u8, 3, O>;
#[doc = "Field `lna_vdd13_sel` reader - "]
pub type LNA_VDD13_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_vdd13_sel` writer - "]
pub type LNA_VDD13_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_MX_SPEC, u8, u8, 2, O>;
#[doc = "Field `lna_load_csw` reader - "]
pub type LNA_LOAD_CSW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_load_csw` writer - "]
pub type LNA_LOAD_CSW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_MX_SPEC, u8, u8, 4, O>;
#[doc = "Field `lna_lg_gsel` reader - "]
pub type LNA_LG_GSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_lg_gsel` writer - "]
pub type LNA_LG_GSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_MX_SPEC, u8, u8, 3, O>;
#[doc = "Field `lna_cap_match` reader - "]
pub type LNA_CAP_MATCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_cap_match` writer - "]
pub type LNA_CAP_MATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_MX_SPEC, u8, u8, 3, O>;
#[doc = "Field `lna_cap_lg` reader - "]
pub type LNA_CAP_LG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_cap_lg` writer - "]
pub type LNA_CAP_LG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_MX_SPEC, u8, u8, 2, O>;
#[doc = "Field `lna_bm_lg` reader - "]
pub type LNA_BM_LG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_bm_lg` writer - "]
pub type LNA_BM_LG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_MX_SPEC, u8, u8, 4, O>;
#[doc = "Field `lna_bm_hg` reader - "]
pub type LNA_BM_HG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_bm_hg` writer - "]
pub type LNA_BM_HG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_MX_SPEC, u8, u8, 4, O>;
#[doc = "Field `lna_bm_hw` reader - "]
pub type LNA_BM_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lna_bm_hw` writer - "]
pub type LNA_BM_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LNA_MX_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&self) -> RMX_BM_R {
        RMX_BM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn lna_rfb_match(&self) -> LNA_RFB_MATCH_R {
        LNA_RFB_MATCH_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lna_vdd13_sel(&self) -> LNA_VDD13_SEL_R {
        LNA_VDD13_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&self) -> LNA_LOAD_CSW_R {
        LNA_LOAD_CSW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn lna_lg_gsel(&self) -> LNA_LG_GSEL_R {
        LNA_LG_GSEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn lna_cap_match(&self) -> LNA_CAP_MATCH_R {
        LNA_CAP_MATCH_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lna_cap_lg(&self) -> LNA_CAP_LG_R {
        LNA_CAP_LG_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lna_bm_lg(&self) -> LNA_BM_LG_R {
        LNA_BM_LG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn lna_bm_hg(&self) -> LNA_BM_HG_R {
        LNA_BM_HG_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn lna_bm_hw(&self) -> LNA_BM_HW_R {
        LNA_BM_HW_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rmx_bm(&mut self) -> RMX_BM_W<0> {
        RMX_BM_W::new(self)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn lna_rfb_match(&mut self) -> LNA_RFB_MATCH_W<3> {
        LNA_RFB_MATCH_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn lna_vdd13_sel(&mut self) -> LNA_VDD13_SEL_W<6> {
        LNA_VDD13_SEL_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn lna_load_csw(&mut self) -> LNA_LOAD_CSW_W<8> {
        LNA_LOAD_CSW_W::new(self)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn lna_lg_gsel(&mut self) -> LNA_LG_GSEL_W<12> {
        LNA_LG_GSEL_W::new(self)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn lna_cap_match(&mut self) -> LNA_CAP_MATCH_W<15> {
        LNA_CAP_MATCH_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn lna_cap_lg(&mut self) -> LNA_CAP_LG_W<18> {
        LNA_CAP_LG_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn lna_bm_lg(&mut self) -> LNA_BM_LG_W<20> {
        LNA_BM_LG_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn lna_bm_hg(&mut self) -> LNA_BM_HG_W<24> {
        LNA_BM_HG_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn lna_bm_hw(&mut self) -> LNA_BM_HW_W<28> {
        LNA_BM_HW_W::new(self)
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
