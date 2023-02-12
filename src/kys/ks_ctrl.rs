#[doc = "Register `ks_ctrl` reader"]
pub struct R(crate::R<KS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ks_ctrl` writer"]
pub struct W(crate::W<KS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KS_CTRL_SPEC>;
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
impl From<crate::W<KS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ks_en` reader - "]
pub type KS_EN_R = crate::BitReader<bool>;
#[doc = "Field `ks_en` writer - "]
pub type KS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KS_CTRL_SPEC, bool, O>;
#[doc = "Field `ghost_en` reader - "]
pub type GHOST_EN_R = crate::BitReader<bool>;
#[doc = "Field `ghost_en` writer - "]
pub type GHOST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KS_CTRL_SPEC, bool, O>;
#[doc = "Field `deg_en` reader - "]
pub type DEG_EN_R = crate::BitReader<bool>;
#[doc = "Field `deg_en` writer - "]
pub type DEG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, KS_CTRL_SPEC, bool, O>;
#[doc = "Field `deg_cnt` reader - "]
pub type DEG_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `deg_cnt` writer - "]
pub type DEG_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KS_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `rc_ext` reader - "]
pub type RC_EXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rc_ext` writer - "]
pub type RC_EXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KS_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `row_num` reader - "]
pub type ROW_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `row_num` writer - "]
pub type ROW_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KS_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `col_num` reader - "]
pub type COL_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `col_num` writer - "]
pub type COL_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KS_CTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ks_en(&self) -> KS_EN_R {
        KS_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ghost_en(&self) -> GHOST_EN_R {
        GHOST_EN_R::new(((self.bits >> 2) & 1) != 0)
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
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn rc_ext(&self) -> RC_EXT_R {
        RC_EXT_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn row_num(&self) -> ROW_NUM_R {
        ROW_NUM_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn col_num(&self) -> COL_NUM_R {
        COL_NUM_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn ks_en(&mut self) -> KS_EN_W<0> {
        KS_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ghost_en(&mut self) -> GHOST_EN_W<2> {
        GHOST_EN_W::new(self)
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
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn rc_ext(&mut self) -> RC_EXT_W<8> {
        RC_EXT_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn row_num(&mut self) -> ROW_NUM_W<16> {
        ROW_NUM_W::new(self)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    #[must_use]
    pub fn col_num(&mut self) -> COL_NUM_W<20> {
        COL_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ks_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ks_ctrl](index.html) module"]
pub struct KS_CTRL_SPEC;
impl crate::RegisterSpec for KS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ks_ctrl::R](R) reader structure"]
impl crate::Readable for KS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ks_ctrl::W](W) writer structure"]
impl crate::Writable for KS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ks_ctrl to value 0"]
impl crate::Resettable for KS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
