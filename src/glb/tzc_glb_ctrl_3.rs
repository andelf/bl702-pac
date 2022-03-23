#[doc = "Register `tzc_glb_ctrl_3` reader"]
pub struct R(crate::R<TZC_GLB_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GLB_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZC_GLB_CTRL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZC_GLB_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_glb_ctrl_3` writer"]
pub struct W(crate::W<TZC_GLB_CTRL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_GLB_CTRL_3_SPEC>;
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
impl From<crate::W<TZC_GLB_CTRL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZC_GLB_CTRL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_glb_gpio_37_lock` reader - "]
pub struct TZC_GLB_GPIO_37_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_37_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_37_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_37_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_37_lock` writer - "]
pub struct TZC_GLB_GPIO_37_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_37_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_36_lock` reader - "]
pub struct TZC_GLB_GPIO_36_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_36_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_36_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_36_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_36_lock` writer - "]
pub struct TZC_GLB_GPIO_36_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_36_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_35_lock` reader - "]
pub struct TZC_GLB_GPIO_35_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_35_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_35_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_35_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_35_lock` writer - "]
pub struct TZC_GLB_GPIO_35_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_35_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_34_lock` reader - "]
pub struct TZC_GLB_GPIO_34_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_34_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_34_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_34_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_34_lock` writer - "]
pub struct TZC_GLB_GPIO_34_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_34_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_33_lock` reader - "]
pub struct TZC_GLB_GPIO_33_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_33_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_33_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_33_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_33_lock` writer - "]
pub struct TZC_GLB_GPIO_33_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_33_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_32_lock` reader - "]
pub struct TZC_GLB_GPIO_32_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_32_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_32_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_32_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_32_lock` writer - "]
pub struct TZC_GLB_GPIO_32_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_32_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_gpio_37_lock(&self) -> TZC_GLB_GPIO_37_LOCK_R {
        TZC_GLB_GPIO_37_LOCK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_gpio_36_lock(&self) -> TZC_GLB_GPIO_36_LOCK_R {
        TZC_GLB_GPIO_36_LOCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_gpio_35_lock(&self) -> TZC_GLB_GPIO_35_LOCK_R {
        TZC_GLB_GPIO_35_LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_gpio_34_lock(&self) -> TZC_GLB_GPIO_34_LOCK_R {
        TZC_GLB_GPIO_34_LOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_gpio_33_lock(&self) -> TZC_GLB_GPIO_33_LOCK_R {
        TZC_GLB_GPIO_33_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_gpio_32_lock(&self) -> TZC_GLB_GPIO_32_LOCK_R {
        TZC_GLB_GPIO_32_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_gpio_37_lock(&mut self) -> TZC_GLB_GPIO_37_LOCK_W {
        TZC_GLB_GPIO_37_LOCK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_gpio_36_lock(&mut self) -> TZC_GLB_GPIO_36_LOCK_W {
        TZC_GLB_GPIO_36_LOCK_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_gpio_35_lock(&mut self) -> TZC_GLB_GPIO_35_LOCK_W {
        TZC_GLB_GPIO_35_LOCK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_gpio_34_lock(&mut self) -> TZC_GLB_GPIO_34_LOCK_W {
        TZC_GLB_GPIO_34_LOCK_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_gpio_33_lock(&mut self) -> TZC_GLB_GPIO_33_LOCK_W {
        TZC_GLB_GPIO_33_LOCK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_gpio_32_lock(&mut self) -> TZC_GLB_GPIO_32_LOCK_W {
        TZC_GLB_GPIO_32_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_glb_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_3](index.html) module"]
pub struct TZC_GLB_CTRL_3_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_3::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_glb_ctrl_3::W](W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_3 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
