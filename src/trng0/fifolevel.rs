#[doc = "Reader of register FIFOLEVEL"]
pub type R = crate::R<u32, super::FIFOLEVEL>;
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFO Level"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R { VALUE_R::new((self.bits & 0xffff_ffff) as u32) }
}
