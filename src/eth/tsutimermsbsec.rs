#[doc = "Reader of register TSUTIMERMSBSEC"]
pub type R = crate::R<u32, super::TSUTIMERMSBSEC>;
#[doc = "Writer for register TSUTIMERMSBSEC"]
pub type W = crate::W<u32, super::TSUTIMERMSBSEC>;
#[doc = "Register TSUTIMERMSBSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::TSUTIMERMSBSEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIMER`"]
pub struct TIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MSB 16 bits of seconds timer count."]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R { TIMER_R::new((self.bits & 0xffff) as u16) }
}
impl W {
    #[doc = "Bits 0:15 - MSB 16 bits of seconds timer count."]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W { TIMER_W { w: self } }
}
