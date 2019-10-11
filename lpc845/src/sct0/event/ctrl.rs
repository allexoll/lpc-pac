#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCHSEL`"]
pub type MATCHSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MATCHSEL`"]
pub struct MATCHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Select L/H counter. Do not set this bit if UNIFY = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HEVENT_A {
    #[doc = "0: Selects the L state and the L match register selected by MATCHSEL."]
    L_COUNTER,
    #[doc = "1: Selects the H state and the H match register selected by MATCHSEL."]
    H_COUNTER,
}
impl From<HEVENT_A> for bool {
    #[inline(always)]
    fn from(variant: HEVENT_A) -> Self {
        match variant {
            HEVENT_A::L_COUNTER => false,
            HEVENT_A::H_COUNTER => true,
        }
    }
}
#[doc = "Reader of field `HEVENT`"]
pub type HEVENT_R = crate::R<bool, HEVENT_A>;
impl HEVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HEVENT_A {
        match self.bits {
            false => HEVENT_A::L_COUNTER,
            true => HEVENT_A::H_COUNTER,
        }
    }
    #[doc = "Checks if the value of the field is `L_COUNTER`"]
    #[inline(always)]
    pub fn is_l_counter(&self) -> bool {
        *self == HEVENT_A::L_COUNTER
    }
    #[doc = "Checks if the value of the field is `H_COUNTER`"]
    #[inline(always)]
    pub fn is_h_counter(&self) -> bool {
        *self == HEVENT_A::H_COUNTER
    }
}
#[doc = "Write proxy for field `HEVENT`"]
pub struct HEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> HEVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HEVENT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects the L state and the L match register selected by MATCHSEL."]
    #[inline(always)]
    pub fn l_counter(self) -> &'a mut W {
        self.variant(HEVENT_A::L_COUNTER)
    }
    #[doc = "Selects the H state and the H match register selected by MATCHSEL."]
    #[inline(always)]
    pub fn h_counter(self) -> &'a mut W {
        self.variant(HEVENT_A::H_COUNTER)
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
#[doc = "Input/output select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTSEL_A {
    #[doc = "0: Selects the inputs selected by IOSEL."]
    INPUT,
    #[doc = "1: Selects the outputs selected by IOSEL."]
    OUTPUT,
}
impl From<OUTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OUTSEL_A) -> Self {
        match variant {
            OUTSEL_A::INPUT => false,
            OUTSEL_A::OUTPUT => true,
        }
    }
}
#[doc = "Reader of field `OUTSEL`"]
pub type OUTSEL_R = crate::R<bool, OUTSEL_A>;
impl OUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTSEL_A {
        match self.bits {
            false => OUTSEL_A::INPUT,
            true => OUTSEL_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == OUTSEL_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == OUTSEL_A::OUTPUT
    }
}
#[doc = "Write proxy for field `OUTSEL`"]
pub struct OUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects the inputs selected by IOSEL."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(OUTSEL_A::INPUT)
    }
    #[doc = "Selects the outputs selected by IOSEL."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(OUTSEL_A::OUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `IOSEL`"]
pub type IOSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IOSEL`"]
pub struct IOSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IOSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCOND_A {
    #[doc = "0: LOW"]
    LOW,
    #[doc = "1: Rise"]
    RISE,
    #[doc = "2: Fall"]
    FALL,
    #[doc = "3: HIGH"]
    HIGH,
}
impl From<IOCOND_A> for u8 {
    #[inline(always)]
    fn from(variant: IOCOND_A) -> Self {
        match variant {
            IOCOND_A::LOW => 0,
            IOCOND_A::RISE => 1,
            IOCOND_A::FALL => 2,
            IOCOND_A::HIGH => 3,
        }
    }
}
#[doc = "Reader of field `IOCOND`"]
pub type IOCOND_R = crate::R<u8, IOCOND_A>;
impl IOCOND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCOND_A {
        match self.bits {
            0 => IOCOND_A::LOW,
            1 => IOCOND_A::RISE,
            2 => IOCOND_A::FALL,
            3 => IOCOND_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IOCOND_A::LOW
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == IOCOND_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == IOCOND_A::FALL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IOCOND_A::HIGH
    }
}
#[doc = "Write proxy for field `IOCOND`"]
pub struct IOCOND_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCOND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCOND_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LOW"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IOCOND_A::LOW)
    }
    #[doc = "Rise"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(IOCOND_A::RISE)
    }
    #[doc = "Fall"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(IOCOND_A::FALL)
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IOCOND_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Selects how the specified match and I/O condition are used and combined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBMODE_A {
    #[doc = "0: OR. The event occurs when either the specified match or I/O condition occurs."]
    OR,
    #[doc = "1: MATCH. Uses the specified match only."]
    MATCH,
    #[doc = "2: IO. Uses the specified I/O condition only."]
    IO,
    #[doc = "3: AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    AND,
}
impl From<COMBMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: COMBMODE_A) -> Self {
        match variant {
            COMBMODE_A::OR => 0,
            COMBMODE_A::MATCH => 1,
            COMBMODE_A::IO => 2,
            COMBMODE_A::AND => 3,
        }
    }
}
#[doc = "Reader of field `COMBMODE`"]
pub type COMBMODE_R = crate::R<u8, COMBMODE_A>;
impl COMBMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMBMODE_A {
        match self.bits {
            0 => COMBMODE_A::OR,
            1 => COMBMODE_A::MATCH,
            2 => COMBMODE_A::IO,
            3 => COMBMODE_A::AND,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OR`"]
    #[inline(always)]
    pub fn is_or(&self) -> bool {
        *self == COMBMODE_A::OR
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match_(&self) -> bool {
        *self == COMBMODE_A::MATCH
    }
    #[doc = "Checks if the value of the field is `IO`"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == COMBMODE_A::IO
    }
    #[doc = "Checks if the value of the field is `AND`"]
    #[inline(always)]
    pub fn is_and(&self) -> bool {
        *self == COMBMODE_A::AND
    }
}
#[doc = "Write proxy for field `COMBMODE`"]
pub struct COMBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMBMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "OR. The event occurs when either the specified match or I/O condition occurs."]
    #[inline(always)]
    pub fn or(self) -> &'a mut W {
        self.variant(COMBMODE_A::OR)
    }
    #[doc = "MATCH. Uses the specified match only."]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(COMBMODE_A::MATCH)
    }
    #[doc = "IO. Uses the specified I/O condition only."]
    #[inline(always)]
    pub fn io(self) -> &'a mut W {
        self.variant(COMBMODE_A::IO)
    }
    #[doc = "AND. The event occurs when the specified match and I/O condition occur simultaneously."]
    #[inline(always)]
    pub fn and(self) -> &'a mut W {
        self.variant(COMBMODE_A::AND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATELD_A {
    #[doc = "0: STATEV value is added into STATE (the carry-out is ignored)."]
    ADD,
    #[doc = "1: STATEV value is loaded into STATE."]
    LOAD,
}
impl From<STATELD_A> for bool {
    #[inline(always)]
    fn from(variant: STATELD_A) -> Self {
        match variant {
            STATELD_A::ADD => false,
            STATELD_A::LOAD => true,
        }
    }
}
#[doc = "Reader of field `STATELD`"]
pub type STATELD_R = crate::R<bool, STATELD_A>;
impl STATELD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATELD_A {
        match self.bits {
            false => STATELD_A::ADD,
            true => STATELD_A::LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `ADD`"]
    #[inline(always)]
    pub fn is_add(&self) -> bool {
        *self == STATELD_A::ADD
    }
    #[doc = "Checks if the value of the field is `LOAD`"]
    #[inline(always)]
    pub fn is_load(&self) -> bool {
        *self == STATELD_A::LOAD
    }
}
#[doc = "Write proxy for field `STATELD`"]
pub struct STATELD_W<'a> {
    w: &'a mut W,
}
impl<'a> STATELD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATELD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "STATEV value is added into STATE (the carry-out is ignored)."]
    #[inline(always)]
    pub fn add(self) -> &'a mut W {
        self.variant(STATELD_A::ADD)
    }
    #[doc = "STATEV value is loaded into STATE."]
    #[inline(always)]
    pub fn load(self) -> &'a mut W {
        self.variant(STATELD_A::LOAD)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `STATEV`"]
pub type STATEV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATEV`"]
pub struct STATEV_W<'a> {
    w: &'a mut W,
}
impl<'a> STATEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `MATCHMEM`"]
pub type MATCHMEM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MATCHMEM`"]
pub struct MATCHMEM_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHMEM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTION_A {
    #[doc = "0: Direction independent. This event is triggered regardless of the count direction."]
    DIRECTION_INDEPENDENT,
    #[doc = "1: Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    COUNTING_UP,
    #[doc = "2: Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    COUNTING_DOWN,
}
impl From<DIRECTION_A> for u8 {
    #[inline(always)]
    fn from(variant: DIRECTION_A) -> Self {
        match variant {
            DIRECTION_A::DIRECTION_INDEPENDENT => 0,
            DIRECTION_A::COUNTING_UP => 1,
            DIRECTION_A::COUNTING_DOWN => 2,
        }
    }
}
#[doc = "Reader of field `DIRECTION`"]
pub type DIRECTION_R = crate::R<u8, DIRECTION_A>;
impl DIRECTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIRECTION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIRECTION_A::DIRECTION_INDEPENDENT),
            1 => Val(DIRECTION_A::COUNTING_UP),
            2 => Val(DIRECTION_A::COUNTING_DOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIRECTION_INDEPENDENT`"]
    #[inline(always)]
    pub fn is_direction_independent(&self) -> bool {
        *self == DIRECTION_A::DIRECTION_INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `COUNTING_UP`"]
    #[inline(always)]
    pub fn is_counting_up(&self) -> bool {
        *self == DIRECTION_A::COUNTING_UP
    }
    #[doc = "Checks if the value of the field is `COUNTING_DOWN`"]
    #[inline(always)]
    pub fn is_counting_down(&self) -> bool {
        *self == DIRECTION_A::COUNTING_DOWN
    }
}
#[doc = "Write proxy for field `DIRECTION`"]
pub struct DIRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRECTION_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Direction independent. This event is triggered regardless of the count direction."]
    #[inline(always)]
    pub fn direction_independent(self) -> &'a mut W {
        self.variant(DIRECTION_A::DIRECTION_INDEPENDENT)
    }
    #[doc = "Counting up. This event is triggered only during up-counting when BIDIR = 1."]
    #[inline(always)]
    pub fn counting_up(self) -> &'a mut W {
        self.variant(DIRECTION_A::COUNTING_UP)
    }
    #[doc = "Counting down. This event is triggered only during down-counting when BIDIR = 1."]
    #[inline(always)]
    pub fn counting_down(self) -> &'a mut W {
        self.variant(DIRECTION_A::COUNTING_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline(always)]
    pub fn matchsel(&self) -> MATCHSEL_R {
        MATCHSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline(always)]
    pub fn hevent(&self) -> HEVENT_R {
        HEVENT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Input/output select"]
    #[inline(always)]
    pub fn outsel(&self) -> OUTSEL_R {
        OUTSEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:9 - Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline(always)]
    pub fn iosel(&self) -> IOSEL_R {
        IOSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[inline(always)]
    pub fn iocond(&self) -> IOCOND_R {
        IOCOND_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Selects how the specified match and I/O condition are used and combined."]
    #[inline(always)]
    pub fn combmode(&self) -> COMBMODE_R {
        COMBMODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline(always)]
    pub fn stateld(&self) -> STATELD_R {
        STATELD_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:19 - This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline(always)]
    pub fn statev(&self) -> STATEV_R {
        STATEV_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline(always)]
    pub fn matchmem(&self) -> MATCHMEM_R {
        MATCHMEM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 21) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the Match register associated with this event (if any). A match can occur only when the counter selected by the HEVENT bit is running."]
    #[inline(always)]
    pub fn matchsel(&mut self) -> MATCHSEL_W {
        MATCHSEL_W { w: self }
    }
    #[doc = "Bit 4 - Select L/H counter. Do not set this bit if UNIFY = 1."]
    #[inline(always)]
    pub fn hevent(&mut self) -> HEVENT_W {
        HEVENT_W { w: self }
    }
    #[doc = "Bit 5 - Input/output select"]
    #[inline(always)]
    pub fn outsel(&mut self) -> OUTSEL_W {
        OUTSEL_W { w: self }
    }
    #[doc = "Bits 6:9 - Selects the input or output signal number associated with this event (if any). Do not select an input in this register if CKMODE is 1x. In this case the clock input is an implicit ingredient of every event."]
    #[inline(always)]
    pub fn iosel(&mut self) -> IOSEL_W {
        IOSEL_W { w: self }
    }
    #[doc = "Bits 10:11 - Selects the I/O condition for event n. (The detection of edges on outputs lag the conditions that switch the outputs by one SCT clock). In order to guarantee proper edge/state detection, an input must have a minimum pulse width of at least one SCT clock period ."]
    #[inline(always)]
    pub fn iocond(&mut self) -> IOCOND_W {
        IOCOND_W { w: self }
    }
    #[doc = "Bits 12:13 - Selects how the specified match and I/O condition are used and combined."]
    #[inline(always)]
    pub fn combmode(&mut self) -> COMBMODE_W {
        COMBMODE_W { w: self }
    }
    #[doc = "Bit 14 - This bit controls how the STATEV value modifies the state selected by HEVENT when this event is the highest-numbered event occurring for that state."]
    #[inline(always)]
    pub fn stateld(&mut self) -> STATELD_W {
        STATELD_W { w: self }
    }
    #[doc = "Bits 15:19 - This value is loaded into or added to the state selected by HEVENT, depending on STATELD, when this event is the highest-numbered event occurring for that state. If STATELD and STATEV are both zero, there is no change to the STATE value."]
    #[inline(always)]
    pub fn statev(&mut self) -> STATEV_W {
        STATEV_W { w: self }
    }
    #[doc = "Bit 20 - If this bit is one and the COMBMODE field specifies a match component to the triggering of this event, then a match is considered to be active whenever the counter value is GREATER THAN OR EQUAL TO the value specified in the match register when counting up, LESS THEN OR EQUAL TO the match value when counting down. If this bit is zero, a match is only be active during the cycle when the counter is equal to the match value."]
    #[inline(always)]
    pub fn matchmem(&mut self) -> MATCHMEM_W {
        MATCHMEM_W { w: self }
    }
    #[doc = "Bits 21:22 - Direction qualifier for event generation. This field only applies when the counters are operating in BIDIR mode. If BIDIR = 0, the SCT ignores this field. Value 0x3 is reserved."]
    #[inline(always)]
    pub fn direction(&mut self) -> DIRECTION_W {
        DIRECTION_W { w: self }
    }
}
