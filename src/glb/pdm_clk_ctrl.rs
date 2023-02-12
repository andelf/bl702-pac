#[doc = "Register `PDM_CLK_CTRL` reader"]
pub struct R(crate::R<PDM_CLK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_CLK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_CLK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_CLK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDM_CLK_CTRL` writer"]
pub struct W(crate::W<PDM_CLK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_CLK_CTRL_SPEC>;
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
impl From<crate::W<PDM_CLK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_CLK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_pdm0_clk_div` reader - "]
pub type REG_PDM0_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_pdm0_clk_div` writer - "]
pub type REG_PDM0_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDM_CLK_CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `reg_pdm0_clk_en` reader - "]
pub type REG_PDM0_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_pdm0_clk_en` writer - "]
pub type REG_PDM0_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDM_CLK_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reg_pdm0_clk_div(&self) -> REG_PDM0_CLK_DIV_R {
        REG_PDM0_CLK_DIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_pdm0_clk_en(&self) -> REG_PDM0_CLK_EN_R {
        REG_PDM0_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pdm0_clk_div(&mut self) -> REG_PDM0_CLK_DIV_W<0> {
        REG_PDM0_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_pdm0_clk_en(&mut self) -> REG_PDM0_CLK_EN_W<7> {
        REG_PDM0_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM_CLK_CTRL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_clk_ctrl](index.html) module"]
pub struct PDM_CLK_CTRL_SPEC;
impl crate::RegisterSpec for PDM_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_clk_ctrl::R](R) reader structure"]
impl crate::Readable for PDM_CLK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_clk_ctrl::W](W) writer structure"]
impl crate::Writable for PDM_CLK_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDM_CLK_CTRL to value 0"]
impl crate::Resettable for PDM_CLK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
