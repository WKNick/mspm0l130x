# [doc = "Register `STAT` reader"] pub type R = crate :: R < STAT_SPEC > ; # [doc = "Field `STAT_RDY` reader - OA ready status."] pub type STAT_RDY_R = crate :: BitReader < STAT_RDY_A > ; # [doc = "OA ready status.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum STAT_RDY_A { # [doc = "0: FALSE"] STAT_RDY_FALSE = 0 , # [doc = "1: TRUE"] STAT_RDY_TRUE = 1 , } impl From < STAT_RDY_A > for bool { # [inline (always)] fn from (variant : STAT_RDY_A) -> Self { variant as u8 != 0 } } impl STAT_RDY_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> STAT_RDY_A { match self . bits { false => STAT_RDY_A :: STAT_RDY_FALSE , true => STAT_RDY_A :: STAT_RDY_TRUE , } } # [doc = "FALSE"] # [inline (always)] pub fn is_stat_rdy_false (& self) -> bool { * self == STAT_RDY_A :: STAT_RDY_FALSE } # [doc = "TRUE"] # [inline (always)] pub fn is_stat_rdy_true (& self) -> bool { * self == STAT_RDY_A :: STAT_RDY_TRUE } } impl R { # [doc = "Bit 0 - OA ready status."] # [inline (always)] pub fn stat_rdy (& self) -> STAT_RDY_R { STAT_RDY_R :: new ((self . bits & 1) != 0) } } # [doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct STAT_SPEC ; impl crate :: RegisterSpec for STAT_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`stat::R`](R) reader structure"] impl crate :: Readable for STAT_SPEC { } # [doc = "`reset()` method sets STAT to value 0"] impl crate :: Resettable for STAT_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }