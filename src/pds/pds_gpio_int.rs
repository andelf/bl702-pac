#[doc = "Register `pds_gpio_int` reader"]
pub struct R(crate::R<PDS_GPIO_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_GPIO_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_GPIO_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_GPIO_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_gpio_int` writer"]
pub struct W(crate::W<PDS_GPIO_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_GPIO_INT_SPEC>;
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
impl From<crate::W<PDS_GPIO_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_GPIO_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pds_gpio_int_mask` reader - "]
pub type PDS_GPIO_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_int_mask` writer - "]
pub type PDS_GPIO_INT_MASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_INT_SPEC, bool, O>;
#[doc = "Field `pds_gpio_int_stat` reader - "]
pub type PDS_GPIO_INT_STAT_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_int_stat` writer - "]
pub type PDS_GPIO_INT_STAT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_INT_SPEC, bool, O>;
#[doc = "Field `pds_gpio_int_clr` reader - "]
pub type PDS_GPIO_INT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `pds_gpio_int_clr` writer - "]
pub type PDS_GPIO_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_GPIO_INT_SPEC, bool, O>;
#[doc = "Field `pds_gpio_int_mode` reader - "]
pub type PDS_GPIO_INT_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_gpio_int_mode` writer - "]
pub type PDS_GPIO_INT_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_GPIO_INT_SPEC, u8, u8, 3, O>;
#[doc = "Field `pds_gpio_int_select` reader - "]
pub type PDS_GPIO_INT_SELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pds_gpio_int_select` writer - "]
pub type PDS_GPIO_INT_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PDS_GPIO_INT_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pds_gpio_int_mask(&self) -> PDS_GPIO_INT_MASK_R {
        PDS_GPIO_INT_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pds_gpio_int_stat(&self) -> PDS_GPIO_INT_STAT_R {
        PDS_GPIO_INT_STAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pds_gpio_int_clr(&self) -> PDS_GPIO_INT_CLR_R {
        PDS_GPIO_INT_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn pds_gpio_int_mode(&self) -> PDS_GPIO_INT_MODE_R {
        PDS_GPIO_INT_MODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pds_gpio_int_select(&self) -> PDS_GPIO_INT_SELECT_R {
        PDS_GPIO_INT_SELECT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_int_mask(&mut self) -> PDS_GPIO_INT_MASK_W<0> {
        PDS_GPIO_INT_MASK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_int_stat(&mut self) -> PDS_GPIO_INT_STAT_W<1> {
        PDS_GPIO_INT_STAT_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_int_clr(&mut self) -> PDS_GPIO_INT_CLR_W<2> {
        PDS_GPIO_INT_CLR_W::new(self)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_int_mode(&mut self) -> PDS_GPIO_INT_MODE_W<4> {
        PDS_GPIO_INT_MODE_W::new(self)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    #[must_use]
    pub fn pds_gpio_int_select(&mut self) -> PDS_GPIO_INT_SELECT_W<8> {
        PDS_GPIO_INT_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pds_gpio_int.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_gpio_int](index.html) module"]
pub struct PDS_GPIO_INT_SPEC;
impl crate::RegisterSpec for PDS_GPIO_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_gpio_int::R](R) reader structure"]
impl crate::Readable for PDS_GPIO_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_gpio_int::W](W) writer structure"]
impl crate::Writable for PDS_GPIO_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_gpio_int to value 0"]
impl crate::Resettable for PDS_GPIO_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
