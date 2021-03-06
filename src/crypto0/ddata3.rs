#[doc = "Reader of register DDATA3"]
pub type R = crate::R<u32, super::DDATA3>;
#[doc = "Writer for register DDATA3"]
pub type W = crate::W<u32, super::DDATA3>;
#[doc = "Register DDATA3 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDATA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `DDATA3`"]
pub type DDATA3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DDATA3`"]
pub struct DDATA3_W<'a> {
    w: &'a mut W,
}
impl<'a> DDATA3_W<'a> {
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
    pub fn ddata3(&self) -> DDATA3_R { DDATA3_R::new((self.bits & 0xffff_ffff) as u32) }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata3(&mut self) -> DDATA3_W { DDATA3_W { w: self } }
}
