#[doc = "Register `testbuf` reader"]
pub struct R(crate::R<TESTBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TESTBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TESTBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TESTBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `testbuf` writer"]
pub struct W(crate::W<TESTBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TESTBUF_SPEC>;
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
impl From<crate::W<TESTBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TESTBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `testbuf_rin` reader - "]
pub type TESTBUF_RIN_R = crate::BitReader<bool>;
#[doc = "Field `testbuf_rin` writer - "]
pub type TESTBUF_RIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TESTBUF_SPEC, bool, O>;
#[doc = "Field `testbuf_rfb` reader - "]
pub type TESTBUF_RFB_R = crate::BitReader<bool>;
#[doc = "Field `testbuf_rfb` writer - "]
pub type TESTBUF_RFB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TESTBUF_SPEC, bool, O>;
#[doc = "Field `testbuf_op_cc` reader - "]
pub type TESTBUF_OP_CC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `testbuf_op_cc` writer - "]
pub type TESTBUF_OP_CC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TESTBUF_SPEC, u8, u8, 4, O>;
#[doc = "Field `testbuf_boost` reader - "]
pub type TESTBUF_BOOST_R = crate::BitReader<bool>;
#[doc = "Field `testbuf_boost` writer - "]
pub type TESTBUF_BOOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TESTBUF_SPEC, bool, O>;
#[doc = "Field `testbuf_bm` reader - "]
pub type TESTBUF_BM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `testbuf_bm` writer - "]
pub type TESTBUF_BM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TESTBUF_SPEC, u8, u8, 3, O>;
#[doc = "Field `testbuf_vcm` reader - "]
pub type TESTBUF_VCM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `testbuf_vcm` writer - "]
pub type TESTBUF_VCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TESTBUF_SPEC, u8, u8, 2, O>;
#[doc = "Field `pu_testbuf` reader - "]
pub type PU_TESTBUF_R = crate::BitReader<bool>;
#[doc = "Field `pu_testbuf` writer - "]
pub type PU_TESTBUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TESTBUF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn testbuf_rin(&self) -> TESTBUF_RIN_R {
        TESTBUF_RIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn testbuf_rfb(&self) -> TESTBUF_RFB_R {
        TESTBUF_RFB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn testbuf_op_cc(&self) -> TESTBUF_OP_CC_R {
        TESTBUF_OP_CC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn testbuf_boost(&self) -> TESTBUF_BOOST_R {
        TESTBUF_BOOST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn testbuf_bm(&self) -> TESTBUF_BM_R {
        TESTBUF_BM_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn testbuf_vcm(&self) -> TESTBUF_VCM_R {
        TESTBUF_VCM_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_testbuf(&self) -> PU_TESTBUF_R {
        PU_TESTBUF_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn testbuf_rin(&mut self) -> TESTBUF_RIN_W<0> {
        TESTBUF_RIN_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn testbuf_rfb(&mut self) -> TESTBUF_RFB_W<4> {
        TESTBUF_RFB_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn testbuf_op_cc(&mut self) -> TESTBUF_OP_CC_W<8> {
        TESTBUF_OP_CC_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn testbuf_boost(&mut self) -> TESTBUF_BOOST_W<12> {
        TESTBUF_BOOST_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn testbuf_bm(&mut self) -> TESTBUF_BM_W<16> {
        TESTBUF_BM_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn testbuf_vcm(&mut self) -> TESTBUF_VCM_W<20> {
        TESTBUF_VCM_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_testbuf(&mut self) -> PU_TESTBUF_W<24> {
        PU_TESTBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "testbuf.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [testbuf](index.html) module"]
pub struct TESTBUF_SPEC;
impl crate::RegisterSpec for TESTBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [testbuf::R](R) reader structure"]
impl crate::Readable for TESTBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [testbuf::W](W) writer structure"]
impl crate::Writable for TESTBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets testbuf to value 0"]
impl crate::Resettable for TESTBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
