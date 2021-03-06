#[doc = "Reader of register MTL_TXQx_ETS_CTRL"]
pub type R = crate::R<u32, super::MTL_TXQX_ETS_CTRL>;
#[doc = "Writer for register MTL_TXQx_ETS_CTRL"]
pub type W = crate::W<u32, super::MTL_TXQX_ETS_CTRL>;
#[doc = "Register MTL_TXQx_ETS_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_TXQX_ETS_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AVALG`"]
pub type AVALG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AVALG`"]
pub struct AVALG_W<'a> {
    w: &'a mut W,
}
impl<'a> AVALG_W<'a> {
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
#[doc = "Reader of field `CC`"]
pub type CC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CC`"]
pub struct CC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC_W<'a> {
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
#[doc = "Reader of field `SLC`"]
pub type SLC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 2 - AV Algorithm."]
    #[inline(always)]
    pub fn avalg(&self) -> AVALG_R {
        AVALG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Credit Control."]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Credit Control."]
    #[inline(always)]
    pub fn slc(&self) -> SLC_R {
        SLC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - AV Algorithm."]
    #[inline(always)]
    pub fn avalg(&mut self) -> AVALG_W {
        AVALG_W { w: self }
    }
    #[doc = "Bit 3 - Credit Control."]
    #[inline(always)]
    pub fn cc(&mut self) -> CC_W {
        CC_W { w: self }
    }
}
