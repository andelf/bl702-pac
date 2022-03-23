#[doc = "Register `GPIO_INT_MODE_SET4` reader"]
pub struct R(crate::R<GPIO_INT_MODE_SET4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_MODE_SET4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_MODE_SET4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_MODE_SET4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_MODE_SET4` writer"]
pub struct W(crate::W<GPIO_INT_MODE_SET4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_MODE_SET4_SPEC>;
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
impl From<crate::W<GPIO_INT_MODE_SET4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_MODE_SET4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_int_mode_set4` reader - "]
pub struct REG_GPIO_INT_MODE_SET4_R(crate::FieldReader<u8, u8>);
impl REG_GPIO_INT_MODE_SET4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_INT_MODE_SET4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_INT_MODE_SET4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_int_mode_set4` writer - "]
pub struct REG_GPIO_INT_MODE_SET4_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_INT_MODE_SET4_W<'a> {
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
    pub fn reg_gpio_int_mode_set4(&self) -> REG_GPIO_INT_MODE_SET4_R {
        REG_GPIO_INT_MODE_SET4_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn reg_gpio_int_mode_set4(&mut self) -> REG_GPIO_INT_MODE_SET4_W {
        REG_GPIO_INT_MODE_SET4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_INT_MODE_SET4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mode_set4](index.html) module"]
pub struct GPIO_INT_MODE_SET4_SPEC;
impl crate::RegisterSpec for GPIO_INT_MODE_SET4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_mode_set4::R](R) reader structure"]
impl crate::Readable for GPIO_INT_MODE_SET4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_mode_set4::W](W) writer structure"]
impl crate::Writable for GPIO_INT_MODE_SET4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_MODE_SET4 to value 0"]
impl crate::Resettable for GPIO_INT_MODE_SET4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
