#[doc = "Register `dvp2ahb_mem_bcnt_0` reader"]
pub struct R(crate::R<DVP2AHB_MEM_BCNT_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVP2AHB_MEM_BCNT_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVP2AHB_MEM_BCNT_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVP2AHB_MEM_BCNT_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dvp2ahb_mem_bcnt_0` writer"]
pub struct W(crate::W<DVP2AHB_MEM_BCNT_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVP2AHB_MEM_BCNT_0_SPEC>;
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
impl From<crate::W<DVP2AHB_MEM_BCNT_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVP2AHB_MEM_BCNT_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mem_burst_cnt_0` reader - "]
pub struct REG_MEM_BURST_CNT_0_R(crate::FieldReader<u32, u32>);
impl REG_MEM_BURST_CNT_0_R {
    pub(crate) fn new(bits: u32) -> Self {
        REG_MEM_BURST_CNT_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MEM_BURST_CNT_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mem_burst_cnt_0` writer - "]
pub struct REG_MEM_BURST_CNT_0_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MEM_BURST_CNT_0_W<'a> {
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
    pub fn reg_mem_burst_cnt_0(&self) -> REG_MEM_BURST_CNT_0_R {
        REG_MEM_BURST_CNT_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_mem_burst_cnt_0(&mut self) -> REG_MEM_BURST_CNT_0_W {
        REG_MEM_BURST_CNT_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dvp2ahb_mem_bcnt_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvp2ahb_mem_bcnt_0](index.html) module"]
pub struct DVP2AHB_MEM_BCNT_0_SPEC;
impl crate::RegisterSpec for DVP2AHB_MEM_BCNT_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvp2ahb_mem_bcnt_0::R](R) reader structure"]
impl crate::Readable for DVP2AHB_MEM_BCNT_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvp2ahb_mem_bcnt_0::W](W) writer structure"]
impl crate::Writable for DVP2AHB_MEM_BCNT_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dvp2ahb_mem_bcnt_0 to value 0"]
impl crate::Resettable for DVP2AHB_MEM_BCNT_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
