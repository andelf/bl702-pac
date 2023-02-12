#[doc = "Register `TICR3` reader"]
pub struct R(crate::R<TICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TICR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TICR3` writer"]
pub struct W(crate::W<TICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TICR3_SPEC>;
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
impl From<crate::W<TICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tclr_0` reader - "]
pub type TCLR_0_R = crate::BitReader<bool>;
#[doc = "Field `tclr_0` writer - "]
pub type TCLR_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TICR3_SPEC, bool, O>;
#[doc = "Field `tclr_1` reader - "]
pub type TCLR_1_R = crate::BitReader<bool>;
#[doc = "Field `tclr_1` writer - "]
pub type TCLR_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TICR3_SPEC, bool, O>;
#[doc = "Field `tclr_2` reader - "]
pub type TCLR_2_R = crate::BitReader<bool>;
#[doc = "Field `tclr_2` writer - "]
pub type TCLR_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TICR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tclr_0(&self) -> TCLR_0_R {
        TCLR_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tclr_1(&self) -> TCLR_1_R {
        TCLR_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tclr_2(&self) -> TCLR_2_R {
        TCLR_2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tclr_0(&mut self) -> TCLR_0_W<0> {
        TCLR_0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tclr_1(&mut self) -> TCLR_1_W<1> {
        TCLR_1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tclr_2(&mut self) -> TCLR_2_W<2> {
        TCLR_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TICR3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ticr3](index.html) module"]
pub struct TICR3_SPEC;
impl crate::RegisterSpec for TICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ticr3::R](R) reader structure"]
impl crate::Readable for TICR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ticr3::W](W) writer structure"]
impl crate::Writable for TICR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TICR3 to value 0"]
impl crate::Resettable for TICR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
