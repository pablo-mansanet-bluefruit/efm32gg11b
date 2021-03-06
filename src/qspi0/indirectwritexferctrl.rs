#[doc = "Reader of register INDIRECTWRITEXFERCTRL"]
pub type R = crate::R<u32, super::INDIRECTWRITEXFERCTRL>;
#[doc = "Writer for register INDIRECTWRITEXFERCTRL"]
pub type W = crate::W<u32, super::INDIRECTWRITEXFERCTRL>;
#[doc = "Register INDIRECTWRITEXFERCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INDIRECTWRITEXFERCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `CANCEL`"]
pub struct CANCEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CANCEL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `WRSTATUS`"]
pub type WRSTATUS_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRQUEUED`"]
pub type WRQUEUED_R = crate::R<bool, bool>;
#[doc = "Reader of field `INDOPSDONESTATUS`"]
pub type INDOPSDONESTATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INDOPSDONESTATUS`"]
pub struct INDOPSDONESTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INDOPSDONESTATUS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W { self.bit(true) }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W { self.bit(false) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `NUMINDOPSDONE`"]
pub type NUMINDOPSDONE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 2 - Indirect Write Status"]
    #[inline(always)]
    pub fn wrstatus(&self) -> WRSTATUS_R { WRSTATUS_R::new(((self.bits >> 2) & 0x01) != 0) }
    #[doc = "Bit 4 - Two Indirect Write Operations Have Been Queued"]
    #[inline(always)]
    pub fn wrqueued(&self) -> WRQUEUED_R { WRQUEUED_R::new(((self.bits >> 4) & 0x01) != 0) }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    pub fn indopsdonestatus(&self) -> INDOPSDONESTATUS_R {
        INDOPSDONESTATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Indirect Operations Done"]
    #[inline(always)]
    pub fn numindopsdone(&self) -> NUMINDOPSDONE_R {
        NUMINDOPSDONE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start Indirect Write"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W { START_W { w: self } }
    #[doc = "Bit 1 - Cancel Indirect Write"]
    #[inline(always)]
    pub fn cancel(&mut self) -> CANCEL_W { CANCEL_W { w: self } }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    pub fn indopsdonestatus(&mut self) -> INDOPSDONESTATUS_W { INDOPSDONESTATUS_W { w: self } }
}
