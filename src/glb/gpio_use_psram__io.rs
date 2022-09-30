#[doc = "Register `GPIO_USE_PSRAM__IO` reader"]
pub struct R(crate::R<GPIO_USE_PSRAM__IO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_USE_PSRAM__IO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_USE_PSRAM__IO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_USE_PSRAM__IO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_USE_PSRAM__IO` writer"]
pub struct W(crate::W<GPIO_USE_PSRAM__IO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_USE_PSRAM__IO_SPEC>;
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
impl From<crate::W<GPIO_USE_PSRAM__IO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_USE_PSRAM__IO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cfg_gpio_use_psram_io` reader - "]
pub type CFG_GPIO_USE_PSRAM_IO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cfg_gpio_use_psram_io` writer - "]
pub type CFG_GPIO_USE_PSRAM_IO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_USE_PSRAM__IO_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn cfg_gpio_use_psram_io(&self) -> CFG_GPIO_USE_PSRAM_IO_R {
        CFG_GPIO_USE_PSRAM_IO_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn cfg_gpio_use_psram_io(&mut self) -> CFG_GPIO_USE_PSRAM_IO_W<0> {
        CFG_GPIO_USE_PSRAM_IO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_USE_PSRAM__IO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_use_psram__io](index.html) module"]
pub struct GPIO_USE_PSRAM__IO_SPEC;
impl crate::RegisterSpec for GPIO_USE_PSRAM__IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_use_psram__io::R](R) reader structure"]
impl crate::Readable for GPIO_USE_PSRAM__IO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_use_psram__io::W](W) writer structure"]
impl crate::Writable for GPIO_USE_PSRAM__IO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_USE_PSRAM__IO to value 0"]
impl crate::Resettable for GPIO_USE_PSRAM__IO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
