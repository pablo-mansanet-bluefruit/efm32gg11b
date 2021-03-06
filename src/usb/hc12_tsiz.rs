#[doc = "Reader of register HC12_TSIZ"]
pub type R = crate::R<u32, super::HC12_TSIZ>;
#[doc = "Writer for register HC12_TSIZ"]
pub type W = crate::W<u32, super::HC12_TSIZ>;
#[doc = "Register HC12_TSIZ `reset()`'s with value 0"]
impl crate::ResetValue for super::HC12_TSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type { 0 }
}
#[doc = "Reader of field `XFERSIZE`"]
pub type XFERSIZE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `XFERSIZE`"]
pub struct XFERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
        self.w
    }
}
#[doc = "Reader of field `PKTCNT`"]
pub type PKTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKTCNT`"]
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | (((value as u32) & 0x03ff) << 19);
        self.w
    }
}
#[doc = "The Application Programs This Field With the Type of\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PID_A {
    #[doc = "0: DATA0 PID."]
    DATA0 = 0,
    #[doc = "1: DATA2 PID."]
    DATA2 = 1,
    #[doc = "2: DATA1 PID."]
    DATA1 = 2,
    #[doc = "3: MDATA (non-control) / SETUP (control) PID."]
    MDATA = 3,
}
impl From<PID_A> for u8 {
    #[inline(always)]
    fn from(variant: PID_A) -> Self { variant as _ }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u8, PID_A>;
impl PID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PID_A {
        match self.bits {
            0 => PID_A::DATA0,
            1 => PID_A::DATA2,
            2 => PID_A::DATA1,
            3 => PID_A::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool { *self == PID_A::DATA0 }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline(always)]
    pub fn is_data2(&self) -> bool { *self == PID_A::DATA2 }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool { *self == PID_A::DATA1 }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline(always)]
    pub fn is_mdata(&self) -> bool { *self == PID_A::MDATA }
}
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PID_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DATA0 PID."]
    #[inline(always)]
    pub fn data0(self) -> &'a mut W { self.variant(PID_A::DATA0) }
    #[doc = "DATA2 PID."]
    #[inline(always)]
    pub fn data2(self) -> &'a mut W { self.variant(PID_A::DATA2) }
    #[doc = "DATA1 PID."]
    #[inline(always)]
    pub fn data1(self) -> &'a mut W { self.variant(PID_A::DATA1) }
    #[doc = "MDATA (non-control) / SETUP (control) PID."]
    #[inline(always)]
    pub fn mdata(self) -> &'a mut W { self.variant(PID_A::MDATA) }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R { XFERSIZE_R::new((self.bits & 0x0007_ffff) as u32) }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R { PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16) }
    #[doc = "Bits 29:30 - The Application Programs This Field With the Type of"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R { PID_R::new(((self.bits >> 29) & 0x03) as u8) }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W { XFERSIZE_W { w: self } }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W { PKTCNT_W { w: self } }
    #[doc = "Bits 29:30 - The Application Programs This Field With the Type of"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W { PID_W { w: self } }
}
