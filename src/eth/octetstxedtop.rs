#[doc = "Reader of register OCTETSTXEDTOP"]
pub type R = crate::R<u32, super::OCTETSTXEDTOP>;
#[doc = "Writer for register OCTETSTXEDTOP"]
pub type W = crate::W<u32, super::OCTETSTXEDTOP>;
#[doc = "Register OCTETSTXEDTOP `reset()`'s with value 0"]
impl crate::ResetValue for super::OCTETSTXEDTOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmitted octets in frame without errors \\[47:32\\]"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R { COUNT_R::new((self.bits & 0xffff) as u16) }
}
impl W {
    #[doc = "Bits 0:15 - Transmitted octets in frame without errors \\[47:32\\]"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W { COUNT_W { w: self } }
}
