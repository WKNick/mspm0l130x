# [doc = "Register `IIDX` reader"] pub type R = crate :: R < IIDX_SPEC > ; # [doc = "Field `IIDX_STAT` reader - Interrupt index status"] pub type IIDX_STAT_R = crate :: FieldReader < IIDX_STAT_A > ; # [doc = "Interrupt index status\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum IIDX_STAT_A { # [doc = "0: NO_INTR"] IIDX_STAT_NO_INTR = 0 , # [doc = "1: TXIFG"] IIDX_STAT_TXIFG = 1 , # [doc = "2: RXIFG"] IIDX_STAT_RXIFG = 2 , # [doc = "3: PWRUP"] IIDX_STAT_PWRUP = 3 , # [doc = "4: PWRDWN"] IIDX_STAT_PWRDWN = 4 , } impl From < IIDX_STAT_A > for u8 { # [inline (always)] fn from (variant : IIDX_STAT_A) -> Self { variant as _ } } impl crate :: FieldSpec for IIDX_STAT_A { type Ux = u8 ; } impl IIDX_STAT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < IIDX_STAT_A > { match self . bits { 0 => Some (IIDX_STAT_A :: IIDX_STAT_NO_INTR) , 1 => Some (IIDX_STAT_A :: IIDX_STAT_TXIFG) , 2 => Some (IIDX_STAT_A :: IIDX_STAT_RXIFG) , 3 => Some (IIDX_STAT_A :: IIDX_STAT_PWRUP) , 4 => Some (IIDX_STAT_A :: IIDX_STAT_PWRDWN) , _ => None , } } # [doc = "NO_INTR"] # [inline (always)] pub fn is_iidx_stat_no_intr (& self) -> bool { * self == IIDX_STAT_A :: IIDX_STAT_NO_INTR } # [doc = "TXIFG"] # [inline (always)] pub fn is_iidx_stat_txifg (& self) -> bool { * self == IIDX_STAT_A :: IIDX_STAT_TXIFG } # [doc = "RXIFG"] # [inline (always)] pub fn is_iidx_stat_rxifg (& self) -> bool { * self == IIDX_STAT_A :: IIDX_STAT_RXIFG } # [doc = "PWRUP"] # [inline (always)] pub fn is_iidx_stat_pwrup (& self) -> bool { * self == IIDX_STAT_A :: IIDX_STAT_PWRUP } # [doc = "PWRDWN"] # [inline (always)] pub fn is_iidx_stat_pwrdwn (& self) -> bool { * self == IIDX_STAT_A :: IIDX_STAT_PWRDWN } } impl R { # [doc = "Bits 0:7 - Interrupt index status"] # [inline (always)] pub fn iidx_stat (& self) -> IIDX_STAT_R { IIDX_STAT_R :: new ((self . bits & 0xff) as u8) } } # [doc = "Interrupt index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iidx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct IIDX_SPEC ; impl crate :: RegisterSpec for IIDX_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`iidx::R`](R) reader structure"] impl crate :: Readable for IIDX_SPEC { } # [doc = "`reset()` method sets IIDX to value 0"] impl crate :: Resettable for IIDX_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }