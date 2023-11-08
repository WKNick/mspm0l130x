# [doc = "Register `INT_GROUP1_IIDX` reader"] pub type R = crate :: R < INT_GROUP1_IIDX_SPEC > ; # [doc = "Field `INT_GROUP1_IIDX_STAT` reader - Interrupt index status"] pub type INT_GROUP1_IIDX_STAT_R = crate :: FieldReader < INT_GROUP1_IIDX_STAT_A > ; # [doc = "Interrupt index status\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum INT_GROUP1_IIDX_STAT_A { # [doc = "0: NO_INTR"] INT_GROUP1_IIDX_STAT_NO_INTR = 0 , # [doc = "1: INT0"] INT_GROUP1_IIDX_STAT_INT0 = 1 , # [doc = "2: INT1"] INT_GROUP1_IIDX_STAT_INT1 = 2 , # [doc = "3: INT2"] INT_GROUP1_IIDX_STAT_INT2 = 3 , # [doc = "4: INT3"] INT_GROUP1_IIDX_STAT_INT3 = 4 , # [doc = "5: INT4"] INT_GROUP1_IIDX_STAT_INT4 = 5 , # [doc = "6: INT5"] INT_GROUP1_IIDX_STAT_INT5 = 6 , # [doc = "7: INT6"] INT_GROUP1_IIDX_STAT_INT6 = 7 , # [doc = "8: INT7"] INT_GROUP1_IIDX_STAT_INT7 = 8 , } impl From < INT_GROUP1_IIDX_STAT_A > for u8 { # [inline (always)] fn from (variant : INT_GROUP1_IIDX_STAT_A) -> Self { variant as _ } } impl crate :: FieldSpec for INT_GROUP1_IIDX_STAT_A { type Ux = u8 ; } impl INT_GROUP1_IIDX_STAT_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < INT_GROUP1_IIDX_STAT_A > { match self . bits { 0 => Some (INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_NO_INTR) , 1 => Some (INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT0) , 2 => Some (INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT1) , 3 => Some (INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT2) , 4 => Some (INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT3) , 5 => Some (INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT4) , 6 => Some (INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT5) , 7 => Some (INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT6) , 8 => Some (INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT7) , _ => None , } } # [doc = "NO_INTR"] # [inline (always)] pub fn is_int_group1_iidx_stat_no_intr (& self) -> bool { * self == INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_NO_INTR } # [doc = "INT0"] # [inline (always)] pub fn is_int_group1_iidx_stat_int0 (& self) -> bool { * self == INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT0 } # [doc = "INT1"] # [inline (always)] pub fn is_int_group1_iidx_stat_int1 (& self) -> bool { * self == INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT1 } # [doc = "INT2"] # [inline (always)] pub fn is_int_group1_iidx_stat_int2 (& self) -> bool { * self == INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT2 } # [doc = "INT3"] # [inline (always)] pub fn is_int_group1_iidx_stat_int3 (& self) -> bool { * self == INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT3 } # [doc = "INT4"] # [inline (always)] pub fn is_int_group1_iidx_stat_int4 (& self) -> bool { * self == INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT4 } # [doc = "INT5"] # [inline (always)] pub fn is_int_group1_iidx_stat_int5 (& self) -> bool { * self == INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT5 } # [doc = "INT6"] # [inline (always)] pub fn is_int_group1_iidx_stat_int6 (& self) -> bool { * self == INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT6 } # [doc = "INT7"] # [inline (always)] pub fn is_int_group1_iidx_stat_int7 (& self) -> bool { * self == INT_GROUP1_IIDX_STAT_A :: INT_GROUP1_IIDX_STAT_INT7 } } impl R { # [doc = "Bits 0:7 - Interrupt index status"] # [inline (always)] pub fn int_group1_iidx_stat (& self) -> INT_GROUP1_IIDX_STAT_R { INT_GROUP1_IIDX_STAT_R :: new ((self . bits & 0xff) as u8) } } # [doc = "Interrupt index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_group1_iidx::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_GROUP1_IIDX_SPEC ; impl crate :: RegisterSpec for INT_GROUP1_IIDX_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_group1_iidx::R`](R) reader structure"] impl crate :: Readable for INT_GROUP1_IIDX_SPEC { } # [doc = "`reset()` method sets INT_GROUP1_IIDX to value 0"] impl crate :: Resettable for INT_GROUP1_IIDX_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }