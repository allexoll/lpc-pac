#[doc = "Reader of register SYSPLLCLKUEN"]
pub type R = crate::R<u32, super::SYSPLLCLKUEN>;
#[doc = "Writer for register SYSPLLCLKUEN"]
pub type W = crate::W<u32, super::SYSPLLCLKUEN>;
#[doc = "Register SYSPLLCLKUEN `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSPLLCLKUEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable system PLL clock source update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_A {
    #[doc = "0: no change"]
    NO_CHANGE = 0,
    #[doc = "1: update clock source"]
    UPDATED = 1,
}
impl From<ENA_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA`"]
pub type ENA_R = crate::R<bool, ENA_A>;
impl ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_A {
        match self.bits {
            false => ENA_A::NO_CHANGE,
            true => ENA_A::UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == ENA_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `UPDATED`"]
    #[inline(always)]
    pub fn is_updated(&self) -> bool {
        *self == ENA_A::UPDATED
    }
}
#[doc = "Write proxy for field `ENA`"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "no change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(ENA_A::NO_CHANGE)
    }
    #[doc = "update clock source"]
    #[inline(always)]
    pub fn updated(self) -> &'a mut W {
        self.variant(ENA_A::UPDATED)
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
impl R {
    #[doc = "Bit 0 - Enable system PLL clock source update"]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable system PLL clock source update"]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
}
