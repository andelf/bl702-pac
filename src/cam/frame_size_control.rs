#[doc = "Register `frame_size_control` reader"]
pub struct R(crate::R<FRAME_SIZE_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAME_SIZE_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAME_SIZE_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAME_SIZE_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `frame_size_control` writer"]
pub struct W(crate::W<FRAME_SIZE_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAME_SIZE_CONTROL_SPEC>;
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
impl From<crate::W<FRAME_SIZE_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAME_SIZE_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_total_hcnt` reader - "]
pub type REG_TOTAL_HCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_total_hcnt` writer - "]
pub type REG_TOTAL_HCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAME_SIZE_CONTROL_SPEC, u16, u16, 16, O>;
#[doc = "Field `reg_total_vcnt` reader - "]
pub type REG_TOTAL_VCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_total_vcnt` writer - "]
pub type REG_TOTAL_VCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRAME_SIZE_CONTROL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_total_hcnt(&self) -> REG_TOTAL_HCNT_R {
        REG_TOTAL_HCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_total_vcnt(&self) -> REG_TOTAL_VCNT_R {
        REG_TOTAL_VCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_total_hcnt(&mut self) -> REG_TOTAL_HCNT_W<0> {
        REG_TOTAL_HCNT_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_total_vcnt(&mut self) -> REG_TOTAL_VCNT_W<16> {
        REG_TOTAL_VCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "frame_size_control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frame_size_control](index.html) module"]
pub struct FRAME_SIZE_CONTROL_SPEC;
impl crate::RegisterSpec for FRAME_SIZE_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frame_size_control::R](R) reader structure"]
impl crate::Readable for FRAME_SIZE_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frame_size_control::W](W) writer structure"]
impl crate::Writable for FRAME_SIZE_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets frame_size_control to value 0"]
impl crate::Resettable for FRAME_SIZE_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
