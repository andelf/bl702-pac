#[doc = "Register `MBIST_STAT` reader"]
pub struct R(crate::R<MBIST_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBIST_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MBIST_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MBIST_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBIST_STAT` writer"]
pub struct W(crate::W<MBIST_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBIST_STAT_SPEC>;
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
impl From<crate::W<MBIST_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MBIST_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `irom_mbist_done` reader - "]
pub type IROM_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `irom_mbist_done` writer - "]
pub type IROM_MBIST_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `hsram_mem_mbist_done` reader - "]
pub type HSRAM_MEM_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `hsram_mem_mbist_done` writer - "]
pub type HSRAM_MEM_MBIST_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `hsram_cache_mbist_done` reader - "]
pub type HSRAM_CACHE_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `hsram_cache_mbist_done` writer - "]
pub type HSRAM_CACHE_MBIST_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `tag_mbist_done` reader - "]
pub type TAG_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `tag_mbist_done` writer - "]
pub type TAG_MBIST_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `ocram_mbist_done` reader - "]
pub type OCRAM_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ocram_mbist_done` writer - "]
pub type OCRAM_MBIST_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `em_ram_mbist_done` reader - "]
pub type EM_RAM_MBIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `em_ram_mbist_done` writer - "]
pub type EM_RAM_MBIST_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `irom_mbist_fail` reader - "]
pub type IROM_MBIST_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `irom_mbist_fail` writer - "]
pub type IROM_MBIST_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `hsram_mem_mbist_fail` reader - "]
pub type HSRAM_MEM_MBIST_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `hsram_mem_mbist_fail` writer - "]
pub type HSRAM_MEM_MBIST_FAIL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `hsram_cache_mbist_fail` reader - "]
pub type HSRAM_CACHE_MBIST_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `hsram_cache_mbist_fail` writer - "]
pub type HSRAM_CACHE_MBIST_FAIL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `tag_mbist_fail` reader - "]
pub type TAG_MBIST_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `tag_mbist_fail` writer - "]
pub type TAG_MBIST_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `ocram_mbist_fail` reader - "]
pub type OCRAM_MBIST_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `ocram_mbist_fail` writer - "]
pub type OCRAM_MBIST_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
#[doc = "Field `em_ram_mbist_fail` reader - "]
pub type EM_RAM_MBIST_FAIL_R = crate::BitReader<bool>;
#[doc = "Field `em_ram_mbist_fail` writer - "]
pub type EM_RAM_MBIST_FAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MBIST_STAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&self) -> IROM_MBIST_DONE_R {
        IROM_MBIST_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mem_mbist_done(&self) -> HSRAM_MEM_MBIST_DONE_R {
        HSRAM_MEM_MBIST_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hsram_cache_mbist_done(&self) -> HSRAM_CACHE_MBIST_DONE_R {
        HSRAM_CACHE_MBIST_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tag_mbist_done(&self) -> TAG_MBIST_DONE_R {
        TAG_MBIST_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ocram_mbist_done(&self) -> OCRAM_MBIST_DONE_R {
        OCRAM_MBIST_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn em_ram_mbist_done(&self) -> EM_RAM_MBIST_DONE_R {
        EM_RAM_MBIST_DONE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&self) -> IROM_MBIST_FAIL_R {
        IROM_MBIST_FAIL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mem_mbist_fail(&self) -> HSRAM_MEM_MBIST_FAIL_R {
        HSRAM_MEM_MBIST_FAIL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn hsram_cache_mbist_fail(&self) -> HSRAM_CACHE_MBIST_FAIL_R {
        HSRAM_CACHE_MBIST_FAIL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tag_mbist_fail(&self) -> TAG_MBIST_FAIL_R {
        TAG_MBIST_FAIL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&self) -> OCRAM_MBIST_FAIL_R {
        OCRAM_MBIST_FAIL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn em_ram_mbist_fail(&self) -> EM_RAM_MBIST_FAIL_R {
        EM_RAM_MBIST_FAIL_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&mut self) -> IROM_MBIST_DONE_W<0> {
        IROM_MBIST_DONE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mem_mbist_done(&mut self) -> HSRAM_MEM_MBIST_DONE_W<1> {
        HSRAM_MEM_MBIST_DONE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hsram_cache_mbist_done(&mut self) -> HSRAM_CACHE_MBIST_DONE_W<2> {
        HSRAM_CACHE_MBIST_DONE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tag_mbist_done(&mut self) -> TAG_MBIST_DONE_W<3> {
        TAG_MBIST_DONE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ocram_mbist_done(&mut self) -> OCRAM_MBIST_DONE_W<4> {
        OCRAM_MBIST_DONE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn em_ram_mbist_done(&mut self) -> EM_RAM_MBIST_DONE_W<5> {
        EM_RAM_MBIST_DONE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&mut self) -> IROM_MBIST_FAIL_W<16> {
        IROM_MBIST_FAIL_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mem_mbist_fail(&mut self) -> HSRAM_MEM_MBIST_FAIL_W<17> {
        HSRAM_MEM_MBIST_FAIL_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn hsram_cache_mbist_fail(&mut self) -> HSRAM_CACHE_MBIST_FAIL_W<18> {
        HSRAM_CACHE_MBIST_FAIL_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tag_mbist_fail(&mut self) -> TAG_MBIST_FAIL_W<19> {
        TAG_MBIST_FAIL_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&mut self) -> OCRAM_MBIST_FAIL_W<20> {
        OCRAM_MBIST_FAIL_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn em_ram_mbist_fail(&mut self) -> EM_RAM_MBIST_FAIL_W<21> {
        EM_RAM_MBIST_FAIL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MBIST_STAT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_stat](index.html) module"]
pub struct MBIST_STAT_SPEC;
impl crate::RegisterSpec for MBIST_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbist_stat::R](R) reader structure"]
impl crate::Readable for MBIST_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbist_stat::W](W) writer structure"]
impl crate::Writable for MBIST_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBIST_STAT to value 0"]
impl crate::Resettable for MBIST_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
