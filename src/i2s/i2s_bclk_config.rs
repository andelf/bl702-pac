#[doc = "Register `i2s_bclk_config` reader"]
pub struct R(crate::R<I2S_BCLK_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_BCLK_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_BCLK_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_BCLK_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2s_bclk_config` writer"]
pub struct W(crate::W<I2S_BCLK_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_BCLK_CONFIG_SPEC>;
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
impl From<crate::W<I2S_BCLK_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_BCLK_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_bclk_div_h` reader - "]
pub struct CR_BCLK_DIV_H_R(crate::FieldReader<u16, u16>);
impl CR_BCLK_DIV_H_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_BCLK_DIV_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_BCLK_DIV_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_bclk_div_h` writer - "]
pub struct CR_BCLK_DIV_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_BCLK_DIV_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
#[doc = "Field `cr_bclk_div_l` reader - "]
pub struct CR_BCLK_DIV_L_R(crate::FieldReader<u16, u16>);
impl CR_BCLK_DIV_L_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_BCLK_DIV_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_BCLK_DIV_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_bclk_div_l` writer - "]
pub struct CR_BCLK_DIV_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_BCLK_DIV_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn cr_bclk_div_h(&self) -> CR_BCLK_DIV_H_R {
        CR_BCLK_DIV_H_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_bclk_div_l(&self) -> CR_BCLK_DIV_L_R {
        CR_BCLK_DIV_L_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn cr_bclk_div_h(&mut self) -> CR_BCLK_DIV_H_W {
        CR_BCLK_DIV_H_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_bclk_div_l(&mut self) -> CR_BCLK_DIV_L_W {
        CR_BCLK_DIV_L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2s_bclk_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_bclk_config](index.html) module"]
pub struct I2S_BCLK_CONFIG_SPEC;
impl crate::RegisterSpec for I2S_BCLK_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_bclk_config::R](R) reader structure"]
impl crate::Readable for I2S_BCLK_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_bclk_config::W](W) writer structure"]
impl crate::Writable for I2S_BCLK_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2s_bclk_config to value 0"]
impl crate::Resettable for I2S_BCLK_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
