# [doc = "Register `GFCTL` reader"] pub type R = crate :: R < GFCTL_SPEC > ; # [doc = "Register `GFCTL` writer"] pub type W = crate :: W < GFCTL_SPEC > ; # [doc = "Field `GFCTL_AGFEN` reader - Analog Glitch Suppression Enable"] pub type GFCTL_AGFEN_R = crate :: BitReader < GFCTL_AGFEN_A > ; # [doc = "Analog Glitch Suppression Enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum GFCTL_AGFEN_A { # [doc = "0: DISABLE"] GFCTL_AGFEN_DISABLE = 0 , # [doc = "1: ENABLE"] GFCTL_AGFEN_ENABLE = 1 , } impl From < GFCTL_AGFEN_A > for bool { # [inline (always)] fn from (variant : GFCTL_AGFEN_A) -> Self { variant as u8 != 0 } } impl GFCTL_AGFEN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> GFCTL_AGFEN_A { match self . bits { false => GFCTL_AGFEN_A :: GFCTL_AGFEN_DISABLE , true => GFCTL_AGFEN_A :: GFCTL_AGFEN_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_gfctl_agfen_disable (& self) -> bool { * self == GFCTL_AGFEN_A :: GFCTL_AGFEN_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_gfctl_agfen_enable (& self) -> bool { * self == GFCTL_AGFEN_A :: GFCTL_AGFEN_ENABLE } } # [doc = "Field `GFCTL_AGFEN` writer - Analog Glitch Suppression Enable"] pub type GFCTL_AGFEN_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , GFCTL_AGFEN_A > ; impl < 'a , REG , const O : u8 > GFCTL_AGFEN_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn gfctl_agfen_disable (self) -> & 'a mut crate :: W < REG > { self . variant (GFCTL_AGFEN_A :: GFCTL_AGFEN_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn gfctl_agfen_enable (self) -> & 'a mut crate :: W < REG > { self . variant (GFCTL_AGFEN_A :: GFCTL_AGFEN_ENABLE) } } # [doc = "Field `GFCTL_AGFSEL` reader - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values. (ULP UART only)"] pub type GFCTL_AGFSEL_R = crate :: FieldReader < GFCTL_AGFSEL_A > ; # [doc = "Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values. (ULP UART only)\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum GFCTL_AGFSEL_A { # [doc = "0: AGLIT_5"] GFCTL_AGFSEL_AGLIT_5 = 0 , # [doc = "1: AGLIT_10"] GFCTL_AGFSEL_AGLIT_10 = 1 , # [doc = "2: AGLIT_25"] GFCTL_AGFSEL_AGLIT_25 = 2 , # [doc = "3: AGLIT_50"] GFCTL_AGFSEL_AGLIT_50 = 3 , } impl From < GFCTL_AGFSEL_A > for u8 { # [inline (always)] fn from (variant : GFCTL_AGFSEL_A) -> Self { variant as _ } } impl crate :: FieldSpec for GFCTL_AGFSEL_A { type Ux = u8 ; } impl GFCTL_AGFSEL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> GFCTL_AGFSEL_A { match self . bits { 0 => GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_5 , 1 => GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_10 , 2 => GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_25 , 3 => GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_50 , _ => unreachable ! () , } } # [doc = "AGLIT_5"] # [inline (always)] pub fn is_gfctl_agfsel_aglit_5 (& self) -> bool { * self == GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_5 } # [doc = "AGLIT_10"] # [inline (always)] pub fn is_gfctl_agfsel_aglit_10 (& self) -> bool { * self == GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_10 } # [doc = "AGLIT_25"] # [inline (always)] pub fn is_gfctl_agfsel_aglit_25 (& self) -> bool { * self == GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_25 } # [doc = "AGLIT_50"] # [inline (always)] pub fn is_gfctl_agfsel_aglit_50 (& self) -> bool { * self == GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_50 } } # [doc = "Field `GFCTL_AGFSEL` writer - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values. (ULP UART only)"] pub type GFCTL_AGFSEL_W < 'a , REG , const O : u8 > = crate :: FieldWriterSafe < 'a , REG , 2 , O , GFCTL_AGFSEL_A > ; impl < 'a , REG , const O : u8 > GFCTL_AGFSEL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "AGLIT_5"] # [inline (always)] pub fn gfctl_agfsel_aglit_5 (self) -> & 'a mut crate :: W < REG > { self . variant (GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_5) } # [doc = "AGLIT_10"] # [inline (always)] pub fn gfctl_agfsel_aglit_10 (self) -> & 'a mut crate :: W < REG > { self . variant (GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_10) } # [doc = "AGLIT_25"] # [inline (always)] pub fn gfctl_agfsel_aglit_25 (self) -> & 'a mut crate :: W < REG > { self . variant (GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_25) } # [doc = "AGLIT_50"] # [inline (always)] pub fn gfctl_agfsel_aglit_50 (self) -> & 'a mut crate :: W < REG > { self . variant (GFCTL_AGFSEL_A :: GFCTL_AGFSEL_AGLIT_50) } } impl R { # [doc = "Bit 8 - Analog Glitch Suppression Enable"] # [inline (always)] pub fn gfctl_agfen (& self) -> GFCTL_AGFEN_R { GFCTL_AGFEN_R :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bits 9:10 - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values. (ULP UART only)"] # [inline (always)] pub fn gfctl_agfsel (& self) -> GFCTL_AGFSEL_R { GFCTL_AGFSEL_R :: new (((self . bits >> 9) & 3) as u8) } } impl W { # [doc = "Bit 8 - Analog Glitch Suppression Enable"] # [inline (always)] # [must_use] pub fn gfctl_agfen (& mut self) -> GFCTL_AGFEN_W < GFCTL_SPEC , 8 > { GFCTL_AGFEN_W :: new (self) } # [doc = "Bits 9:10 - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values. (ULP UART only)"] # [inline (always)] # [must_use] pub fn gfctl_agfsel (& mut self) -> GFCTL_AGFSEL_W < GFCTL_SPEC , 9 > { GFCTL_AGFSEL_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "Glitch Filter Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gfctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gfctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct GFCTL_SPEC ; impl crate :: RegisterSpec for GFCTL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`gfctl::R`](R) reader structure"] impl crate :: Readable for GFCTL_SPEC { } # [doc = "`write(|w| ..)` method takes [`gfctl::W`](W) writer structure"] impl crate :: Writable for GFCTL_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets GFCTL to value 0"] impl crate :: Resettable for GFCTL_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }