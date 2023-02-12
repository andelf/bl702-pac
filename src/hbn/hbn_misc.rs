#[doc = "Register `HBN_MISC` reader"]
pub struct R(crate::R<HBN_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_MISC` writer"]
pub struct W(crate::W<HBN_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_MISC_SPEC>;
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
impl From<crate::W<HBN_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bor_sel` reader - "]
pub type BOR_SEL_R = crate::BitReader<bool>;
#[doc = "Field `bor_sel` writer - "]
pub type BOR_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_MISC_SPEC, bool, O>;
#[doc = "Field `bor_vth` reader - "]
pub type BOR_VTH_R = crate::BitReader<bool>;
#[doc = "Field `bor_vth` writer - "]
pub type BOR_VTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_MISC_SPEC, bool, O>;
#[doc = "Field `pu_bor` reader - "]
pub type PU_BOR_R = crate::BitReader<bool>;
#[doc = "Field `pu_bor` writer - "]
pub type PU_BOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_MISC_SPEC, bool, O>;
#[doc = "Field `r_bor_out` reader - "]
pub type R_BOR_OUT_R = crate::BitReader<bool>;
#[doc = "Field `r_bor_out` writer - "]
pub type R_BOR_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HBN_MISC_SPEC, bool, O>;
#[doc = "Field `hbn_flash_pullup_aon` reader - "]
pub type HBN_FLASH_PULLUP_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_flash_pullup_aon` writer - "]
pub type HBN_FLASH_PULLUP_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_MISC_SPEC, u8, u8, 6, O>;
#[doc = "Field `hbn_flash_pulldown_aon` reader - "]
pub type HBN_FLASH_PULLDOWN_AON_R = crate::FieldReader<u8, u8>;
#[doc = "Field `hbn_flash_pulldown_aon` writer - "]
pub type HBN_FLASH_PULLDOWN_AON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HBN_MISC_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bor_sel(&self) -> BOR_SEL_R {
        BOR_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bor_vth(&self) -> BOR_VTH_R {
        BOR_VTH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_bor(&self) -> PU_BOR_R {
        PU_BOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn r_bor_out(&self) -> R_BOR_OUT_R {
        R_BOR_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn hbn_flash_pullup_aon(&self) -> HBN_FLASH_PULLUP_AON_R {
        HBN_FLASH_PULLUP_AON_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn hbn_flash_pulldown_aon(&self) -> HBN_FLASH_PULLDOWN_AON_R {
        HBN_FLASH_PULLDOWN_AON_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bor_sel(&mut self) -> BOR_SEL_W<0> {
        BOR_SEL_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn bor_vth(&mut self) -> BOR_VTH_W<1> {
        BOR_VTH_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pu_bor(&mut self) -> PU_BOR_W<2> {
        PU_BOR_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn r_bor_out(&mut self) -> R_BOR_OUT_W<3> {
        R_BOR_OUT_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_flash_pullup_aon(&mut self) -> HBN_FLASH_PULLUP_AON_W<16> {
        HBN_FLASH_PULLUP_AON_W::new(self)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    #[must_use]
    pub fn hbn_flash_pulldown_aon(&mut self) -> HBN_FLASH_PULLDOWN_AON_W<24> {
        HBN_FLASH_PULLDOWN_AON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_MISC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_misc](index.html) module"]
pub struct HBN_MISC_SPEC;
impl crate::RegisterSpec for HBN_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_misc::R](R) reader structure"]
impl crate::Readable for HBN_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_misc::W](W) writer structure"]
impl crate::Writable for HBN_MISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HBN_MISC to value 0"]
impl crate::Resettable for HBN_MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
