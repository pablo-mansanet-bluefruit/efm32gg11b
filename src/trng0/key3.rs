#[doc = "Reader of register KEY3"]
pub type R = crate::R<u32, super::KEY3>;
#[doc = "Writer for register KEY3"]
pub type W = crate::W<u32, super::KEY3>;
#[doc = "Register KEY3 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Key 3"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R { VALUE_R::new((self.bits & 0xffff_ffff) as u32) }
}
impl W {
    #[doc = "Bits 0:31 - Key 3"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W { VALUE_W { w: self } }
}
