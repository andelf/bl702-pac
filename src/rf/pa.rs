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
#[doc = "Field `pa_seri_cs_tx` reader - "]
pub type PA_SERI_CS_TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_seri_cs_tx` writer - "]
pub type PA_SERI_CS_TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_seri_cs_rx` reader - "]
pub type PA_SERI_CS_RX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_seri_cs_rx` writer - "]
pub type PA_SERI_CS_RX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_seri_cs_hw` reader - "]
pub type PA_SERI_CS_HW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_seri_cs_hw` writer - "]
pub type PA_SERI_CS_HW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_para_cs` reader - "]
pub type PA_PARA_CS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_para_cs` writer - "]
pub type PA_PARA_CS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA_SPEC, u8, u8, 4, O>;
#[doc = "Field `pa_vdd11_sel` reader - "]
pub type PA_VDD11_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_vdd11_sel` writer - "]
pub type PA_VDD11_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA_SPEC, u8, u8, 3, O>;
#[doc = "Field `pa_ldo_bm` reader - "]
pub type PA_LDO_BM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pa_ldo_bm` writer - "]
pub type PA_LDO_BM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PA_SPEC, u8, u8, 3, O>;
#[doc = "Field `pa_lp_en` reader - "]
pub type PA_LP_EN_R = crate::BitReader<bool>;
#[doc = "Field `pa_lp_en` writer - "]
pub type PA_LP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA_SPEC, bool, O>;
#[doc = "Field `pa_hp_en` reader - "]
pub type PA_HP_EN_R = crate::BitReader<bool>;
#[doc = "Field `pa_hp_en` writer - "]
pub type PA_HP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA_SPEC, bool, O>;
#[doc = "Field `pa_force_short_open` reader - "]
pub type PA_FORCE_SHORT_OPEN_R = crate::BitReader<bool>;
#[doc = "Field `pa_force_short_open` writer - "]
pub type PA_FORCE_SHORT_OPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_seri_cs_tx(&self) -> PA_SERI_CS_TX_R {
        PA_SERI_CS_TX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_seri_cs_rx(&self) -> PA_SERI_CS_RX_R {
        PA_SERI_CS_RX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_seri_cs_hw(&self) -> PA_SERI_CS_HW_R {
        PA_SERI_CS_HW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_para_cs(&self) -> PA_PARA_CS_R {
        PA_PARA_CS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vdd11_sel(&self) -> PA_VDD11_SEL_R {
        PA_VDD11_SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_ldo_bm(&self) -> PA_LDO_BM_R {
        PA_LDO_BM_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pa_lp_en(&self) -> PA_LP_EN_R {
        PA_LP_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pa_hp_en(&self) -> PA_HP_EN_R {
        PA_HP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pa_force_short_open(&self) -> PA_FORCE_SHORT_OPEN_R {
        PA_FORCE_SHORT_OPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_seri_cs_tx(&mut self) -> PA_SERI_CS_TX_W<0> {
        PA_SERI_CS_TX_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_seri_cs_rx(&mut self) -> PA_SERI_CS_RX_W<4> {
        PA_SERI_CS_RX_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pa_seri_cs_hw(&mut self) -> PA_SERI_CS_HW_W<8> {
        PA_SERI_CS_HW_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_para_cs(&mut self) -> PA_PARA_CS_W<12> {
        PA_PARA_CS_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vdd11_sel(&mut self) -> PA_VDD11_SEL_W<20> {
        PA_VDD11_SEL_W::new(self)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn pa_ldo_bm(&mut self) -> PA_LDO_BM_W<24> {
        PA_LDO_BM_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pa_lp_en(&mut self) -> PA_LP_EN_W<28> {
        PA_LP_EN_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pa_hp_en(&mut self) -> PA_HP_EN_W<29> {
        PA_HP_EN_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pa_force_short_open(&mut self) -> PA_FORCE_SHORT_OPEN_W<30> {
        PA_FORCE_SHORT_OPEN_W::new(self)
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
