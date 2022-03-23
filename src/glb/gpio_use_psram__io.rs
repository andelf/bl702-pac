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
pub struct CFG_GPIO_USE_PSRAM_IO_R(crate::FieldReader<u8, u8>);
impl CFG_GPIO_USE_PSRAM_IO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFG_GPIO_USE_PSRAM_IO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_GPIO_USE_PSRAM_IO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cfg_gpio_use_psram_io` writer - "]
pub struct CFG_GPIO_USE_PSRAM_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_GPIO_USE_PSRAM_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
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
    pub fn cfg_gpio_use_psram_io(&mut self) -> CFG_GPIO_USE_PSRAM_IO_W {
        CFG_GPIO_USE_PSRAM_IO_W { w: self }
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
