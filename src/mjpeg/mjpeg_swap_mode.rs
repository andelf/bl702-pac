#[doc = "Register `mjpeg_swap_mode` reader"]
pub struct R(crate::R<MJPEG_SWAP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MJPEG_SWAP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MJPEG_SWAP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MJPEG_SWAP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mjpeg_swap_mode` writer"]
pub struct W(crate::W<MJPEG_SWAP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MJPEG_SWAP_MODE_SPEC>;
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
impl From<crate::W<MJPEG_SWAP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MJPEG_SWAP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_w_swap_mode` reader - "]
pub type REG_W_SWAP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `reg_w_swap_mode` writer - "]
pub type REG_W_SWAP_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_SWAP_MODE_SPEC, bool, O>;
#[doc = "Field `sts_swap0_full` reader - "]
pub type STS_SWAP0_FULL_R = crate::BitReader<bool>;
#[doc = "Field `sts_swap0_full` writer - "]
pub type STS_SWAP0_FULL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_SWAP_MODE_SPEC, bool, O>;
#[doc = "Field `sts_swap1_full` reader - "]
pub type STS_SWAP1_FULL_R = crate::BitReader<bool>;
#[doc = "Field `sts_swap1_full` writer - "]
pub type STS_SWAP1_FULL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_SWAP_MODE_SPEC, bool, O>;
#[doc = "Field `sts_read_swap_idx` reader - "]
pub type STS_READ_SWAP_IDX_R = crate::BitReader<bool>;
#[doc = "Field `sts_read_swap_idx` writer - "]
pub type STS_READ_SWAP_IDX_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_SWAP_MODE_SPEC, bool, O>;
#[doc = "Field `sts_swap_fstart` reader - "]
pub type STS_SWAP_FSTART_R = crate::BitReader<bool>;
#[doc = "Field `sts_swap_fstart` writer - "]
pub type STS_SWAP_FSTART_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_SWAP_MODE_SPEC, bool, O>;
#[doc = "Field `sts_swap_fend` reader - "]
pub type STS_SWAP_FEND_R = crate::BitReader<bool>;
#[doc = "Field `sts_swap_fend` writer - "]
pub type STS_SWAP_FEND_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MJPEG_SWAP_MODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_w_swap_mode(&self) -> REG_W_SWAP_MODE_R {
        REG_W_SWAP_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sts_swap0_full(&self) -> STS_SWAP0_FULL_R {
        STS_SWAP0_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sts_swap1_full(&self) -> STS_SWAP1_FULL_R {
        STS_SWAP1_FULL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sts_read_swap_idx(&self) -> STS_READ_SWAP_IDX_R {
        STS_READ_SWAP_IDX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sts_swap_fstart(&self) -> STS_SWAP_FSTART_R {
        STS_SWAP_FSTART_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sts_swap_fend(&self) -> STS_SWAP_FEND_R {
        STS_SWAP_FEND_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_w_swap_mode(&mut self) -> REG_W_SWAP_MODE_W<0> {
        REG_W_SWAP_MODE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sts_swap0_full(&mut self) -> STS_SWAP0_FULL_W<8> {
        STS_SWAP0_FULL_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sts_swap1_full(&mut self) -> STS_SWAP1_FULL_W<9> {
        STS_SWAP1_FULL_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sts_read_swap_idx(&mut self) -> STS_READ_SWAP_IDX_W<10> {
        STS_READ_SWAP_IDX_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sts_swap_fstart(&mut self) -> STS_SWAP_FSTART_W<11> {
        STS_SWAP_FSTART_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn sts_swap_fend(&mut self) -> STS_SWAP_FEND_W<12> {
        STS_SWAP_FEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "mjpeg_swap_mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mjpeg_swap_mode](index.html) module"]
pub struct MJPEG_SWAP_MODE_SPEC;
impl crate::RegisterSpec for MJPEG_SWAP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mjpeg_swap_mode::R](R) reader structure"]
impl crate::Readable for MJPEG_SWAP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mjpeg_swap_mode::W](W) writer structure"]
impl crate::Writable for MJPEG_SWAP_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mjpeg_swap_mode to value 0"]
impl crate::Resettable for MJPEG_SWAP_MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
