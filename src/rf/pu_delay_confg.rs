#[doc = "Register `pu_delay_confg` reader"]
pub struct R(crate::R<PU_DELAY_CONFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PU_DELAY_CONFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PU_DELAY_CONFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PU_DELAY_CONFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pu_delay_confg` writer"]
pub struct W(crate::W<PU_DELAY_CONFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PU_DELAY_CONFG_SPEC>;
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
impl From<crate::W<PU_DELAY_CONFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PU_DELAY_CONFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ppu_lead` reader - "]
pub type PPU_LEAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ppu_lead` writer - "]
pub type PPU_LEAD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PU_DELAY_CONFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `pud_delay` reader - "]
pub type PUD_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pud_delay` writer - "]
pub type PUD_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PU_DELAY_CONFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_reset_delay` reader - "]
pub type LO_RESET_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_reset_delay` writer - "]
pub type LO_RESET_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PU_DELAY_CONFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `lo_reset_width` reader - "]
pub type LO_RESET_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `lo_reset_width` writer - "]
pub type LO_RESET_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PU_DELAY_CONFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `adpll_reset_width` reader - "]
pub type ADPLL_RESET_WIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `adpll_reset_width` writer - "]
pub type ADPLL_RESET_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PU_DELAY_CONFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ppu_lead(&self) -> PPU_LEAD_R {
        PPU_LEAD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pud_delay(&self) -> PUD_DELAY_R {
        PUD_DELAY_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_reset_delay(&self) -> LO_RESET_DELAY_R {
        LO_RESET_DELAY_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_reset_width(&self) -> LO_RESET_WIDTH_R {
        LO_RESET_WIDTH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_reset_width(&self) -> ADPLL_RESET_WIDTH_R {
        ADPLL_RESET_WIDTH_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ppu_lead(&mut self) -> PPU_LEAD_W<0> {
        PPU_LEAD_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pud_delay(&mut self) -> PUD_DELAY_W<4> {
        PUD_DELAY_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_reset_delay(&mut self) -> LO_RESET_DELAY_W<12> {
        LO_RESET_DELAY_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_reset_width(&mut self) -> LO_RESET_WIDTH_W<16> {
        LO_RESET_WIDTH_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn adpll_reset_width(&mut self) -> ADPLL_RESET_WIDTH_W<20> {
        ADPLL_RESET_WIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pu_delay_confg.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pu_delay_confg](index.html) module"]
pub struct PU_DELAY_CONFG_SPEC;
impl crate::RegisterSpec for PU_DELAY_CONFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pu_delay_confg::R](R) reader structure"]
impl crate::Readable for PU_DELAY_CONFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pu_delay_confg::W](W) writer structure"]
impl crate::Writable for PU_DELAY_CONFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pu_delay_confg to value 0"]
impl crate::Resettable for PU_DELAY_CONFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
