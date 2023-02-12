#[doc = "Register `GPIO_CFGCTL32` reader"]
pub struct R(crate::R<GPIO_CFGCTL32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL32` writer"]
pub struct W(crate::W<GPIO_CFGCTL32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL32_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_0_o` reader - "]
pub type REG_GPIO_0_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_o` writer - "]
pub type REG_GPIO_0_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_1_o` reader - "]
pub type REG_GPIO_1_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_1_o` writer - "]
pub type REG_GPIO_1_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_2_o` reader - "]
pub type REG_GPIO_2_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_2_o` writer - "]
pub type REG_GPIO_2_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_3_o` reader - "]
pub type REG_GPIO_3_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_3_o` writer - "]
pub type REG_GPIO_3_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_4_o` reader - "]
pub type REG_GPIO_4_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_4_o` writer - "]
pub type REG_GPIO_4_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_5_o` reader - "]
pub type REG_GPIO_5_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_5_o` writer - "]
pub type REG_GPIO_5_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_6_o` reader - "]
pub type REG_GPIO_6_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_6_o` writer - "]
pub type REG_GPIO_6_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_7_o` reader - "]
pub type REG_GPIO_7_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_7_o` writer - "]
pub type REG_GPIO_7_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_8_o` reader - "]
pub type REG_GPIO_8_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_8_o` writer - "]
pub type REG_GPIO_8_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_9_o` reader - "]
pub type REG_GPIO_9_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_9_o` writer - "]
pub type REG_GPIO_9_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_10_o` reader - "]
pub type REG_GPIO_10_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_10_o` writer - "]
pub type REG_GPIO_10_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_11_o` reader - "]
pub type REG_GPIO_11_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_11_o` writer - "]
pub type REG_GPIO_11_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_12_o` reader - "]
pub type REG_GPIO_12_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_12_o` writer - "]
pub type REG_GPIO_12_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_13_o` reader - "]
pub type REG_GPIO_13_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_13_o` writer - "]
pub type REG_GPIO_13_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_14_o` reader - "]
pub type REG_GPIO_14_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_14_o` writer - "]
pub type REG_GPIO_14_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_15_o` reader - "]
pub type REG_GPIO_15_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_15_o` writer - "]
pub type REG_GPIO_15_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_16_o` reader - "]
pub type REG_GPIO_16_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_16_o` writer - "]
pub type REG_GPIO_16_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_17_o` reader - "]
pub type REG_GPIO_17_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_17_o` writer - "]
pub type REG_GPIO_17_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_18_o` reader - "]
pub type REG_GPIO_18_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_18_o` writer - "]
pub type REG_GPIO_18_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_19_o` reader - "]
pub type REG_GPIO_19_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_19_o` writer - "]
pub type REG_GPIO_19_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_20_o` reader - "]
pub type REG_GPIO_20_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_20_o` writer - "]
pub type REG_GPIO_20_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_21_o` reader - "]
pub type REG_GPIO_21_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_21_o` writer - "]
pub type REG_GPIO_21_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_22_o` reader - "]
pub type REG_GPIO_22_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_22_o` writer - "]
pub type REG_GPIO_22_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_23_o` reader - "]
pub type REG_GPIO_23_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_23_o` writer - "]
pub type REG_GPIO_23_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_24_o` reader - "]
pub type REG_GPIO_24_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_24_o` writer - "]
pub type REG_GPIO_24_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_25_o` reader - "]
pub type REG_GPIO_25_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_25_o` writer - "]
pub type REG_GPIO_25_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_26_o` reader - "]
pub type REG_GPIO_26_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_26_o` writer - "]
pub type REG_GPIO_26_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_27_o` reader - "]
pub type REG_GPIO_27_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_27_o` writer - "]
pub type REG_GPIO_27_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_28_o` reader - "]
pub type REG_GPIO_28_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_28_o` writer - "]
pub type REG_GPIO_28_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_29_o` reader - "]
pub type REG_GPIO_29_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_29_o` writer - "]
pub type REG_GPIO_29_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_30_o` reader - "]
pub type REG_GPIO_30_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_30_o` writer - "]
pub type REG_GPIO_30_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
#[doc = "Field `reg_gpio_31_o` reader - "]
pub type REG_GPIO_31_O_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_31_o` writer - "]
pub type REG_GPIO_31_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL32_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_o(&self) -> REG_GPIO_0_O_R {
        REG_GPIO_0_O_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_o(&self) -> REG_GPIO_1_O_R {
        REG_GPIO_1_O_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_o(&self) -> REG_GPIO_2_O_R {
        REG_GPIO_2_O_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_o(&self) -> REG_GPIO_3_O_R {
        REG_GPIO_3_O_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_o(&self) -> REG_GPIO_4_O_R {
        REG_GPIO_4_O_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_o(&self) -> REG_GPIO_5_O_R {
        REG_GPIO_5_O_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_o(&self) -> REG_GPIO_6_O_R {
        REG_GPIO_6_O_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_o(&self) -> REG_GPIO_7_O_R {
        REG_GPIO_7_O_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_o(&self) -> REG_GPIO_8_O_R {
        REG_GPIO_8_O_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_o(&self) -> REG_GPIO_9_O_R {
        REG_GPIO_9_O_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_o(&self) -> REG_GPIO_10_O_R {
        REG_GPIO_10_O_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_o(&self) -> REG_GPIO_11_O_R {
        REG_GPIO_11_O_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_o(&self) -> REG_GPIO_12_O_R {
        REG_GPIO_12_O_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_o(&self) -> REG_GPIO_13_O_R {
        REG_GPIO_13_O_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_o(&self) -> REG_GPIO_14_O_R {
        REG_GPIO_14_O_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_o(&self) -> REG_GPIO_15_O_R {
        REG_GPIO_15_O_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_o(&self) -> REG_GPIO_16_O_R {
        REG_GPIO_16_O_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_o(&self) -> REG_GPIO_17_O_R {
        REG_GPIO_17_O_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_o(&self) -> REG_GPIO_18_O_R {
        REG_GPIO_18_O_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_o(&self) -> REG_GPIO_19_O_R {
        REG_GPIO_19_O_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_o(&self) -> REG_GPIO_20_O_R {
        REG_GPIO_20_O_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_o(&self) -> REG_GPIO_21_O_R {
        REG_GPIO_21_O_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_o(&self) -> REG_GPIO_22_O_R {
        REG_GPIO_22_O_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_gpio_23_o(&self) -> REG_GPIO_23_O_R {
        REG_GPIO_23_O_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_gpio_24_o(&self) -> REG_GPIO_24_O_R {
        REG_GPIO_24_O_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg_gpio_25_o(&self) -> REG_GPIO_25_O_R {
        REG_GPIO_25_O_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn reg_gpio_26_o(&self) -> REG_GPIO_26_O_R {
        REG_GPIO_26_O_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_gpio_27_o(&self) -> REG_GPIO_27_O_R {
        REG_GPIO_27_O_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_gpio_28_o(&self) -> REG_GPIO_28_O_R {
        REG_GPIO_28_O_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_gpio_29_o(&self) -> REG_GPIO_29_O_R {
        REG_GPIO_29_O_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reg_gpio_30_o(&self) -> REG_GPIO_30_O_R {
        REG_GPIO_30_O_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_gpio_31_o(&self) -> REG_GPIO_31_O_R {
        REG_GPIO_31_O_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_0_o(&mut self) -> REG_GPIO_0_O_W<0> {
        REG_GPIO_0_O_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_1_o(&mut self) -> REG_GPIO_1_O_W<1> {
        REG_GPIO_1_O_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_2_o(&mut self) -> REG_GPIO_2_O_W<2> {
        REG_GPIO_2_O_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_3_o(&mut self) -> REG_GPIO_3_O_W<3> {
        REG_GPIO_3_O_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_4_o(&mut self) -> REG_GPIO_4_O_W<4> {
        REG_GPIO_4_O_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_5_o(&mut self) -> REG_GPIO_5_O_W<5> {
        REG_GPIO_5_O_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_6_o(&mut self) -> REG_GPIO_6_O_W<6> {
        REG_GPIO_6_O_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_7_o(&mut self) -> REG_GPIO_7_O_W<7> {
        REG_GPIO_7_O_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_8_o(&mut self) -> REG_GPIO_8_O_W<8> {
        REG_GPIO_8_O_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_9_o(&mut self) -> REG_GPIO_9_O_W<9> {
        REG_GPIO_9_O_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_10_o(&mut self) -> REG_GPIO_10_O_W<10> {
        REG_GPIO_10_O_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_11_o(&mut self) -> REG_GPIO_11_O_W<11> {
        REG_GPIO_11_O_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_12_o(&mut self) -> REG_GPIO_12_O_W<12> {
        REG_GPIO_12_O_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_13_o(&mut self) -> REG_GPIO_13_O_W<13> {
        REG_GPIO_13_O_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_14_o(&mut self) -> REG_GPIO_14_O_W<14> {
        REG_GPIO_14_O_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_15_o(&mut self) -> REG_GPIO_15_O_W<15> {
        REG_GPIO_15_O_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_16_o(&mut self) -> REG_GPIO_16_O_W<16> {
        REG_GPIO_16_O_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_17_o(&mut self) -> REG_GPIO_17_O_W<17> {
        REG_GPIO_17_O_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_18_o(&mut self) -> REG_GPIO_18_O_W<18> {
        REG_GPIO_18_O_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_19_o(&mut self) -> REG_GPIO_19_O_W<19> {
        REG_GPIO_19_O_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_20_o(&mut self) -> REG_GPIO_20_O_W<20> {
        REG_GPIO_20_O_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_21_o(&mut self) -> REG_GPIO_21_O_W<21> {
        REG_GPIO_21_O_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_22_o(&mut self) -> REG_GPIO_22_O_W<22> {
        REG_GPIO_22_O_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_23_o(&mut self) -> REG_GPIO_23_O_W<23> {
        REG_GPIO_23_O_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_24_o(&mut self) -> REG_GPIO_24_O_W<24> {
        REG_GPIO_24_O_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_25_o(&mut self) -> REG_GPIO_25_O_W<25> {
        REG_GPIO_25_O_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_26_o(&mut self) -> REG_GPIO_26_O_W<26> {
        REG_GPIO_26_O_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_27_o(&mut self) -> REG_GPIO_27_O_W<27> {
        REG_GPIO_27_O_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_28_o(&mut self) -> REG_GPIO_28_O_W<28> {
        REG_GPIO_28_O_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_29_o(&mut self) -> REG_GPIO_29_O_W<29> {
        REG_GPIO_29_O_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_30_o(&mut self) -> REG_GPIO_30_O_W<30> {
        REG_GPIO_30_O_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn reg_gpio_31_o(&mut self) -> REG_GPIO_31_O_W<31> {
        REG_GPIO_31_O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_CFGCTL32.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl32](index.html) module"]
pub struct GPIO_CFGCTL32_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl32::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl32::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_CFGCTL32 to value 0"]
impl crate::Resettable for GPIO_CFGCTL32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
