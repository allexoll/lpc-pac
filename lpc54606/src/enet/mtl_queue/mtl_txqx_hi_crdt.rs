#[doc = "Reader of register MTL_TXQx_HI_CRDT"]
pub type R = crate::R<u32, super::MTL_TXQX_HI_CRDT>;
#[doc = "Writer for register MTL_TXQx_HI_CRDT"]
pub type W = crate::W<u32, super::MTL_TXQX_HI_CRDT>;
#[doc = "Register MTL_TXQx_HI_CRDT `reset()`'s with value 0"]
impl crate::ResetValue for super::MTL_TXQX_HI_CRDT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HC`"]
pub type HC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HC`"]
pub struct HC_W<'a> {
    w: &'a mut W,
}
impl<'a> HC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - hiCredit."]
    #[inline(always)]
    pub fn hc(&self) -> HC_R {
        HC_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - hiCredit."]
    #[inline(always)]
    pub fn hc(&mut self) -> HC_W {
        HC_W { w: self }
    }
}
