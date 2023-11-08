# [doc = "Register `IRCTL` reader"] pub type R = crate :: R < IRCTL_SPEC > ; # [doc = "Register `IRCTL` writer"] pub type W = crate :: W < IRCTL_SPEC > ; # [doc = "Field `IRCTL_IREN` reader - IrDA encoder/decoder enable"] pub type IRCTL_IREN_R = crate :: BitReader < IRCTL_IREN_A > ; # [doc = "IrDA encoder/decoder enable\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IRCTL_IREN_A { # [doc = "0: DISABLE"] IRCTL_IREN_DISABLE = 0 , # [doc = "1: ENABLE"] IRCTL_IREN_ENABLE = 1 , } impl From < IRCTL_IREN_A > for bool { # [inline (always)] fn from (variant : IRCTL_IREN_A) -> Self { variant as u8 != 0 } } impl IRCTL_IREN_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IRCTL_IREN_A { match self . bits { false => IRCTL_IREN_A :: IRCTL_IREN_DISABLE , true => IRCTL_IREN_A :: IRCTL_IREN_ENABLE , } } # [doc = "DISABLE"] # [inline (always)] pub fn is_irctl_iren_disable (& self) -> bool { * self == IRCTL_IREN_A :: IRCTL_IREN_DISABLE } # [doc = "ENABLE"] # [inline (always)] pub fn is_irctl_iren_enable (& self) -> bool { * self == IRCTL_IREN_A :: IRCTL_IREN_ENABLE } } # [doc = "Field `IRCTL_IREN` writer - IrDA encoder/decoder enable"] pub type IRCTL_IREN_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IRCTL_IREN_A > ; impl < 'a , REG , const O : u8 > IRCTL_IREN_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "DISABLE"] # [inline (always)] pub fn irctl_iren_disable (self) -> & 'a mut crate :: W < REG > { self . variant (IRCTL_IREN_A :: IRCTL_IREN_DISABLE) } # [doc = "ENABLE"] # [inline (always)] pub fn irctl_iren_enable (self) -> & 'a mut crate :: W < REG > { self . variant (IRCTL_IREN_A :: IRCTL_IREN_ENABLE) } } # [doc = "Field `IRCTL_IRTXCLK` reader - IrDA transmit pulse clock select"] pub type IRCTL_IRTXCLK_R = crate :: BitReader < IRCTL_IRTXCLK_A > ; # [doc = "IrDA transmit pulse clock select\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IRCTL_IRTXCLK_A { # [doc = "0: BITCLK"] IRCTL_IRTXCLK_BITCLK = 0 , # [doc = "1: BRCLK"] IRCTL_IRTXCLK_BRCLK = 1 , } impl From < IRCTL_IRTXCLK_A > for bool { # [inline (always)] fn from (variant : IRCTL_IRTXCLK_A) -> Self { variant as u8 != 0 } } impl IRCTL_IRTXCLK_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IRCTL_IRTXCLK_A { match self . bits { false => IRCTL_IRTXCLK_A :: IRCTL_IRTXCLK_BITCLK , true => IRCTL_IRTXCLK_A :: IRCTL_IRTXCLK_BRCLK , } } # [doc = "BITCLK"] # [inline (always)] pub fn is_irctl_irtxclk_bitclk (& self) -> bool { * self == IRCTL_IRTXCLK_A :: IRCTL_IRTXCLK_BITCLK } # [doc = "BRCLK"] # [inline (always)] pub fn is_irctl_irtxclk_brclk (& self) -> bool { * self == IRCTL_IRTXCLK_A :: IRCTL_IRTXCLK_BRCLK } } # [doc = "Field `IRCTL_IRTXCLK` writer - IrDA transmit pulse clock select"] pub type IRCTL_IRTXCLK_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IRCTL_IRTXCLK_A > ; impl < 'a , REG , const O : u8 > IRCTL_IRTXCLK_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "BITCLK"] # [inline (always)] pub fn irctl_irtxclk_bitclk (self) -> & 'a mut crate :: W < REG > { self . variant (IRCTL_IRTXCLK_A :: IRCTL_IRTXCLK_BITCLK) } # [doc = "BRCLK"] # [inline (always)] pub fn irctl_irtxclk_brclk (self) -> & 'a mut crate :: W < REG > { self . variant (IRCTL_IRTXCLK_A :: IRCTL_IRTXCLK_BRCLK) } } # [doc = "Field `IRCTL_IRTXPL` reader - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\]
(IRTXCLK = functional clock of the UART)"] pub type IRCTL_IRTXPL_R = crate :: FieldReader ; # [doc = "Field `IRCTL_IRTXPL` writer - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\]
(IRTXCLK = functional clock of the UART)"] pub type IRCTL_IRTXPL_W < 'a , REG , const O : u8 > = crate :: FieldWriter < 'a , REG , 6 , O > ; # [doc = "Field `IRCTL_IRRXPL` reader - IrDA receive input UCAxRXD polarity"] pub type IRCTL_IRRXPL_R = crate :: BitReader < IRCTL_IRRXPL_A > ; # [doc = "IrDA receive input UCAxRXD polarity\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum IRCTL_IRRXPL_A { # [doc = "0: HIGHPULSE"] IRCTL_IRRXPL_HIGH = 0 , # [doc = "1: LOWPULSE"] IRCTL_IRRXPL_LOW = 1 , } impl From < IRCTL_IRRXPL_A > for bool { # [inline (always)] fn from (variant : IRCTL_IRRXPL_A) -> Self { variant as u8 != 0 } } impl IRCTL_IRRXPL_R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> IRCTL_IRRXPL_A { match self . bits { false => IRCTL_IRRXPL_A :: IRCTL_IRRXPL_HIGH , true => IRCTL_IRRXPL_A :: IRCTL_IRRXPL_LOW , } } # [doc = "HIGHPULSE"] # [inline (always)] pub fn is_irctl_irrxpl_high (& self) -> bool { * self == IRCTL_IRRXPL_A :: IRCTL_IRRXPL_HIGH } # [doc = "LOWPULSE"] # [inline (always)] pub fn is_irctl_irrxpl_low (& self) -> bool { * self == IRCTL_IRRXPL_A :: IRCTL_IRRXPL_LOW } } # [doc = "Field `IRCTL_IRRXPL` writer - IrDA receive input UCAxRXD polarity"] pub type IRCTL_IRRXPL_W < 'a , REG , const O : u8 > = crate :: BitWriter < 'a , REG , O , IRCTL_IRRXPL_A > ; impl < 'a , REG , const O : u8 > IRCTL_IRRXPL_W < 'a , REG , O > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "HIGHPULSE"] # [inline (always)] pub fn irctl_irrxpl_high (self) -> & 'a mut crate :: W < REG > { self . variant (IRCTL_IRRXPL_A :: IRCTL_IRRXPL_HIGH) } # [doc = "LOWPULSE"] # [inline (always)] pub fn irctl_irrxpl_low (self) -> & 'a mut crate :: W < REG > { self . variant (IRCTL_IRRXPL_A :: IRCTL_IRRXPL_LOW) } } impl R { # [doc = "Bit 0 - IrDA encoder/decoder enable"] # [inline (always)] pub fn irctl_iren (& self) -> IRCTL_IREN_R { IRCTL_IREN_R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - IrDA transmit pulse clock select"] # [inline (always)] pub fn irctl_irtxclk (& self) -> IRCTL_IRTXCLK_R { IRCTL_IRTXCLK_R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bits 2:7 - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\]
(IRTXCLK = functional clock of the UART)"] # [inline (always)] pub fn irctl_irtxpl (& self) -> IRCTL_IRTXPL_R { IRCTL_IRTXPL_R :: new (((self . bits >> 2) & 0x3f) as u8) } # [doc = "Bit 9 - IrDA receive input UCAxRXD polarity"] # [inline (always)] pub fn irctl_irrxpl (& self) -> IRCTL_IRRXPL_R { IRCTL_IRRXPL_R :: new (((self . bits >> 9) & 1) != 0) } } impl W { # [doc = "Bit 0 - IrDA encoder/decoder enable"] # [inline (always)] # [must_use] pub fn irctl_iren (& mut self) -> IRCTL_IREN_W < IRCTL_SPEC , 0 > { IRCTL_IREN_W :: new (self) } # [doc = "Bit 1 - IrDA transmit pulse clock select"] # [inline (always)] # [must_use] pub fn irctl_irtxclk (& mut self) -> IRCTL_IRTXCLK_W < IRCTL_SPEC , 1 > { IRCTL_IRTXCLK_W :: new (self) } # [doc = "Bits 2:7 - Transmit pulse length. Pulse length t(PULSE) = (IRTXPLx + 1) / \\[2 * f(IRTXCLK)\\]
(IRTXCLK = functional clock of the UART)"] # [inline (always)] # [must_use] pub fn irctl_irtxpl (& mut self) -> IRCTL_IRTXPL_W < IRCTL_SPEC , 2 > { IRCTL_IRTXPL_W :: new (self) } # [doc = "Bit 9 - IrDA receive input UCAxRXD polarity"] # [inline (always)] # [must_use] pub fn irctl_irrxpl (& mut self) -> IRCTL_IRRXPL_W < IRCTL_SPEC , 9 > { IRCTL_IRRXPL_W :: new (self) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . bits = bits ; self } } # [doc = "eUSCI_Ax IrDA Control Word Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct IRCTL_SPEC ; impl crate :: RegisterSpec for IRCTL_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [`irctl::R`](R) reader structure"] impl crate :: Readable for IRCTL_SPEC { } # [doc = "`write(|w| ..)` method takes [`irctl::W`](W) writer structure"] impl crate :: Writable for IRCTL_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets IRCTL to value 0"] impl crate :: Resettable for IRCTL_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }