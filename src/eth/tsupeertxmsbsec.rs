#[doc = "Reader of register TSUPEERTXMSBSEC"]
pub type R = crate::R<u32, super::TSUPEERTXMSBSEC>;
#[doc = "Reader of field `TIMERSEC`"]
pub type TIMERSEC_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - PTP Peer Event Frame TX Seconds"]
    #[inline(always)]
    pub fn timersec(&self) -> TIMERSEC_R { TIMERSEC_R::new((self.bits & 0xffff) as u16) }
}
