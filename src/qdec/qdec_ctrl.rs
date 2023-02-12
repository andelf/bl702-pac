#[doc = "Register `qdec_ctrl` reader"]
pub struct R(crate::R<QDEC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QDEC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QDEC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QDEC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `qdec_ctrl` writer"]
pub struct W(crate::W<QDEC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QDEC_CTRL_SPEC>;
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
impl From<crate::W<QDEC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QDEC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `qdec_en` reader - "]
pub type QDEC_EN_R = crate::BitReader<bool>;
#[doc = "Field `qdec_en` writer - "]
pub type QDEC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_CTRL_SPEC, bool, O>;
#[doc = "Field `led_en` reader - "]
pub type LED_EN_R = crate::BitReader<bool>;
#[doc = "Field `led_en` writer - "]
pub type LED_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_CTRL_SPEC, bool, O>;
#[doc = "Field `led_pol` reader - "]
pub type LED_POL_R = crate::BitReader<bool>;
#[doc = "Field `led_pol` writer - "]
pub type LED_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_CTRL_SPEC, bool, O>;
#[doc = "Field `deg_en` reader - "]
pub type DEG_EN_R = crate::BitReader<bool>;
#[doc = "Field `deg_en` writer - "]
pub type DEG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_CTRL_SPEC, bool, O>;
#[doc = "Field `deg_cnt` reader - "]
pub type DEG_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `deg_cnt` writer - "]
pub type DEG_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QDEC_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `spl_period` reader - "]
pub type SPL_PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `spl_period` writer - "]
pub type SPL_PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QDEC_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `rpt_period` reader - "]
pub type RPT_PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rpt_period` writer - "]
pub type RPT_PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QDEC_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `led_period` reader - "]
pub type LED_PERIOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `led_period` writer - "]
pub type LED_PERIOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, QDEC_CTRL_SPEC, u16, u16, 9, O>;
#[doc = "Field `spl_mode` reader - "]
pub type SPL_MODE_R = crate::BitReader<bool>;
#[doc = "Field `spl_mode` writer - "]
pub type SPL_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_CTRL_SPEC, bool, O>;
#[doc = "Field `rpt_mode` reader - "]
pub type RPT_MODE_R = crate::BitReader<bool>;
#[doc = "Field `rpt_mode` writer - "]
pub type RPT_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_CTRL_SPEC, bool, O>;
#[doc = "Field `input_swap` reader - "]
pub type INPUT_SWAP_R = crate::BitReader<bool>;
#[doc = "Field `input_swap` writer - "]
pub type INPUT_SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, QDEC_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn qdec_en(&self) -> QDEC_EN_R {
        QDEC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn led_en(&self) -> LED_EN_R {
        LED_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_pol(&self) -> LED_POL_R {
        LED_POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn deg_en(&self) -> DEG_EN_R {
        DEG_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn deg_cnt(&self) -> DEG_CNT_R {
        DEG_CNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn spl_period(&self) -> SPL_PERIOD_R {
        SPL_PERIOD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    pub fn rpt_period(&self) -> RPT_PERIOD_R {
        RPT_PERIOD_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn led_period(&self) -> LED_PERIOD_R {
        LED_PERIOD_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spl_mode(&self) -> SPL_MODE_R {
        SPL_MODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rpt_mode(&self) -> RPT_MODE_R {
        RPT_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn input_swap(&self) -> INPUT_SWAP_R {
        INPUT_SWAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn qdec_en(&mut self) -> QDEC_EN_W<0> {
        QDEC_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn led_en(&mut self) -> LED_EN_W<1> {
        LED_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn led_pol(&mut self) -> LED_POL_W<2> {
        LED_POL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn deg_en(&mut self) -> DEG_EN_W<3> {
        DEG_EN_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn deg_cnt(&mut self) -> DEG_CNT_W<4> {
        DEG_CNT_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn spl_period(&mut self) -> SPL_PERIOD_W<8> {
        SPL_PERIOD_W::new(self)
    }
    #[doc = "Bits 12:19"]
    #[inline(always)]
    #[must_use]
    pub fn rpt_period(&mut self) -> RPT_PERIOD_W<12> {
        RPT_PERIOD_W::new(self)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    #[must_use]
    pub fn led_period(&mut self) -> LED_PERIOD_W<20> {
        LED_PERIOD_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn spl_mode(&mut self) -> SPL_MODE_W<29> {
        SPL_MODE_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rpt_mode(&mut self) -> RPT_MODE_W<30> {
        RPT_MODE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn input_swap(&mut self) -> INPUT_SWAP_W<31> {
        INPUT_SWAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "qdec_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qdec_ctrl](index.html) module"]
pub struct QDEC_CTRL_SPEC;
impl crate::RegisterSpec for QDEC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qdec_ctrl::R](R) reader structure"]
impl crate::Readable for QDEC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qdec_ctrl::W](W) writer structure"]
impl crate::Writable for QDEC_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets qdec_ctrl to value 0"]
impl crate::Resettable for QDEC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
