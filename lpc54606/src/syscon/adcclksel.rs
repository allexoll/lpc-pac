#[doc = "Reader of register ADCCLKSEL"]
pub type R = crate::R<u32, super::ADCCLKSEL>;
#[doc = "Writer for register ADCCLKSEL"]
pub type W = crate::W<u32, super::ADCCLKSEL>;
#[doc = "Register ADCCLKSEL `reset()`'s with value 0x07"]
impl crate::ResetValue for super::ADCCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "ADC clock source selection\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: FRO 96 or 48 MHz (fro_hf)"]
    FRO_HF = 0,
    #[doc = "1: System PLL output (pll_clk)"]
    SYSTEM_PLL_OUTPUT = 1,
    #[doc = "2: USB PLL clock (usb_pll_clk)"]
    USB_PLL_CLOCK = 2,
    #[doc = "3: Audio PLL clock (audio_pll_clk)"]
    AUDIO_PLL_CLOCK = 3,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    NONE = 7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL_A::FRO_HF),
            1 => Val(SEL_A::SYSTEM_PLL_OUTPUT),
            2 => Val(SEL_A::USB_PLL_CLOCK),
            3 => Val(SEL_A::AUDIO_PLL_CLOCK),
            7 => Val(SEL_A::NONE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FRO_HF`"]
    #[inline(always)]
    pub fn is_fro_hf(&self) -> bool {
        *self == SEL_A::FRO_HF
    }
    #[doc = "Checks if the value of the field is `SYSTEM_PLL_OUTPUT`"]
    #[inline(always)]
    pub fn is_system_pll_output(&self) -> bool {
        *self == SEL_A::SYSTEM_PLL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `USB_PLL_CLOCK`"]
    #[inline(always)]
    pub fn is_usb_pll_clock(&self) -> bool {
        *self == SEL_A::USB_PLL_CLOCK
    }
    #[doc = "Checks if the value of the field is `AUDIO_PLL_CLOCK`"]
    #[inline(always)]
    pub fn is_audio_pll_clock(&self) -> bool {
        *self == SEL_A::AUDIO_PLL_CLOCK
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SEL_A::NONE
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "FRO 96 or 48 MHz (fro_hf)"]
    #[inline(always)]
    pub fn fro_hf(self) -> &'a mut W {
        self.variant(SEL_A::FRO_HF)
    }
    #[doc = "System PLL output (pll_clk)"]
    #[inline(always)]
    pub fn system_pll_output(self) -> &'a mut W {
        self.variant(SEL_A::SYSTEM_PLL_OUTPUT)
    }
    #[doc = "USB PLL clock (usb_pll_clk)"]
    #[inline(always)]
    pub fn usb_pll_clock(self) -> &'a mut W {
        self.variant(SEL_A::USB_PLL_CLOCK)
    }
    #[doc = "Audio PLL clock (audio_pll_clk)"]
    #[inline(always)]
    pub fn audio_pll_clock(self) -> &'a mut W {
        self.variant(SEL_A::AUDIO_PLL_CLOCK)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SEL_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - ADC clock source selection"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ADC clock source selection"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
