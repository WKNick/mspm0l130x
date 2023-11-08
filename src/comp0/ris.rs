# [doc = "Register `RIS` reader"] pub type R = crate :: R < RIS_SPEC > ; # [doc = "Field `RIS_COMPIFG` reader - Raw interrupt status for comparator output interrupt flag. The IES bit defines the transition of the comparator output setting this bit."] pub type RIS_COMPIFG_R = crate :: BitReader < RIS_COMPIFG_A > ; # [doc = "Raw interrupt status for comparator output interrupt flag. The IES bit defines the transition of the comparator output setting this bit.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RIS_COMPIFG_A { # [doc = "0: CLR"] RIS_COMPIFG_CLR = 0 , # [doc = "1: SET"] RIS_COMPIFG_SET = 1 , } impl From < RIS_COMPIFG_A > for bool { # [inline (always)] fn from (variant : RIS_COMPIFG_A) -> Self { variant as u8 != 0 } } impl RIS_COMPIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> RIS_COMPIFG_A { match self . bits { false => RIS_COMPIFG_A :: RIS_COMPIFG_CLR , true => RIS_COMPIFG_A :: RIS_COMPIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_ris_compifg_clr (& self) -> bool { * self == RIS_COMPIFG_A :: RIS_COMPIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_ris_compifg_set (& self) -> bool { * self == RIS_COMPIFG_A :: RIS_COMPIFG_SET } } # [doc = "Field `RIS_COMPINVIFG` reader - Raw interrupt status for comparator output inverted interrupt flag. The IES bit defines the transition of the comparator output setting this bit."] pub type RIS_COMPINVIFG_R = crate :: BitReader < RIS_COMPINVIFG_A > ; # [doc = "Raw interrupt status for comparator output inverted interrupt flag. The IES bit defines the transition of the comparator output setting this bit.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RIS_COMPINVIFG_A { # [doc = "0: CLR"] RIS_COMPINVIFG_CLR = 0 , # [doc = "1: SET"] RIS_COMPINVIFG_SET = 1 , } impl From < RIS_COMPINVIFG_A > for bool { # [inline (always)] fn from (variant : RIS_COMPINVIFG_A) -> Self { variant as u8 != 0 } } impl RIS_COMPINVIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> RIS_COMPINVIFG_A { match self . bits { false => RIS_COMPINVIFG_A :: RIS_COMPINVIFG_CLR , true => RIS_COMPINVIFG_A :: RIS_COMPINVIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_ris_compinvifg_clr (& self) -> bool { * self == RIS_COMPINVIFG_A :: RIS_COMPINVIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_ris_compinvifg_set (& self) -> bool { * self == RIS_COMPINVIFG_A :: RIS_COMPINVIFG_SET } } # [doc = "Field `RIS_OUTRDYIFG` reader - Raw interrupt status for comparator output ready interrupt flag. This bit is set when the comparator output is valid."] pub type RIS_OUTRDYIFG_R = crate :: BitReader < RIS_OUTRDYIFG_A > ; # [doc = "Raw interrupt status for comparator output ready interrupt flag. This bit is set when the comparator output is valid.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum RIS_OUTRDYIFG_A { # [doc = "0: CLR"] RIS_OUTRDYIFG_CLR = 0 , # [doc = "1: SET"] RIS_OUTRDYIFG_SET = 1 , } impl From < RIS_OUTRDYIFG_A > for bool { # [inline (always)] fn from (variant : RIS_OUTRDYIFG_A) -> Self { variant as u8 != 0 } } impl RIS_OUTRDYIFG_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> RIS_OUTRDYIFG_A { match self . bits { false => RIS_OUTRDYIFG_A :: RIS_OUTRDYIFG_CLR , true => RIS_OUTRDYIFG_A :: RIS_OUTRDYIFG_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_ris_outrdyifg_clr (& self) -> bool { * self == RIS_OUTRDYIFG_A :: RIS_OUTRDYIFG_CLR } # [doc = "SET"] # [inline (always)] pub fn is_ris_outrdyifg_set (& self) -> bool { * self == RIS_OUTRDYIFG_A :: RIS_OUTRDYIFG_SET } } impl R { # [doc = "Bit 1 - Raw interrupt status for comparator output interrupt flag. The IES bit defines the transition of the comparator output setting this bit."] # [inline (always)] pub fn ris_compifg (& self) -> RIS_COMPIFG_R { RIS_COMPIFG_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Raw interrupt status for comparator output inverted interrupt flag. The IES bit defines the transition of the comparator output setting this bit."] # [inline (always)] pub fn ris_compinvifg (& self) -> RIS_COMPINVIFG_R { RIS_COMPINVIFG_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Raw interrupt status for comparator output ready interrupt flag. This bit is set when the comparator output is valid."] # [inline (always)] pub fn ris_outrdyifg (& self) -> RIS_OUTRDYIFG_R { RIS_OUTRDYIFG_R :: new (((self . bits >> 3) & 1) != 0) } } # [doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct RIS_SPEC ; impl crate :: RegisterSpec for RIS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`ris::R`](R) reader structure"] impl crate :: Readable for RIS_SPEC { } # [doc = "`reset()` method sets RIS to value 0"] impl crate :: Resettable for RIS_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }