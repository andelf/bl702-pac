#[doc = "Register `rxadc` reader"]
pub struct R(crate::R<RXADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rxadc` writer"]
pub struct W(crate::W<RXADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXADC_SPEC>;
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
impl From<crate::W<RXADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxadc_oscal_en` reader - "]
pub type RXADC_OSCAL_EN_R = crate::BitReader<bool>;
#[doc = "Field `rxadc_oscal_en` writer - "]
pub type RXADC_OSCAL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADC_SPEC, bool, O>;
#[doc = "Field `rxadc_vref_sel` reader - "]
pub type RXADC_VREF_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rxadc_vref_sel` writer - "]
pub type RXADC_VREF_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXADC_SPEC, u8, u8, 2, O>;
#[doc = "Field `rxadc_clk_sync_inv` reader - "]
pub type RXADC_CLK_SYNC_INV_R = crate::BitReader<bool>;
#[doc = "Field `rxadc_clk_sync_inv` writer - "]
pub type RXADC_CLK_SYNC_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADC_SPEC, bool, O>;
#[doc = "Field `rxadc_clk_inv` reader - "]
pub type RXADC_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `rxadc_clk_inv` writer - "]
pub type RXADC_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADC_SPEC, bool, O>;
#[doc = "Field `rxadc_clk_div_sel` reader - "]
pub type RXADC_CLK_DIV_SEL_R = crate::BitReader<bool>;
#[doc = "Field `rxadc_clk_div_sel` writer - "]
pub type RXADC_CLK_DIV_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADC_SPEC, bool, O>;
#[doc = "Field `rxadc_glitch_remove` reader - "]
pub type RXADC_GLITCH_REMOVE_R = crate::BitReader<bool>;
#[doc = "Field `rxadc_glitch_remove` writer - "]
pub type RXADC_GLITCH_REMOVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXADC_SPEC, bool, O>;
#[doc = "Field `rxadc_dly_ctrl` reader - "]
pub type RXADC_DLY_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rxadc_dly_ctrl` writer - "]
pub type RXADC_DLY_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXADC_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxadc_oscal_en(&self) -> RXADC_OSCAL_EN_R {
        RXADC_OSCAL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn rxadc_vref_sel(&self) -> RXADC_VREF_SEL_R {
        RXADC_VREF_SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxadc_clk_sync_inv(&self) -> RXADC_CLK_SYNC_INV_R {
        RXADC_CLK_SYNC_INV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxadc_clk_inv(&self) -> RXADC_CLK_INV_R {
        RXADC_CLK_INV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rxadc_clk_div_sel(&self) -> RXADC_CLK_DIV_SEL_R {
        RXADC_CLK_DIV_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rxadc_glitch_remove(&self) -> RXADC_GLITCH_REMOVE_R {
        RXADC_GLITCH_REMOVE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn rxadc_dly_ctrl(&self) -> RXADC_DLY_CTRL_R {
        RXADC_DLY_CTRL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxadc_oscal_en(&mut self) -> RXADC_OSCAL_EN_W<0> {
        RXADC_OSCAL_EN_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn rxadc_vref_sel(&mut self) -> RXADC_VREF_SEL_W<4> {
        RXADC_VREF_SEL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn rxadc_clk_sync_inv(&mut self) -> RXADC_CLK_SYNC_INV_W<8> {
        RXADC_CLK_SYNC_INV_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rxadc_clk_inv(&mut self) -> RXADC_CLK_INV_W<12> {
        RXADC_CLK_INV_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn rxadc_clk_div_sel(&mut self) -> RXADC_CLK_DIV_SEL_W<16> {
        RXADC_CLK_DIV_SEL_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn rxadc_glitch_remove(&mut self) -> RXADC_GLITCH_REMOVE_W<20> {
        RXADC_GLITCH_REMOVE_W::new(self)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    #[must_use]
    pub fn rxadc_dly_ctrl(&mut self) -> RXADC_DLY_CTRL_W<24> {
        RXADC_DLY_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rxadc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxadc](index.html) module"]
pub struct RXADC_SPEC;
impl crate::RegisterSpec for RXADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxadc::R](R) reader structure"]
impl crate::Readable for RXADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxadc::W](W) writer structure"]
impl crate::Writable for RXADC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets rxadc to value 0"]
impl crate::Resettable for RXADC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
