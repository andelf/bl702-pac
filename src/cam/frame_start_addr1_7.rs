#[doc = "Register `frame_start_addr1_7` reader"]
pub struct R(crate::R<FRAME_START_ADDR1_7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAME_START_ADDR1_7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAME_START_ADDR1_7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAME_START_ADDR1_7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `frame_start_addr1_7` writer"]
pub struct W(crate::W<FRAME_START_ADDR1_7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAME_START_ADDR1_7_SPEC>;
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
impl From<crate::W<FRAME_START_ADDR1_7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAME_START_ADDR1_7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `frame_start_addr_1_7` reader - "]
pub struct FRAME_START_ADDR_1_7_R(crate::FieldReader<u32, u32>);
impl FRAME_START_ADDR_1_7_R {
    pub(crate) fn new(bits: u32) -> Self {
        FRAME_START_ADDR_1_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_START_ADDR_1_7_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `frame_start_addr_1_7` writer - "]
pub struct FRAME_START_ADDR_1_7_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_START_ADDR_1_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn frame_start_addr_1_7(&self) -> FRAME_START_ADDR_1_7_R {
        FRAME_START_ADDR_1_7_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn frame_start_addr_1_7(&mut self) -> FRAME_START_ADDR_1_7_W {
        FRAME_START_ADDR_1_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "frame_start_addr1_7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frame_start_addr1_7](index.html) module"]
pub struct FRAME_START_ADDR1_7_SPEC;
impl crate::RegisterSpec for FRAME_START_ADDR1_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frame_start_addr1_7::R](R) reader structure"]
impl crate::Readable for FRAME_START_ADDR1_7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frame_start_addr1_7::W](W) writer structure"]
impl crate::Writable for FRAME_START_ADDR1_7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets frame_start_addr1_7 to value 0"]
impl crate::Resettable for FRAME_START_ADDR1_7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
