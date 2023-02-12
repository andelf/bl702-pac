#[doc = "Register `GPIO_CFGCTL30` reader"]
pub struct R(crate::R<GPIO_CFGCTL30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL30` writer"]
pub struct W(crate::W<GPIO_CFGCTL30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL30_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_0_i` reader - "]
pub type REG_GPIO_0_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_i` writer - "]
pub type REG_GPIO_0_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_1_i` reader - "]
pub type REG_GPIO_1_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_1_i` writer - "]
pub type REG_GPIO_1_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_2_i` reader - "]
pub type REG_GPIO_2_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_2_i` writer - "]
pub type REG_GPIO_2_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_3_i` reader - "]
pub type REG_GPIO_3_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_3_i` writer - "]
pub type REG_GPIO_3_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_4_i` reader - "]
pub type REG_GPIO_4_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_4_i` writer - "]
pub type REG_GPIO_4_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_5_i` reader - "]
pub type REG_GPIO_5_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_5_i` writer - "]
pub type REG_GPIO_5_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_6_i` reader - "]
pub type REG_GPIO_6_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_6_i` writer - "]
pub type REG_GPIO_6_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_7_i` reader - "]
pub type REG_GPIO_7_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_7_i` writer - "]
pub type REG_GPIO_7_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_8_i` reader - "]
pub type REG_GPIO_8_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_8_i` writer - "]
pub type REG_GPIO_8_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_9_i` reader - "]
pub type REG_GPIO_9_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_9_i` writer - "]
pub type REG_GPIO_9_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_10_i` reader - "]
pub type REG_GPIO_10_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_10_i` writer - "]
pub type REG_GPIO_10_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_11_i` reader - "]
pub type REG_GPIO_11_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_11_i` writer - "]
pub type REG_GPIO_11_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_12_i` reader - "]
pub type REG_GPIO_12_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_12_i` writer - "]
pub type REG_GPIO_12_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_13_i` reader - "]
pub type REG_GPIO_13_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_13_i` writer - "]
pub type REG_GPIO_13_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_14_i` reader - "]
pub type REG_GPIO_14_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_14_i` writer - "]
pub type REG_GPIO_14_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_15_i` reader - "]
pub type REG_GPIO_15_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_15_i` writer - "]
pub type REG_GPIO_15_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_16_i` reader - "]
pub type REG_GPIO_16_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_16_i` writer - "]
pub type REG_GPIO_16_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_17_i` reader - "]
pub type REG_GPIO_17_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_17_i` writer - "]
pub type REG_GPIO_17_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_18_i` reader - "]
pub type REG_GPIO_18_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_18_i` writer - "]
pub type REG_GPIO_18_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_19_i` reader - "]
pub type REG_GPIO_19_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_19_i` writer - "]
pub type REG_GPIO_19_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_20_i` reader - "]
pub type REG_GPIO_20_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_20_i` writer - "]
pub type REG_GPIO_20_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_21_i` reader - "]
pub type REG_GPIO_21_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_21_i` writer - "]
pub type REG_GPIO_21_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_22_i` reader - "]
pub type REG_GPIO_22_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_22_i` writer - "]
pub type REG_GPIO_22_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_23_i` reader - "]
pub type REG_GPIO_23_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_23_i` writer - "]
pub type REG_GPIO_23_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_24_i` reader - "]
pub type REG_GPIO_24_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_24_i` writer - "]
pub type REG_GPIO_24_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_25_i` reader - "]
pub type REG_GPIO_25_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_25_i` writer - "]
pub type REG_GPIO_25_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_26_i` reader - "]
pub type REG_GPIO_26_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_26_i` writer - "]
pub type REG_GPIO_26_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_27_i` reader - "]
pub type REG_GPIO_27_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_27_i` writer - "]
pub type REG_GPIO_27_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_28_i` reader - "]
pub type REG_GPIO_28_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_28_i` writer - "]
pub type REG_GPIO_28_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_29_i` reader - "]
pub type REG_GPIO_29_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_29_i` writer - "]
pub type REG_GPIO_29_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_30_i` reader - "]
pub type REG_GPIO_30_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_30_i` writer - "]
pub type REG_GPIO_30_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
#[doc = "Field `reg_gpio_31_i` reader - "]
pub type REG_GPIO_31_I_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_31_i` writer - "]
pub type REG_GPIO_31_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL30_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_i(&self) -> REG_GPIO_0_I_R {
        REG_GPIO_0_I_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_i(&self) -> REG_GPIO_1_I_R {
        REG_GPIO_1_I_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_i(&self) -> REG_GPIO_2_I_R {
        REG_GPIO_2_I_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_i(&self) -> REG_GPIO_3_I_R {
        REG_GPIO_3_I_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_i(&self) -> REG_GPIO_4_I_R {
        REG_GPIO_4_I_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_i(&self) -> REG_GPIO_5_I_R {
        REG_GPIO_5_I_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_i(&self) -> REG_GPIO_6_I_R {
        REG_GPIO_6_I_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_i(&self) -> REG_GPIO_7_I_R {
        REG_GPIO_7_I_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_i(&self) -> REG_GPIO_8_I_R {
        REG_GPIO_8_I_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_i(&self) -> REG_GPIO_9_I_R {
        REG_GPIO_9_I_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_i(&self) -> REG_GPIO_10_I_R {
        REG_GPIO_10_I_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_i(&self) -> REG_GPIO_11_I_R {
        REG_GPIO_11_I_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_i(&self) -> REG_GPIO_12_I_R {
        REG_GPIO_12_I_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_i(&self) -> REG_GPIO_13_I_R {
        REG_GPIO_13_I_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_i(&self) -> REG_GPIO_14_I_R {
        REG_GPIO_14_I_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_i(&self) -> REG_GPIO_15_I_R {
        REG_GPIO_15_I_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_i(&self) -> REG_GPIO_16_I_R {
        REG_GPIO_16_I_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_i(&self) -> REG_GPIO_17_I_R {
        REG_GPIO_17_I_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_i(&self) -> REG_GPIO_18_I_R {
        REG_GPIO_18_I_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_i(&self) -> REG_GPIO_19_I_R {
        REG_GPIO_19_I_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_i(&self) -> REG_GPIO_20_I_R {
        REG_GPIO_20_I_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_i(&self) -> REG_GPIO_21_I_R {
        REG_GPIO_21_I_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_i(&self) -> REG_GPIO_22_I_R {
        REG_GPIO_22_I_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_gpio_23_i(&self) -> REG_GPIO_23_I_R {
        REG_GPIO_23_I_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_gpio_24_i(&self) -> REG_GPIO_24_I_R {
        REG_GPIO_24_I_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg_gpio_25_i(&self) -> REG_GPIO_25_I_R {
        REG_GPIO_25_I_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn reg_gpio_26_i(&self) -> REG_GPIO_26_I_R {
        REG_GPIO_26_I_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_gpio_27_i(&self) -> REG_GPIO_27_I_R {
        REG_GPIO_27_I_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_gpio_28_i(&self) -> REG_GPIO_28_I_R {
        REG_GPIO_28_I_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_gpio_29_i(&self) -> REG_GPIO_29_I_R {
        REG_GPIO_29_I_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reg_gpio_30_i(&self) -> REG_GPIO_30_I_R {
        REG_GPIO_30_I_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_gpio_31_i(&self) -> REG_GPIO_31_I_R {
        REG_GPIO_31_I_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_i(&mut self) -> REG_GPIO_0_I_W<0> {
        REG_GPIO_0_I_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_i(&mut self) -> REG_GPIO_1_I_W<1> {
        REG_GPIO_1_I_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_i(&mut self) -> REG_GPIO_2_I_W<2> {
        REG_GPIO_2_I_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_i(&mut self) -> REG_GPIO_3_I_W<3> {
        REG_GPIO_3_I_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_i(&mut self) -> REG_GPIO_4_I_W<4> {
        REG_GPIO_4_I_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_i(&mut self) -> REG_GPIO_5_I_W<5> {
        REG_GPIO_5_I_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_i(&mut self) -> REG_GPIO_6_I_W<6> {
        REG_GPIO_6_I_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_i(&mut self) -> REG_GPIO_7_I_W<7> {
        REG_GPIO_7_I_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_8_i(&mut self) -> REG_GPIO_8_I_W<8> {
        REG_GPIO_8_I_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_9_i(&mut self) -> REG_GPIO_9_I_W<9> {
        REG_GPIO_9_I_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_10_i(&mut self) -> REG_GPIO_10_I_W<10> {
        REG_GPIO_10_I_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_11_i(&mut self) -> REG_GPIO_11_I_W<11> {
        REG_GPIO_11_I_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_i(&mut self) -> REG_GPIO_12_I_W<12> {
        REG_GPIO_12_I_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_i(&mut self) -> REG_GPIO_13_I_W<13> {
        REG_GPIO_13_I_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_i(&mut self) -> REG_GPIO_14_I_W<14> {
        REG_GPIO_14_I_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_i(&mut self) -> REG_GPIO_15_I_W<15> {
        REG_GPIO_15_I_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_16_i(&mut self) -> REG_GPIO_16_I_W<16> {
        REG_GPIO_16_I_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_17_i(&mut self) -> REG_GPIO_17_I_W<17> {
        REG_GPIO_17_I_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_i(&mut self) -> REG_GPIO_18_I_W<18> {
        REG_GPIO_18_I_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_i(&mut self) -> REG_GPIO_19_I_W<19> {
        REG_GPIO_19_I_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_i(&mut self) -> REG_GPIO_20_I_W<20> {
        REG_GPIO_20_I_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_i(&mut self) -> REG_GPIO_21_I_W<21> {
        REG_GPIO_21_I_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_i(&mut self) -> REG_GPIO_22_I_W<22> {
        REG_GPIO_22_I_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_i(&mut self) -> REG_GPIO_23_I_W<23> {
        REG_GPIO_23_I_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_i(&mut self) -> REG_GPIO_24_I_W<24> {
        REG_GPIO_24_I_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_i(&mut self) -> REG_GPIO_25_I_W<25> {
        REG_GPIO_25_I_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_i(&mut self) -> REG_GPIO_26_I_W<26> {
        REG_GPIO_26_I_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_i(&mut self) -> REG_GPIO_27_I_W<27> {
        REG_GPIO_27_I_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_i(&mut self) -> REG_GPIO_28_I_W<28> {
        REG_GPIO_28_I_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_29_i(&mut self) -> REG_GPIO_29_I_W<29> {
        REG_GPIO_29_I_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_30_i(&mut self) -> REG_GPIO_30_I_W<30> {
        REG_GPIO_30_I_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_31_i(&mut self) -> REG_GPIO_31_I_W<31> {
        REG_GPIO_31_I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_CFGCTL30.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl30](index.html) module"]
pub struct GPIO_CFGCTL30_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl30::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl30::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL30_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL30 to value 0"]
impl crate::Resettable for GPIO_CFGCTL30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
