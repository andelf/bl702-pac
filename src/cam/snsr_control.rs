#[doc = "Register `snsr_control` reader"]
pub struct R(crate::R<SNSR_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNSR_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNSR_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNSR_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `snsr_control` writer"]
pub struct W(crate::W<SNSR_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNSR_CONTROL_SPEC>;
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
impl From<crate::W<SNSR_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNSR_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_cam_rst` reader - "]
pub type REG_CAM_RST_R = crate::BitReader<bool>;
#[doc = "Field `reg_cam_rst` writer - "]
pub type REG_CAM_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNSR_CONTROL_SPEC, bool, O>;
#[doc = "Field `reg_cam_pwdn` reader - "]
pub type REG_CAM_PWDN_R = crate::BitReader<bool>;
#[doc = "Field `reg_cam_pwdn` writer - "]
pub type REG_CAM_PWDN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNSR_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_cam_rst(&self) -> REG_CAM_RST_R {
        REG_CAM_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_cam_pwdn(&self) -> REG_CAM_PWDN_R {
        REG_CAM_PWDN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cam_rst(&mut self) -> REG_CAM_RST_W<0> {
        REG_CAM_RST_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cam_pwdn(&mut self) -> REG_CAM_PWDN_W<1> {
        REG_CAM_PWDN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "snsr_control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [snsr_control](index.html) module"]
pub struct SNSR_CONTROL_SPEC;
impl crate::RegisterSpec for SNSR_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [snsr_control::R](R) reader structure"]
impl crate::Readable for SNSR_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [snsr_control::W](W) writer structure"]
impl crate::Writable for SNSR_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets snsr_control to value 0"]
impl crate::Resettable for SNSR_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
