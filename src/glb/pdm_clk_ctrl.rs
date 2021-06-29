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
#[doc = "Field `reg_pdm0_clk_en` reader - "]
pub struct REG_PDM0_CLK_EN_R(crate::FieldReader<bool, bool>);
impl REG_PDM0_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_PDM0_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_PDM0_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_pdm0_clk_en` writer - "]
pub struct REG_PDM0_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PDM0_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `reg_pdm0_clk_div` reader - "]
pub struct REG_PDM0_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl REG_PDM0_CLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_PDM0_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_PDM0_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_pdm0_clk_div` writer - "]
pub struct REG_PDM0_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_PDM0_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_pdm0_clk_en(&self) -> REG_PDM0_CLK_EN_R {
        REG_PDM0_CLK_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reg_pdm0_clk_div(&self) -> REG_PDM0_CLK_DIV_R {
        REG_PDM0_CLK_DIV_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_pdm0_clk_en(&mut self) -> REG_PDM0_CLK_EN_W {
        REG_PDM0_CLK_EN_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reg_pdm0_clk_div(&mut self) -> REG_PDM0_CLK_DIV_W {
        REG_PDM0_CLK_DIV_W { w: self }
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
}
#[doc = "`reset()` method sets PDM_CLK_CTRL to value 0"]
impl crate::Resettable for PDM_CLK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
