#[doc = "Register `vsync_control` reader"]
pub struct R(crate::R<VSYNC_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VSYNC_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VSYNC_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VSYNC_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `vsync_control` writer"]
pub struct W(crate::W<VSYNC_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VSYNC_CONTROL_SPEC>;
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
impl From<crate::W<VSYNC_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VSYNC_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_vsync_act_start` reader - "]
pub struct REG_VSYNC_ACT_START_R(crate::FieldReader<u16, u16>);
impl REG_VSYNC_ACT_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REG_VSYNC_ACT_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_VSYNC_ACT_START_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_vsync_act_start` writer - "]
pub struct REG_VSYNC_ACT_START_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_VSYNC_ACT_START_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `reg_vsync_act_end` reader - "]
pub struct REG_VSYNC_ACT_END_R(crate::FieldReader<u16, u16>);
impl REG_VSYNC_ACT_END_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        REG_VSYNC_ACT_END_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_VSYNC_ACT_END_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_vsync_act_end` writer - "]
pub struct REG_VSYNC_ACT_END_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_VSYNC_ACT_END_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_vsync_act_start(&self) -> REG_VSYNC_ACT_START_R {
        REG_VSYNC_ACT_START_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_vsync_act_end(&self) -> REG_VSYNC_ACT_END_R {
        REG_VSYNC_ACT_END_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn reg_vsync_act_start(&mut self) -> REG_VSYNC_ACT_START_W {
        REG_VSYNC_ACT_START_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn reg_vsync_act_end(&mut self) -> REG_VSYNC_ACT_END_W {
        REG_VSYNC_ACT_END_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "vsync_control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vsync_control](index.html) module"]
pub struct VSYNC_CONTROL_SPEC;
impl crate::RegisterSpec for VSYNC_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vsync_control::R](R) reader structure"]
impl crate::Readable for VSYNC_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vsync_control::W](W) writer structure"]
impl crate::Writable for VSYNC_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets vsync_control to value 0"]
impl crate::Resettable for VSYNC_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
