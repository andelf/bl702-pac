#[doc = "Register `MIICOMMAND` reader"]
pub struct R(crate::R<MIICOMMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIICOMMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIICOMMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIICOMMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIICOMMAND` writer"]
pub struct W(crate::W<MIICOMMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIICOMMAND_SPEC>;
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
impl From<crate::W<MIICOMMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIICOMMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCANSTAT` reader - "]
pub type SCANSTAT_R = crate::BitReader<bool>;
#[doc = "Field `SCANSTAT` writer - "]
pub type SCANSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIICOMMAND_SPEC, bool, O>;
#[doc = "Field `RSTAT` reader - "]
pub type RSTAT_R = crate::BitReader<bool>;
#[doc = "Field `RSTAT` writer - "]
pub type RSTAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIICOMMAND_SPEC, bool, O>;
#[doc = "Field `WCTRLDATA` reader - "]
pub type WCTRLDATA_R = crate::BitReader<bool>;
#[doc = "Field `WCTRLDATA` writer - "]
pub type WCTRLDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIICOMMAND_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scanstat(&self) -> SCANSTAT_R {
        SCANSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstat(&self) -> RSTAT_R {
        RSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wctrldata(&self) -> WCTRLDATA_R {
        WCTRLDATA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scanstat(&mut self) -> SCANSTAT_W<0> {
        SCANSTAT_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rstat(&mut self) -> RSTAT_W<1> {
        RSTAT_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wctrldata(&mut self) -> WCTRLDATA_W<2> {
        WCTRLDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIICOMMAND.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miicommand](index.html) module"]
pub struct MIICOMMAND_SPEC;
impl crate::RegisterSpec for MIICOMMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miicommand::R](R) reader structure"]
impl crate::Readable for MIICOMMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miicommand::W](W) writer structure"]
impl crate::Writable for MIICOMMAND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIICOMMAND to value 0"]
impl crate::Resettable for MIICOMMAND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
