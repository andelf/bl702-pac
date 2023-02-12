#[doc = "Register `TMSR2` reader"]
pub struct R(crate::R<TMSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMSR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMSR2` writer"]
pub struct W(crate::W<TMSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMSR2_SPEC>;
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
impl From<crate::W<TMSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMSR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tmsr_0` reader - "]
pub type TMSR_0_R = crate::BitReader<bool>;
#[doc = "Field `tmsr_0` writer - "]
pub type TMSR_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMSR2_SPEC, bool, O>;
#[doc = "Field `tmsr_1` reader - "]
pub type TMSR_1_R = crate::BitReader<bool>;
#[doc = "Field `tmsr_1` writer - "]
pub type TMSR_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMSR2_SPEC, bool, O>;
#[doc = "Field `tmsr_2` reader - "]
pub type TMSR_2_R = crate::BitReader<bool>;
#[doc = "Field `tmsr_2` writer - "]
pub type TMSR_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMSR2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmsr_0(&self) -> TMSR_0_R {
        TMSR_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmsr_1(&self) -> TMSR_1_R {
        TMSR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tmsr_2(&self) -> TMSR_2_R {
        TMSR_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tmsr_0(&mut self) -> TMSR_0_W<0> {
        TMSR_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tmsr_1(&mut self) -> TMSR_1_W<1> {
        TMSR_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tmsr_2(&mut self) -> TMSR_2_W<2> {
        TMSR_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TMSR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmsr2](index.html) module"]
pub struct TMSR2_SPEC;
impl crate::RegisterSpec for TMSR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmsr2::R](R) reader structure"]
impl crate::Readable for TMSR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmsr2::W](W) writer structure"]
impl crate::Writable for TMSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMSR2 to value 0"]
impl crate::Resettable for TMSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
