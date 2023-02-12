#[doc = "Register `tzc_glb_ctrl_2` reader"]
pub struct R(crate::R<TZC_GLB_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GLB_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_GLB_CTRL_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_GLB_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_glb_ctrl_2` writer"]
pub struct W(crate::W<TZC_GLB_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_GLB_CTRL_2_SPEC>;
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
impl From<crate::W<TZC_GLB_CTRL_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_GLB_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_glb_gpio_0_lock` reader - "]
pub type TZC_GLB_GPIO_0_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_0_lock` writer - "]
pub type TZC_GLB_GPIO_0_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_1_lock` reader - "]
pub type TZC_GLB_GPIO_1_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_1_lock` writer - "]
pub type TZC_GLB_GPIO_1_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_2_lock` reader - "]
pub type TZC_GLB_GPIO_2_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_2_lock` writer - "]
pub type TZC_GLB_GPIO_2_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_3_lock` reader - "]
pub type TZC_GLB_GPIO_3_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_3_lock` writer - "]
pub type TZC_GLB_GPIO_3_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_4_lock` reader - "]
pub type TZC_GLB_GPIO_4_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_4_lock` writer - "]
pub type TZC_GLB_GPIO_4_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_5_lock` reader - "]
pub type TZC_GLB_GPIO_5_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_5_lock` writer - "]
pub type TZC_GLB_GPIO_5_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_6_lock` reader - "]
pub type TZC_GLB_GPIO_6_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_6_lock` writer - "]
pub type TZC_GLB_GPIO_6_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_7_lock` reader - "]
pub type TZC_GLB_GPIO_7_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_7_lock` writer - "]
pub type TZC_GLB_GPIO_7_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_8_lock` reader - "]
pub type TZC_GLB_GPIO_8_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_8_lock` writer - "]
pub type TZC_GLB_GPIO_8_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_9_lock` reader - "]
pub type TZC_GLB_GPIO_9_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_9_lock` writer - "]
pub type TZC_GLB_GPIO_9_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_10_lock` reader - "]
pub type TZC_GLB_GPIO_10_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_10_lock` writer - "]
pub type TZC_GLB_GPIO_10_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_11_lock` reader - "]
pub type TZC_GLB_GPIO_11_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_11_lock` writer - "]
pub type TZC_GLB_GPIO_11_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_12_lock` reader - "]
pub type TZC_GLB_GPIO_12_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_12_lock` writer - "]
pub type TZC_GLB_GPIO_12_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_13_lock` reader - "]
pub type TZC_GLB_GPIO_13_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_13_lock` writer - "]
pub type TZC_GLB_GPIO_13_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_14_lock` reader - "]
pub type TZC_GLB_GPIO_14_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_14_lock` writer - "]
pub type TZC_GLB_GPIO_14_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_15_lock` reader - "]
pub type TZC_GLB_GPIO_15_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_15_lock` writer - "]
pub type TZC_GLB_GPIO_15_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_16_lock` reader - "]
pub type TZC_GLB_GPIO_16_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_16_lock` writer - "]
pub type TZC_GLB_GPIO_16_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_17_lock` reader - "]
pub type TZC_GLB_GPIO_17_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_17_lock` writer - "]
pub type TZC_GLB_GPIO_17_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_18_lock` reader - "]
pub type TZC_GLB_GPIO_18_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_18_lock` writer - "]
pub type TZC_GLB_GPIO_18_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_19_lock` reader - "]
pub type TZC_GLB_GPIO_19_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_19_lock` writer - "]
pub type TZC_GLB_GPIO_19_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_20_lock` reader - "]
pub type TZC_GLB_GPIO_20_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_20_lock` writer - "]
pub type TZC_GLB_GPIO_20_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_21_lock` reader - "]
pub type TZC_GLB_GPIO_21_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_21_lock` writer - "]
pub type TZC_GLB_GPIO_21_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_22_lock` reader - "]
pub type TZC_GLB_GPIO_22_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_22_lock` writer - "]
pub type TZC_GLB_GPIO_22_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_23_lock` reader - "]
pub type TZC_GLB_GPIO_23_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_23_lock` writer - "]
pub type TZC_GLB_GPIO_23_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_24_lock` reader - "]
pub type TZC_GLB_GPIO_24_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_24_lock` writer - "]
pub type TZC_GLB_GPIO_24_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_25_lock` reader - "]
pub type TZC_GLB_GPIO_25_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_25_lock` writer - "]
pub type TZC_GLB_GPIO_25_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_26_lock` reader - "]
pub type TZC_GLB_GPIO_26_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_26_lock` writer - "]
pub type TZC_GLB_GPIO_26_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_27_lock` reader - "]
pub type TZC_GLB_GPIO_27_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_27_lock` writer - "]
pub type TZC_GLB_GPIO_27_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_28_lock` reader - "]
pub type TZC_GLB_GPIO_28_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_28_lock` writer - "]
pub type TZC_GLB_GPIO_28_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_29_lock` reader - "]
pub type TZC_GLB_GPIO_29_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_29_lock` writer - "]
pub type TZC_GLB_GPIO_29_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_30_lock` reader - "]
pub type TZC_GLB_GPIO_30_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_30_lock` writer - "]
pub type TZC_GLB_GPIO_30_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
#[doc = "Field `tzc_glb_gpio_31_lock` reader - "]
pub type TZC_GLB_GPIO_31_LOCK_R = crate::BitReader<bool>;
#[doc = "Field `tzc_glb_gpio_31_lock` writer - "]
pub type TZC_GLB_GPIO_31_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TZC_GLB_CTRL_2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_gpio_0_lock(&self) -> TZC_GLB_GPIO_0_LOCK_R {
        TZC_GLB_GPIO_0_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_gpio_1_lock(&self) -> TZC_GLB_GPIO_1_LOCK_R {
        TZC_GLB_GPIO_1_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_gpio_2_lock(&self) -> TZC_GLB_GPIO_2_LOCK_R {
        TZC_GLB_GPIO_2_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_gpio_3_lock(&self) -> TZC_GLB_GPIO_3_LOCK_R {
        TZC_GLB_GPIO_3_LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_gpio_4_lock(&self) -> TZC_GLB_GPIO_4_LOCK_R {
        TZC_GLB_GPIO_4_LOCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_gpio_5_lock(&self) -> TZC_GLB_GPIO_5_LOCK_R {
        TZC_GLB_GPIO_5_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_glb_gpio_6_lock(&self) -> TZC_GLB_GPIO_6_LOCK_R {
        TZC_GLB_GPIO_6_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_glb_gpio_7_lock(&self) -> TZC_GLB_GPIO_7_LOCK_R {
        TZC_GLB_GPIO_7_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_gpio_8_lock(&self) -> TZC_GLB_GPIO_8_LOCK_R {
        TZC_GLB_GPIO_8_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_glb_gpio_9_lock(&self) -> TZC_GLB_GPIO_9_LOCK_R {
        TZC_GLB_GPIO_9_LOCK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_glb_gpio_10_lock(&self) -> TZC_GLB_GPIO_10_LOCK_R {
        TZC_GLB_GPIO_10_LOCK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_glb_gpio_11_lock(&self) -> TZC_GLB_GPIO_11_LOCK_R {
        TZC_GLB_GPIO_11_LOCK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_gpio_12_lock(&self) -> TZC_GLB_GPIO_12_LOCK_R {
        TZC_GLB_GPIO_12_LOCK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_gpio_13_lock(&self) -> TZC_GLB_GPIO_13_LOCK_R {
        TZC_GLB_GPIO_13_LOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_gpio_14_lock(&self) -> TZC_GLB_GPIO_14_LOCK_R {
        TZC_GLB_GPIO_14_LOCK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_gpio_15_lock(&self) -> TZC_GLB_GPIO_15_LOCK_R {
        TZC_GLB_GPIO_15_LOCK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_glb_gpio_16_lock(&self) -> TZC_GLB_GPIO_16_LOCK_R {
        TZC_GLB_GPIO_16_LOCK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_glb_gpio_17_lock(&self) -> TZC_GLB_GPIO_17_LOCK_R {
        TZC_GLB_GPIO_17_LOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_glb_gpio_18_lock(&self) -> TZC_GLB_GPIO_18_LOCK_R {
        TZC_GLB_GPIO_18_LOCK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_glb_gpio_19_lock(&self) -> TZC_GLB_GPIO_19_LOCK_R {
        TZC_GLB_GPIO_19_LOCK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_glb_gpio_20_lock(&self) -> TZC_GLB_GPIO_20_LOCK_R {
        TZC_GLB_GPIO_20_LOCK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_gpio_21_lock(&self) -> TZC_GLB_GPIO_21_LOCK_R {
        TZC_GLB_GPIO_21_LOCK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_gpio_22_lock(&self) -> TZC_GLB_GPIO_22_LOCK_R {
        TZC_GLB_GPIO_22_LOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_glb_gpio_23_lock(&self) -> TZC_GLB_GPIO_23_LOCK_R {
        TZC_GLB_GPIO_23_LOCK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_gpio_24_lock(&self) -> TZC_GLB_GPIO_24_LOCK_R {
        TZC_GLB_GPIO_24_LOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_gpio_25_lock(&self) -> TZC_GLB_GPIO_25_LOCK_R {
        TZC_GLB_GPIO_25_LOCK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_gpio_26_lock(&self) -> TZC_GLB_GPIO_26_LOCK_R {
        TZC_GLB_GPIO_26_LOCK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_gpio_27_lock(&self) -> TZC_GLB_GPIO_27_LOCK_R {
        TZC_GLB_GPIO_27_LOCK_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_gpio_28_lock(&self) -> TZC_GLB_GPIO_28_LOCK_R {
        TZC_GLB_GPIO_28_LOCK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_gpio_29_lock(&self) -> TZC_GLB_GPIO_29_LOCK_R {
        TZC_GLB_GPIO_29_LOCK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_gpio_30_lock(&self) -> TZC_GLB_GPIO_30_LOCK_R {
        TZC_GLB_GPIO_30_LOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_gpio_31_lock(&self) -> TZC_GLB_GPIO_31_LOCK_R {
        TZC_GLB_GPIO_31_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_0_lock(&mut self) -> TZC_GLB_GPIO_0_LOCK_W<0> {
        TZC_GLB_GPIO_0_LOCK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_1_lock(&mut self) -> TZC_GLB_GPIO_1_LOCK_W<1> {
        TZC_GLB_GPIO_1_LOCK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_2_lock(&mut self) -> TZC_GLB_GPIO_2_LOCK_W<2> {
        TZC_GLB_GPIO_2_LOCK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_3_lock(&mut self) -> TZC_GLB_GPIO_3_LOCK_W<3> {
        TZC_GLB_GPIO_3_LOCK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_4_lock(&mut self) -> TZC_GLB_GPIO_4_LOCK_W<4> {
        TZC_GLB_GPIO_4_LOCK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_5_lock(&mut self) -> TZC_GLB_GPIO_5_LOCK_W<5> {
        TZC_GLB_GPIO_5_LOCK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_6_lock(&mut self) -> TZC_GLB_GPIO_6_LOCK_W<6> {
        TZC_GLB_GPIO_6_LOCK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_7_lock(&mut self) -> TZC_GLB_GPIO_7_LOCK_W<7> {
        TZC_GLB_GPIO_7_LOCK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_8_lock(&mut self) -> TZC_GLB_GPIO_8_LOCK_W<8> {
        TZC_GLB_GPIO_8_LOCK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_9_lock(&mut self) -> TZC_GLB_GPIO_9_LOCK_W<9> {
        TZC_GLB_GPIO_9_LOCK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_10_lock(&mut self) -> TZC_GLB_GPIO_10_LOCK_W<10> {
        TZC_GLB_GPIO_10_LOCK_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_11_lock(&mut self) -> TZC_GLB_GPIO_11_LOCK_W<11> {
        TZC_GLB_GPIO_11_LOCK_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_12_lock(&mut self) -> TZC_GLB_GPIO_12_LOCK_W<12> {
        TZC_GLB_GPIO_12_LOCK_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_13_lock(&mut self) -> TZC_GLB_GPIO_13_LOCK_W<13> {
        TZC_GLB_GPIO_13_LOCK_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_14_lock(&mut self) -> TZC_GLB_GPIO_14_LOCK_W<14> {
        TZC_GLB_GPIO_14_LOCK_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_15_lock(&mut self) -> TZC_GLB_GPIO_15_LOCK_W<15> {
        TZC_GLB_GPIO_15_LOCK_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_16_lock(&mut self) -> TZC_GLB_GPIO_16_LOCK_W<16> {
        TZC_GLB_GPIO_16_LOCK_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_17_lock(&mut self) -> TZC_GLB_GPIO_17_LOCK_W<17> {
        TZC_GLB_GPIO_17_LOCK_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_18_lock(&mut self) -> TZC_GLB_GPIO_18_LOCK_W<18> {
        TZC_GLB_GPIO_18_LOCK_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_19_lock(&mut self) -> TZC_GLB_GPIO_19_LOCK_W<19> {
        TZC_GLB_GPIO_19_LOCK_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_20_lock(&mut self) -> TZC_GLB_GPIO_20_LOCK_W<20> {
        TZC_GLB_GPIO_20_LOCK_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_21_lock(&mut self) -> TZC_GLB_GPIO_21_LOCK_W<21> {
        TZC_GLB_GPIO_21_LOCK_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_22_lock(&mut self) -> TZC_GLB_GPIO_22_LOCK_W<22> {
        TZC_GLB_GPIO_22_LOCK_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_23_lock(&mut self) -> TZC_GLB_GPIO_23_LOCK_W<23> {
        TZC_GLB_GPIO_23_LOCK_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_24_lock(&mut self) -> TZC_GLB_GPIO_24_LOCK_W<24> {
        TZC_GLB_GPIO_24_LOCK_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_25_lock(&mut self) -> TZC_GLB_GPIO_25_LOCK_W<25> {
        TZC_GLB_GPIO_25_LOCK_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_26_lock(&mut self) -> TZC_GLB_GPIO_26_LOCK_W<26> {
        TZC_GLB_GPIO_26_LOCK_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_27_lock(&mut self) -> TZC_GLB_GPIO_27_LOCK_W<27> {
        TZC_GLB_GPIO_27_LOCK_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_28_lock(&mut self) -> TZC_GLB_GPIO_28_LOCK_W<28> {
        TZC_GLB_GPIO_28_LOCK_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_29_lock(&mut self) -> TZC_GLB_GPIO_29_LOCK_W<29> {
        TZC_GLB_GPIO_29_LOCK_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_30_lock(&mut self) -> TZC_GLB_GPIO_30_LOCK_W<30> {
        TZC_GLB_GPIO_30_LOCK_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn tzc_glb_gpio_31_lock(&mut self) -> TZC_GLB_GPIO_31_LOCK_W<31> {
        TZC_GLB_GPIO_31_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_glb_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_2](index.html) module"]
pub struct TZC_GLB_CTRL_2_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_2::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_glb_ctrl_2::W](W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_2 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
