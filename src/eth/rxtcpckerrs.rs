#[doc = "Reader of register RXTCPCKERRS"]
pub type R = crate::R<u32, super::RXTCPCKERRS>;
#[doc = "Writer for register RXTCPCKERRS"]
pub type W = crate::W<u32, super::RXTCPCKERRS>;
#[doc = "Register RXTCPCKERRS `reset()`'s with value 0"]
impl crate::ResetValue for super::RXTCPCKERRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TCP checksum errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R { COUNT_R::new((self.bits & 0xff) as u8) }
}
impl W {
    #[doc = "Bits 0:7 - TCP checksum errors"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W { COUNT_W { w: self } }
}
