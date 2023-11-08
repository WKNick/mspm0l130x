# [doc = "Register `STATPCNT` reader"] pub type R = crate :: R < STATPCNT_SPEC > ; # [doc = "Field `STATPCNT_PULSECNT` reader - Current Pulse Counter Value"] pub type STATPCNT_PULSECNT_R = crate :: FieldReader < u16 > ; impl R { # [doc = "Bits 0:11 - Current Pulse Counter Value"] # [inline (always)] pub fn statpcnt_pulsecnt (& self) -> STATPCNT_PULSECNT_R { STATPCNT_PULSECNT_R :: new ((self . bits & 0x0fff) as u16) } } # [doc = "Pulse Count Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statpcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct STATPCNT_SPEC ; impl crate :: RegisterSpec for STATPCNT_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`statpcnt::R`](R) reader structure"] impl crate :: Readable for STATPCNT_SPEC { } # [doc = "`reset()` method sets STATPCNT to value 0"] impl crate :: Resettable for STATPCNT_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }