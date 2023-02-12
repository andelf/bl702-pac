#[doc = "Register `GPIO_CFGCTL0` reader"]
pub struct R(crate::R<GPIO_CFGCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL0` writer"]
pub struct W(crate::W<GPIO_CFGCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL0_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_0_ie` reader - "]
pub type REG_GPIO_0_IE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_ie` writer - "]
pub type REG_GPIO_0_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_0_smt` reader - "]
pub type REG_GPIO_0_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_smt` writer - "]
pub type REG_GPIO_0_SMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_0_drv` reader - "]
pub type REG_GPIO_0_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_0_drv` writer - "]
pub type REG_GPIO_0_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_gpio_0_pu` reader - "]
pub type REG_GPIO_0_PU_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_pu` writer - "]
pub type REG_GPIO_0_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_0_pd` reader - "]
pub type REG_GPIO_0_PD_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_pd` writer - "]
pub type REG_GPIO_0_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_0_func_sel` reader - "]
pub type REG_GPIO_0_FUNC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_0_func_sel` writer - "]
pub type REG_GPIO_0_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL0_SPEC, u8, u8, 5, O>;
#[doc = "Field `reg_gpio_1_ie` reader - "]
pub type REG_GPIO_1_IE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_1_ie` writer - "]
pub type REG_GPIO_1_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_1_smt` reader - "]
pub type REG_GPIO_1_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_1_smt` writer - "]
pub type REG_GPIO_1_SMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_1_drv` reader - "]
pub type REG_GPIO_1_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_1_drv` writer - "]
pub type REG_GPIO_1_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_gpio_1_pu` reader - "]
pub type REG_GPIO_1_PU_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_1_pu` writer - "]
pub type REG_GPIO_1_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_1_pd` reader - "]
pub type REG_GPIO_1_PD_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_1_pd` writer - "]
pub type REG_GPIO_1_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL0_SPEC, bool, O>;
#[doc = "Field `reg_gpio_1_func_sel` reader - "]
pub type REG_GPIO_1_FUNC_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_1_func_sel` writer - "]
pub type REG_GPIO_1_FUNC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_ie(&self) -> REG_GPIO_0_IE_R {
        REG_GPIO_0_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_0_smt(&self) -> REG_GPIO_0_SMT_R {
        REG_GPIO_0_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_0_drv(&self) -> REG_GPIO_0_DRV_R {
        REG_GPIO_0_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_0_pu(&self) -> REG_GPIO_0_PU_R {
        REG_GPIO_0_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_0_pd(&self) -> REG_GPIO_0_PD_R {
        REG_GPIO_0_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn reg_gpio_0_func_sel(&self) -> REG_GPIO_0_FUNC_SEL_R {
        REG_GPIO_0_FUNC_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_1_ie(&self) -> REG_GPIO_1_IE_R {
        REG_GPIO_1_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_1_smt(&self) -> REG_GPIO_1_SMT_R {
        REG_GPIO_1_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_1_drv(&self) -> REG_GPIO_1_DRV_R {
        REG_GPIO_1_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_1_pu(&self) -> REG_GPIO_1_PU_R {
        REG_GPIO_1_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_1_pd(&self) -> REG_GPIO_1_PD_R {
        REG_GPIO_1_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn reg_gpio_1_func_sel(&self) -> REG_GPIO_1_FUNC_SEL_R {
        REG_GPIO_1_FUNC_SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_ie(&mut self) -> REG_GPIO_0_IE_W<0> {
        REG_GPIO_0_IE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_smt(&mut self) -> REG_GPIO_0_SMT_W<1> {
        REG_GPIO_0_SMT_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_drv(&mut self) -> REG_GPIO_0_DRV_W<2> {
        REG_GPIO_0_DRV_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_pu(&mut self) -> REG_GPIO_0_PU_W<4> {
        REG_GPIO_0_PU_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_pd(&mut self) -> REG_GPIO_0_PD_W<5> {
        REG_GPIO_0_PD_W::new(self)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_func_sel(&mut self) -> REG_GPIO_0_FUNC_SEL_W<8> {
        REG_GPIO_0_FUNC_SEL_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_ie(&mut self) -> REG_GPIO_1_IE_W<16> {
        REG_GPIO_1_IE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_smt(&mut self) -> REG_GPIO_1_SMT_W<17> {
        REG_GPIO_1_SMT_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_drv(&mut self) -> REG_GPIO_1_DRV_W<18> {
        REG_GPIO_1_DRV_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_pu(&mut self) -> REG_GPIO_1_PU_W<20> {
        REG_GPIO_1_PU_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_pd(&mut self) -> REG_GPIO_1_PD_W<21> {
        REG_GPIO_1_PD_W::new(self)
    }
    #[doc = "Bits 24:28"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_func_sel(&mut self) -> REG_GPIO_1_FUNC_SEL_W<24> {
        REG_GPIO_1_FUNC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_CFGCTL0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl0](index.html) module"]
pub struct GPIO_CFGCTL0_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl0::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl0::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL0 to value 0"]
impl crate::Resettable for GPIO_CFGCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
