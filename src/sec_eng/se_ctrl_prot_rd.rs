#[doc = "Register `se_ctrl_prot_rd` reader"]
pub struct R(crate::R<SE_CTRL_PROT_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_CTRL_PROT_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SE_CTRL_PROT_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SE_CTRL_PROT_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_ctrl_prot_rd` writer"]
pub struct W(crate::W<SE_CTRL_PROT_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_CTRL_PROT_RD_SPEC>;
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
impl From<crate::W<SE_CTRL_PROT_RD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SE_CTRL_PROT_RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_sha_prot_en_rd` reader - "]
pub type SE_SHA_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_sha_prot_en_rd` writer - "]
pub type SE_SHA_PROT_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_sha_id0_en_rd` reader - "]
pub type SE_SHA_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_sha_id0_en_rd` writer - "]
pub type SE_SHA_ID0_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_sha_id1_en_rd` reader - "]
pub type SE_SHA_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_sha_id1_en_rd` writer - "]
pub type SE_SHA_ID1_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_aes_prot_en_rd` reader - "]
pub type SE_AES_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_prot_en_rd` writer - "]
pub type SE_AES_PROT_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_aes_id0_en_rd` reader - "]
pub type SE_AES_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_id0_en_rd` writer - "]
pub type SE_AES_ID0_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_aes_id1_en_rd` reader - "]
pub type SE_AES_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_aes_id1_en_rd` writer - "]
pub type SE_AES_ID1_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_trng_prot_en_rd` reader - "]
pub type SE_TRNG_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_prot_en_rd` writer - "]
pub type SE_TRNG_PROT_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_trng_id0_en_rd` reader - "]
pub type SE_TRNG_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_id0_en_rd` writer - "]
pub type SE_TRNG_ID0_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_trng_id1_en_rd` reader - "]
pub type SE_TRNG_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_trng_id1_en_rd` writer - "]
pub type SE_TRNG_ID1_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_pka_prot_en_rd` reader - "]
pub type SE_PKA_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_prot_en_rd` writer - "]
pub type SE_PKA_PROT_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_pka_id0_en_rd` reader - "]
pub type SE_PKA_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_id0_en_rd` writer - "]
pub type SE_PKA_ID0_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_pka_id1_en_rd` reader - "]
pub type SE_PKA_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_pka_id1_en_rd` writer - "]
pub type SE_PKA_ID1_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_cdet_prot_en_rd` reader - "]
pub type SE_CDET_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_cdet_prot_en_rd` writer - "]
pub type SE_CDET_PROT_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_cdet_id0_en_rd` reader - "]
pub type SE_CDET_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_cdet_id0_en_rd` writer - "]
pub type SE_CDET_ID0_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_cdet_id1_en_rd` reader - "]
pub type SE_CDET_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_cdet_id1_en_rd` writer - "]
pub type SE_CDET_ID1_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_gmac_prot_en_rd` reader - "]
pub type SE_GMAC_PROT_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_prot_en_rd` writer - "]
pub type SE_GMAC_PROT_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_gmac_id0_en_rd` reader - "]
pub type SE_GMAC_ID0_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_id0_en_rd` writer - "]
pub type SE_GMAC_ID0_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_gmac_id1_en_rd` reader - "]
pub type SE_GMAC_ID1_EN_RD_R = crate::BitReader<bool>;
#[doc = "Field `se_gmac_id1_en_rd` writer - "]
pub type SE_GMAC_ID1_EN_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
#[doc = "Field `se_dbg_dis` reader - "]
pub type SE_DBG_DIS_R = crate::BitReader<bool>;
#[doc = "Field `se_dbg_dis` writer - "]
pub type SE_DBG_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SE_CTRL_PROT_RD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_prot_en_rd(&self) -> SE_SHA_PROT_EN_RD_R {
        SE_SHA_PROT_EN_RD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_id0_en_rd(&self) -> SE_SHA_ID0_EN_RD_R {
        SE_SHA_ID0_EN_RD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_sha_id1_en_rd(&self) -> SE_SHA_ID1_EN_RD_R {
        SE_SHA_ID1_EN_RD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_aes_prot_en_rd(&self) -> SE_AES_PROT_EN_RD_R {
        SE_AES_PROT_EN_RD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_aes_id0_en_rd(&self) -> SE_AES_ID0_EN_RD_R {
        SE_AES_ID0_EN_RD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_id1_en_rd(&self) -> SE_AES_ID1_EN_RD_R {
        SE_AES_ID1_EN_RD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_prot_en_rd(&self) -> SE_TRNG_PROT_EN_RD_R {
        SE_TRNG_PROT_EN_RD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_id0_en_rd(&self) -> SE_TRNG_ID0_EN_RD_R {
        SE_TRNG_ID0_EN_RD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_id1_en_rd(&self) -> SE_TRNG_ID1_EN_RD_R {
        SE_TRNG_ID1_EN_RD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_prot_en_rd(&self) -> SE_PKA_PROT_EN_RD_R {
        SE_PKA_PROT_EN_RD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_id0_en_rd(&self) -> SE_PKA_ID0_EN_RD_R {
        SE_PKA_ID0_EN_RD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_pka_id1_en_rd(&self) -> SE_PKA_ID1_EN_RD_R {
        SE_PKA_ID1_EN_RD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_cdet_prot_en_rd(&self) -> SE_CDET_PROT_EN_RD_R {
        SE_CDET_PROT_EN_RD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn se_cdet_id0_en_rd(&self) -> SE_CDET_ID0_EN_RD_R {
        SE_CDET_ID0_EN_RD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn se_cdet_id1_en_rd(&self) -> SE_CDET_ID1_EN_RD_R {
        SE_CDET_ID1_EN_RD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn se_gmac_prot_en_rd(&self) -> SE_GMAC_PROT_EN_RD_R {
        SE_GMAC_PROT_EN_RD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn se_gmac_id0_en_rd(&self) -> SE_GMAC_ID0_EN_RD_R {
        SE_GMAC_ID0_EN_RD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn se_gmac_id1_en_rd(&self) -> SE_GMAC_ID1_EN_RD_R {
        SE_GMAC_ID1_EN_RD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn se_dbg_dis(&self) -> SE_DBG_DIS_R {
        SE_DBG_DIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_prot_en_rd(&mut self) -> SE_SHA_PROT_EN_RD_W<0> {
        SE_SHA_PROT_EN_RD_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_id0_en_rd(&mut self) -> SE_SHA_ID0_EN_RD_W<1> {
        SE_SHA_ID0_EN_RD_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_sha_id1_en_rd(&mut self) -> SE_SHA_ID1_EN_RD_W<2> {
        SE_SHA_ID1_EN_RD_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_aes_prot_en_rd(&mut self) -> SE_AES_PROT_EN_RD_W<4> {
        SE_AES_PROT_EN_RD_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_aes_id0_en_rd(&mut self) -> SE_AES_ID0_EN_RD_W<5> {
        SE_AES_ID0_EN_RD_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_aes_id1_en_rd(&mut self) -> SE_AES_ID1_EN_RD_W<6> {
        SE_AES_ID1_EN_RD_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_prot_en_rd(&mut self) -> SE_TRNG_PROT_EN_RD_W<8> {
        SE_TRNG_PROT_EN_RD_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_id0_en_rd(&mut self) -> SE_TRNG_ID0_EN_RD_W<9> {
        SE_TRNG_ID0_EN_RD_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_id1_en_rd(&mut self) -> SE_TRNG_ID1_EN_RD_W<10> {
        SE_TRNG_ID1_EN_RD_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_prot_en_rd(&mut self) -> SE_PKA_PROT_EN_RD_W<12> {
        SE_PKA_PROT_EN_RD_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_id0_en_rd(&mut self) -> SE_PKA_ID0_EN_RD_W<13> {
        SE_PKA_ID0_EN_RD_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_pka_id1_en_rd(&mut self) -> SE_PKA_ID1_EN_RD_W<14> {
        SE_PKA_ID1_EN_RD_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_cdet_prot_en_rd(&mut self) -> SE_CDET_PROT_EN_RD_W<16> {
        SE_CDET_PROT_EN_RD_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn se_cdet_id0_en_rd(&mut self) -> SE_CDET_ID0_EN_RD_W<17> {
        SE_CDET_ID0_EN_RD_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn se_cdet_id1_en_rd(&mut self) -> SE_CDET_ID1_EN_RD_W<18> {
        SE_CDET_ID1_EN_RD_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn se_gmac_prot_en_rd(&mut self) -> SE_GMAC_PROT_EN_RD_W<20> {
        SE_GMAC_PROT_EN_RD_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn se_gmac_id0_en_rd(&mut self) -> SE_GMAC_ID0_EN_RD_W<21> {
        SE_GMAC_ID0_EN_RD_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn se_gmac_id1_en_rd(&mut self) -> SE_GMAC_ID1_EN_RD_W<22> {
        SE_GMAC_ID1_EN_RD_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn se_dbg_dis(&mut self) -> SE_DBG_DIS_W<31> {
        SE_DBG_DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_ctrl_prot_rd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_ctrl_prot_rd](index.html) module"]
pub struct SE_CTRL_PROT_RD_SPEC;
impl crate::RegisterSpec for SE_CTRL_PROT_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_ctrl_prot_rd::R](R) reader structure"]
impl crate::Readable for SE_CTRL_PROT_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_ctrl_prot_rd::W](W) writer structure"]
impl crate::Writable for SE_CTRL_PROT_RD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_ctrl_prot_rd to value 0"]
impl crate::Resettable for SE_CTRL_PROT_RD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
