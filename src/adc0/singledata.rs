#[doc = "Reader of register SINGLEDATA"]
pub type R = crate::R<u32, super::SINGLEDATA>;
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Single Conversion Result Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R { DATA_R::new((self.bits & 0xffff_ffff) as u32) }
}
