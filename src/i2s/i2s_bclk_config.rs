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
#[doc = "Field `cr_bclk_div_l` reader - "]
pub type CR_BCLK_DIV_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_bclk_div_l` writer - "]
pub type CR_BCLK_DIV_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_BCLK_CONFIG_SPEC, u16, u16, 12, O>;
#[doc = "Field `cr_bclk_div_h` reader - "]
pub type CR_BCLK_DIV_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `cr_bclk_div_h` writer - "]
pub type CR_BCLK_DIV_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2S_BCLK_CONFIG_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_bclk_div_l(&self) -> CR_BCLK_DIV_L_R {
        CR_BCLK_DIV_L_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn cr_bclk_div_h(&self) -> CR_BCLK_DIV_H_R {
        CR_BCLK_DIV_H_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_bclk_div_l(&mut self) -> CR_BCLK_DIV_L_W<0> {
        CR_BCLK_DIV_L_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn cr_bclk_div_h(&mut self) -> CR_BCLK_DIV_H_W<16> {
        CR_BCLK_DIV_H_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets i2s_bclk_config to value 0"]
impl crate::Resettable for I2S_BCLK_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
