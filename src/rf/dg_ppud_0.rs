#[doc = "Register `dg_ppud_0` reader"]
pub struct R(crate::R<DG_PPUD_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DG_PPUD_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DG_PPUD_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DG_PPUD_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dg_ppud_0` writer"]
pub struct W(crate::W<DG_PPUD_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DG_PPUD_0_SPEC>;
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
impl From<crate::W<DG_PPUD_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DG_PPUD_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ppud_cnt2` reader - "]
pub type PPUD_CNT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ppud_cnt2` writer - "]
pub type PPUD_CNT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DG_PPUD_0_SPEC, u16, u16, 9, O>;
#[doc = "Field `ppud_cnt1` reader - "]
pub type PPUD_CNT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ppud_cnt1` writer - "]
pub type PPUD_CNT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DG_PPUD_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `ppud_manaual_en` reader - "]
pub type PPUD_MANAUAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `ppud_manaual_en` writer - "]
pub type PPUD_MANAUAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DG_PPUD_0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn ppud_cnt2(&self) -> PPUD_CNT2_R {
        PPUD_CNT2_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn ppud_cnt1(&self) -> PPUD_CNT1_R {
        PPUD_CNT1_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ppud_manaual_en(&self) -> PPUD_MANAUAL_EN_R {
        PPUD_MANAUAL_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn ppud_cnt2(&mut self) -> PPUD_CNT2_W<16> {
        PPUD_CNT2_W::new(self)
    }
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn ppud_cnt1(&mut self) -> PPUD_CNT1_W<25> {
        PPUD_CNT1_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ppud_manaual_en(&mut self) -> PPUD_MANAUAL_EN_W<30> {
        PPUD_MANAUAL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dg_ppud_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dg_ppud_0](index.html) module"]
pub struct DG_PPUD_0_SPEC;
impl crate::RegisterSpec for DG_PPUD_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dg_ppud_0::R](R) reader structure"]
impl crate::Readable for DG_PPUD_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dg_ppud_0::W](W) writer structure"]
impl crate::Writable for DG_PPUD_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dg_ppud_0 to value 0"]
impl crate::Resettable for DG_PPUD_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
