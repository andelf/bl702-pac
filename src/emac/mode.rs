#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXEN` reader - "]
pub type RXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` writer - "]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `TXEN` reader - "]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` writer - "]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `NOPRE` reader - "]
pub type NOPRE_R = crate::BitReader<bool>;
#[doc = "Field `NOPRE` writer - "]
pub type NOPRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `BRO` reader - "]
pub type BRO_R = crate::BitReader<bool>;
#[doc = "Field `BRO` writer - "]
pub type BRO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `rsvd_4` reader - "]
pub type RSVD_4_R = crate::BitReader<bool>;
#[doc = "Field `rsvd_4` writer - "]
pub type RSVD_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `PRO` reader - "]
pub type PRO_R = crate::BitReader<bool>;
#[doc = "Field `PRO` writer - "]
pub type PRO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `IFG` reader - "]
pub type IFG_R = crate::BitReader<bool>;
#[doc = "Field `IFG` writer - "]
pub type IFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `rsvd_9_7` reader - "]
pub type RSVD_9_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rsvd_9_7` writer - "]
pub type RSVD_9_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 3, O>;
#[doc = "Field `FULLD` reader - "]
pub type FULLD_R = crate::BitReader<bool>;
#[doc = "Field `FULLD` writer - "]
pub type FULLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `rsvd_12_11` reader - "]
pub type RSVD_12_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rsvd_12_11` writer - "]
pub type RSVD_12_11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 2, O>;
#[doc = "Field `CRCEN` reader - "]
pub type CRCEN_R = crate::BitReader<bool>;
#[doc = "Field `CRCEN` writer - "]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `HUGEN` reader - "]
pub type HUGEN_R = crate::BitReader<bool>;
#[doc = "Field `HUGEN` writer - "]
pub type HUGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `PAD` reader - "]
pub type PAD_R = crate::BitReader<bool>;
#[doc = "Field `PAD` writer - "]
pub type PAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `RECSMALL` reader - "]
pub type RECSMALL_R = crate::BitReader<bool>;
#[doc = "Field `RECSMALL` writer - "]
pub type RECSMALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `RMII_EN` reader - "]
pub type RMII_EN_R = crate::BitReader<bool>;
#[doc = "Field `RMII_EN` writer - "]
pub type RMII_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `rsvd_23_18` reader - "]
pub type RSVD_23_18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `rsvd_23_18` writer - "]
pub type RSVD_23_18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn nopre(&self) -> NOPRE_R {
        NOPRE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bro(&self) -> BRO_R {
        BRO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rsvd_4(&self) -> RSVD_4_R {
        RSVD_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pro(&self) -> PRO_R {
        PRO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ifg(&self) -> IFG_R {
        IFG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    pub fn rsvd_9_7(&self) -> RSVD_9_7_R {
        RSVD_9_7_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fulld(&self) -> FULLD_R {
        FULLD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn rsvd_12_11(&self) -> RSVD_12_11_R {
        RSVD_12_11_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hugen(&self) -> HUGEN_R {
        HUGEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pad(&self) -> PAD_R {
        PAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn recsmall(&self) -> RECSMALL_R {
        RECSMALL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rmii_en(&self) -> RMII_EN_R {
        RMII_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rsvd_23_18(&self) -> RSVD_23_18_R {
        RSVD_23_18_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rxen(&mut self) -> RXEN_W<0> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn txen(&mut self) -> TXEN_W<1> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn nopre(&mut self) -> NOPRE_W<2> {
        NOPRE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn bro(&mut self) -> BRO_W<3> {
        BRO_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_4(&mut self) -> RSVD_4_W<4> {
        RSVD_4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn pro(&mut self) -> PRO_W<5> {
        PRO_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn ifg(&mut self) -> IFG_W<6> {
        IFG_W::new(self)
    }
    #[doc = "Bits 7:9"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_9_7(&mut self) -> RSVD_9_7_W<7> {
        RSVD_9_7_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn fulld(&mut self) -> FULLD_W<10> {
        FULLD_W::new(self)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_12_11(&mut self) -> RSVD_12_11_W<11> {
        RSVD_12_11_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<13> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn hugen(&mut self) -> HUGEN_W<14> {
        HUGEN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn pad(&mut self) -> PAD_W<15> {
        PAD_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn recsmall(&mut self) -> RECSMALL_W<16> {
        RECSMALL_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn rmii_en(&mut self) -> RMII_EN_W<17> {
        RMII_EN_W::new(self)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_23_18(&mut self) -> RSVD_23_18_W<18> {
        RSVD_23_18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MODE.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
