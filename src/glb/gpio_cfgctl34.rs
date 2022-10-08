#[doc = "Register `GPIO_CFGCTL34` reader"]
pub struct R(crate::R<GPIO_CFGCTL34_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL34_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL34_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL34_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL34` writer"]
pub struct W(crate::W<GPIO_CFGCTL34_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL34_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL34_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL34_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_0_oe` reader - "]
pub type REG_GPIO_0_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_0_oe` writer - "]
pub type REG_GPIO_0_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_1_oe` reader - "]
pub type REG_GPIO_1_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_1_oe` writer - "]
pub type REG_GPIO_1_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_2_oe` reader - "]
pub type REG_GPIO_2_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_2_oe` writer - "]
pub type REG_GPIO_2_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_3_oe` reader - "]
pub type REG_GPIO_3_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_3_oe` writer - "]
pub type REG_GPIO_3_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_4_oe` reader - "]
pub type REG_GPIO_4_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_4_oe` writer - "]
pub type REG_GPIO_4_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_5_oe` reader - "]
pub type REG_GPIO_5_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_5_oe` writer - "]
pub type REG_GPIO_5_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_6_oe` reader - "]
pub type REG_GPIO_6_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_6_oe` writer - "]
pub type REG_GPIO_6_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_7_oe` reader - "]
pub type REG_GPIO_7_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_7_oe` writer - "]
pub type REG_GPIO_7_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_8_oe` reader - "]
pub type REG_GPIO_8_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_8_oe` writer - "]
pub type REG_GPIO_8_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_9_oe` reader - "]
pub type REG_GPIO_9_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_9_oe` writer - "]
pub type REG_GPIO_9_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_10_oe` reader - "]
pub type REG_GPIO_10_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_10_oe` writer - "]
pub type REG_GPIO_10_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_11_oe` reader - "]
pub type REG_GPIO_11_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_11_oe` writer - "]
pub type REG_GPIO_11_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_12_oe` reader - "]
pub type REG_GPIO_12_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_12_oe` writer - "]
pub type REG_GPIO_12_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_13_oe` reader - "]
pub type REG_GPIO_13_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_13_oe` writer - "]
pub type REG_GPIO_13_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_14_oe` reader - "]
pub type REG_GPIO_14_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_14_oe` writer - "]
pub type REG_GPIO_14_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_15_oe` reader - "]
pub type REG_GPIO_15_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_15_oe` writer - "]
pub type REG_GPIO_15_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_16_oe` reader - "]
pub type REG_GPIO_16_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_16_oe` writer - "]
pub type REG_GPIO_16_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_17_oe` reader - "]
pub type REG_GPIO_17_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_17_oe` writer - "]
pub type REG_GPIO_17_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_18_oe` reader - "]
pub type REG_GPIO_18_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_18_oe` writer - "]
pub type REG_GPIO_18_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_19_oe` reader - "]
pub type REG_GPIO_19_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_19_oe` writer - "]
pub type REG_GPIO_19_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_20_oe` reader - "]
pub type REG_GPIO_20_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_20_oe` writer - "]
pub type REG_GPIO_20_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_21_oe` reader - "]
pub type REG_GPIO_21_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_21_oe` writer - "]
pub type REG_GPIO_21_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_22_oe` reader - "]
pub type REG_GPIO_22_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_22_oe` writer - "]
pub type REG_GPIO_22_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_23_oe` reader - "]
pub type REG_GPIO_23_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_23_oe` writer - "]
pub type REG_GPIO_23_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_24_oe` reader - "]
pub type REG_GPIO_24_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_24_oe` writer - "]
pub type REG_GPIO_24_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_25_oe` reader - "]
pub type REG_GPIO_25_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_25_oe` writer - "]
pub type REG_GPIO_25_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_26_oe` reader - "]
pub type REG_GPIO_26_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_26_oe` writer - "]
pub type REG_GPIO_26_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_27_oe` reader - "]
pub type REG_GPIO_27_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_27_oe` writer - "]
pub type REG_GPIO_27_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_28_oe` reader - "]
pub type REG_GPIO_28_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_28_oe` writer - "]
pub type REG_GPIO_28_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_29_oe` reader - "]
pub type REG_GPIO_29_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_29_oe` writer - "]
pub type REG_GPIO_29_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_30_oe` reader - "]
pub type REG_GPIO_30_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_30_oe` writer - "]
pub type REG_GPIO_30_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
#[doc = "Field `reg_gpio_31_oe` reader - "]
pub type REG_GPIO_31_OE_R = crate::BitReader<bool>;
#[doc = "Field `reg_gpio_31_oe` writer - "]
pub type REG_GPIO_31_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO_CFGCTL34_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_oe(&self) -> REG_GPIO_0_OE_R {
        REG_GPIO_0_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_oe(&self) -> REG_GPIO_1_OE_R {
        REG_GPIO_1_OE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_oe(&self) -> REG_GPIO_2_OE_R {
        REG_GPIO_2_OE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_oe(&self) -> REG_GPIO_3_OE_R {
        REG_GPIO_3_OE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_oe(&self) -> REG_GPIO_4_OE_R {
        REG_GPIO_4_OE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_oe(&self) -> REG_GPIO_5_OE_R {
        REG_GPIO_5_OE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_oe(&self) -> REG_GPIO_6_OE_R {
        REG_GPIO_6_OE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_oe(&self) -> REG_GPIO_7_OE_R {
        REG_GPIO_7_OE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_oe(&self) -> REG_GPIO_8_OE_R {
        REG_GPIO_8_OE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_oe(&self) -> REG_GPIO_9_OE_R {
        REG_GPIO_9_OE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_oe(&self) -> REG_GPIO_10_OE_R {
        REG_GPIO_10_OE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_oe(&self) -> REG_GPIO_11_OE_R {
        REG_GPIO_11_OE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_oe(&self) -> REG_GPIO_12_OE_R {
        REG_GPIO_12_OE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_oe(&self) -> REG_GPIO_13_OE_R {
        REG_GPIO_13_OE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_oe(&self) -> REG_GPIO_14_OE_R {
        REG_GPIO_14_OE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_oe(&self) -> REG_GPIO_15_OE_R {
        REG_GPIO_15_OE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_oe(&self) -> REG_GPIO_16_OE_R {
        REG_GPIO_16_OE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_oe(&self) -> REG_GPIO_17_OE_R {
        REG_GPIO_17_OE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_oe(&self) -> REG_GPIO_18_OE_R {
        REG_GPIO_18_OE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_oe(&self) -> REG_GPIO_19_OE_R {
        REG_GPIO_19_OE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_oe(&self) -> REG_GPIO_20_OE_R {
        REG_GPIO_20_OE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_oe(&self) -> REG_GPIO_21_OE_R {
        REG_GPIO_21_OE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_oe(&self) -> REG_GPIO_22_OE_R {
        REG_GPIO_22_OE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_gpio_23_oe(&self) -> REG_GPIO_23_OE_R {
        REG_GPIO_23_OE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_gpio_24_oe(&self) -> REG_GPIO_24_OE_R {
        REG_GPIO_24_OE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg_gpio_25_oe(&self) -> REG_GPIO_25_OE_R {
        REG_GPIO_25_OE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn reg_gpio_26_oe(&self) -> REG_GPIO_26_OE_R {
        REG_GPIO_26_OE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_gpio_27_oe(&self) -> REG_GPIO_27_OE_R {
        REG_GPIO_27_OE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_gpio_28_oe(&self) -> REG_GPIO_28_OE_R {
        REG_GPIO_28_OE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_gpio_29_oe(&self) -> REG_GPIO_29_OE_R {
        REG_GPIO_29_OE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reg_gpio_30_oe(&self) -> REG_GPIO_30_OE_R {
        REG_GPIO_30_OE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_gpio_31_oe(&self) -> REG_GPIO_31_OE_R {
        REG_GPIO_31_OE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_oe(&mut self) -> REG_GPIO_0_OE_W<0> {
        REG_GPIO_0_OE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_oe(&mut self) -> REG_GPIO_1_OE_W<1> {
        REG_GPIO_1_OE_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_oe(&mut self) -> REG_GPIO_2_OE_W<2> {
        REG_GPIO_2_OE_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_oe(&mut self) -> REG_GPIO_3_OE_W<3> {
        REG_GPIO_3_OE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_oe(&mut self) -> REG_GPIO_4_OE_W<4> {
        REG_GPIO_4_OE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_oe(&mut self) -> REG_GPIO_5_OE_W<5> {
        REG_GPIO_5_OE_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_oe(&mut self) -> REG_GPIO_6_OE_W<6> {
        REG_GPIO_6_OE_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_oe(&mut self) -> REG_GPIO_7_OE_W<7> {
        REG_GPIO_7_OE_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_oe(&mut self) -> REG_GPIO_8_OE_W<8> {
        REG_GPIO_8_OE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_oe(&mut self) -> REG_GPIO_9_OE_W<9> {
        REG_GPIO_9_OE_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_oe(&mut self) -> REG_GPIO_10_OE_W<10> {
        REG_GPIO_10_OE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_oe(&mut self) -> REG_GPIO_11_OE_W<11> {
        REG_GPIO_11_OE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_oe(&mut self) -> REG_GPIO_12_OE_W<12> {
        REG_GPIO_12_OE_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_oe(&mut self) -> REG_GPIO_13_OE_W<13> {
        REG_GPIO_13_OE_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_oe(&mut self) -> REG_GPIO_14_OE_W<14> {
        REG_GPIO_14_OE_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_oe(&mut self) -> REG_GPIO_15_OE_W<15> {
        REG_GPIO_15_OE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_oe(&mut self) -> REG_GPIO_16_OE_W<16> {
        REG_GPIO_16_OE_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_oe(&mut self) -> REG_GPIO_17_OE_W<17> {
        REG_GPIO_17_OE_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_oe(&mut self) -> REG_GPIO_18_OE_W<18> {
        REG_GPIO_18_OE_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_oe(&mut self) -> REG_GPIO_19_OE_W<19> {
        REG_GPIO_19_OE_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_oe(&mut self) -> REG_GPIO_20_OE_W<20> {
        REG_GPIO_20_OE_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_oe(&mut self) -> REG_GPIO_21_OE_W<21> {
        REG_GPIO_21_OE_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_oe(&mut self) -> REG_GPIO_22_OE_W<22> {
        REG_GPIO_22_OE_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_gpio_23_oe(&mut self) -> REG_GPIO_23_OE_W<23> {
        REG_GPIO_23_OE_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_gpio_24_oe(&mut self) -> REG_GPIO_24_OE_W<24> {
        REG_GPIO_24_OE_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg_gpio_25_oe(&mut self) -> REG_GPIO_25_OE_W<25> {
        REG_GPIO_25_OE_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn reg_gpio_26_oe(&mut self) -> REG_GPIO_26_OE_W<26> {
        REG_GPIO_26_OE_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_gpio_27_oe(&mut self) -> REG_GPIO_27_OE_W<27> {
        REG_GPIO_27_OE_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn reg_gpio_28_oe(&mut self) -> REG_GPIO_28_OE_W<28> {
        REG_GPIO_28_OE_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn reg_gpio_29_oe(&mut self) -> REG_GPIO_29_OE_W<29> {
        REG_GPIO_29_OE_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn reg_gpio_30_oe(&mut self) -> REG_GPIO_30_OE_W<30> {
        REG_GPIO_30_OE_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_gpio_31_oe(&mut self) -> REG_GPIO_31_OE_W<31> {
        REG_GPIO_31_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_CFGCTL34.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl34](index.html) module"]
pub struct GPIO_CFGCTL34_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl34::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL34_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl34::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL34_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL34 to value 0"]
impl crate::Resettable for GPIO_CFGCTL34_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
