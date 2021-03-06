#[doc = "Reader of register DDATA2"]
pub type R = crate::R<u32, super::DDATA2>;
#[doc = "Writer for register DDATA2"]
pub type W = crate::W<u32, super::DDATA2>;
#[doc = "Register DDATA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDATA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `DDATA2`"]
pub type DDATA2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DDATA2`"]
pub struct DDATA2_W<'a> {
    w: &'a mut W,
}
impl<'a> DDATA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata2(&self) -> DDATA2_R { DDATA2_R::new((self.bits & 0xffff_ffff) as u32) }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata2(&mut self) -> DDATA2_W { DDATA2_W { w: self } }
}
