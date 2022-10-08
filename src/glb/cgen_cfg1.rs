#[doc = "Register `cgen_cfg1` reader"]
pub struct R(crate::R<CGEN_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGEN_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGEN_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg1` writer"]
pub struct W(crate::W<CGEN_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG1_SPEC>;
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
impl From<crate::W<CGEN_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGEN_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLB` reader - GLB"]
pub type GLB_R = crate::BitReader<bool>;
#[doc = "Field `GLB` writer - GLB"]
pub type GLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `MIX` reader - MIX"]
pub type MIX_R = crate::BitReader<bool>;
#[doc = "Field `MIX` writer - MIX"]
pub type MIX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `GPIP` reader - gpip (gpadc, gpdac) clock ungate enable"]
pub type GPIP_R = crate::BitReader<bool>;
#[doc = "Field `GPIP` writer - gpip (gpadc, gpdac) clock ungate enable"]
pub type GPIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `SEC_DBG` reader - sec_dbg clock ungate enable"]
pub type SEC_DBG_R = crate::BitReader<bool>;
#[doc = "Field `SEC_DBG` writer - sec_dbg clock ungate enable"]
pub type SEC_DBG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `SEC` reader - sec_eng clock ungate enable"]
pub type SEC_R = crate::BitReader<bool>;
#[doc = "Field `SEC` writer - sec_eng clock ungate enable"]
pub type SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `TZ1` reader - TZC clock ungate enable"]
pub type TZ1_R = crate::BitReader<bool>;
#[doc = "Field `TZ1` writer - TZC clock ungate enable"]
pub type TZ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `TZ2` reader - TZC2 clock ungate enable"]
pub type TZ2_R = crate::BitReader<bool>;
#[doc = "Field `TZ2` writer - TZC2 clock ungate enable"]
pub type TZ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `EFUSE` reader - ef_ctrl clock ungate enable"]
pub type EFUSE_R = crate::BitReader<bool>;
#[doc = "Field `EFUSE` writer - ef_ctrl clock ungate enable"]
pub type EFUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `CCI` reader - CCI (efuse?)"]
pub type CCI_R = crate::BitReader<bool>;
#[doc = "Field `CCI` writer - CCI (efuse?)"]
pub type CCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `L1C` reader - L1C (efuse?)"]
pub type L1C_R = crate::BitReader<bool>;
#[doc = "Field `L1C` writer - L1C (efuse?)"]
pub type L1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `S1A_ALL` reader - S1A_ALL (efuse?)"]
pub type S1A_ALL_R = crate::BitReader<bool>;
#[doc = "Field `S1A_ALL` writer - S1A_ALL (efuse?)"]
pub type S1A_ALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `SFC` reader - sf_ctrl clock ungate enable"]
pub type SFC_R = crate::BitReader<bool>;
#[doc = "Field `SFC` writer - sf_ctrl clock ungate enable"]
pub type SFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `DMA` reader - DMA clock ungate enable"]
pub type DMA_R = crate::BitReader<bool>;
#[doc = "Field `DMA` writer - DMA clock ungate enable"]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `EMAC` reader - EMAC clock ungate enable"]
pub type EMAC_R = crate::BitReader<bool>;
#[doc = "Field `EMAC` writer - EMAC clock ungate enable"]
pub type EMAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `PDS_HBN_AON_HBNRAM` reader - DS_HBN_AON_HBNRAM"]
pub type PDS_HBN_AON_HBNRAM_R = crate::BitReader<bool>;
#[doc = "Field `PDS_HBN_AON_HBNRAM` writer - DS_HBN_AON_HBNRAM"]
pub type PDS_HBN_AON_HBNRAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `RSVD0F` reader - RSVD0F"]
pub type RSVD0F_R = crate::BitReader<bool>;
#[doc = "Field `RSVD0F` writer - RSVD0F"]
pub type RSVD0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `UART0` reader - uart0 clock ungate enable"]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - uart0 clock ungate enable"]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `UART1` reader - uart1 clock ungate enable"]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - uart1 clock ungate enable"]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `SPI` reader - spi clock ungate enable"]
pub type SPI_R = crate::BitReader<bool>;
#[doc = "Field `SPI` writer - spi clock ungate enable"]
pub type SPI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `I2C` reader - i2c clock ungate enable"]
pub type I2C_R = crate::BitReader<bool>;
#[doc = "Field `I2C` writer - i2c clock ungate enable"]
pub type I2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `PWM` reader - pwm clock ungate enable"]
pub type PWM_R = crate::BitReader<bool>;
#[doc = "Field `PWM` writer - pwm clock ungate enable"]
pub type PWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `TMR` reader - timer clock ungate enable"]
pub type TMR_R = crate::BitReader<bool>;
#[doc = "Field `TMR` writer - timer clock ungate enable"]
pub type TMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `IRR` reader - ir_remote clock ungate enable"]
pub type IRR_R = crate::BitReader<bool>;
#[doc = "Field `IRR` writer - ir_remote clock ungate enable"]
pub type IRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `CKS` reader - checksum clock ungate enable"]
pub type CKS_R = crate::BitReader<bool>;
#[doc = "Field `CKS` writer - checksum clock ungate enable"]
pub type CKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `QDEC` reader - qdec0 clock ungate enable"]
pub type QDEC_R = crate::BitReader<bool>;
#[doc = "Field `QDEC` writer - qdec0 clock ungate enable"]
pub type QDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `KYS` reader - KYS"]
pub type KYS_R = crate::BitReader<bool>;
#[doc = "Field `KYS` writer - KYS"]
pub type KYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `I2S` reader - i2s and qdec2 clock ungate enable"]
pub type I2S_R = crate::BitReader<bool>;
#[doc = "Field `I2S` writer - i2s and qdec2 clock ungate enable"]
pub type I2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `RSVD1B` reader - RSVD1B"]
pub type RSVD1B_R = crate::BitReader<bool>;
#[doc = "Field `RSVD1B` writer - RSVD1B"]
pub type RSVD1B_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `USB` reader - usb clock ungate enable"]
pub type USB_R = crate::BitReader<bool>;
#[doc = "Field `USB` writer - usb clock ungate enable"]
pub type USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `CAM` reader - CAM"]
pub type CAM_R = crate::BitReader<bool>;
#[doc = "Field `CAM` writer - CAM"]
pub type CAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `MJPEG` reader - MJPEG"]
pub type MJPEG_R = crate::BitReader<bool>;
#[doc = "Field `MJPEG` writer - MJPEG"]
pub type MJPEG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
#[doc = "Field `MAX` reader - MAX"]
pub type MAX_R = crate::BitReader<bool>;
#[doc = "Field `MAX` writer - MAX"]
pub type MAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGEN_CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GLB"]
    #[inline(always)]
    pub fn glb(&self) -> GLB_R {
        GLB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MIX"]
    #[inline(always)]
    pub fn mix(&self) -> MIX_R {
        MIX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - gpip (gpadc, gpdac) clock ungate enable"]
    #[inline(always)]
    pub fn gpip(&self) -> GPIP_R {
        GPIP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sec_dbg clock ungate enable"]
    #[inline(always)]
    pub fn sec_dbg(&self) -> SEC_DBG_R {
        SEC_DBG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - sec_eng clock ungate enable"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TZC clock ungate enable"]
    #[inline(always)]
    pub fn tz1(&self) -> TZ1_R {
        TZ1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TZC2 clock ungate enable"]
    #[inline(always)]
    pub fn tz2(&self) -> TZ2_R {
        TZ2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ef_ctrl clock ungate enable"]
    #[inline(always)]
    pub fn efuse(&self) -> EFUSE_R {
        EFUSE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CCI (efuse?)"]
    #[inline(always)]
    pub fn cci(&self) -> CCI_R {
        CCI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - L1C (efuse?)"]
    #[inline(always)]
    pub fn l1c(&self) -> L1C_R {
        L1C_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - S1A_ALL (efuse?)"]
    #[inline(always)]
    pub fn s1a_all(&self) -> S1A_ALL_R {
        S1A_ALL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - sf_ctrl clock ungate enable"]
    #[inline(always)]
    pub fn sfc(&self) -> SFC_R {
        SFC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA clock ungate enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EMAC clock ungate enable"]
    #[inline(always)]
    pub fn emac(&self) -> EMAC_R {
        EMAC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DS_HBN_AON_HBNRAM"]
    #[inline(always)]
    pub fn pds_hbn_aon_hbnram(&self) -> PDS_HBN_AON_HBNRAM_R {
        PDS_HBN_AON_HBNRAM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RSVD0F"]
    #[inline(always)]
    pub fn rsvd0f(&self) -> RSVD0F_R {
        RSVD0F_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - uart0 clock ungate enable"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - uart1 clock ungate enable"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - spi clock ungate enable"]
    #[inline(always)]
    pub fn spi(&self) -> SPI_R {
        SPI_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - i2c clock ungate enable"]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - pwm clock ungate enable"]
    #[inline(always)]
    pub fn pwm(&self) -> PWM_R {
        PWM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - timer clock ungate enable"]
    #[inline(always)]
    pub fn tmr(&self) -> TMR_R {
        TMR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ir_remote clock ungate enable"]
    #[inline(always)]
    pub fn irr(&self) -> IRR_R {
        IRR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - checksum clock ungate enable"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - qdec0 clock ungate enable"]
    #[inline(always)]
    pub fn qdec(&self) -> QDEC_R {
        QDEC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - KYS"]
    #[inline(always)]
    pub fn kys(&self) -> KYS_R {
        KYS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - i2s and qdec2 clock ungate enable"]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RSVD1B"]
    #[inline(always)]
    pub fn rsvd1b(&self) -> RSVD1B_R {
        RSVD1B_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - usb clock ungate enable"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CAM"]
    #[inline(always)]
    pub fn cam(&self) -> CAM_R {
        CAM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MJPEG"]
    #[inline(always)]
    pub fn mjpeg(&self) -> MJPEG_R {
        MJPEG_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MAX"]
    #[inline(always)]
    pub fn max(&self) -> MAX_R {
        MAX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GLB"]
    #[inline(always)]
    pub fn glb(&mut self) -> GLB_W<0> {
        GLB_W::new(self)
    }
    #[doc = "Bit 1 - MIX"]
    #[inline(always)]
    pub fn mix(&mut self) -> MIX_W<1> {
        MIX_W::new(self)
    }
    #[doc = "Bit 2 - gpip (gpadc, gpdac) clock ungate enable"]
    #[inline(always)]
    pub fn gpip(&mut self) -> GPIP_W<2> {
        GPIP_W::new(self)
    }
    #[doc = "Bit 3 - sec_dbg clock ungate enable"]
    #[inline(always)]
    pub fn sec_dbg(&mut self) -> SEC_DBG_W<3> {
        SEC_DBG_W::new(self)
    }
    #[doc = "Bit 4 - sec_eng clock ungate enable"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<4> {
        SEC_W::new(self)
    }
    #[doc = "Bit 5 - TZC clock ungate enable"]
    #[inline(always)]
    pub fn tz1(&mut self) -> TZ1_W<5> {
        TZ1_W::new(self)
    }
    #[doc = "Bit 6 - TZC2 clock ungate enable"]
    #[inline(always)]
    pub fn tz2(&mut self) -> TZ2_W<6> {
        TZ2_W::new(self)
    }
    #[doc = "Bit 7 - ef_ctrl clock ungate enable"]
    #[inline(always)]
    pub fn efuse(&mut self) -> EFUSE_W<7> {
        EFUSE_W::new(self)
    }
    #[doc = "Bit 8 - CCI (efuse?)"]
    #[inline(always)]
    pub fn cci(&mut self) -> CCI_W<8> {
        CCI_W::new(self)
    }
    #[doc = "Bit 9 - L1C (efuse?)"]
    #[inline(always)]
    pub fn l1c(&mut self) -> L1C_W<9> {
        L1C_W::new(self)
    }
    #[doc = "Bit 10 - S1A_ALL (efuse?)"]
    #[inline(always)]
    pub fn s1a_all(&mut self) -> S1A_ALL_W<10> {
        S1A_ALL_W::new(self)
    }
    #[doc = "Bit 11 - sf_ctrl clock ungate enable"]
    #[inline(always)]
    pub fn sfc(&mut self) -> SFC_W<11> {
        SFC_W::new(self)
    }
    #[doc = "Bit 12 - DMA clock ungate enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<12> {
        DMA_W::new(self)
    }
    #[doc = "Bit 13 - EMAC clock ungate enable"]
    #[inline(always)]
    pub fn emac(&mut self) -> EMAC_W<13> {
        EMAC_W::new(self)
    }
    #[doc = "Bit 14 - DS_HBN_AON_HBNRAM"]
    #[inline(always)]
    pub fn pds_hbn_aon_hbnram(&mut self) -> PDS_HBN_AON_HBNRAM_W<14> {
        PDS_HBN_AON_HBNRAM_W::new(self)
    }
    #[doc = "Bit 15 - RSVD0F"]
    #[inline(always)]
    pub fn rsvd0f(&mut self) -> RSVD0F_W<15> {
        RSVD0F_W::new(self)
    }
    #[doc = "Bit 16 - uart0 clock ungate enable"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W<16> {
        UART0_W::new(self)
    }
    #[doc = "Bit 17 - uart1 clock ungate enable"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W<17> {
        UART1_W::new(self)
    }
    #[doc = "Bit 18 - spi clock ungate enable"]
    #[inline(always)]
    pub fn spi(&mut self) -> SPI_W<18> {
        SPI_W::new(self)
    }
    #[doc = "Bit 19 - i2c clock ungate enable"]
    #[inline(always)]
    pub fn i2c(&mut self) -> I2C_W<19> {
        I2C_W::new(self)
    }
    #[doc = "Bit 20 - pwm clock ungate enable"]
    #[inline(always)]
    pub fn pwm(&mut self) -> PWM_W<20> {
        PWM_W::new(self)
    }
    #[doc = "Bit 21 - timer clock ungate enable"]
    #[inline(always)]
    pub fn tmr(&mut self) -> TMR_W<21> {
        TMR_W::new(self)
    }
    #[doc = "Bit 22 - ir_remote clock ungate enable"]
    #[inline(always)]
    pub fn irr(&mut self) -> IRR_W<22> {
        IRR_W::new(self)
    }
    #[doc = "Bit 23 - checksum clock ungate enable"]
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W<23> {
        CKS_W::new(self)
    }
    #[doc = "Bit 24 - qdec0 clock ungate enable"]
    #[inline(always)]
    pub fn qdec(&mut self) -> QDEC_W<24> {
        QDEC_W::new(self)
    }
    #[doc = "Bit 25 - KYS"]
    #[inline(always)]
    pub fn kys(&mut self) -> KYS_W<25> {
        KYS_W::new(self)
    }
    #[doc = "Bit 26 - i2s and qdec2 clock ungate enable"]
    #[inline(always)]
    pub fn i2s(&mut self) -> I2S_W<26> {
        I2S_W::new(self)
    }
    #[doc = "Bit 27 - RSVD1B"]
    #[inline(always)]
    pub fn rsvd1b(&mut self) -> RSVD1B_W<27> {
        RSVD1B_W::new(self)
    }
    #[doc = "Bit 28 - usb clock ungate enable"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W<28> {
        USB_W::new(self)
    }
    #[doc = "Bit 29 - CAM"]
    #[inline(always)]
    pub fn cam(&mut self) -> CAM_W<29> {
        CAM_W::new(self)
    }
    #[doc = "Bit 30 - MJPEG"]
    #[inline(always)]
    pub fn mjpeg(&mut self) -> MJPEG_W<30> {
        MJPEG_W::new(self)
    }
    #[doc = "Bit 31 - MAX"]
    #[inline(always)]
    pub fn max(&mut self) -> MAX_W<31> {
        MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg1](index.html) module"]
pub struct CGEN_CFG1_SPEC;
impl crate::RegisterSpec for CGEN_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg1::R](R) reader structure"]
impl crate::Readable for CGEN_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg1::W](W) writer structure"]
impl crate::Writable for CGEN_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cgen_cfg1 to value 0"]
impl crate::Resettable for CGEN_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
