#[doc = "Reader of register ROUTEPEN"]
pub type R = crate::R<u32, super::ROUTEPEN>;
#[doc = "Writer for register ROUTEPEN"]
pub type W = crate::W<u32, super::ROUTEPEN>;
#[doc = "Register ROUTEPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `SDAPEN`"]
pub type SDAPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDAPEN`"]
pub struct SDAPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDAPEN_W<'a> {
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
#[doc = "Reader of field `SCLPEN`"]
pub type SCLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLPEN`"]
pub struct SCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SDA Pin Enable"]
    #[inline(always)]
    pub fn sdapen(&self) -> SDAPEN_R { SDAPEN_R::new((self.bits & 0x01) != 0) }
    #[doc = "Bit 1 - SCL Pin Enable"]
    #[inline(always)]
    pub fn sclpen(&self) -> SCLPEN_R { SCLPEN_R::new(((self.bits >> 1) & 0x01) != 0) }
}
impl W {
    #[doc = "Bit 0 - SDA Pin Enable"]
    #[inline(always)]
    pub fn sdapen(&mut self) -> SDAPEN_W { SDAPEN_W { w: self } }
    #[doc = "Bit 1 - SCL Pin Enable"]
    #[inline(always)]
    pub fn sclpen(&mut self) -> SCLPEN_W { SCLPEN_W { w: self } }
}
