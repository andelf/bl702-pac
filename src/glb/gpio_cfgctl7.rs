#[doc = "Register `GPIO_CFGCTL7` reader"]
pub struct R(crate::R<GPIO_CFGCTL7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL7` writer"]
pub struct W(crate::W<GPIO_CFGCTL7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL7_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_14_ie` reader - "]
pub type REG_GPIO_14_IE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_14_ie` writer - "]
pub type REG_GPIO_14_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, bool, O>;
#[doc = "Field `reg_gpio_14_smt` reader - "]
pub type REG_GPIO_14_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_14_smt` writer - "]
pub type REG_GPIO_14_SMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, bool, O>;
#[doc = "Field `reg_gpio_14_drv` reader - "]
pub type REG_GPIO_14_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_14_drv` writer - "]
pub type REG_GPIO_14_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL7_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_gpio_14_pu` reader - "]
pub type REG_GPIO_14_PU_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_14_pu` writer - "]
pub type REG_GPIO_14_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, bool, O>;
#[doc = "Field `reg_gpio_14_pd` reader - "]
pub type REG_GPIO_14_PD_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_14_pd` writer - "]
pub type REG_GPIO_14_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, bool, O>;
#[doc = "Field `reg_gpio_14_func_sel` reader - "]
pub type REG_GPIO_14_FUNC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_14_func_sel` writer - "]
pub type REG_GPIO_14_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL7_SPEC, u8, u8, 5, O>;
#[doc = "Field `reg_gpio_15_ie` reader - "]
pub type REG_GPIO_15_IE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_15_ie` writer - "]
pub type REG_GPIO_15_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, bool, O>;
#[doc = "Field `reg_gpio_15_smt` reader - "]
pub type REG_GPIO_15_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_15_smt` writer - "]
pub type REG_GPIO_15_SMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, bool, O>;
#[doc = "Field `reg_gpio_15_drv` reader - "]
pub type REG_GPIO_15_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_15_drv` writer - "]
pub type REG_GPIO_15_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL7_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_gpio_15_pu` reader - "]
pub type REG_GPIO_15_PU_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_15_pu` writer - "]
pub type REG_GPIO_15_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, bool, O>;
#[doc = "Field `reg_gpio_15_pd` reader - "]
pub type REG_GPIO_15_PD_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_15_pd` writer - "]
pub type REG_GPIO_15_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL7_SPEC, bool, O>;
#[doc = "Field `reg_gpio_15_func_sel` reader - "]
pub type REG_GPIO_15_FUNC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_15_func_sel` writer - "]
pub type REG_GPIO_15_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL7_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_14_ie(&self) -> REG_GPIO_14_IE_R {
        REG_GPIO_14_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_14_smt(&self) -> REG_GPIO_14_SMT_R {
        REG_GPIO_14_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_14_drv(&self) -> REG_GPIO_14_DRV_R {
        REG_GPIO_14_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_14_pu(&self) -> REG_GPIO_14_PU_R {
        REG_GPIO_14_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_14_pd(&self) -> REG_GPIO_14_PD_R {
        REG_GPIO_14_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn reg_gpio_14_func_sel(&self) -> REG_GPIO_14_FUNC_SEL_R {
        REG_GPIO_14_FUNC_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_15_ie(&self) -> REG_GPIO_15_IE_R {
        REG_GPIO_15_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_15_smt(&self) -> REG_GPIO_15_SMT_R {
        REG_GPIO_15_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_15_drv(&self) -> REG_GPIO_15_DRV_R {
        REG_GPIO_15_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_15_pu(&self) -> REG_GPIO_15_PU_R {
        REG_GPIO_15_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_15_pd(&self) -> REG_GPIO_15_PD_R {
        REG_GPIO_15_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn reg_gpio_15_func_sel(&self) -> REG_GPIO_15_FUNC_SEL_R {
        REG_GPIO_15_FUNC_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_14_ie(&mut self) -> REG_GPIO_14_IE_W<0> {
        REG_GPIO_14_IE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_14_smt(&mut self) -> REG_GPIO_14_SMT_W<1> {
        REG_GPIO_14_SMT_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_14_drv(&mut self) -> REG_GPIO_14_DRV_W<2> {
        REG_GPIO_14_DRV_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_14_pu(&mut self) -> REG_GPIO_14_PU_W<4> {
        REG_GPIO_14_PU_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_14_pd(&mut self) -> REG_GPIO_14_PD_W<5> {
        REG_GPIO_14_PD_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn reg_gpio_14_func_sel(&mut self) -> REG_GPIO_14_FUNC_SEL_W<8> {
        REG_GPIO_14_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_15_ie(&mut self) -> REG_GPIO_15_IE_W<16> {
        REG_GPIO_15_IE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_15_smt(&mut self) -> REG_GPIO_15_SMT_W<17> {
        REG_GPIO_15_SMT_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_15_drv(&mut self) -> REG_GPIO_15_DRV_W<18> {
        REG_GPIO_15_DRV_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_15_pu(&mut self) -> REG_GPIO_15_PU_W<20> {
        REG_GPIO_15_PU_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_15_pd(&mut self) -> REG_GPIO_15_PD_W<21> {
        REG_GPIO_15_PD_W::new(self)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn reg_gpio_15_func_sel(&mut self) -> REG_GPIO_15_FUNC_SEL_W<24> {
        REG_GPIO_15_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_CFGCTL7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl7](index.html) module"]
pub struct GPIO_CFGCTL7_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl7::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl7::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL7 to value 0"]
impl crate::Resettable for GPIO_CFGCTL7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
