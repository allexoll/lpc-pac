#[doc = "Reader of register SYSRSTSTAT"]
pub type R = crate::R<u32, super::SYSRSTSTAT>;
#[doc = "Writer for register SYSRSTSTAT"]
pub type W = crate::W<u32, super::SYSRSTSTAT>;
#[doc = "Register SYSRSTSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSRSTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "POR reset status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POR_A {
    #[doc = "0: No POR detected"]
    NO_POR_DETECTED = 0,
    #[doc = "1: POR detected. Writing a one clears this reset."]
    POR_DETECTED = 1,
}
impl From<POR_A> for bool {
    #[inline(always)]
    fn from(variant: POR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POR`"]
pub type POR_R = crate::R<bool, POR_A>;
impl POR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_A {
        match self.bits {
            false => POR_A::NO_POR_DETECTED,
            true => POR_A::POR_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_POR_DETECTED`"]
    #[inline(always)]
    pub fn is_no_por_detected(&self) -> bool {
        *self == POR_A::NO_POR_DETECTED
    }
    #[doc = "Checks if the value of the field is `POR_DETECTED`"]
    #[inline(always)]
    pub fn is_por_detected(&self) -> bool {
        *self == POR_A::POR_DETECTED
    }
}
#[doc = "Write proxy for field `POR`"]
pub struct POR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No POR detected"]
    #[inline(always)]
    pub fn no_por_detected(self) -> &'a mut W {
        self.variant(POR_A::NO_POR_DETECTED)
    }
    #[doc = "POR detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn por_detected(self) -> &'a mut W {
        self.variant(POR_A::POR_DETECTED)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Status of the external RESET pin. External reset status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTRST_A {
    #[doc = "0: No reset event detected."]
    NO_RESET_DETECTED = 0,
    #[doc = "1: Reset detected. Writing a one clears this reset."]
    RESET_DETECTED = 1,
}
impl From<EXTRST_A> for bool {
    #[inline(always)]
    fn from(variant: EXTRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTRST`"]
pub type EXTRST_R = crate::R<bool, EXTRST_A>;
impl EXTRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTRST_A {
        match self.bits {
            false => EXTRST_A::NO_RESET_DETECTED,
            true => EXTRST_A::RESET_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_no_reset_detected(&self) -> bool {
        *self == EXTRST_A::NO_RESET_DETECTED
    }
    #[doc = "Checks if the value of the field is `RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_reset_detected(&self) -> bool {
        *self == EXTRST_A::RESET_DETECTED
    }
}
#[doc = "Write proxy for field `EXTRST`"]
pub struct EXTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No reset event detected."]
    #[inline(always)]
    pub fn no_reset_detected(self) -> &'a mut W {
        self.variant(EXTRST_A::NO_RESET_DETECTED)
    }
    #[doc = "Reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn reset_detected(self) -> &'a mut W {
        self.variant(EXTRST_A::RESET_DETECTED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Status of the Watchdog reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDT_A {
    #[doc = "0: No WDT reset detected"]
    NO_WDT_RESET_DETECTED = 0,
    #[doc = "1: WDT reset detected. Writing a one clears this reset."]
    WDT_RESET_DETECTED = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDT`"]
pub type WDT_R = crate::R<bool, WDT_A>;
impl WDT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::NO_WDT_RESET_DETECTED,
            true => WDT_A::WDT_RESET_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_WDT_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_no_wdt_reset_detected(&self) -> bool {
        *self == WDT_A::NO_WDT_RESET_DETECTED
    }
    #[doc = "Checks if the value of the field is `WDT_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_wdt_reset_detected(&self) -> bool {
        *self == WDT_A::WDT_RESET_DETECTED
    }
}
#[doc = "Write proxy for field `WDT`"]
pub struct WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No WDT reset detected"]
    #[inline(always)]
    pub fn no_wdt_reset_detected(self) -> &'a mut W {
        self.variant(WDT_A::NO_WDT_RESET_DETECTED)
    }
    #[doc = "WDT reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn wdt_reset_detected(self) -> &'a mut W {
        self.variant(WDT_A::WDT_RESET_DETECTED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Status of the Brown-out detect reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_A {
    #[doc = "0: No BOD reset detected"]
    NO_BOD_RESET_DETECTED = 0,
    #[doc = "1: BOD reset detected. Writing a one clears this reset."]
    BOD_RESET_DETECTED = 1,
}
impl From<BOD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOD`"]
pub type BOD_R = crate::R<bool, BOD_A>;
impl BOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_A {
        match self.bits {
            false => BOD_A::NO_BOD_RESET_DETECTED,
            true => BOD_A::BOD_RESET_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BOD_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_no_bod_reset_detected(&self) -> bool {
        *self == BOD_A::NO_BOD_RESET_DETECTED
    }
    #[doc = "Checks if the value of the field is `BOD_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_bod_reset_detected(&self) -> bool {
        *self == BOD_A::BOD_RESET_DETECTED
    }
}
#[doc = "Write proxy for field `BOD`"]
pub struct BOD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No BOD reset detected"]
    #[inline(always)]
    pub fn no_bod_reset_detected(self) -> &'a mut W {
        self.variant(BOD_A::NO_BOD_RESET_DETECTED)
    }
    #[doc = "BOD reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn bod_reset_detected(self) -> &'a mut W {
        self.variant(BOD_A::BOD_RESET_DETECTED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Status of the software system reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSRST_A {
    #[doc = "0: No System reset detected"]
    NO_SYSTEM_RESET_DETECTED = 0,
    #[doc = "1: System reset detected. Writing a one clears this reset."]
    SYSTEM_RESET_DETECTED = 1,
}
impl From<SYSRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSRST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SYSRST`"]
pub type SYSRST_R = crate::R<bool, SYSRST_A>;
impl SYSRST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSRST_A {
        match self.bits {
            false => SYSRST_A::NO_SYSTEM_RESET_DETECTED,
            true => SYSRST_A::SYSTEM_RESET_DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYSTEM_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_no_system_reset_detected(&self) -> bool {
        *self == SYSRST_A::NO_SYSTEM_RESET_DETECTED
    }
    #[doc = "Checks if the value of the field is `SYSTEM_RESET_DETECTED`"]
    #[inline(always)]
    pub fn is_system_reset_detected(&self) -> bool {
        *self == SYSRST_A::SYSTEM_RESET_DETECTED
    }
}
#[doc = "Write proxy for field `SYSRST`"]
pub struct SYSRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No System reset detected"]
    #[inline(always)]
    pub fn no_system_reset_detected(self) -> &'a mut W {
        self.variant(SYSRST_A::NO_SYSTEM_RESET_DETECTED)
    }
    #[doc = "System reset detected. Writing a one clears this reset."]
    #[inline(always)]
    pub fn system_reset_detected(self) -> &'a mut W {
        self.variant(SYSRST_A::SYSTEM_RESET_DETECTED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - POR reset status"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of the external RESET pin. External reset status"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of the Watchdog reset"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset"]
    #[inline(always)]
    pub fn bod(&self) -> BOD_R {
        BOD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of the software system reset"]
    #[inline(always)]
    pub fn sysrst(&self) -> SYSRST_R {
        SYSRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POR reset status"]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W {
        POR_W { w: self }
    }
    #[doc = "Bit 1 - Status of the external RESET pin. External reset status"]
    #[inline(always)]
    pub fn extrst(&mut self) -> EXTRST_W {
        EXTRST_W { w: self }
    }
    #[doc = "Bit 2 - Status of the Watchdog reset"]
    #[inline(always)]
    pub fn wdt(&mut self) -> WDT_W {
        WDT_W { w: self }
    }
    #[doc = "Bit 3 - Status of the Brown-out detect reset"]
    #[inline(always)]
    pub fn bod(&mut self) -> BOD_W {
        BOD_W { w: self }
    }
    #[doc = "Bit 4 - Status of the software system reset"]
    #[inline(always)]
    pub fn sysrst(&mut self) -> SYSRST_W {
        SYSRST_W { w: self }
    }
}
