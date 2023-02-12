#[doc = "Register `GPIO_CFGCTL17` reader"]
pub struct R(crate::R<GPIO_CFGCTL17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL17` writer"]
pub struct W(crate::W<GPIO_CFGCTL17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL17_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_34_ie` reader - "]
pub type REG_GPIO_34_IE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_34_ie` writer - "]
pub type REG_GPIO_34_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL17_SPEC, bool, O>;
#[doc = "Field `reg_gpio_34_smt` reader - "]
pub type REG_GPIO_34_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_34_smt` writer - "]
pub type REG_GPIO_34_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL17_SPEC, bool, O>;
#[doc = "Field `reg_gpio_34_drv` reader - "]
pub type REG_GPIO_34_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_34_drv` writer - "]
pub type REG_GPIO_34_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL17_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_gpio_34_pu` reader - "]
pub type REG_GPIO_34_PU_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_34_pu` writer - "]
pub type REG_GPIO_34_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL17_SPEC, bool, O>;
#[doc = "Field `reg_gpio_34_pd` reader - "]
pub type REG_GPIO_34_PD_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_34_pd` writer - "]
pub type REG_GPIO_34_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL17_SPEC, bool, O>;
#[doc = "Field `reg_gpio_35_ie` reader - "]
pub type REG_GPIO_35_IE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_35_ie` writer - "]
pub type REG_GPIO_35_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL17_SPEC, bool, O>;
#[doc = "Field `reg_gpio_35_smt` reader - "]
pub type REG_GPIO_35_SMT_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_35_smt` writer - "]
pub type REG_GPIO_35_SMT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPIO_CFGCTL17_SPEC, bool, O>;
#[doc = "Field `reg_gpio_35_drv` reader - "]
pub type REG_GPIO_35_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reg_gpio_35_drv` writer - "]
pub type REG_GPIO_35_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_CFGCTL17_SPEC, u8, u8, 2, O>;
#[doc = "Field `reg_gpio_35_pu` reader - "]
pub type REG_GPIO_35_PU_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_35_pu` writer - "]
pub type REG_GPIO_35_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL17_SPEC, bool, O>;
#[doc = "Field `reg_gpio_35_pd` reader - "]
pub type REG_GPIO_35_PD_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_35_pd` writer - "]
pub type REG_GPIO_35_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL17_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_34_ie(&self) -> REG_GPIO_34_IE_R {
        REG_GPIO_34_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_34_smt(&self) -> REG_GPIO_34_SMT_R {
        REG_GPIO_34_SMT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn reg_gpio_34_drv(&self) -> REG_GPIO_34_DRV_R {
        REG_GPIO_34_DRV_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_34_pu(&self) -> REG_GPIO_34_PU_R {
        REG_GPIO_34_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_34_pd(&self) -> REG_GPIO_34_PD_R {
        REG_GPIO_34_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_35_ie(&self) -> REG_GPIO_35_IE_R {
        REG_GPIO_35_IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_35_smt(&self) -> REG_GPIO_35_SMT_R {
        REG_GPIO_35_SMT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn reg_gpio_35_drv(&self) -> REG_GPIO_35_DRV_R {
        REG_GPIO_35_DRV_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_35_pu(&self) -> REG_GPIO_35_PU_R {
        REG_GPIO_35_PU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_35_pd(&self) -> REG_GPIO_35_PD_R {
        REG_GPIO_35_PD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_34_ie(&mut self) -> REG_GPIO_34_IE_W<0> {
        REG_GPIO_34_IE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_34_smt(&mut self) -> REG_GPIO_34_SMT_W<1> {
        REG_GPIO_34_SMT_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_34_drv(&mut self) -> REG_GPIO_34_DRV_W<2> {
        REG_GPIO_34_DRV_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_34_pu(&mut self) -> REG_GPIO_34_PU_W<4> {
        REG_GPIO_34_PU_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_34_pd(&mut self) -> REG_GPIO_34_PD_W<5> {
        REG_GPIO_34_PD_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_35_ie(&mut self) -> REG_GPIO_35_IE_W<16> {
        REG_GPIO_35_IE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_35_smt(&mut self) -> REG_GPIO_35_SMT_W<17> {
        REG_GPIO_35_SMT_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_35_drv(&mut self) -> REG_GPIO_35_DRV_W<18> {
        REG_GPIO_35_DRV_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_35_pu(&mut self) -> REG_GPIO_35_PU_W<20> {
        REG_GPIO_35_PU_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_35_pd(&mut self) -> REG_GPIO_35_PD_W<21> {
        REG_GPIO_35_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_CFGCTL17.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl17](index.html) module"]
pub struct GPIO_CFGCTL17_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl17::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl17::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL17_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL17 to value 0"]
impl crate::Resettable for GPIO_CFGCTL17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
