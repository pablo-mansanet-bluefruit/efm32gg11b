#[doc = "Reader of register KEY"]
pub type R = crate::R<u32, super::KEY>;
#[doc = "Writer for register KEY"]
pub type W = crate::W<u32, super::KEY>;
#[doc = "Register KEY `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Key Access"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R { KEY_R::new((self.bits & 0xffff_ffff) as u32) }
}
impl W {
    #[doc = "Bits 0:31 - Key Access"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W { KEY_W { w: self } }
}
