#[doc = "Reader of register PB_DIN"]
pub type R = crate::R<u32, super::PB_DIN>;
#[doc = "Reader of field `DIN`"]
pub type DIN_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data in"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R { DIN_R::new((self.bits & 0xffff) as u16) }
}
