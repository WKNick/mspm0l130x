# [doc = "Register `INT_EVENT1_IIDX` reader"] pub type R = crate :: R < INT_EVENT1_IIDX_SPEC > ; # [doc = "Field `INT_EVENT1_IIDX_STAT` reader - Interrupt index status"] pub type INT_EVENT1_IIDX_STAT_R = crate :: FieldReader < INT_EVENT1_IIDX_STAT_A > ; # [doc = "Interrupt index status\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum INT_EVENT1_IIDX_STAT_A { # [doc = "0: NO_INTR"] INT_EVENT1_IIDX_STAT_NO_INTR = 0 , # [doc = "3: RTOUT_EVT"] INT_EVENT1_IIDX_STAT_RTOUT_EVT = 3 , # [doc = "4: RX_EVT"] INT_EVENT1_IIDX_STAT_RX_EVT = 4 , } impl From < INT_EVENT1_IIDX_STAT_A > for u8 { # [inline (always)] fn from (variant : INT_EVENT1_IIDX_STAT_A) -> Self { variant as _ } } impl crate :: FieldSpec for INT_EVENT1_IIDX_STAT_A { type Ux = u8 ; } impl INT_EVENT1_IIDX_STAT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < INT_EVENT1_IIDX_STAT_A > { match self . bits { 0 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_NO_INTR) , 3 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_RTOUT_EVT) , 4 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_RX_EVT) , _ => None , } } # [doc = "NO_INTR"] # [inline (always)] pub fn is_int_event1_iidx_stat_no_intr (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_NO_INTR } # [doc = "RTOUT_EVT"] # [inline (always)] pub fn is_int_event1_iidx_stat_rtout_evt (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_RTOUT_EVT } # [doc = "RX_EVT"] # [inline (always)] pub fn is_int_event1_iidx_stat_rx_evt (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_RX_EVT } } impl R { # [doc = "Bits 0:7 - Interrupt index status"] # [inline (always)] pub fn int_event1_iidx_stat (& self) -> INT_EVENT1_IIDX_STAT_R { INT_EVENT1_IIDX_STAT_R :: new ((self . bits & 0xff) as u8) } } # [doc = "Interrupt Index Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event1_iidx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT1_IIDX_SPEC ; impl crate :: RegisterSpec for INT_EVENT1_IIDX_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event1_iidx::R`](R) reader structure"] impl crate :: Readable for INT_EVENT1_IIDX_SPEC { } # [doc = "`reset()` method sets INT_EVENT1_IIDX to value 0"] impl crate :: Resettable for INT_EVENT1_IIDX_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }