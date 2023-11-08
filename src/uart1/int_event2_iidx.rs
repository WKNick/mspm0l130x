# [doc = "Register `INT_EVENT2_IIDX` reader"] pub type R = crate :: R < INT_EVENT2_IIDX_SPEC > ; # [doc = "Field `INT_EVENT2_IIDX_STAT` reader - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved"] pub type INT_EVENT2_IIDX_STAT_R = crate :: FieldReader < INT_EVENT2_IIDX_STAT_A > ; # [doc = "UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum INT_EVENT2_IIDX_STAT_A { # [doc = "0: NO_INTR"] INT_EVENT2_IIDX_STAT_NO_INTR = 0 , # [doc = "12: TXIFG"] INT_EVENT2_IIDX_STAT_TXIFG = 12 , } impl From < INT_EVENT2_IIDX_STAT_A > for u8 { # [inline (always)] fn from (variant : INT_EVENT2_IIDX_STAT_A) -> Self { variant as _ } } impl crate :: FieldSpec for INT_EVENT2_IIDX_STAT_A { type Ux = u8 ; } impl INT_EVENT2_IIDX_STAT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < INT_EVENT2_IIDX_STAT_A > { match self . bits { 0 => Some (INT_EVENT2_IIDX_STAT_A :: INT_EVENT2_IIDX_STAT_NO_INTR) , 12 => Some (INT_EVENT2_IIDX_STAT_A :: INT_EVENT2_IIDX_STAT_TXIFG) , _ => None , } } # [doc = "NO_INTR"] # [inline (always)] pub fn is_int_event2_iidx_stat_no_intr (& self) -> bool { * self == INT_EVENT2_IIDX_STAT_A :: INT_EVENT2_IIDX_STAT_NO_INTR } # [doc = "TXIFG"] # [inline (always)] pub fn is_int_event2_iidx_stat_txifg (& self) -> bool { * self == INT_EVENT2_IIDX_STAT_A :: INT_EVENT2_IIDX_STAT_TXIFG } } impl R { # [doc = "Bits 0:7 - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved"] # [inline (always)] pub fn int_event2_iidx_stat (& self) -> INT_EVENT2_IIDX_STAT_R { INT_EVENT2_IIDX_STAT_R :: new ((self . bits & 0xff) as u8) } } # [doc = "Interrupt index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event2_iidx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT2_IIDX_SPEC ; impl crate :: RegisterSpec for INT_EVENT2_IIDX_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event2_iidx::R`](R) reader structure"] impl crate :: Readable for INT_EVENT2_IIDX_SPEC { } # [doc = "`reset()` method sets INT_EVENT2_IIDX to value 0"] impl crate :: Resettable for INT_EVENT2_IIDX_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }