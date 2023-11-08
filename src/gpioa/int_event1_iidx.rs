# [doc = "Register `INT_EVENT1_IIDX` reader"] pub type R = crate :: R < INT_EVENT1_IIDX_SPEC > ; # [doc = "Field `INT_EVENT1_IIDX_STAT` reader - Interrupt index status"] pub type INT_EVENT1_IIDX_STAT_R = crate :: FieldReader < INT_EVENT1_IIDX_STAT_A > ; # [doc = "Interrupt index status\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum INT_EVENT1_IIDX_STAT_A { # [doc = "0: NO_INTR"] INT_EVENT1_IIDX_STAT_NO_INTR = 0 , # [doc = "1: DIO0"] INT_EVENT1_IIDX_STAT_DIO0 = 1 , # [doc = "2: DIO1"] INT_EVENT1_IIDX_STAT_DIO1 = 2 , # [doc = "3: DIO2"] INT_EVENT1_IIDX_STAT_DIO2 = 3 , # [doc = "4: DIO3"] INT_EVENT1_IIDX_STAT_DIO3 = 4 , # [doc = "5: DIO4"] INT_EVENT1_IIDX_STAT_DIO4 = 5 , # [doc = "6: DIO5"] INT_EVENT1_IIDX_STAT_DIO5 = 6 , # [doc = "7: DIO6"] INT_EVENT1_IIDX_STAT_DIO6 = 7 , # [doc = "8: DIO7"] INT_EVENT1_IIDX_STAT_DIO7 = 8 , # [doc = "9: DIO8"] INT_EVENT1_IIDX_STAT_DIO8 = 9 , # [doc = "10: DIO9"] INT_EVENT1_IIDX_STAT_DIO9 = 10 , # [doc = "11: DIO10"] INT_EVENT1_IIDX_STAT_DIO10 = 11 , # [doc = "12: DIO11"] INT_EVENT1_IIDX_STAT_DIO11 = 12 , # [doc = "13: DIO12"] INT_EVENT1_IIDX_STAT_DIO12 = 13 , # [doc = "14: DIO13"] INT_EVENT1_IIDX_STAT_DIO13 = 14 , # [doc = "15: DIO14"] INT_EVENT1_IIDX_STAT_DIO14 = 15 , # [doc = "16: DIO15"] INT_EVENT1_IIDX_STAT_DIO15 = 16 , } impl From < INT_EVENT1_IIDX_STAT_A > for u8 { # [inline (always)] fn from (variant : INT_EVENT1_IIDX_STAT_A) -> Self { variant as _ } } impl crate :: FieldSpec for INT_EVENT1_IIDX_STAT_A { type Ux = u8 ; } impl INT_EVENT1_IIDX_STAT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < INT_EVENT1_IIDX_STAT_A > { match self . bits { 0 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_NO_INTR) , 1 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO0) , 2 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO1) , 3 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO2) , 4 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO3) , 5 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO4) , 6 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO5) , 7 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO6) , 8 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO7) , 9 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO8) , 10 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO9) , 11 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO10) , 12 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO11) , 13 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO12) , 14 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO13) , 15 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO14) , 16 => Some (INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO15) , _ => None , } } # [doc = "NO_INTR"] # [inline (always)] pub fn is_int_event1_iidx_stat_no_intr (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_NO_INTR } # [doc = "DIO0"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio0 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO0 } # [doc = "DIO1"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio1 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO1 } # [doc = "DIO2"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio2 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO2 } # [doc = "DIO3"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio3 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO3 } # [doc = "DIO4"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio4 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO4 } # [doc = "DIO5"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio5 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO5 } # [doc = "DIO6"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio6 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO6 } # [doc = "DIO7"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio7 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO7 } # [doc = "DIO8"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio8 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO8 } # [doc = "DIO9"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio9 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO9 } # [doc = "DIO10"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio10 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO10 } # [doc = "DIO11"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio11 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO11 } # [doc = "DIO12"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio12 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO12 } # [doc = "DIO13"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio13 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO13 } # [doc = "DIO14"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio14 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO14 } # [doc = "DIO15"] # [inline (always)] pub fn is_int_event1_iidx_stat_dio15 (& self) -> bool { * self == INT_EVENT1_IIDX_STAT_A :: INT_EVENT1_IIDX_STAT_DIO15 } } impl R { # [doc = "Bits 0:7 - Interrupt index status"] # [inline (always)] pub fn int_event1_iidx_stat (& self) -> INT_EVENT1_IIDX_STAT_R { INT_EVENT1_IIDX_STAT_R :: new ((self . bits & 0xff) as u8) } } # [doc = "Interrupt index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event1_iidx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT1_IIDX_SPEC ; impl crate :: RegisterSpec for INT_EVENT1_IIDX_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event1_iidx::R`](R) reader structure"] impl crate :: Readable for INT_EVENT1_IIDX_SPEC { } # [doc = "`reset()` method sets INT_EVENT1_IIDX to value 0"] impl crate :: Resettable for INT_EVENT1_IIDX_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }