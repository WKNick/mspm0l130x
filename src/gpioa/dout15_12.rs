# [doc = "Register `DOUT15_12` writer"] pub type W = crate :: W < DOUT15_12_SPEC > ; # [doc = "This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DOUT15_12_DIO12_AW { # [doc = "0: ZERO"] DOUT15_12_DIO12_ZERO = 0 , # [doc = "1: ONE"] DOUT15_12_DIO12_ONE = 1 , } impl From < DOUT15_12_DIO12_AW > for bool { # [inline (always)] fn from (variant : DOUT15_12_DIO12_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `DOUT15_12_DIO12` writer - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."] pub type DOUT15_12_DIO12_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , DOUT15_12_DIO12_AW > ; impl < 'a , REG , const O : u8 > DOUT15_12_DIO12_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ZERO"] # [inline (always)] pub fn dout15_12_dio12_zero (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT15_12_DIO12_AW :: DOUT15_12_DIO12_ZERO) } # [doc = "ONE"] # [inline (always)] pub fn dout15_12_dio12_one (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT15_12_DIO12_AW :: DOUT15_12_DIO12_ONE) } } # [doc = "This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DOUT15_12_DIO13_AW { # [doc = "0: ZERO"] DOUT15_12_DIO13_ZERO = 0 , # [doc = "1: ONE"] DOUT15_12_DIO13_ONE = 1 , } impl From < DOUT15_12_DIO13_AW > for bool { # [inline (always)] fn from (variant : DOUT15_12_DIO13_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `DOUT15_12_DIO13` writer - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."] pub type DOUT15_12_DIO13_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , DOUT15_12_DIO13_AW > ; impl < 'a , REG , const O : u8 > DOUT15_12_DIO13_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ZERO"] # [inline (always)] pub fn dout15_12_dio13_zero (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT15_12_DIO13_AW :: DOUT15_12_DIO13_ZERO) } # [doc = "ONE"] # [inline (always)] pub fn dout15_12_dio13_one (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT15_12_DIO13_AW :: DOUT15_12_DIO13_ONE) } } # [doc = "This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DOUT15_12_DIO14_AW { # [doc = "0: ZERO"] DOUT15_12_DIO14_ZERO = 0 , # [doc = "1: ONE"] DOUT15_12_DIO14_ONE = 1 , } impl From < DOUT15_12_DIO14_AW > for bool { # [inline (always)] fn from (variant : DOUT15_12_DIO14_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `DOUT15_12_DIO14` writer - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."] pub type DOUT15_12_DIO14_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , DOUT15_12_DIO14_AW > ; impl < 'a , REG , const O : u8 > DOUT15_12_DIO14_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ZERO"] # [inline (always)] pub fn dout15_12_dio14_zero (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT15_12_DIO14_AW :: DOUT15_12_DIO14_ZERO) } # [doc = "ONE"] # [inline (always)] pub fn dout15_12_dio14_one (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT15_12_DIO14_AW :: DOUT15_12_DIO14_ONE) } } # [doc = "This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DOUT15_12_DIO15_AW { # [doc = "0: ZERO"] DOUT15_12_DIO15_ZERO = 0 , # [doc = "1: ONE"] DOUT15_12_DIO15_ONE = 1 , } impl From < DOUT15_12_DIO15_AW > for bool { # [inline (always)] fn from (variant : DOUT15_12_DIO15_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `DOUT15_12_DIO15` writer - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."] pub type DOUT15_12_DIO15_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , DOUT15_12_DIO15_AW > ; impl < 'a , REG , const O : u8 > DOUT15_12_DIO15_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ZERO"] # [inline (always)] pub fn dout15_12_dio15_zero (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT15_12_DIO15_AW :: DOUT15_12_DIO15_ZERO) } # [doc = "ONE"] # [inline (always)] pub fn dout15_12_dio15_one (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT15_12_DIO15_AW :: DOUT15_12_DIO15_ONE) } } impl W { # [doc = "Bit 0 - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."] # [inline (always)] # [must_use] pub fn dout15_12_dio12 (& mut self) -> DOUT15_12_DIO12_W < DOUT15_12_SPEC , 0 > { DOUT15_12_DIO12_W :: new (self) } # [doc = "Bit 8 - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."] # [inline (always)] # [must_use] pub fn dout15_12_dio13 (& mut self) -> DOUT15_12_DIO13_W < DOUT15_12_SPEC , 8 > { DOUT15_12_DIO13_W :: new (self) } # [doc = "Bit 16 - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."] # [inline (always)] # [must_use] pub fn dout15_12_dio14 (& mut self) -> DOUT15_12_DIO14_W < DOUT15_12_SPEC , 16 > { DOUT15_12_DIO14_W :: new (self) } # [doc = "Bit 24 - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."] # [inline (always)] # [must_use] pub fn dout15_12_dio15 (& mut self) -> DOUT15_12_DIO15_W < DOUT15_12_SPEC , 24 > { DOUT15_12_DIO15_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Data output 15 to 12\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout15_12::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct DOUT15_12_SPEC ; impl crate :: RegisterSpec for DOUT15_12_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`dout15_12::W`](W) writer structure"] impl crate :: Writable for DOUT15_12_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets DOUT15_12 to value 0"] impl crate :: Resettable for DOUT15_12_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }