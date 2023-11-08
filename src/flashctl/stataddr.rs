# [doc = "Register `STATADDR` reader"] pub type R = crate :: R < STATADDR_SPEC > ; # [doc = "Field `STATADDR_BANKADDR` reader - Current Bank Address A bank offset address is stored in this register."] pub type STATADDR_BANKADDR_R = crate :: FieldReader < u16 > ; # [doc = "Field `STATADDR_REGIONID` reader - Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."] pub type STATADDR_REGIONID_R = crate :: FieldReader < STATADDR_REGIONID_A > ; # [doc = "Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum STATADDR_REGIONID_A { # [doc = "1: MAIN"] STATADDR_REGIONID_MAIN = 1 , # [doc = "2: NONMAIN"] STATADDR_REGIONID_NONMAIN = 2 , # [doc = "4: TRIM"] STATADDR_REGIONID_TRIM = 4 , # [doc = "8: ENGR"] STATADDR_REGIONID_ENGR = 8 , } impl From < STATADDR_REGIONID_A > for u8 { # [inline (always)] fn from (variant : STATADDR_REGIONID_A) -> Self { variant as _ } } impl crate :: FieldSpec for STATADDR_REGIONID_A { type Ux = u8 ; } impl STATADDR_REGIONID_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < STATADDR_REGIONID_A > { match self . bits { 1 => Some (STATADDR_REGIONID_A :: STATADDR_REGIONID_MAIN) , 2 => Some (STATADDR_REGIONID_A :: STATADDR_REGIONID_NONMAIN) , 4 => Some (STATADDR_REGIONID_A :: STATADDR_REGIONID_TRIM) , 8 => Some (STATADDR_REGIONID_A :: STATADDR_REGIONID_ENGR) , _ => None , } } # [doc = "MAIN"] # [inline (always)] pub fn is_stataddr_regionid_main (& self) -> bool { * self == STATADDR_REGIONID_A :: STATADDR_REGIONID_MAIN } # [doc = "NONMAIN"] # [inline (always)] pub fn is_stataddr_regionid_nonmain (& self) -> bool { * self == STATADDR_REGIONID_A :: STATADDR_REGIONID_NONMAIN } # [doc = "TRIM"] # [inline (always)] pub fn is_stataddr_regionid_trim (& self) -> bool { * self == STATADDR_REGIONID_A :: STATADDR_REGIONID_TRIM } # [doc = "ENGR"] # [inline (always)] pub fn is_stataddr_regionid_engr (& self) -> bool { * self == STATADDR_REGIONID_A :: STATADDR_REGIONID_ENGR } } # [doc = "Field `STATADDR_BANKID` reader - Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."] pub type STATADDR_BANKID_R = crate :: FieldReader < STATADDR_BANKID_A > ; # [doc = "Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum STATADDR_BANKID_A { # [doc = "1: BANK0"] STATADDR_BANKID_BANK0 = 1 , # [doc = "2: BANK1"] STATADDR_BANKID_BANK1 = 2 , # [doc = "4: BANK2"] STATADDR_BANKID_BANK2 = 4 , # [doc = "8: BANK3"] STATADDR_BANKID_BANK3 = 8 , # [doc = "16: BANK4"] STATADDR_BANKID_BANK4 = 16 , } impl From < STATADDR_BANKID_A > for u8 { # [inline (always)] fn from (variant : STATADDR_BANKID_A) -> Self { variant as _ } } impl crate :: FieldSpec for STATADDR_BANKID_A { type Ux = u8 ; } impl STATADDR_BANKID_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Option < STATADDR_BANKID_A > { match self . bits { 1 => Some (STATADDR_BANKID_A :: STATADDR_BANKID_BANK0) , 2 => Some (STATADDR_BANKID_A :: STATADDR_BANKID_BANK1) , 4 => Some (STATADDR_BANKID_A :: STATADDR_BANKID_BANK2) , 8 => Some (STATADDR_BANKID_A :: STATADDR_BANKID_BANK3) , 16 => Some (STATADDR_BANKID_A :: STATADDR_BANKID_BANK4) , _ => None , } } # [doc = "BANK0"] # [inline (always)] pub fn is_stataddr_bankid_bank0 (& self) -> bool { * self == STATADDR_BANKID_A :: STATADDR_BANKID_BANK0 } # [doc = "BANK1"] # [inline (always)] pub fn is_stataddr_bankid_bank1 (& self) -> bool { * self == STATADDR_BANKID_A :: STATADDR_BANKID_BANK1 } # [doc = "BANK2"] # [inline (always)] pub fn is_stataddr_bankid_bank2 (& self) -> bool { * self == STATADDR_BANKID_A :: STATADDR_BANKID_BANK2 } # [doc = "BANK3"] # [inline (always)] pub fn is_stataddr_bankid_bank3 (& self) -> bool { * self == STATADDR_BANKID_A :: STATADDR_BANKID_BANK3 } # [doc = "BANK4"] # [inline (always)] pub fn is_stataddr_bankid_bank4 (& self) -> bool { * self == STATADDR_BANKID_A :: STATADDR_BANKID_BANK4 } } impl R { # [doc = "Bits 0:15 - Current Bank Address A bank offset address is stored in this register."] # [inline (always)] pub fn stataddr_bankaddr (& self) -> STATADDR_BANKADDR_R { STATADDR_BANKADDR_R :: new ((self . bits & 0xffff) as u16) } # [doc = "Bits 16:20 - Current Region ID A region indicator is stored in this register which represents the current flash region on which the state machine is operating."] # [inline (always)] pub fn stataddr_regionid (& self) -> STATADDR_REGIONID_R { STATADDR_REGIONID_R :: new (((self . bits >> 16) & 0x1f) as u8) } # [doc = "Bits 21:25 - Current Bank ID A bank indicator is stored in this register which represents the current bank on which the state machine is operating. There is 1 bit per bank."] # [inline (always)] pub fn stataddr_bankid (& self) -> STATADDR_BANKID_R { STATADDR_BANKID_R :: new (((self . bits >> 21) & 0x1f) as u8) } } # [doc = "Address Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stataddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct STATADDR_SPEC ; impl crate :: RegisterSpec for STATADDR_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`stataddr::R`](R) reader structure"] impl crate :: Readable for STATADDR_SPEC { } # [doc = "`reset()` method sets STATADDR to value 0"] impl crate :: Resettable for STATADDR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }