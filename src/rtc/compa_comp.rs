#[doc = "Reader of register COMPA_COMP"]
pub type R = crate::R<u32, super::COMPA_COMP>;
#[doc = "Writer for register COMPA_COMP"]
pub type W = crate::W<u32, super::COMPA_COMP>;
#[doc = "Register COMPA_COMP `reset()`'s with value 0"]
impl crate::ResetValue for super::COMPA_COMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R { COMP_R::new((self.bits & 0x00ff_ffff) as u32) }
}
impl W {
    #[doc = "Bits 0:23 - Compare Value"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W { COMP_W { w: self } }
}
