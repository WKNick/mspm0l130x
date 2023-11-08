# [doc = "Register `INT_EVENT2_ICLR` writer"] pub type W = crate :: W < INT_EVENT2_ICLR_SPEC > ; # [doc = "DIO16 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO16_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO16_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO16_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO16_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO16_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO16` writer - DIO16 event"] pub type INT_EVENT2_ICLR_DIO16_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO16_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO16_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio16_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO16_AW :: INT_EVENT2_ICLR_DIO16_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio16_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO16_AW :: INT_EVENT2_ICLR_DIO16_CLR) } } # [doc = "DIO17 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO17_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO17_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO17_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO17_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO17_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO17` writer - DIO17 event"] pub type INT_EVENT2_ICLR_DIO17_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO17_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO17_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio17_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO17_AW :: INT_EVENT2_ICLR_DIO17_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio17_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO17_AW :: INT_EVENT2_ICLR_DIO17_CLR) } } # [doc = "DIO18 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO18_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO18_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO18_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO18_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO18_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO18` writer - DIO18 event"] pub type INT_EVENT2_ICLR_DIO18_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO18_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO18_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio18_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO18_AW :: INT_EVENT2_ICLR_DIO18_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio18_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO18_AW :: INT_EVENT2_ICLR_DIO18_CLR) } } # [doc = "DIO19 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO19_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO19_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO19_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO19_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO19_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO19` writer - DIO19 event"] pub type INT_EVENT2_ICLR_DIO19_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO19_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO19_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio19_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO19_AW :: INT_EVENT2_ICLR_DIO19_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio19_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO19_AW :: INT_EVENT2_ICLR_DIO19_CLR) } } # [doc = "DIO20 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO20_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO20_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO20_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO20_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO20_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO20` writer - DIO20 event"] pub type INT_EVENT2_ICLR_DIO20_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO20_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO20_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio20_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO20_AW :: INT_EVENT2_ICLR_DIO20_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio20_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO20_AW :: INT_EVENT2_ICLR_DIO20_CLR) } } # [doc = "DIO21 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO21_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO21_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO21_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO21_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO21_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO21` writer - DIO21 event"] pub type INT_EVENT2_ICLR_DIO21_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO21_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO21_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio21_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO21_AW :: INT_EVENT2_ICLR_DIO21_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio21_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO21_AW :: INT_EVENT2_ICLR_DIO21_CLR) } } # [doc = "DIO22 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO22_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO22_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO22_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO22_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO22_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO22` writer - DIO22 event"] pub type INT_EVENT2_ICLR_DIO22_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO22_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO22_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio22_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO22_AW :: INT_EVENT2_ICLR_DIO22_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio22_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO22_AW :: INT_EVENT2_ICLR_DIO22_CLR) } } # [doc = "DIO23 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO23_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO23_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO23_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO23_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO23_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO23` writer - DIO23 event"] pub type INT_EVENT2_ICLR_DIO23_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO23_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO23_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio23_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO23_AW :: INT_EVENT2_ICLR_DIO23_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio23_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO23_AW :: INT_EVENT2_ICLR_DIO23_CLR) } } # [doc = "DIO24 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO24_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO24_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO24_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO24_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO24_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO24` writer - DIO24 event"] pub type INT_EVENT2_ICLR_DIO24_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO24_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO24_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio24_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO24_AW :: INT_EVENT2_ICLR_DIO24_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio24_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO24_AW :: INT_EVENT2_ICLR_DIO24_CLR) } } # [doc = "DIO25 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO25_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO25_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO25_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO25_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO25_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO25` writer - DIO25 event"] pub type INT_EVENT2_ICLR_DIO25_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO25_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO25_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio25_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO25_AW :: INT_EVENT2_ICLR_DIO25_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio25_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO25_AW :: INT_EVENT2_ICLR_DIO25_CLR) } } # [doc = "DIO26 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO26_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO26_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO26_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO26_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO26_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO26` writer - DIO26 event"] pub type INT_EVENT2_ICLR_DIO26_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO26_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO26_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio26_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO26_AW :: INT_EVENT2_ICLR_DIO26_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio26_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO26_AW :: INT_EVENT2_ICLR_DIO26_CLR) } } # [doc = "DIO27 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO27_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO27_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO27_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO27_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO27_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO27` writer - DIO27 event"] pub type INT_EVENT2_ICLR_DIO27_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO27_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO27_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio27_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO27_AW :: INT_EVENT2_ICLR_DIO27_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio27_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO27_AW :: INT_EVENT2_ICLR_DIO27_CLR) } } # [doc = "DIO28 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO28_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO28_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO28_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO28_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO28_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO28` writer - DIO28 event"] pub type INT_EVENT2_ICLR_DIO28_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO28_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO28_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio28_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO28_AW :: INT_EVENT2_ICLR_DIO28_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio28_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO28_AW :: INT_EVENT2_ICLR_DIO28_CLR) } } # [doc = "DIO29 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO29_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO29_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO29_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO29_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO29_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO29` writer - DIO29 event"] pub type INT_EVENT2_ICLR_DIO29_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO29_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO29_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio29_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO29_AW :: INT_EVENT2_ICLR_DIO29_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio29_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO29_AW :: INT_EVENT2_ICLR_DIO29_CLR) } } # [doc = "DIO30 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO30_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO30_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO30_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO30_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO30_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO30` writer - DIO30 event"] pub type INT_EVENT2_ICLR_DIO30_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO30_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO30_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio30_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO30_AW :: INT_EVENT2_ICLR_DIO30_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio30_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO30_AW :: INT_EVENT2_ICLR_DIO30_CLR) } } # [doc = "DIO31 event\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum INT_EVENT2_ICLR_DIO31_AW { # [doc = "0: NO_EFFECT"] INT_EVENT2_ICLR_DIO31_NO_EFFECT = 0 , # [doc = "1: CLR"] INT_EVENT2_ICLR_DIO31_CLR = 1 , } impl From < INT_EVENT2_ICLR_DIO31_AW > for bool { # [inline (always)] fn from (variant : INT_EVENT2_ICLR_DIO31_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `INT_EVENT2_ICLR_DIO31` writer - DIO31 event"] pub type INT_EVENT2_ICLR_DIO31_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , INT_EVENT2_ICLR_DIO31_AW > ; impl < 'a , REG , const O : u8 > INT_EVENT2_ICLR_DIO31_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "NO_EFFECT"] # [inline (always)] pub fn int_event2_iclr_dio31_no_effect (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO31_AW :: INT_EVENT2_ICLR_DIO31_NO_EFFECT) } # [doc = "CLR"] # [inline (always)] pub fn int_event2_iclr_dio31_clr (self) -> & 'a mut crate :: W < REG > { self . variant (INT_EVENT2_ICLR_DIO31_AW :: INT_EVENT2_ICLR_DIO31_CLR) } } impl W { # [doc = "Bit 16 - DIO16 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio16 (& mut self) -> INT_EVENT2_ICLR_DIO16_W < INT_EVENT2_ICLR_SPEC , 16 > { INT_EVENT2_ICLR_DIO16_W :: new (self) } # [doc = "Bit 17 - DIO17 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio17 (& mut self) -> INT_EVENT2_ICLR_DIO17_W < INT_EVENT2_ICLR_SPEC , 17 > { INT_EVENT2_ICLR_DIO17_W :: new (self) } # [doc = "Bit 18 - DIO18 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio18 (& mut self) -> INT_EVENT2_ICLR_DIO18_W < INT_EVENT2_ICLR_SPEC , 18 > { INT_EVENT2_ICLR_DIO18_W :: new (self) } # [doc = "Bit 19 - DIO19 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio19 (& mut self) -> INT_EVENT2_ICLR_DIO19_W < INT_EVENT2_ICLR_SPEC , 19 > { INT_EVENT2_ICLR_DIO19_W :: new (self) } # [doc = "Bit 20 - DIO20 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio20 (& mut self) -> INT_EVENT2_ICLR_DIO20_W < INT_EVENT2_ICLR_SPEC , 20 > { INT_EVENT2_ICLR_DIO20_W :: new (self) } # [doc = "Bit 21 - DIO21 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio21 (& mut self) -> INT_EVENT2_ICLR_DIO21_W < INT_EVENT2_ICLR_SPEC , 21 > { INT_EVENT2_ICLR_DIO21_W :: new (self) } # [doc = "Bit 22 - DIO22 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio22 (& mut self) -> INT_EVENT2_ICLR_DIO22_W < INT_EVENT2_ICLR_SPEC , 22 > { INT_EVENT2_ICLR_DIO22_W :: new (self) } # [doc = "Bit 23 - DIO23 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio23 (& mut self) -> INT_EVENT2_ICLR_DIO23_W < INT_EVENT2_ICLR_SPEC , 23 > { INT_EVENT2_ICLR_DIO23_W :: new (self) } # [doc = "Bit 24 - DIO24 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio24 (& mut self) -> INT_EVENT2_ICLR_DIO24_W < INT_EVENT2_ICLR_SPEC , 24 > { INT_EVENT2_ICLR_DIO24_W :: new (self) } # [doc = "Bit 25 - DIO25 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio25 (& mut self) -> INT_EVENT2_ICLR_DIO25_W < INT_EVENT2_ICLR_SPEC , 25 > { INT_EVENT2_ICLR_DIO25_W :: new (self) } # [doc = "Bit 26 - DIO26 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio26 (& mut self) -> INT_EVENT2_ICLR_DIO26_W < INT_EVENT2_ICLR_SPEC , 26 > { INT_EVENT2_ICLR_DIO26_W :: new (self) } # [doc = "Bit 27 - DIO27 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio27 (& mut self) -> INT_EVENT2_ICLR_DIO27_W < INT_EVENT2_ICLR_SPEC , 27 > { INT_EVENT2_ICLR_DIO27_W :: new (self) } # [doc = "Bit 28 - DIO28 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio28 (& mut self) -> INT_EVENT2_ICLR_DIO28_W < INT_EVENT2_ICLR_SPEC , 28 > { INT_EVENT2_ICLR_DIO28_W :: new (self) } # [doc = "Bit 29 - DIO29 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio29 (& mut self) -> INT_EVENT2_ICLR_DIO29_W < INT_EVENT2_ICLR_SPEC , 29 > { INT_EVENT2_ICLR_DIO29_W :: new (self) } # [doc = "Bit 30 - DIO30 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio30 (& mut self) -> INT_EVENT2_ICLR_DIO30_W < INT_EVENT2_ICLR_SPEC , 30 > { INT_EVENT2_ICLR_DIO30_W :: new (self) } # [doc = "Bit 31 - DIO31 event"] # [inline (always)] # [must_use] pub fn int_event2_iclr_dio31 (& mut self) -> INT_EVENT2_ICLR_DIO31_W < INT_EVENT2_ICLR_SPEC , 31 > { INT_EVENT2_ICLR_DIO31_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_event2_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INT_EVENT2_ICLR_SPEC ; impl crate :: RegisterSpec for INT_EVENT2_ICLR_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`int_event2_iclr::W`](W) writer structure"] impl crate :: Writable for INT_EVENT2_ICLR_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INT_EVENT2_ICLR to value 0"] impl crate :: Resettable for INT_EVENT2_ICLR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }