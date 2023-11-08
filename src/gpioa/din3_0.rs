# [doc = "Register `DIN3_0` reader"] pub type R = crate :: R < DIN3_0_SPEC > ; # [doc = "Field `DIN3_0_DIO0` reader - This bit reads the data input value of DIO0."] pub type DIN3_0_DIO0_R = crate :: BitReader < DIN3_0_DIO0_A > ; # [doc = "This bit reads the data input value of DIO0.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DIN3_0_DIO0_A { # [doc = "0: ZERO"] DIN3_0_DIO0_ZERO = 0 , # [doc = "1: ONE"] DIN3_0_DIO0_ONE = 1 , } impl From < DIN3_0_DIO0_A > for bool { # [inline (always)] fn from (variant : DIN3_0_DIO0_A) -> Self { variant as u8 != 0 } } impl DIN3_0_DIO0_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> DIN3_0_DIO0_A { match self . bits { false => DIN3_0_DIO0_A :: DIN3_0_DIO0_ZERO , true => DIN3_0_DIO0_A :: DIN3_0_DIO0_ONE , } } # [doc = "ZERO"] # [inline (always)] pub fn is_din3_0_dio0_zero (& self) -> bool { * self == DIN3_0_DIO0_A :: DIN3_0_DIO0_ZERO } # [doc = "ONE"] # [inline (always)] pub fn is_din3_0_dio0_one (& self) -> bool { * self == DIN3_0_DIO0_A :: DIN3_0_DIO0_ONE } } # [doc = "Field `DIN3_0_DIO1` reader - This bit reads the data input value of DIO1."] pub type DIN3_0_DIO1_R = crate :: BitReader < DIN3_0_DIO1_A > ; # [doc = "This bit reads the data input value of DIO1.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DIN3_0_DIO1_A { # [doc = "0: ZERO"] DIN3_0_DIO1_ZERO = 0 , # [doc = "1: ONE"] DIN3_0_DIO1_ONE = 1 , } impl From < DIN3_0_DIO1_A > for bool { # [inline (always)] fn from (variant : DIN3_0_DIO1_A) -> Self { variant as u8 != 0 } } impl DIN3_0_DIO1_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> DIN3_0_DIO1_A { match self . bits { false => DIN3_0_DIO1_A :: DIN3_0_DIO1_ZERO , true => DIN3_0_DIO1_A :: DIN3_0_DIO1_ONE , } } # [doc = "ZERO"] # [inline (always)] pub fn is_din3_0_dio1_zero (& self) -> bool { * self == DIN3_0_DIO1_A :: DIN3_0_DIO1_ZERO } # [doc = "ONE"] # [inline (always)] pub fn is_din3_0_dio1_one (& self) -> bool { * self == DIN3_0_DIO1_A :: DIN3_0_DIO1_ONE } } # [doc = "Field `DIN3_0_DIO2` reader - This bit reads the data input value of DIO2."] pub type DIN3_0_DIO2_R = crate :: BitReader < DIN3_0_DIO2_A > ; # [doc = "This bit reads the data input value of DIO2.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DIN3_0_DIO2_A { # [doc = "0: ZERO"] DIN3_0_DIO2_ZERO = 0 , # [doc = "1: ONE"] DIN3_0_DIO2_ONE = 1 , } impl From < DIN3_0_DIO2_A > for bool { # [inline (always)] fn from (variant : DIN3_0_DIO2_A) -> Self { variant as u8 != 0 } } impl DIN3_0_DIO2_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> DIN3_0_DIO2_A { match self . bits { false => DIN3_0_DIO2_A :: DIN3_0_DIO2_ZERO , true => DIN3_0_DIO2_A :: DIN3_0_DIO2_ONE , } } # [doc = "ZERO"] # [inline (always)] pub fn is_din3_0_dio2_zero (& self) -> bool { * self == DIN3_0_DIO2_A :: DIN3_0_DIO2_ZERO } # [doc = "ONE"] # [inline (always)] pub fn is_din3_0_dio2_one (& self) -> bool { * self == DIN3_0_DIO2_A :: DIN3_0_DIO2_ONE } } # [doc = "Field `DIN3_0_DIO3` reader - This bit reads the data input value of DIO3."] pub type DIN3_0_DIO3_R = crate :: BitReader < DIN3_0_DIO3_A > ; # [doc = "This bit reads the data input value of DIO3.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DIN3_0_DIO3_A { # [doc = "0: ZERO"] DIN3_0_DIO3_ZERO = 0 , # [doc = "1: ONE"] DIN3_0_DIO3_ONE = 1 , } impl From < DIN3_0_DIO3_A > for bool { # [inline (always)] fn from (variant : DIN3_0_DIO3_A) -> Self { variant as u8 != 0 } } impl DIN3_0_DIO3_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> DIN3_0_DIO3_A { match self . bits { false => DIN3_0_DIO3_A :: DIN3_0_DIO3_ZERO , true => DIN3_0_DIO3_A :: DIN3_0_DIO3_ONE , } } # [doc = "ZERO"] # [inline (always)] pub fn is_din3_0_dio3_zero (& self) -> bool { * self == DIN3_0_DIO3_A :: DIN3_0_DIO3_ZERO } # [doc = "ONE"] # [inline (always)] pub fn is_din3_0_dio3_one (& self) -> bool { * self == DIN3_0_DIO3_A :: DIN3_0_DIO3_ONE } } impl R { # [doc = "Bit 0 - This bit reads the data input value of DIO0."] # [inline (always)] pub fn din3_0_dio0 (& self) -> DIN3_0_DIO0_R { DIN3_0_DIO0_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 8 - This bit reads the data input value of DIO1."] # [inline (always)] pub fn din3_0_dio1 (& self) -> DIN3_0_DIO1_R { DIN3_0_DIO1_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 16 - This bit reads the data input value of DIO2."] # [inline (always)] pub fn din3_0_dio2 (& self) -> DIN3_0_DIO2_R { DIN3_0_DIO2_R :: new (((self . bits >> 16) & 1) != 0) } # [doc = "Bit 24 - This bit reads the data input value of DIO3."] # [inline (always)] pub fn din3_0_dio3 (& self) -> DIN3_0_DIO3_R { DIN3_0_DIO3_R :: new (((self . bits >> 24) & 1) != 0) } } # [doc = "Data input 3 to 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din3_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct DIN3_0_SPEC ; impl crate :: RegisterSpec for DIN3_0_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`din3_0::R`](R) reader structure"] impl crate :: Readable for DIN3_0_SPEC { } # [doc = "`reset()` method sets DIN3_0 to value 0"] impl crate :: Resettable for DIN3_0_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }