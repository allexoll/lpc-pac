#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sequence A interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQA_INTEN_A {
    #[doc = "0: Disabled. The sequence A interrupt/DMA trigger is disabled."]
    DISABLED,
    #[doc = "1: Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    ENABLED,
}
impl From<SEQA_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SEQA_INTEN_A) -> Self {
        match variant {
            SEQA_INTEN_A::DISABLED => false,
            SEQA_INTEN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `SEQA_INTEN`"]
pub type SEQA_INTEN_R = crate::R<bool, SEQA_INTEN_A>;
impl SEQA_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQA_INTEN_A {
        match self.bits {
            false => SEQA_INTEN_A::DISABLED,
            true => SEQA_INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQA_INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQA_INTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `SEQA_INTEN`"]
pub struct SEQA_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQA_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQA_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The sequence A interrupt/DMA trigger is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQA_INTEN_A::DISABLED)
    }
    #[doc = "Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQA_INTEN_A::ENABLED)
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
#[doc = "Sequence B interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQB_INTEN_A {
    #[doc = "0: Disabled. The sequence B interrupt/DMA trigger is disabled."]
    DISABLED,
    #[doc = "1: Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    ENABLED,
}
impl From<SEQB_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SEQB_INTEN_A) -> Self {
        match variant {
            SEQB_INTEN_A::DISABLED => false,
            SEQB_INTEN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `SEQB_INTEN`"]
pub type SEQB_INTEN_R = crate::R<bool, SEQB_INTEN_A>;
impl SEQB_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQB_INTEN_A {
        match self.bits {
            false => SEQB_INTEN_A::DISABLED,
            true => SEQB_INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQB_INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQB_INTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `SEQB_INTEN`"]
pub struct SEQB_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQB_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQB_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The sequence B interrupt/DMA trigger is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQB_INTEN_A::DISABLED)
    }
    #[doc = "Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQB_INTEN_A::ENABLED)
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
#[doc = "Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_INTEN_A {
    #[doc = "0: Disabled. The overrun interrupt is disabled."]
    DISABLED,
    #[doc = "1: Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
    ENABLED,
}
impl From<OVR_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_INTEN_A) -> Self {
        match variant {
            OVR_INTEN_A::DISABLED => false,
            OVR_INTEN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `OVR_INTEN`"]
pub type OVR_INTEN_R = crate::R<bool, OVR_INTEN_A>;
impl OVR_INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_INTEN_A {
        match self.bits {
            false => OVR_INTEN_A::DISABLED,
            true => OVR_INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVR_INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVR_INTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `OVR_INTEN`"]
pub struct OVR_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVR_INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The overrun interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVR_INTEN_A::DISABLED)
    }
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVR_INTEN_A::ENABLED)
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
#[doc = "Threshold comparison interrupt enable for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCMPINTEN0_A {
    #[doc = "0: Disabled."]
    DISABLED,
    #[doc = "1: Outside threshold."]
    OUTSIDE_THRESHOLD,
    #[doc = "2: Crossing threshold."]
    CROSSING_THRESHOLD,
}
impl From<ADCMPINTEN0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCMPINTEN0_A) -> Self {
        match variant {
            ADCMPINTEN0_A::DISABLED => 0,
            ADCMPINTEN0_A::OUTSIDE_THRESHOLD => 1,
            ADCMPINTEN0_A::CROSSING_THRESHOLD => 2,
        }
    }
}
#[doc = "Reader of field `ADCMPINTEN0`"]
pub type ADCMPINTEN0_R = crate::R<u8, ADCMPINTEN0_A>;
impl ADCMPINTEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADCMPINTEN0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADCMPINTEN0_A::DISABLED),
            1 => Val(ADCMPINTEN0_A::OUTSIDE_THRESHOLD),
            2 => Val(ADCMPINTEN0_A::CROSSING_THRESHOLD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCMPINTEN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        *self == ADCMPINTEN0_A::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        *self == ADCMPINTEN0_A::CROSSING_THRESHOLD
    }
}
#[doc = "Write proxy for field `ADCMPINTEN0`"]
pub struct ADCMPINTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN0_A::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0_A::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0_A::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN1`"]
pub type ADCMPINTEN1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN1`"]
pub struct ADCMPINTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN2`"]
pub type ADCMPINTEN2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN2`"]
pub struct ADCMPINTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN3`"]
pub type ADCMPINTEN3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN3`"]
pub struct ADCMPINTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN4`"]
pub type ADCMPINTEN4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN4`"]
pub struct ADCMPINTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN5`"]
pub type ADCMPINTEN5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN5`"]
pub struct ADCMPINTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN6`"]
pub type ADCMPINTEN6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN6`"]
pub struct ADCMPINTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN7`"]
pub type ADCMPINTEN7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN7`"]
pub struct ADCMPINTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN8`"]
pub type ADCMPINTEN8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN8`"]
pub struct ADCMPINTEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | (((value as u32) & 0x03) << 19);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN9`"]
pub type ADCMPINTEN9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN9`"]
pub struct ADCMPINTEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN10`"]
pub type ADCMPINTEN10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN10`"]
pub struct ADCMPINTEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = "Reader of field `ADCMPINTEN11`"]
pub type ADCMPINTEN11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCMPINTEN11`"]
pub struct ADCMPINTEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline(always)]
    pub fn seqa_inten(&self) -> SEQA_INTEN_R {
        SEQA_INTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline(always)]
    pub fn seqb_inten(&self) -> SEQB_INTEN_R {
        SEQB_INTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline(always)]
    pub fn ovr_inten(&self) -> OVR_INTEN_R {
        OVR_INTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable for channel 0."]
    #[inline(always)]
    pub fn adcmpinten0(&self) -> ADCMPINTEN0_R {
        ADCMPINTEN0_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten1(&self) -> ADCMPINTEN1_R {
        ADCMPINTEN1_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 7:8 - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten2(&self) -> ADCMPINTEN2_R {
        ADCMPINTEN2_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 9:10 - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten3(&self) -> ADCMPINTEN3_R {
        ADCMPINTEN3_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten4(&self) -> ADCMPINTEN4_R {
        ADCMPINTEN4_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten5(&self) -> ADCMPINTEN5_R {
        ADCMPINTEN5_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 15:16 - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten6(&self) -> ADCMPINTEN6_R {
        ADCMPINTEN6_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 17:18 - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten7(&self) -> ADCMPINTEN7_R {
        ADCMPINTEN7_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:20 - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten8(&self) -> ADCMPINTEN8_R {
        ADCMPINTEN8_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 21:22 - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten9(&self) -> ADCMPINTEN9_R {
        ADCMPINTEN9_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 23:24 - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten10(&self) -> ADCMPINTEN10_R {
        ADCMPINTEN10_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 25:26 - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten11(&self) -> ADCMPINTEN11_R {
        ADCMPINTEN11_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline(always)]
    pub fn seqa_inten(&mut self) -> SEQA_INTEN_W {
        SEQA_INTEN_W { w: self }
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline(always)]
    pub fn seqb_inten(&mut self) -> SEQB_INTEN_W {
        SEQB_INTEN_W { w: self }
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline(always)]
    pub fn ovr_inten(&mut self) -> OVR_INTEN_W {
        OVR_INTEN_W { w: self }
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable for channel 0."]
    #[inline(always)]
    pub fn adcmpinten0(&mut self) -> ADCMPINTEN0_W {
        ADCMPINTEN0_W { w: self }
    }
    #[doc = "Bits 5:6 - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten1(&mut self) -> ADCMPINTEN1_W {
        ADCMPINTEN1_W { w: self }
    }
    #[doc = "Bits 7:8 - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten2(&mut self) -> ADCMPINTEN2_W {
        ADCMPINTEN2_W { w: self }
    }
    #[doc = "Bits 9:10 - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten3(&mut self) -> ADCMPINTEN3_W {
        ADCMPINTEN3_W { w: self }
    }
    #[doc = "Bits 11:12 - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten4(&mut self) -> ADCMPINTEN4_W {
        ADCMPINTEN4_W { w: self }
    }
    #[doc = "Bits 13:14 - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten5(&mut self) -> ADCMPINTEN5_W {
        ADCMPINTEN5_W { w: self }
    }
    #[doc = "Bits 15:16 - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten6(&mut self) -> ADCMPINTEN6_W {
        ADCMPINTEN6_W { w: self }
    }
    #[doc = "Bits 17:18 - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten7(&mut self) -> ADCMPINTEN7_W {
        ADCMPINTEN7_W { w: self }
    }
    #[doc = "Bits 19:20 - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten8(&mut self) -> ADCMPINTEN8_W {
        ADCMPINTEN8_W { w: self }
    }
    #[doc = "Bits 21:22 - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten9(&mut self) -> ADCMPINTEN9_W {
        ADCMPINTEN9_W { w: self }
    }
    #[doc = "Bits 23:24 - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten10(&mut self) -> ADCMPINTEN10_W {
        ADCMPINTEN10_W { w: self }
    }
    #[doc = "Bits 25:26 - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten11(&mut self) -> ADCMPINTEN11_W {
        ADCMPINTEN11_W { w: self }
    }
}
