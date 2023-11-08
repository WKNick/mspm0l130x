# [doc = "Register `INT_EVENT1_IMASK` reader"] pub type R = crate :: R < INT_EVENT1_IMASK_SPEC > ; # [doc = "Register `INT_EVENT1_IMASK` writer"] pub type W = crate :: W < INT_EVENT1_IMASK_SPEC > ; # [doc = "Field `INT_EVENT1_IMASK_DIO0` reader - DIO0 event mask"] pub type INT_EVENT1_IMASK_DIO0_R = crate :: BitReader < INT_EVENT1_IMASK_DIO0_A > ; # [doc = "DIO0 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO0_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO0_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO0_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO0_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO0_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO0_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO0_A { match self . bits { false => INT_EVENT1_IMASK_DIO0_A :: INT_EVENT1_IMASK_DIO0_CLR , true => INT_EVENT1_IMASK_DIO0_A :: INT_EVENT1_IMASK_DIO0_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio0_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO0_A :: INT_EVENT1_IMASK_DIO0_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio0_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO0_A :: INT_EVENT1_IMASK_DIO0_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO0` writer - DIO0 event mask"] pub type INT_EVENT1_IMASK_DIO0_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO0_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO0_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio0_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO0_A :: INT_EVENT1_IMASK_DIO0_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio0_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO0_A :: INT_EVENT1_IMASK_DIO0_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO1` reader - DIO1 event mask"] pub type INT_EVENT1_IMASK_DIO1_R = crate :: BitReader < INT_EVENT1_IMASK_DIO1_A > ; # [doc = "DIO1 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO1_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO1_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO1_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO1_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO1_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO1_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO1_A { match self . bits { false => INT_EVENT1_IMASK_DIO1_A :: INT_EVENT1_IMASK_DIO1_CLR , true => INT_EVENT1_IMASK_DIO1_A :: INT_EVENT1_IMASK_DIO1_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio1_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO1_A :: INT_EVENT1_IMASK_DIO1_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio1_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO1_A :: INT_EVENT1_IMASK_DIO1_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO1` writer - DIO1 event mask"] pub type INT_EVENT1_IMASK_DIO1_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO1_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO1_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio1_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO1_A :: INT_EVENT1_IMASK_DIO1_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio1_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO1_A :: INT_EVENT1_IMASK_DIO1_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO2` reader - DIO2 event mask"] pub type INT_EVENT1_IMASK_DIO2_R = crate :: BitReader < INT_EVENT1_IMASK_DIO2_A > ; # [doc = "DIO2 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO2_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO2_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO2_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO2_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO2_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO2_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO2_A { match self . bits { false => INT_EVENT1_IMASK_DIO2_A :: INT_EVENT1_IMASK_DIO2_CLR , true => INT_EVENT1_IMASK_DIO2_A :: INT_EVENT1_IMASK_DIO2_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio2_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO2_A :: INT_EVENT1_IMASK_DIO2_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio2_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO2_A :: INT_EVENT1_IMASK_DIO2_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO2` writer - DIO2 event mask"] pub type INT_EVENT1_IMASK_DIO2_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO2_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO2_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio2_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO2_A :: INT_EVENT1_IMASK_DIO2_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio2_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO2_A :: INT_EVENT1_IMASK_DIO2_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO3` reader - DIO3 event mask"] pub type INT_EVENT1_IMASK_DIO3_R = crate :: BitReader < INT_EVENT1_IMASK_DIO3_A > ; # [doc = "DIO3 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO3_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO3_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO3_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO3_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO3_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO3_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO3_A { match self . bits { false => INT_EVENT1_IMASK_DIO3_A :: INT_EVENT1_IMASK_DIO3_CLR , true => INT_EVENT1_IMASK_DIO3_A :: INT_EVENT1_IMASK_DIO3_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio3_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO3_A :: INT_EVENT1_IMASK_DIO3_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio3_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO3_A :: INT_EVENT1_IMASK_DIO3_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO3` writer - DIO3 event mask"] pub type INT_EVENT1_IMASK_DIO3_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO3_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO3_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio3_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO3_A :: INT_EVENT1_IMASK_DIO3_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio3_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO3_A :: INT_EVENT1_IMASK_DIO3_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO4` reader - DIO4 event mask"] pub type INT_EVENT1_IMASK_DIO4_R = crate :: BitReader < INT_EVENT1_IMASK_DIO4_A > ; # [doc = "DIO4 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO4_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO4_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO4_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO4_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO4_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO4_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO4_A { match self . bits { false => INT_EVENT1_IMASK_DIO4_A :: INT_EVENT1_IMASK_DIO4_CLR , true => INT_EVENT1_IMASK_DIO4_A :: INT_EVENT1_IMASK_DIO4_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio4_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO4_A :: INT_EVENT1_IMASK_DIO4_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio4_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO4_A :: INT_EVENT1_IMASK_DIO4_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO4` writer - DIO4 event mask"] pub type INT_EVENT1_IMASK_DIO4_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO4_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO4_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio4_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO4_A :: INT_EVENT1_IMASK_DIO4_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio4_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO4_A :: INT_EVENT1_IMASK_DIO4_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO5` reader - DIO5 event mask"] pub type INT_EVENT1_IMASK_DIO5_R = crate :: BitReader < INT_EVENT1_IMASK_DIO5_A > ; # [doc = "DIO5 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO5_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO5_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO5_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO5_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO5_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO5_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO5_A { match self . bits { false => INT_EVENT1_IMASK_DIO5_A :: INT_EVENT1_IMASK_DIO5_CLR , true => INT_EVENT1_IMASK_DIO5_A :: INT_EVENT1_IMASK_DIO5_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio5_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO5_A :: INT_EVENT1_IMASK_DIO5_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio5_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO5_A :: INT_EVENT1_IMASK_DIO5_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO5` writer - DIO5 event mask"] pub type INT_EVENT1_IMASK_DIO5_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO5_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO5_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio5_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO5_A :: INT_EVENT1_IMASK_DIO5_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio5_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO5_A :: INT_EVENT1_IMASK_DIO5_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO6` reader - DIO6 event mask"] pub type INT_EVENT1_IMASK_DIO6_R = crate :: BitReader < INT_EVENT1_IMASK_DIO6_A > ; # [doc = "DIO6 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO6_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO6_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO6_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO6_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO6_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO6_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO6_A { match self . bits { false => INT_EVENT1_IMASK_DIO6_A :: INT_EVENT1_IMASK_DIO6_CLR , true => INT_EVENT1_IMASK_DIO6_A :: INT_EVENT1_IMASK_DIO6_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio6_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO6_A :: INT_EVENT1_IMASK_DIO6_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio6_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO6_A :: INT_EVENT1_IMASK_DIO6_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO6` writer - DIO6 event mask"] pub type INT_EVENT1_IMASK_DIO6_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO6_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO6_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio6_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO6_A :: INT_EVENT1_IMASK_DIO6_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio6_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO6_A :: INT_EVENT1_IMASK_DIO6_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO7` reader - DIO7 event mask"] pub type INT_EVENT1_IMASK_DIO7_R = crate :: BitReader < INT_EVENT1_IMASK_DIO7_A > ; # [doc = "DIO7 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO7_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO7_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO7_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO7_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO7_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO7_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO7_A { match self . bits { false => INT_EVENT1_IMASK_DIO7_A :: INT_EVENT1_IMASK_DIO7_CLR , true => INT_EVENT1_IMASK_DIO7_A :: INT_EVENT1_IMASK_DIO7_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio7_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO7_A :: INT_EVENT1_IMASK_DIO7_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio7_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO7_A :: INT_EVENT1_IMASK_DIO7_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO7` writer - DIO7 event mask"] pub type INT_EVENT1_IMASK_DIO7_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO7_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO7_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio7_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO7_A :: INT_EVENT1_IMASK_DIO7_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio7_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO7_A :: INT_EVENT1_IMASK_DIO7_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO8` reader - DIO8 event mask"] pub type INT_EVENT1_IMASK_DIO8_R = crate :: BitReader < INT_EVENT1_IMASK_DIO8_A > ; # [doc = "DIO8 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO8_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO8_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO8_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO8_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO8_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO8_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO8_A { match self . bits { false => INT_EVENT1_IMASK_DIO8_A :: INT_EVENT1_IMASK_DIO8_CLR , true => INT_EVENT1_IMASK_DIO8_A :: INT_EVENT1_IMASK_DIO8_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio8_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO8_A :: INT_EVENT1_IMASK_DIO8_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio8_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO8_A :: INT_EVENT1_IMASK_DIO8_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO8` writer - DIO8 event mask"] pub type INT_EVENT1_IMASK_DIO8_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO8_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO8_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio8_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO8_A :: INT_EVENT1_IMASK_DIO8_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio8_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO8_A :: INT_EVENT1_IMASK_DIO8_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO9` reader - DIO9 event mask"] pub type INT_EVENT1_IMASK_DIO9_R = crate :: BitReader < INT_EVENT1_IMASK_DIO9_A > ; # [doc = "DIO9 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO9_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO9_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO9_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO9_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO9_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO9_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO9_A { match self . bits { false => INT_EVENT1_IMASK_DIO9_A :: INT_EVENT1_IMASK_DIO9_CLR , true => INT_EVENT1_IMASK_DIO9_A :: INT_EVENT1_IMASK_DIO9_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio9_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO9_A :: INT_EVENT1_IMASK_DIO9_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio9_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO9_A :: INT_EVENT1_IMASK_DIO9_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO9` writer - DIO9 event mask"] pub type INT_EVENT1_IMASK_DIO9_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO9_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO9_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio9_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO9_A :: INT_EVENT1_IMASK_DIO9_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio9_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO9_A :: INT_EVENT1_IMASK_DIO9_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO10` reader - DIO10 event mask"] pub type INT_EVENT1_IMASK_DIO10_R = crate :: BitReader < INT_EVENT1_IMASK_DIO10_A > ; # [doc = "DIO10 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO10_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO10_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO10_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO10_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO10_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO10_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO10_A { match self . bits { false => INT_EVENT1_IMASK_DIO10_A :: INT_EVENT1_IMASK_DIO10_CLR , true => INT_EVENT1_IMASK_DIO10_A :: INT_EVENT1_IMASK_DIO10_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio10_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO10_A :: INT_EVENT1_IMASK_DIO10_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio10_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO10_A :: INT_EVENT1_IMASK_DIO10_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO10` writer - DIO10 event mask"] pub type INT_EVENT1_IMASK_DIO10_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO10_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO10_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio10_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO10_A :: INT_EVENT1_IMASK_DIO10_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio10_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO10_A :: INT_EVENT1_IMASK_DIO10_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO11` reader - DIO11 event mask"] pub type INT_EVENT1_IMASK_DIO11_R = crate :: BitReader < INT_EVENT1_IMASK_DIO11_A > ; # [doc = "DIO11 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO11_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO11_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO11_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO11_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO11_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO11_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO11_A { match self . bits { false => INT_EVENT1_IMASK_DIO11_A :: INT_EVENT1_IMASK_DIO11_CLR , true => INT_EVENT1_IMASK_DIO11_A :: INT_EVENT1_IMASK_DIO11_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio11_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO11_A :: INT_EVENT1_IMASK_DIO11_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio11_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO11_A :: INT_EVENT1_IMASK_DIO11_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO11` writer - DIO11 event mask"] pub type INT_EVENT1_IMASK_DIO11_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO11_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO11_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio11_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO11_A :: INT_EVENT1_IMASK_DIO11_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio11_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO11_A :: INT_EVENT1_IMASK_DIO11_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO12` reader - DIO12 event mask"] pub type INT_EVENT1_IMASK_DIO12_R = crate :: BitReader < INT_EVENT1_IMASK_DIO12_A > ; # [doc = "DIO12 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO12_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO12_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO12_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO12_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO12_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO12_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO12_A { match self . bits { false => INT_EVENT1_IMASK_DIO12_A :: INT_EVENT1_IMASK_DIO12_CLR , true => INT_EVENT1_IMASK_DIO12_A :: INT_EVENT1_IMASK_DIO12_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio12_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO12_A :: INT_EVENT1_IMASK_DIO12_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio12_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO12_A :: INT_EVENT1_IMASK_DIO12_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO12` writer - DIO12 event mask"] pub type INT_EVENT1_IMASK_DIO12_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO12_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO12_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio12_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO12_A :: INT_EVENT1_IMASK_DIO12_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio12_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO12_A :: INT_EVENT1_IMASK_DIO12_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO13` reader - DIO13 event mask"] pub type INT_EVENT1_IMASK_DIO13_R = crate :: BitReader < INT_EVENT1_IMASK_DIO13_A > ; # [doc = "DIO13 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO13_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO13_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO13_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO13_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO13_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO13_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO13_A { match self . bits { false => INT_EVENT1_IMASK_DIO13_A :: INT_EVENT1_IMASK_DIO13_CLR , true => INT_EVENT1_IMASK_DIO13_A :: INT_EVENT1_IMASK_DIO13_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio13_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO13_A :: INT_EVENT1_IMASK_DIO13_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio13_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO13_A :: INT_EVENT1_IMASK_DIO13_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO13` writer - DIO13 event mask"] pub type INT_EVENT1_IMASK_DIO13_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO13_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO13_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio13_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO13_A :: INT_EVENT1_IMASK_DIO13_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio13_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO13_A :: INT_EVENT1_IMASK_DIO13_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO14` reader - DIO14 event mask"] pub type INT_EVENT1_IMASK_DIO14_R = crate :: BitReader < INT_EVENT1_IMASK_DIO14_A > ; # [doc = "DIO14 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO14_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO14_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO14_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO14_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO14_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO14_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO14_A { match self . bits { false => INT_EVENT1_IMASK_DIO14_A :: INT_EVENT1_IMASK_DIO14_CLR , true => INT_EVENT1_IMASK_DIO14_A :: INT_EVENT1_IMASK_DIO14_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio14_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO14_A :: INT_EVENT1_IMASK_DIO14_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio14_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO14_A :: INT_EVENT1_IMASK_DIO14_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO14` writer - DIO14 event mask"] pub type INT_EVENT1_IMASK_DIO14_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO14_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO14_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio14_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO14_A :: INT_EVENT1_IMASK_DIO14_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio14_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO14_A :: INT_EVENT1_IMASK_DIO14_SET) } } # [doc = "Field `INT_EVENT1_IMASK_DIO15` reader - DIO15 event mask"] pub type INT_EVENT1_IMASK_DIO15_R = crate :: BitReader < INT_EVENT1_IMASK_DIO15_A > ; # [doc = "DIO15 event mask\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT1_IMASK_DIO15_A { # [doc = "0: CLR"] INT_EVENT1_IMASK_DIO15_CLR = 0 , # [doc = "1: SET"] INT_EVENT1_IMASK_DIO15_SET = 1 , } impl From < INT_EVENT1_IMASK_DIO15_A > for bool { # [inline (always)] fn from (variant : INT_EVENT1_IMASK_DIO15_A) -> Self { variant as u8 != 0 } } impl INT_EVENT1_IMASK_DIO15_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> INT_EVENT1_IMASK_DIO15_A { match self . bits { false => INT_EVENT1_IMASK_DIO15_A :: INT_EVENT1_IMASK_DIO15_CLR , true => INT_EVENT1_IMASK_DIO15_A :: INT_EVENT1_IMASK_DIO15_SET , } } # [doc = "CLR"] # [inline (always)] pub fn is_int_event1_imask_dio15_clr (& self) -> bool { * self == INT_EVENT1_IMASK_DIO15_A :: INT_EVENT1_IMASK_DIO15_CLR } # [doc = "SET"] # [inline (always)] pub fn is_int_event1_imask_dio15_set (& self) -> bool { * self == INT_EVENT1_IMASK_DIO15_A :: INT_EVENT1_IMASK_DIO15_SET } } # [doc = "Field `INT_EVENT1_IMASK_DIO15` writer - DIO15 event mask"] pub type INT_EVENT1_IMASK_DIO15_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT1_IMASK_DIO15_A > ; impl < 'a , REG , const O : u8 > INT_EVENT1_IMASK_DIO15_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "CLR"] # [inline (always)] pub fn int_event1_imask_dio15_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO15_A :: INT_EVENT1_IMASK_DIO15_CLR) } # [doc = "SET"] # [inline (always)] pub fn int_event1_imask_dio15_set (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT1_IMASK_DIO15_A :: INT_EVENT1_IMASK_DIO15_SET) } } impl R { # [doc = "Bit 0 - DIO0 event mask"] # [inline (always)] pub fn int_event1_imask_dio0 (& self) -> INT_EVENT1_IMASK_DIO0_R { INT_EVENT1_IMASK_DIO0_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - DIO1 event mask"] # [inline (always)] pub fn int_event1_imask_dio1 (& self) -> INT_EVENT1_IMASK_DIO1_R { INT_EVENT1_IMASK_DIO1_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - DIO2 event mask"] # [inline (always)] pub fn int_event1_imask_dio2 (& self) -> INT_EVENT1_IMASK_DIO2_R { INT_EVENT1_IMASK_DIO2_R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - DIO3 event mask"] # [inline (always)] pub fn int_event1_imask_dio3 (& self) -> INT_EVENT1_IMASK_DIO3_R { INT_EVENT1_IMASK_DIO3_R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - DIO4 event mask"] # [inline (always)] pub fn int_event1_imask_dio4 (& self) -> INT_EVENT1_IMASK_DIO4_R { INT_EVENT1_IMASK_DIO4_R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - DIO5 event mask"] # [inline (always)] pub fn int_event1_imask_dio5 (& self) -> INT_EVENT1_IMASK_DIO5_R { INT_EVENT1_IMASK_DIO5_R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - DIO6 event mask"] # [inline (always)] pub fn int_event1_imask_dio6 (& self) -> INT_EVENT1_IMASK_DIO6_R { INT_EVENT1_IMASK_DIO6_R :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 7 - DIO7 event mask"] # [inline (always)] pub fn int_event1_imask_dio7 (& self) -> INT_EVENT1_IMASK_DIO7_R { INT_EVENT1_IMASK_DIO7_R :: new (((self . bits >> 7) & 1) != 0) } # [doc = "Bit 8 - DIO8 event mask"] # [inline (always)] pub fn int_event1_imask_dio8 (& self) -> INT_EVENT1_IMASK_DIO8_R { INT_EVENT1_IMASK_DIO8_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - DIO9 event mask"] # [inline (always)] pub fn int_event1_imask_dio9 (& self) -> INT_EVENT1_IMASK_DIO9_R { INT_EVENT1_IMASK_DIO9_R :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - DIO10 event mask"] # [inline (always)] pub fn int_event1_imask_dio10 (& self) -> INT_EVENT1_IMASK_DIO10_R { INT_EVENT1_IMASK_DIO10_R :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - DIO11 event mask"] # [inline (always)] pub fn int_event1_imask_dio11 (& self) -> INT_EVENT1_IMASK_DIO11_R { INT_EVENT1_IMASK_DIO11_R :: new (((self . bits >> 11) & 1) != 0) } # [doc = "Bit 12 - DIO12 event mask"] # [inline (always)] pub fn int_event1_imask_dio12 (& self) -> INT_EVENT1_IMASK_DIO12_R { INT_EVENT1_IMASK_DIO12_R :: new (((self . bits >> 12) & 1) != 0) } # [doc = "Bit 13 - DIO13 event mask"] # [inline (always)] pub fn int_event1_imask_dio13 (& self) -> INT_EVENT1_IMASK_DIO13_R { INT_EVENT1_IMASK_DIO13_R :: new (((self . bits >> 13) & 1) != 0) } # [doc = "Bit 14 - DIO14 event mask"] # [inline (always)] pub fn int_event1_imask_dio14 (& self) -> INT_EVENT1_IMASK_DIO14_R { INT_EVENT1_IMASK_DIO14_R :: new (((self . bits >> 14) & 1) != 0) } # [doc = "Bit 15 - DIO15 event mask"] # [inline (always)] pub fn int_event1_imask_dio15 (& self) -> INT_EVENT1_IMASK_DIO15_R { INT_EVENT1_IMASK_DIO15_R :: new (((self . bits >> 15) & 1) != 0) } } impl W { # [doc = "Bit 0 - DIO0 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio0 (& mut self) -> INT_EVENT1_IMASK_DIO0_W < INT_EVENT1_IMASK_SPEC , 0 > { INT_EVENT1_IMASK_DIO0_W :: new (self) } # [doc = "Bit 1 - DIO1 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio1 (& mut self) -> INT_EVENT1_IMASK_DIO1_W < INT_EVENT1_IMASK_SPEC , 1 > { INT_EVENT1_IMASK_DIO1_W :: new (self) } # [doc = "Bit 2 - DIO2 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio2 (& mut self) -> INT_EVENT1_IMASK_DIO2_W < INT_EVENT1_IMASK_SPEC , 2 > { INT_EVENT1_IMASK_DIO2_W :: new (self) } # [doc = "Bit 3 - DIO3 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio3 (& mut self) -> INT_EVENT1_IMASK_DIO3_W < INT_EVENT1_IMASK_SPEC , 3 > { INT_EVENT1_IMASK_DIO3_W :: new (self) } # [doc = "Bit 4 - DIO4 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio4 (& mut self) -> INT_EVENT1_IMASK_DIO4_W < INT_EVENT1_IMASK_SPEC , 4 > { INT_EVENT1_IMASK_DIO4_W :: new (self) } # [doc = "Bit 5 - DIO5 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio5 (& mut self) -> INT_EVENT1_IMASK_DIO5_W < INT_EVENT1_IMASK_SPEC , 5 > { INT_EVENT1_IMASK_DIO5_W :: new (self) } # [doc = "Bit 6 - DIO6 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio6 (& mut self) -> INT_EVENT1_IMASK_DIO6_W < INT_EVENT1_IMASK_SPEC , 6 > { INT_EVENT1_IMASK_DIO6_W :: new (self) } # [doc = "Bit 7 - DIO7 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio7 (& mut self) -> INT_EVENT1_IMASK_DIO7_W < INT_EVENT1_IMASK_SPEC , 7 > { INT_EVENT1_IMASK_DIO7_W :: new (self) } # [doc = "Bit 8 - DIO8 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio8 (& mut self) -> INT_EVENT1_IMASK_DIO8_W < INT_EVENT1_IMASK_SPEC , 8 > { INT_EVENT1_IMASK_DIO8_W :: new (self) } # [doc = "Bit 9 - DIO9 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio9 (& mut self) -> INT_EVENT1_IMASK_DIO9_W < INT_EVENT1_IMASK_SPEC , 9 > { INT_EVENT1_IMASK_DIO9_W :: new (self) } # [doc = "Bit 10 - DIO10 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio10 (& mut self) -> INT_EVENT1_IMASK_DIO10_W < INT_EVENT1_IMASK_SPEC , 10 > { INT_EVENT1_IMASK_DIO10_W :: new (self) } # [doc = "Bit 11 - DIO11 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio11 (& mut self) -> INT_EVENT1_IMASK_DIO11_W < INT_EVENT1_IMASK_SPEC , 11 > { INT_EVENT1_IMASK_DIO11_W :: new (self) } # [doc = "Bit 12 - DIO12 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio12 (& mut self) -> INT_EVENT1_IMASK_DIO12_W < INT_EVENT1_IMASK_SPEC , 12 > { INT_EVENT1_IMASK_DIO12_W :: new (self) } # [doc = "Bit 13 - DIO13 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio13 (& mut self) -> INT_EVENT1_IMASK_DIO13_W < INT_EVENT1_IMASK_SPEC , 13 > { INT_EVENT1_IMASK_DIO13_W :: new (self) } # [doc = "Bit 14 - DIO14 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio14 (& mut self) -> INT_EVENT1_IMASK_DIO14_W < INT_EVENT1_IMASK_SPEC , 14 > { INT_EVENT1_IMASK_DIO14_W :: new (self) } # [doc = "Bit 15 - DIO15 event mask"] # [inline (always)] # [must_use] pub fn int_event1_imask_dio15 (& mut self) -> INT_EVENT1_IMASK_DIO15_W < INT_EVENT1_IMASK_SPEC , 15 > { INT_EVENT1_IMASK_DIO15_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_event1_imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event1_imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT1_IMASK_SPEC ; impl crate :: RegisterSpec for INT_EVENT1_IMASK_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`int_event1_imask::R`](R) reader structure"] impl crate :: Readable for INT_EVENT1_IMASK_SPEC { } # [doc = "`write(|w| ..)` method takes [`int_event1_imask::W`](W) writer structure"] impl crate :: Writable for INT_EVENT1_IMASK_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT1_IMASK to value 0"] impl crate :: Resettable for INT_EVENT1_IMASK_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }