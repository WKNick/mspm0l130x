# [doc = "Register `DOUT27_24` writer"] pub type W = crate :: W < DOUT27_24_SPEC > ; # [doc = "This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DOUT27_24_DIO24_AW { # [doc = "0: ZERO"] DOUT27_24_DIO24_ZERO = 0 , # [doc = "1: ONE"] DOUT27_24_DIO24_ONE = 1 , } impl From < DOUT27_24_DIO24_AW > for bool { # [inline (always)] fn from (variant : DOUT27_24_DIO24_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `DOUT27_24_DIO24` writer - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."] pub type DOUT27_24_DIO24_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , DOUT27_24_DIO24_AW > ; impl < 'a , REG , const O : u8 > DOUT27_24_DIO24_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ZERO"] # [inline (always)] pub fn dout27_24_dio24_zero (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT27_24_DIO24_AW :: DOUT27_24_DIO24_ZERO) } # [doc = "ONE"] # [inline (always)] pub fn dout27_24_dio24_one (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT27_24_DIO24_AW :: DOUT27_24_DIO24_ONE) } } # [doc = "This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DOUT27_24_DIO25_AW { # [doc = "0: ZERO"] DOUT27_24_DIO25_ZERO = 0 , # [doc = "1: ONE"] DOUT27_24_DIO25_ONE = 1 , } impl From < DOUT27_24_DIO25_AW > for bool { # [inline (always)] fn from (variant : DOUT27_24_DIO25_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `DOUT27_24_DIO25` writer - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."] pub type DOUT27_24_DIO25_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , DOUT27_24_DIO25_AW > ; impl < 'a , REG , const O : u8 > DOUT27_24_DIO25_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ZERO"] # [inline (always)] pub fn dout27_24_dio25_zero (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT27_24_DIO25_AW :: DOUT27_24_DIO25_ZERO) } # [doc = "ONE"] # [inline (always)] pub fn dout27_24_dio25_one (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT27_24_DIO25_AW :: DOUT27_24_DIO25_ONE) } } # [doc = "This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DOUT27_24_DIO26_AW { # [doc = "0: ZERO"] DOUT27_24_DIO26_ZERO = 0 , # [doc = "1: ONE"] DOUT27_24_DIO26_ONE = 1 , } impl From < DOUT27_24_DIO26_AW > for bool { # [inline (always)] fn from (variant : DOUT27_24_DIO26_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `DOUT27_24_DIO26` writer - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."] pub type DOUT27_24_DIO26_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , DOUT27_24_DIO26_AW > ; impl < 'a , REG , const O : u8 > DOUT27_24_DIO26_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ZERO"] # [inline (always)] pub fn dout27_24_dio26_zero (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT27_24_DIO26_AW :: DOUT27_24_DIO26_ZERO) } # [doc = "ONE"] # [inline (always)] pub fn dout27_24_dio26_one (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT27_24_DIO26_AW :: DOUT27_24_DIO26_ONE) } } # [doc = "This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum DOUT27_24_DIO27_AW { # [doc = "0: ZERO"] DOUT27_24_DIO27_ZERO = 0 , # [doc = "1: ONE"] DOUT27_24_DIO27_ONE = 1 , } impl From < DOUT27_24_DIO27_AW > for bool { # [inline (always)] fn from (variant : DOUT27_24_DIO27_AW) -> Self { variant as u8 != 0 } } # [doc = "Field `DOUT27_24_DIO27` writer - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."] pub type DOUT27_24_DIO27_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , DOUT27_24_DIO27_AW > ; impl < 'a , REG , const O : u8 > DOUT27_24_DIO27_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "ZERO"] # [inline (always)] pub fn dout27_24_dio27_zero (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT27_24_DIO27_AW :: DOUT27_24_DIO27_ZERO) } # [doc = "ONE"] # [inline (always)] pub fn dout27_24_dio27_one (self) -> & 'a mut crate :: W < REG > { self . variant (DOUT27_24_DIO27_AW :: DOUT27_24_DIO27_ONE) } } impl W { # [doc = "Bit 0 - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."] # [inline (always)] # [must_use] pub fn dout27_24_dio24 (& mut self) -> DOUT27_24_DIO24_W < DOUT27_24_SPEC , 0 > { DOUT27_24_DIO24_W :: new (self) } # [doc = "Bit 8 - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."] # [inline (always)] # [must_use] pub fn dout27_24_dio25 (& mut self) -> DOUT27_24_DIO25_W < DOUT27_24_SPEC , 8 > { DOUT27_24_DIO25_W :: new (self) } # [doc = "Bit 16 - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."] # [inline (always)] # [must_use] pub fn dout27_24_dio26 (& mut self) -> DOUT27_24_DIO26_W < DOUT27_24_SPEC , 16 > { DOUT27_24_DIO26_W :: new (self) } # [doc = "Bit 24 - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."] # [inline (always)] # [must_use] pub fn dout27_24_dio27 (& mut self) -> DOUT27_24_DIO27_W < DOUT27_24_SPEC , 24 > { DOUT27_24_DIO27_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Data output 27 to 24\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout27_24::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct DOUT27_24_SPEC ; impl crate :: RegisterSpec for DOUT27_24_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [`dout27_24::W`](W) writer structure"] impl crate :: Writable for DOUT27_24_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets DOUT27_24 to value 0"] impl crate :: Resettable for DOUT27_24_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }