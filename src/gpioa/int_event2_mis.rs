# [doc = "Register `INT_EVENT2_MIS` reader"] pub type R = crate :: R < INT_EVENT2_MIS_SPEC > ; # [doc = "Field `INT_EVENT2_MIS_DIO16` reader - DIO16 event"] pub type INT_EVENT2_MIS_DIO16_R = crate :: BitReader < INT_EVENT2_MIS_DIO16_A > ; # [doc = "DIO16 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO16_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO16_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO16_SET = 1 , } impl From < INT_EVENT2_MIS_DIO16_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO16_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO16_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO16_A { match self . bits { false => INT_EVENT2_MIS_DIO16_A :: INT_EVENT2_MIS_DIO16_CLR , true => INT_EVENT2_MIS_DIO16_A :: INT_EVENT2_MIS_DIO16_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio16_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO16_A :: INT_EVENT2_MIS_DIO16_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio16_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO16_A :: INT_EVENT2_MIS_DIO16_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO17` reader - DIO17 event"] pub type INT_EVENT2_MIS_DIO17_R = crate :: BitReader < INT_EVENT2_MIS_DIO17_A > ; # [doc = "DIO17 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO17_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO17_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO17_SET = 1 , } impl From < INT_EVENT2_MIS_DIO17_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO17_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO17_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO17_A { match self . bits { false => INT_EVENT2_MIS_DIO17_A :: INT_EVENT2_MIS_DIO17_CLR , true => INT_EVENT2_MIS_DIO17_A :: INT_EVENT2_MIS_DIO17_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio17_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO17_A :: INT_EVENT2_MIS_DIO17_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio17_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO17_A :: INT_EVENT2_MIS_DIO17_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO18` reader - DIO18 event"] pub type INT_EVENT2_MIS_DIO18_R = crate :: BitReader < INT_EVENT2_MIS_DIO18_A > ; # [doc = "DIO18 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO18_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO18_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO18_SET = 1 , } impl From < INT_EVENT2_MIS_DIO18_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO18_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO18_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO18_A { match self . bits { false => INT_EVENT2_MIS_DIO18_A :: INT_EVENT2_MIS_DIO18_CLR , true => INT_EVENT2_MIS_DIO18_A :: INT_EVENT2_MIS_DIO18_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio18_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO18_A :: INT_EVENT2_MIS_DIO18_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio18_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO18_A :: INT_EVENT2_MIS_DIO18_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO19` reader - DIO19 event"] pub type INT_EVENT2_MIS_DIO19_R = crate :: BitReader < INT_EVENT2_MIS_DIO19_A > ; # [doc = "DIO19 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO19_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO19_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO19_SET = 1 , } impl From < INT_EVENT2_MIS_DIO19_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO19_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO19_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO19_A { match self . bits { false => INT_EVENT2_MIS_DIO19_A :: INT_EVENT2_MIS_DIO19_CLR , true => INT_EVENT2_MIS_DIO19_A :: INT_EVENT2_MIS_DIO19_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio19_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO19_A :: INT_EVENT2_MIS_DIO19_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio19_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO19_A :: INT_EVENT2_MIS_DIO19_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO20` reader - DIO20 event"] pub type INT_EVENT2_MIS_DIO20_R = crate :: BitReader < INT_EVENT2_MIS_DIO20_A > ; # [doc = "DIO20 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO20_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO20_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO20_SET = 1 , } impl From < INT_EVENT2_MIS_DIO20_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO20_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO20_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO20_A { match self . bits { false => INT_EVENT2_MIS_DIO20_A :: INT_EVENT2_MIS_DIO20_CLR , true => INT_EVENT2_MIS_DIO20_A :: INT_EVENT2_MIS_DIO20_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio20_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO20_A :: INT_EVENT2_MIS_DIO20_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio20_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO20_A :: INT_EVENT2_MIS_DIO20_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO21` reader - DIO21 event"] pub type INT_EVENT2_MIS_DIO21_R = crate :: BitReader < INT_EVENT2_MIS_DIO21_A > ; # [doc = "DIO21 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO21_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO21_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO21_SET = 1 , } impl From < INT_EVENT2_MIS_DIO21_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO21_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO21_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO21_A { match self . bits { false => INT_EVENT2_MIS_DIO21_A :: INT_EVENT2_MIS_DIO21_CLR , true => INT_EVENT2_MIS_DIO21_A :: INT_EVENT2_MIS_DIO21_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio21_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO21_A :: INT_EVENT2_MIS_DIO21_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio21_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO21_A :: INT_EVENT2_MIS_DIO21_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO22` reader - DIO22 event"] pub type INT_EVENT2_MIS_DIO22_R = crate :: BitReader < INT_EVENT2_MIS_DIO22_A > ; # [doc = "DIO22 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO22_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO22_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO22_SET = 1 , } impl From < INT_EVENT2_MIS_DIO22_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO22_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO22_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO22_A { match self . bits { false => INT_EVENT2_MIS_DIO22_A :: INT_EVENT2_MIS_DIO22_CLR , true => INT_EVENT2_MIS_DIO22_A :: INT_EVENT2_MIS_DIO22_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio22_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO22_A :: INT_EVENT2_MIS_DIO22_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio22_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO22_A :: INT_EVENT2_MIS_DIO22_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO23` reader - DIO23 event"] pub type INT_EVENT2_MIS_DIO23_R = crate :: BitReader < INT_EVENT2_MIS_DIO23_A > ; # [doc = "DIO23 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO23_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO23_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO23_SET = 1 , } impl From < INT_EVENT2_MIS_DIO23_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO23_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO23_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO23_A { match self . bits { false => INT_EVENT2_MIS_DIO23_A :: INT_EVENT2_MIS_DIO23_CLR , true => INT_EVENT2_MIS_DIO23_A :: INT_EVENT2_MIS_DIO23_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio23_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO23_A :: INT_EVENT2_MIS_DIO23_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio23_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO23_A :: INT_EVENT2_MIS_DIO23_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO24` reader - DIO24 event"] pub type INT_EVENT2_MIS_DIO24_R = crate :: BitReader < INT_EVENT2_MIS_DIO24_A > ; # [doc = "DIO24 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO24_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO24_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO24_SET = 1 , } impl From < INT_EVENT2_MIS_DIO24_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO24_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO24_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO24_A { match self . bits { false => INT_EVENT2_MIS_DIO24_A :: INT_EVENT2_MIS_DIO24_CLR , true => INT_EVENT2_MIS_DIO24_A :: INT_EVENT2_MIS_DIO24_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio24_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO24_A :: INT_EVENT2_MIS_DIO24_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio24_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO24_A :: INT_EVENT2_MIS_DIO24_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO25` reader - DIO25 event"] pub type INT_EVENT2_MIS_DIO25_R = crate :: BitReader < INT_EVENT2_MIS_DIO25_A > ; # [doc = "DIO25 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO25_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO25_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO25_SET = 1 , } impl From < INT_EVENT2_MIS_DIO25_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO25_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO25_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO25_A { match self . bits { false => INT_EVENT2_MIS_DIO25_A :: INT_EVENT2_MIS_DIO25_CLR , true => INT_EVENT2_MIS_DIO25_A :: INT_EVENT2_MIS_DIO25_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio25_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO25_A :: INT_EVENT2_MIS_DIO25_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio25_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO25_A :: INT_EVENT2_MIS_DIO25_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO26` reader - DIO26 event"] pub type INT_EVENT2_MIS_DIO26_R = crate :: BitReader < INT_EVENT2_MIS_DIO26_A > ; # [doc = "DIO26 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO26_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO26_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO26_SET = 1 , } impl From < INT_EVENT2_MIS_DIO26_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO26_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO26_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO26_A { match self . bits { false => INT_EVENT2_MIS_DIO26_A :: INT_EVENT2_MIS_DIO26_CLR , true => INT_EVENT2_MIS_DIO26_A :: INT_EVENT2_MIS_DIO26_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio26_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO26_A :: INT_EVENT2_MIS_DIO26_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio26_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO26_A :: INT_EVENT2_MIS_DIO26_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO27` reader - DIO27 event"] pub type INT_EVENT2_MIS_DIO27_R = crate :: BitReader < INT_EVENT2_MIS_DIO27_A > ; # [doc = "DIO27 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO27_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO27_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO27_SET = 1 , } impl From < INT_EVENT2_MIS_DIO27_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO27_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO27_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO27_A { match self . bits { false => INT_EVENT2_MIS_DIO27_A :: INT_EVENT2_MIS_DIO27_CLR , true => INT_EVENT2_MIS_DIO27_A :: INT_EVENT2_MIS_DIO27_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio27_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO27_A :: INT_EVENT2_MIS_DIO27_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio27_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO27_A :: INT_EVENT2_MIS_DIO27_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO28` reader - DIO28 event"] pub type INT_EVENT2_MIS_DIO28_R = crate :: BitReader < INT_EVENT2_MIS_DIO28_A > ; # [doc = "DIO28 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO28_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO28_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO28_SET = 1 , } impl From < INT_EVENT2_MIS_DIO28_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO28_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO28_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO28_A { match self . bits { false => INT_EVENT2_MIS_DIO28_A :: INT_EVENT2_MIS_DIO28_CLR , true => INT_EVENT2_MIS_DIO28_A :: INT_EVENT2_MIS_DIO28_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio28_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO28_A :: INT_EVENT2_MIS_DIO28_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio28_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO28_A :: INT_EVENT2_MIS_DIO28_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO29` reader - DIO29 event"] pub type INT_EVENT2_MIS_DIO29_R = crate :: BitReader < INT_EVENT2_MIS_DIO29_A > ; # [doc = "DIO29 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO29_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO29_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO29_SET = 1 , } impl From < INT_EVENT2_MIS_DIO29_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO29_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO29_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO29_A { match self . bits { false => INT_EVENT2_MIS_DIO29_A :: INT_EVENT2_MIS_DIO29_CLR , true => INT_EVENT2_MIS_DIO29_A :: INT_EVENT2_MIS_DIO29_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio29_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO29_A :: INT_EVENT2_MIS_DIO29_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio29_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO29_A :: INT_EVENT2_MIS_DIO29_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO30` reader - DIO30 event"] pub type INT_EVENT2_MIS_DIO30_R = crate :: BitReader < INT_EVENT2_MIS_DIO30_A > ; # [doc = "DIO30 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO30_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO30_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO30_SET = 1 , } impl From < INT_EVENT2_MIS_DIO30_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO30_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO30_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO30_A { match self . bits { false => INT_EVENT2_MIS_DIO30_A :: INT_EVENT2_MIS_DIO30_CLR , true => INT_EVENT2_MIS_DIO30_A :: INT_EVENT2_MIS_DIO30_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio30_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO30_A :: INT_EVENT2_MIS_DIO30_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio30_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO30_A :: INT_EVENT2_MIS_DIO30_SET } } # [doc = "Field `INT_EVENT2_MIS_DIO31` reader - DIO31 event"] pub type INT_EVENT2_MIS_DIO31_R = crate :: BitReader < INT_EVENT2_MIS_DIO31_A > ; # [doc = "DIO31 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_MIS_DIO31_A { # [doc = "0: CLR"] INT_EVENT2_MIS_DIO31_CLR = 0 , # [doc = "1: SET"] INT_EVENT2_MIS_DIO31_SET = 1 , } impl From < INT_EVENT2_MIS_DIO31_A > for bool { # [inline (always)] fn from (variant : INT_EVENT2_MIS_DIO31_A) -> Self { variant as u8 != 0 } } impl INT_EVENT2_MIS_DIO31_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT2_MIS_DIO31_A { match self . bits { false => INT_EVENT2_MIS_DIO31_A :: INT_EVENT2_MIS_DIO31_CLR , true => INT_EVENT2_MIS_DIO31_A :: INT_EVENT2_MIS_DIO31_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event2_mis_dio31_clr (& self) -> bool { * self == INT_EVENT2_MIS_DIO31_A :: INT_EVENT2_MIS_DIO31_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event2_mis_dio31_set (& self) -> bool { * self == INT_EVENT2_MIS_DIO31_A :: INT_EVENT2_MIS_DIO31_SET } } impl R { # [doc = "Bit 16 - DIO16 event"] # [inline (always)] pub fn int_event2_mis_dio16 (& self) -> INT_EVENT2_MIS_DIO16_R { INT_EVENT2_MIS_DIO16_R :: new (((self . bits >> 16) & 1) != 0) } # [doc = "Bit 17 - DIO17 event"] # [inline (always)] pub fn int_event2_mis_dio17 (& self) -> INT_EVENT2_MIS_DIO17_R { INT_EVENT2_MIS_DIO17_R :: new (((self . bits >> 17) & 1) != 0) } # [doc = "Bit 18 - DIO18 event"] # [inline (always)] pub fn int_event2_mis_dio18 (& self) -> INT_EVENT2_MIS_DIO18_R { INT_EVENT2_MIS_DIO18_R :: new (((self . bits >> 18) & 1) != 0) } # [doc = "Bit 19 - DIO19 event"] # [inline (always)] pub fn int_event2_mis_dio19 (& self) -> INT_EVENT2_MIS_DIO19_R { INT_EVENT2_MIS_DIO19_R :: new (((self . bits >> 19) & 1) != 0) } # [doc = "Bit 20 - DIO20 event"] # [inline (always)] pub fn int_event2_mis_dio20 (& self) -> INT_EVENT2_MIS_DIO20_R { INT_EVENT2_MIS_DIO20_R :: new (((self . bits >> 20) & 1) != 0) } # [doc = "Bit 21 - DIO21 event"] # [inline (always)] pub fn int_event2_mis_dio21 (& self) -> INT_EVENT2_MIS_DIO21_R { INT_EVENT2_MIS_DIO21_R :: new (((self . bits >> 21) & 1) != 0) } # [doc = "Bit 22 - DIO22 event"] # [inline (always)] pub fn int_event2_mis_dio22 (& self) -> INT_EVENT2_MIS_DIO22_R { INT_EVENT2_MIS_DIO22_R :: new (((self . bits >> 22) & 1) != 0) } # [doc = "Bit 23 - DIO23 event"] # [inline (always)] pub fn int_event2_mis_dio23 (& self) -> INT_EVENT2_MIS_DIO23_R { INT_EVENT2_MIS_DIO23_R :: new (((self . bits >> 23) & 1) != 0) } # [doc = "Bit 24 - DIO24 event"] # [inline (always)] pub fn int_event2_mis_dio24 (& self) -> INT_EVENT2_MIS_DIO24_R { INT_EVENT2_MIS_DIO24_R :: new (((self . bits >> 24) & 1) != 0) } # [doc = "Bit 25 - DIO25 event"] # [inline (always)] pub fn int_event2_mis_dio25 (& self) -> INT_EVENT2_MIS_DIO25_R { INT_EVENT2_MIS_DIO25_R :: new (((self . bits >> 25) & 1) != 0) } # [doc = "Bit 26 - DIO26 event"] # [inline (always)] pub fn int_event2_mis_dio26 (& self) -> INT_EVENT2_MIS_DIO26_R { INT_EVENT2_MIS_DIO26_R :: new (((self . bits >> 26) & 1) != 0) } # [doc = "Bit 27 - DIO27 event"] # [inline (always)] pub fn int_event2_mis_dio27 (& self) -> INT_EVENT2_MIS_DIO27_R { INT_EVENT2_MIS_DIO27_R :: new (((self . bits >> 27) & 1) != 0) } # [doc = "Bit 28 - DIO28 event"] # [inline (always)] pub fn int_event2_mis_dio28 (& self) -> INT_EVENT2_MIS_DIO28_R { INT_EVENT2_MIS_DIO28_R :: new (((self . bits >> 28) & 1) != 0) } # [doc = "Bit 29 - DIO29 event"] # [inline (always)] pub fn int_event2_mis_dio29 (& self) -> INT_EVENT2_MIS_DIO29_R { INT_EVENT2_MIS_DIO29_R :: new (((self . bits >> 29) & 1) != 0) } # [doc = "Bit 30 - DIO30 event"] # [inline (always)] pub fn int_event2_mis_dio30 (& self) -> INT_EVENT2_MIS_DIO30_R { INT_EVENT2_MIS_DIO30_R :: new (((self . bits >> 30) & 1) != 0) } # [doc = "Bit 31 - DIO31 event"] # [inline (always)] pub fn int_event2_mis_dio31 (& self) -> INT_EVENT2_MIS_DIO31_R { INT_EVENT2_MIS_DIO31_R :: new (((self . bits >> 31) & 1) != 0) } } # [doc = "Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event2_mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT2_MIS_SPEC ; impl crate :: RegisterSpec for INT_EVENT2_MIS_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event2_mis::R`](R) reader structure"] impl crate :: Readable for INT_EVENT2_MIS_SPEC { } # [doc = "`reset()` method sets INT_EVENT2_MIS to value 0"] impl crate :: Resettable for INT_EVENT2_MIS_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }