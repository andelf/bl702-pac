#[doc = "Register `jpeg_store_memory` reader"]
pub struct R(crate::R<JPEG_STORE_MEMORY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JPEG_STORE_MEMORY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JPEG_STORE_MEMORY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JPEG_STORE_MEMORY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `jpeg_store_memory` writer"]
pub struct W(crate::W<JPEG_STORE_MEMORY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JPEG_STORE_MEMORY_SPEC>;
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
impl From<crate::W<JPEG_STORE_MEMORY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JPEG_STORE_MEMORY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_w_burst_cnt` reader - "]
pub struct REG_W_BURST_CNT_R(crate::FieldReader<u32, u32>);
impl REG_W_BURST_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        REG_W_BURST_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_W_BURST_CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_w_burst_cnt` writer - "]
pub struct REG_W_BURST_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_W_BURST_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_w_burst_cnt(&self) -> REG_W_BURST_CNT_R {
        REG_W_BURST_CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_w_burst_cnt(&mut self) -> REG_W_BURST_CNT_W {
        REG_W_BURST_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "jpeg_store_memory.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jpeg_store_memory](index.html) module"]
pub struct JPEG_STORE_MEMORY_SPEC;
impl crate::RegisterSpec for JPEG_STORE_MEMORY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jpeg_store_memory::R](R) reader structure"]
impl crate::Readable for JPEG_STORE_MEMORY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jpeg_store_memory::W](W) writer structure"]
impl crate::Writable for JPEG_STORE_MEMORY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets jpeg_store_memory to value 0"]
impl crate::Resettable for JPEG_STORE_MEMORY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
