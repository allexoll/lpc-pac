#[doc = "Reader of register POSITION"]
pub type R = crate::R<u32, super::POSITION>;
#[doc = "Writer for register POSITION"]
pub type W = crate::W<u32, super::POSITION>;
#[doc = "Register POSITION `reset()`'s with value 0"]
impl crate::ResetValue for super::POSITION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WRAP`"]
pub type WRAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRAP`"]
pub struct WRAP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAP_W<'a> {
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
#[doc = "Reader of field `POINTER`"]
pub type POINTER_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `POINTER`"]
pub struct POINTER_W<'a> {
    w: &'a mut W,
}
impl<'a> POINTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff_ffff << 3)) | (((value as u32) & 0x1fff_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - This bit is set to 1 automatically when the POINTER value wraps as determined by the MASTER.MASK field in the MASTER Trace Control Register."]
    #[inline(always)]
    pub fn wrap(&self) -> WRAP_R {
        WRAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:31 - Trace packet location pointer. Because a packet consists of two words, the POINTER field is the location of the first word of a packet. This field contains bits \\[31:3\\] of the address, in the SRAM, where the next trace packet will be written. The field points to an unused location and is automatically incremented. A debug agent can calculate the system address, on the AHB-Lite bus, of the SRAM location pointed to by the POSITION register using the following equation: system address = BASE + ((P + (2AWIDTH - (BASE MOD 2AWIDTH))) MOD 2AWIDTH). Where P = POSITION AND 0xFFFF_FFF8. Where BASE is the BASE register value"]
    #[inline(always)]
    pub fn pointer(&self) -> POINTER_R {
        POINTER_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 2 - This bit is set to 1 automatically when the POINTER value wraps as determined by the MASTER.MASK field in the MASTER Trace Control Register."]
    #[inline(always)]
    pub fn wrap(&mut self) -> WRAP_W {
        WRAP_W { w: self }
    }
    #[doc = "Bits 3:31 - Trace packet location pointer. Because a packet consists of two words, the POINTER field is the location of the first word of a packet. This field contains bits \\[31:3\\] of the address, in the SRAM, where the next trace packet will be written. The field points to an unused location and is automatically incremented. A debug agent can calculate the system address, on the AHB-Lite bus, of the SRAM location pointed to by the POSITION register using the following equation: system address = BASE + ((P + (2AWIDTH - (BASE MOD 2AWIDTH))) MOD 2AWIDTH). Where P = POSITION AND 0xFFFF_FFF8. Where BASE is the BASE register value"]
    #[inline(always)]
    pub fn pointer(&mut self) -> POINTER_W {
        POINTER_W { w: self }
    }
}
