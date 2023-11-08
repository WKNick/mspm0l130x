# [doc = "Register `DIN11_8` reader"] pub type R = crate :: R < DIN11_8_SPEC > ; # [doc = "Field `DIN11_8_DIO8` reader - This bit reads the data input value of DIO8."] pub type DIN11_8_DIO8_R = crate :: BitReader < DIN11_8_DIO8_A > ; # [doc = "This bit reads the data input value of DIO8.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DIN11_8_DIO8_A { # [doc = "0: ZERO"] DIN11_8_DIO8_ZERO = 0 , # [doc = "1: ONE"] DIN11_8_DIO8_ONE = 1 , } impl From < DIN11_8_DIO8_A > for bool { # [inline (always)] fn from (variant : DIN11_8_DIO8_A) -> Self { variant as u8 != 0 } } impl DIN11_8_DIO8_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> DIN11_8_DIO8_A { match self . bits { false => DIN11_8_DIO8_A :: DIN11_8_DIO8_ZERO , true => DIN11_8_DIO8_A :: DIN11_8_DIO8_ONE , } } # [doc = "ZERO"] # [inline (always)] pub fn is_din11_8_dio8_zero (& self) -> bool { * self == DIN11_8_DIO8_A :: DIN11_8_DIO8_ZERO } # [doc = "ONE"] # [inline (always)] pub fn is_din11_8_dio8_one (& self) -> bool { * self == DIN11_8_DIO8_A :: DIN11_8_DIO8_ONE } } # [doc = "Field `DIN11_8_DIO9` reader - This bit reads the data input value of DIO9."] pub type DIN11_8_DIO9_R = crate :: BitReader < DIN11_8_DIO9_A > ; # [doc = "This bit reads the data input value of DIO9.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DIN11_8_DIO9_A { # [doc = "0: ZERO"] DIN11_8_DIO9_ZERO = 0 , # [doc = "1: ONE"] DIN11_8_DIO9_ONE = 1 , } impl From < DIN11_8_DIO9_A > for bool { # [inline (always)] fn from (variant : DIN11_8_DIO9_A) -> Self { variant as u8 != 0 } } impl DIN11_8_DIO9_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> DIN11_8_DIO9_A { match self . bits { false => DIN11_8_DIO9_A :: DIN11_8_DIO9_ZERO , true => DIN11_8_DIO9_A :: DIN11_8_DIO9_ONE , } } # [doc = "ZERO"] # [inline (always)] pub fn is_din11_8_dio9_zero (& self) -> bool { * self == DIN11_8_DIO9_A :: DIN11_8_DIO9_ZERO } # [doc = "ONE"] # [inline (always)] pub fn is_din11_8_dio9_one (& self) -> bool { * self == DIN11_8_DIO9_A :: DIN11_8_DIO9_ONE } } # [doc = "Field `DIN11_8_DIO10` reader - This bit reads the data input value of DIO10."] pub type DIN11_8_DIO10_R = crate :: BitReader < DIN11_8_DIO10_A > ; # [doc = "This bit reads the data input value of DIO10.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DIN11_8_DIO10_A { # [doc = "0: ZERO"] DIN11_8_DIO10_ZERO = 0 , # [doc = "1: ONE"] DIN11_8_DIO10_ONE = 1 , } impl From < DIN11_8_DIO10_A > for bool { # [inline (always)] fn from (variant : DIN11_8_DIO10_A) -> Self { variant as u8 != 0 } } impl DIN11_8_DIO10_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> DIN11_8_DIO10_A { match self . bits { false => DIN11_8_DIO10_A :: DIN11_8_DIO10_ZERO , true => DIN11_8_DIO10_A :: DIN11_8_DIO10_ONE , } } # [doc = "ZERO"] # [inline (always)] pub fn is_din11_8_dio10_zero (& self) -> bool { * self == DIN11_8_DIO10_A :: DIN11_8_DIO10_ZERO } # [doc = "ONE"] # [inline (always)] pub fn is_din11_8_dio10_one (& self) -> bool { * self == DIN11_8_DIO10_A :: DIN11_8_DIO10_ONE } } # [doc = "Field `DIN11_8_DIO11` reader - This bit reads the data input value of DIO11."] pub type DIN11_8_DIO11_R = crate :: BitReader < DIN11_8_DIO11_A > ; # [doc = "This bit reads the data input value of DIO11.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DIN11_8_DIO11_A { # [doc = "0: ZERO"] DIN11_8_DIO11_ZERO = 0 , # [doc = "1: ONE"] DIN11_8_DIO11_ONE = 1 , } impl From < DIN11_8_DIO11_A > for bool { # [inline (always)] fn from (variant : DIN11_8_DIO11_A) -> Self { variant as u8 != 0 } } impl DIN11_8_DIO11_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> DIN11_8_DIO11_A { match self . bits { false => DIN11_8_DIO11_A :: DIN11_8_DIO11_ZERO , true => DIN11_8_DIO11_A :: DIN11_8_DIO11_ONE , } } # [doc = "ZERO"] # [inline (always)] pub fn is_din11_8_dio11_zero (& self) -> bool { * self == DIN11_8_DIO11_A :: DIN11_8_DIO11_ZERO } # [doc = "ONE"] # [inline (always)] pub fn is_din11_8_dio11_one (& self) -> bool { * self == DIN11_8_DIO11_A :: DIN11_8_DIO11_ONE } } impl R { # [doc = "Bit 0 - This bit reads the data input value of DIO8."] # [inline (always)] pub fn din11_8_dio8 (& self) -> DIN11_8_DIO8_R { DIN11_8_DIO8_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 8 - This bit reads the data input value of DIO9."] # [inline (always)] pub fn din11_8_dio9 (& self) -> DIN11_8_DIO9_R { DIN11_8_DIO9_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 16 - This bit reads the data input value of DIO10."] # [inline (always)] pub fn din11_8_dio10 (& self) -> DIN11_8_DIO10_R { DIN11_8_DIO10_R :: new (((self . bits >> 16) & 1) != 0) } # [doc = "Bit 24 - This bit reads the data input value of DIO11."] # [inline (always)] pub fn din11_8_dio11 (& self) -> DIN11_8_DIO11_R { DIN11_8_DIO11_R :: new (((self . bits >> 24) & 1) != 0) } } # [doc = "Data input 11 to 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din11_8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct DIN11_8_SPEC ; impl crate :: RegisterSpec for DIN11_8_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`din11_8::R`](R) reader structure"] impl crate :: Readable for DIN11_8_SPEC { } # [doc = "`reset()` method sets DIN11_8 to value 0"] impl crate :: Resettable for DIN11_8_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }