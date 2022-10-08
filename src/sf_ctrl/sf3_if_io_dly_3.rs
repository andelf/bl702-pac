#[doc = "Register `sf3_if_io_dly_3` reader"]
pub struct R(crate::R<SF3_IF_IO_DLY_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF3_IF_IO_DLY_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF3_IF_IO_DLY_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF3_IF_IO_DLY_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf3_if_io_dly_3` writer"]
pub struct W(crate::W<SF3_IF_IO_DLY_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF3_IF_IO_DLY_3_SPEC>;
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
impl From<crate::W<SF3_IF_IO_DLY_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF3_IF_IO_DLY_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf3_io_2_oe_dly_sel` reader - "]
pub type SF3_IO_2_OE_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf3_io_2_oe_dly_sel` writer - "]
pub type SF3_IO_2_OE_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF3_IF_IO_DLY_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf3_io_2_di_dly_sel` reader - "]
pub type SF3_IO_2_DI_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf3_io_2_di_dly_sel` writer - "]
pub type SF3_IO_2_DI_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF3_IF_IO_DLY_3_SPEC, u8, u8, 2, O>;
#[doc = "Field `sf3_io_2_do_dly_sel` reader - "]
pub type SF3_IO_2_DO_DLY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `sf3_io_2_do_dly_sel` writer - "]
pub type SF3_IO_2_DO_DLY_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SF3_IF_IO_DLY_3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_2_oe_dly_sel(&self) -> SF3_IO_2_OE_DLY_SEL_R {
        SF3_IO_2_OE_DLY_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_2_di_dly_sel(&self) -> SF3_IO_2_DI_DLY_SEL_R {
        SF3_IO_2_DI_DLY_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_2_do_dly_sel(&self) -> SF3_IO_2_DO_DLY_SEL_R {
        SF3_IO_2_DO_DLY_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf3_io_2_oe_dly_sel(&mut self) -> SF3_IO_2_OE_DLY_SEL_W<0> {
        SF3_IO_2_OE_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf3_io_2_di_dly_sel(&mut self) -> SF3_IO_2_DI_DLY_SEL_W<8> {
        SF3_IO_2_DI_DLY_SEL_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf3_io_2_do_dly_sel(&mut self) -> SF3_IO_2_DO_DLY_SEL_W<16> {
        SF3_IO_2_DO_DLY_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf3_if_io_dly_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf3_if_io_dly_3](index.html) module"]
pub struct SF3_IF_IO_DLY_3_SPEC;
impl crate::RegisterSpec for SF3_IF_IO_DLY_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf3_if_io_dly_3::R](R) reader structure"]
impl crate::Readable for SF3_IF_IO_DLY_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf3_if_io_dly_3::W](W) writer structure"]
impl crate::Writable for SF3_IF_IO_DLY_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf3_if_io_dly_3 to value 0"]
impl crate::Resettable for SF3_IF_IO_DLY_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
